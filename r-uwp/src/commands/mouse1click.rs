// Dependencies
use super::CommandError;

/// The arguments expected.
pub type Payload = ();

/// The ID of the command.
pub const ID: &str = "3";

/// Parses the arguments provided to the correct type.
/// This cannot fail.
pub fn parse() -> Result<Payload, CommandError> {
    Ok(())
}

/// The runner.
pub fn run(_payload: Payload) -> Result<Option<Vec<String>>, CommandError> {
    // Click the left mouse button
    let mouse_button = inputbot::MouseButton::LeftButton;
    mouse_button.press();
    mouse_button.release();

    // Return
    Ok(None)
}