// Dependencies
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, DataEnum, Data};

/// Automatically implements each command.
#[proc_macro_derive(Command)]
pub fn command_derive(input: TokenStream) -> TokenStream {
    // Parse the input
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    // Error out if we're not annotating an enum
    let data: DataEnum = match ast.data {
        Data::Enum(d) => d,
        _ => panic!("this macro only works on enums"),
    };

    // Holds all of the execute lines.
    let mut execute = Vec::new();
    
    // Holds all of the parse lines.
    let mut from = Vec::new();

    // Iterate through each variant
    for variant in data.variants {
        // Grab data about the variant
        let variant_name = &variant.ident;
        let variant_data = variant.fields;

        // Make sure we have only one variant_data
        if variant_data.len() != 1 {
            panic!("only specify one variant_data per enum variant");
        }

        // Grabbing the command itself
        let command = match variant_data {
            syn::Fields::Unnamed(x) => {
                match &x.unnamed.first().unwrap().ty {
                    syn::Type::Path(x) => {
                        x.path.segments.first().unwrap().ident.clone()
                    },
                    _ => panic!("only specify unnamed variant_data")
                }
            },
            _ => panic!("only specify unnamed variant_data")
        };

        // Get the documentation comment that holds the arguments
        let doc = match &variant.attrs.first().unwrap().meta {
            syn::Meta::NameValue(x) => {
                match &x.value {
                    syn::Expr::Lit(x) => {
                        match &x.lit {
                            syn::Lit::Str(y) => y.value().trim().to_string(),
                            _ => panic!("only specify string literal"),
                        }
                    },
                    _ => panic!("only specify string literal"),
                }
            },
            _ => panic!("only specify string literal")
        };
        let docs = doc.split(", ").map(|x| format_ident!("{}", x)).collect::<Vec<_>>();

        // Add each to the vectors
        execute.push(quote! {
            Self::#variant_name(x) => #command::run(x)
        });
        from.push(quote! {
            [#command::ID, #(#docs),*] => Ok(Self::#variant_name(#command::parse(#(#docs),*)?)),
            [#command::ID, ..] => Err(CommandError::NotEnoughArguments)
        })
    }

    // Add the catch-all
    from.push(quote!{ 
        _ => Err(CommandError::CouldNotFindCommand)
    });

    // Output
    quote! {
        impl #name {
            /// Runs the command.
            pub fn execute(self) -> Result<Option<Vec<String>>, CommandError> {
                match self {
                    #(#execute),*
                }
            }
        }

        impl ::std::str::FromStr for #name {
            type Err = CommandError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.split("|").collect::<Vec<&str>>().as_slice() {
                    #(#from),*
                }
            }
        }
    }.into()
}