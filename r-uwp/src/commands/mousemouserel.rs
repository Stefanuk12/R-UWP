// Dependencies
use super::CommandError;

/// The arguments expected.
pub type Payload = (i32, i32);

/// The ID of the command.
pub const ID: &str = "0";

/// Parses the arguments provided to the correct type.
pub fn parse(x: &str, y: &str) -> Result<Payload, CommandError> {
    Ok((
        x.parse::<i32>().map_err(|_| CommandError::InvalidArgument(0, x.to_owned()))?,
        y.parse::<i32>().map_err(|_| CommandError::InvalidArgument(1, y.to_owned()))?
    ))
}

/// The runner.
pub fn run(payload: Payload) -> Result<Option<Vec<String>>, CommandError> {
    // Move the mouse
    inputbot::MouseCursor::move_rel(payload.0, payload.1);

    // Return
    Ok(None)
}