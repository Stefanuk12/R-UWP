// Dependencies
use std::{time::Duration, collections::HashSet, path::PathBuf};
use inputbot::MouseButton;
use sysinfo::{System, SystemExt, ProcessExt, Pid};
use windows::Win32::{UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextA, GetWindowRect, ClipCursor, GetClipCursor}, Foundation::RECT};

/// All of the processes to look for.
const PROCESSES: [&str; 1] = ["Windows10Universal.exe"];

/// A macro for printing to the console only in debug mode.
macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::log::info!($($arg)*); })
}

/// Grabs the window text of the active window.
fn get_window_text() -> String {
    unsafe {
        // Grab the handle for the active window
        let handle = GetForegroundWindow();

        // Attempt to get the text
        let mut lpstring = [0u8; 256];
        GetWindowTextA(handle, &mut lpstring);

        // Convert to a string
        let full_lpstring = String::from_utf8_lossy(&lpstring);
        full_lpstring.trim_matches(char::from(0)).to_string()
    }
}

/// Checks if Roblox is active.
fn is_roblox_active() -> bool {
    // Check if the window text is "Roblox"
    get_window_text() == "Roblox"
}

/// Clips the mouse to active window.
fn clip_mouse() {
    // Grab the bounds of the window
    let mut rect = RECT::default();
    unsafe {
        if let Err(e) = GetWindowRect(GetForegroundWindow(), &mut rect as *mut RECT) {
            println!("An error occured while GetWindowRect: {}", e);
            return
        }
    }

    // Grab the current clip
    let mut current_clip = RECT::default();
    unsafe {
        if let Err(e) = GetClipCursor(&mut current_clip as *mut RECT) {
            println!("An error occured while GetClipCursor: {}", e);
            return
        };
    }

    // Check if the clip is already the same
    if rect == current_clip {
        return
    }

    // Confine our mouse to the window
    unsafe {
        if let Err(e) = ClipCursor(Some(&rect as *const RECT)) {
            println!("An error occured while ClipCursor: {}", e);
            return
        }
    }
    debug_println!("Confined mouse to window");
}

/// Unclips the mouse from the window
fn unclip_mouse() {
    unsafe {
        if let Err(e) = ClipCursor(None) {
            println!("An error occured while ClipCursor: {}", e);
            return
        }
    }
    debug_println!("Unconfined mouse from window");
}

/// A fix for the right click teleporting bug.
fn fix_right_click_tp() {
    MouseButton::RightButton.bind(move || {
        // Make sure we are in Roblox
        if !is_roblox_active() {
            return;
        }

        // Clip our mouse
        clip_mouse();

        // Save our mouse position
        let mouse_location = inputbot::MouseCursor::pos();

        // Wait to release right click
        while MouseButton::RightButton.is_pressed() {
            std::thread::sleep(Duration::from_millis(1));
        }

        // Mouse mouse
        inputbot::MouseCursor::move_abs(mouse_location.0, mouse_location.1);
        debug_println!("Teleported mouse back");

        // Unclip
        unclip_mouse();
    });
}

type Process = (String, PathBuf, Pid);
/// Grabs all of the processes
fn get_rbx_processes(sys: &mut System) -> Vec<Process> {
    // Refresh the processes
    sys.refresh_processes();

    // Grab the processes
    sys
        .processes()
        .into_iter()
        .map(|(_, process)| (process.name().to_string(), process.exe().to_owned(), process.pid()))
        .filter(|(name, path, _)| PROCESSES.contains(&name.as_str()) && path.to_str().unwrap_or("").contains("ROBLOXCORPORATION.ROBLOX_2"))
        .collect::<Vec<_>>()
}

/// Removes duplicate Roblox instances.
/// Usually occurs when teleporting to a new place.
fn remove_duplicate_instances() {
    // Vars
    let mut sys = System::new();
    let mut old_processes: Vec<Process> = Vec::new();
    let mut rbx_pids: HashSet<Pid> = HashSet::new();

    // Main loop
    loop {
        // So we don't use 100% CPU
        std::thread::sleep(Duration::from_millis(1));

        // Grab all of the changed processes
        let mut changed_processes = get_rbx_processes(&mut sys);
        
        // Check if empty
        if changed_processes.is_empty() {
            changed_processes = old_processes.clone();
        } else {
            changed_processes = changed_processes
                .into_iter()
                .filter(|x| !old_processes.contains(x))
                .collect::<Vec<_>>();
        }

        // Reset for the next iteration
        old_processes = get_rbx_processes(&mut sys);

        // Grab all of the Roblox / RuntimeBroker processes ** missing this, need to test if apart of the roblox process **
        changed_processes
            .into_iter()
            .for_each(|(_, _, pid)| {
                // Check if the process exists
                let Some(process) = sys.process(pid) else {
                    rbx_pids.remove(&pid);
                    debug_println!("Roblox has been closed");
                    return
                };

                // A new process, add it to the list
                if rbx_pids.insert(pid) {
                    debug_println!("Roblox has been launched");
                    return;
                }

                // Attempt to kill the process (already in the list)
                if !process.kill() {
                    println!("Failed to close Roblox");
                    return
                }
                
                // Success
                rbx_pids.remove(&pid);
                debug_println!("Successfully closed Roblox");
            });
    }
}

/// Starts the UWP fixes.
pub fn start_uwp() {
    // Fix mouse tp
    fix_right_click_tp();
    debug_println!("Bound RightClick");

    // Check if a new instance spawns
    std::thread::spawn(remove_duplicate_instances);

    // Start listening for mouse events, yields the current thread
    debug_println!("Listening for input events...");
    inputbot::handle_input_events();
}