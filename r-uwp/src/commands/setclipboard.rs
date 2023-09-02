// Dependencies
use super::CommandError;
use clipboard_win::{set_clipboard, formats};

/// The arguments expected.
pub type Payload = String;

/// The ID of the command.
pub const ID: &str = "2";

/// Parses the arguments provided to the correct type.
/// 
/// This cannot fail.
pub fn parse(x: &str) -> Result<Payload, CommandError> {
    Ok(x.to_string())
}

/// The runner.
pub fn run(payload: Payload) -> Result<Option<Vec<String>>, CommandError> {
    // Set clipboard
    if let Err(e) = set_clipboard(formats::Unicode, payload) {
        return Err(CommandError::SystemError(e.raw_code()))
    };

    // Return
    Ok(None)
}