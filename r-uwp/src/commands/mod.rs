// Exports
pub mod mousemouserel;
pub mod mousemoveabs;
pub mod mouse1click;
pub mod setclipboard;

// Dependencies
use strum::{EnumString, Display};
use bytestring::ByteString;

/// A command response.
pub struct CommandResponse {
    /// The response id (job id).
    pub id: String,
    /// The response data.
    pub data: Vec<String>,
}
impl Into<ByteString> for CommandResponse {
    fn into(self) -> ByteString {
        format!("{}|{}", self.id, self.data.join("|")).into()
    }
}

/// The possible error codes.
#[derive(Debug, EnumString, Display)]
pub enum CommandError {
    #[strum(serialize = "0")]
    CouldNotFindCommand,
    #[strum(serialize = "1")]
    NotEnoughArguments,
    #[strum(serialize = "2")]
    InvalidArgument(u8, String),
    #[strum(serialize = "3")]
    SystemError(i32),
    #[strum(serialize = "4")]
    BadlyFormattedCommand,
}
impl Into<ByteString> for CommandError {
    fn into(self) -> ByteString {
        self.to_string().into()
    }
}

/// Holds all of the commands.
#[derive(r_uwp_derive::Command)]
pub enum Command {
    /// x
    SetClipboard(setclipboard::Payload),
    /// x, y
    MouseMoveRel(mousemouserel::Payload),
    /// x, y
    MouseMoveAbs(mousemoveabs::Payload),
    Mouse1Click(mouse1click::Payload),
}