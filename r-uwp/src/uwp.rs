// Dependencies
use std::time::Duration;
use inputbot::MouseButton;
use sysinfo::{System, SystemExt, ProcessExt, Pid};
use winsafe::{prelude::user_Hwnd, HWND, GetClipCursor, ClipCursor, RECT};

use crate::Args;

/// Grabs the window text of the active window.
fn get_window_text() -> String {
    // Grab the handle for the active window
    let handle = match HWND::GetForegroundWindow() {
        Some(handle) => handle,
        None => {
            log::error!("An error occured while GetForegroundWindow");
            return String::new()
        }
    };

    // Attempt to get the text
    match handle.GetWindowText() {
        Ok(text) => text,
        Err(e) => {
            log::error!("An error occured while GetWindowText: {}", e);
            String::new()
        }
    }
}

/// Checks if Roblox is active.
fn is_roblox_active() -> bool {
    // Check if the window text is "Roblox"
    get_window_text() == "Roblox"
}

/// Grab the current clip.
fn get_clip() -> Option<(RECT, RECT)> {
    // Grab the bounds of the window
    let handle = HWND::GetForegroundWindow().expect("failed to get handle");
    let rect = match handle.GetWindowRect() {
        Ok(rect) => rect,
        Err(e) => {
            log::error!("An error occured while GetWindowRect: {}", e);
            return None
        }
    };

    // Grab the current clip
    let current_clip = match GetClipCursor() {
        Ok(rect) => rect,
        Err(e) => {
            log::error!("An error occured while GetClipCursor: {}", e);
            return None
        }
    };

    // Return
    Some((rect, current_clip))
}

/// Check if the mouse is clipped to the window.
fn is_clipped() -> bool {
    // Grab the clip
    let (rect, current_clip) = match get_clip() {
        Some(clip) => clip,
        None => return false
    };

    // Return
    rect == current_clip
}

/// Clips the mouse to active window.
fn clip_mouse() {
    // Grab the clip
    let (rect, current_clip) = match get_clip() {
        Some(clip) => clip,
        None => return
    };

    // Check if the clip is already the same
    if rect == current_clip {
        return
    }

    // Confine our mouse to the window
    if let Err(e) = ClipCursor(Some(&rect)) {
        log::error!("An error occured while ClipCursor: {}", e);
        return
    };
    log::info!("Confined mouse to window");
}

/// Unclips the mouse from the window
fn unclip_mouse() {
    if let Err(e) = ClipCursor(None) {
        log::error!("An error occured while ClipCursor: {}", e);
        return
    }
    log::info!("Unconfined mouse from window");
}

/// A fix for the right click teleporting bug.
fn fix_right_click_tp(clip: bool) {
    MouseButton::RightButton.bind(move || {
        // Make sure we are in Roblox
        if !is_roblox_active() {
            return;
        }

        // Clip our mouse
        if clip {
            clip_mouse();
        }

        // Save our mouse position
        let mouse_location = inputbot::MouseCursor::pos();

        // Wait to release right click
        while MouseButton::RightButton.is_pressed() {
            std::thread::sleep(Duration::from_millis(1));
        }

        // Mouse mouse
        inputbot::MouseCursor::move_abs(mouse_location.0, mouse_location.1);
        log::info!("Teleported mouse back");

        // Unclip
        if clip {
            unclip_mouse();
        }
    });
}

/// Binds mouse to window during shift lock.
fn fix_shift_lock(key: u64) {
    // Vars
    let key: inputbot::KeybdKey = key.into();

    // Bind to the key
    key.bind(move || {
        // Make sure we are in Roblox
        if !is_roblox_active() {
            return;
        }

        // Clip
        if is_clipped() {
            unclip_mouse();
        } else {
            clip_mouse();
        }
    });
}

/// Grabs all of the store apps from `tasklist`.
#[derive(Debug, Clone)]
struct TaskListEntry {
    /// The name of the process.
    name: String,
    /// The PID of the process.
    pid: Pid,
    /// The package name of the process.
    package_name: String,
}
impl PartialEq for TaskListEntry {
    fn eq(&self, other: &Self) -> bool {
        self.pid == other.pid
    }
}

/// Grabs all of the store apps from `tasklist`.
/// Filters for Roblox packages only.
/// Only `RuntimeBroker.exe` are returned
fn get_rbx_processes() -> Vec<TaskListEntry> {
    // Grab the tasklist from `cmd`
    let tasklist = std::process::Command::new("cmd")
        .args(&["/C", "tasklist", "/apps"])
        .output()
        .expect("failed to execute process");
    let binding = String::from_utf8_lossy(&tasklist.stdout);
    let tasklist = binding.trim();

    // Loop through all of the lines, ignore the first couple
    tasklist
        .lines()
        .filter_map(|x| {
            // Grab the exe, pid, memory usage, and package name
            let split = x.split_whitespace().collect::<Vec<_>>();
            let (exe, pid, package_name) = (
                split.get(0)?,
                split.get(2)?.parse::<Pid>().ok()?,
                split.get(5)?,
            );

            // Check if the process is Roblox
            if !(package_name.contains("ROBLOXCORPORATION") && exe == &"RuntimeBroker.exe") {
                return None
            }

            // Return as object
            Some(TaskListEntry {
                name: exe.to_string(),
                pid,
                package_name: package_name.to_string(),
            })
        })
        .collect()
}

/// Removes duplicate Roblox instances.
/// Usually occurs when teleporting to a new place.
fn remove_duplicate_instances() {
    // Vars
    let sys = System::new();
    let mut old_processes: Vec<TaskListEntry> = Vec::new();

    // Main loop
    loop {
        // So we don't use 100% CPU
        std::thread::sleep(Duration::from_millis(1));

        // Grab all of the changed processes
        let mut changed_processes = get_rbx_processes();
        
        // Check if empty
        if changed_processes.is_empty() {
            changed_processes = old_processes.clone();
        } else {
            changed_processes.retain(|x| !old_processes.contains(x));
        }

        // Reset for the next iteration
        old_processes = get_rbx_processes();

        // Only retain duplicate RuntimeBroker processes with the same package name
        let changed_copy = changed_processes.clone();
        changed_processes.retain(|x| changed_copy.iter().filter(|y| y.package_name == x.package_name).count() > 1);

        // Loop through each duplicate process
        changed_processes
            .into_iter()
            .for_each(|x| {
                // Check if the process exists
                let pid = x.pid;
                let Some(process) = sys.process(pid) else {
                    log::info!("{} has been closed", x.name);
                    return
                };

                // Attempt to kill the process (already in the list)
                if !process.kill() {
                    println!("Failed to close {}", x.name);
                    return
                }
                
                // Success
                log::info!("Successfully closed {}", x.name);
            });
    }
}

/// Starts the UWP fixes.
pub fn start_uwp(args: Args) {
    // Fix mouse tp
    if !args.disable_mouse_tp {
        fix_right_click_tp(!args.disable_clip_mouse);
        log::info!("Bound RightClick");
    }

    // Shift lock
    if !args.disable_clip_shift {
        fix_shift_lock(args.shift_lock_key);
        log::info!("Bound ShiftLock");
    }

    // Check if a new instance spawns
    if !args.disable_tp_crash {
        std::thread::spawn(remove_duplicate_instances);
        log::info!("Spawned duplicate instance checker");
    }

    // Start listening for mouse events, yields the current thread
    log::info!("Listening for input events...");
    inputbot::handle_input_events();
}