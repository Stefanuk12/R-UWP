// Dependencies
use std::{time::Duration, collections::HashSet, path::PathBuf, sync::{Arc, Mutex}};
use inputbot::MouseButton;
use sysinfo::{System, SystemExt, ProcessExt, Pid};
use winsafe::{prelude::user_Hwnd, HWND, GetClipCursor, ClipCursor};

use crate::Args;

/// All of the processes to look for.
const PROCESSES: [&str; 1] = ["Windows10Universal.exe"];

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

/// Clips the mouse to active window.
fn clip_mouse() {
    // Grab the bounds of the window
    let handle = HWND::GetForegroundWindow().expect("failed to get handle");
    let rect = match handle.GetWindowRect() {
        Ok(rect) => rect,
        Err(e) => {
            log::error!("An error occured while GetWindowRect: {}", e);
            return
        }
    };

    // Grab the current clip
    let current_clip = match GetClipCursor() {
        Ok(rect) => rect,
        Err(e) => {
            log::error!("An error occured while GetClipCursor: {}", e);
            return
        }
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
    let toggled = Arc::new(Mutex::new(false));
    let key: inputbot::KeybdKey = key.into();

    // Bind to the key
    key.bind(move || {
        // Make sure we are in Roblox
        if !is_roblox_active() {
            return;
        }

        // Toggle
        let mut toggled = toggled.lock().unwrap();
        *toggled = !*toggled;

        // Clip
        if *toggled {
            clip_mouse();
        } else {
            unclip_mouse();
        }
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
        .filter_map(|(_, process)| {
            // Grab the name, path, and pid
            let (name, path, pid) = (process.name().to_string(), process.exe().to_owned(), process.pid());

            // Check if the process is Roblox
            if PROCESSES.contains(&name.as_str()) && path.to_str().unwrap_or("").contains("ROBLOXCORPORATION.ROBLOX_2") {
                Some((name, path, pid))
            } else {
                None
            }
        })
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
                    log::info!("Roblox has been closed");
                    return
                };

                // A new process, add it to the list
                if rbx_pids.insert(pid) {
                    log::info!("Roblox has been launched");
                    return;
                }

                // Attempt to kill the process (already in the list)
                if !process.kill() {
                    println!("Failed to close Roblox");
                    return
                }
                
                // Success
                rbx_pids.remove(&pid);
                log::info!("Successfully closed Roblox");
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