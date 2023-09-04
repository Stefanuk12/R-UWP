# R-UWP

This program serves to fix UWP and to provide some additional fixes when it comes to exploits and their custom functions.

- This can be ran in the background as a service
  - Written in Rust takes up very little resources

## Usage

```
General and exploit fixes for UWP Roblox

Usage: r-uwp.exe [OPTIONS]

Options:
  -w, --ws           Enable the websocket server
  -p, --port <PORT>  The port the websocket server is attached to [default: 8080]
  -c, --clip-mouse   Clips the mouse to the window during right click
  -m, --mouse-tp     Attempts to teleport the mouse back after letting go of right click
  -t, --tp-crash     Attempts to fix teleport crashes
  -s, --silent       Supresses all messages
  -h, --help         Print help
  -V, --version      Print version
```
## Roblox fixes

- Fixes crashes on teleport[^1]
- Fixed mouse teleporting when letting go of right click
- Makes the cursor not able to escape the window

[^1]: not fully functional yet

## Exploit fixes

To get this to work, put the [client](./Client.lua) within your `autoexec` folder.

Then open `cmd` and run the following command:
```
CheckNetIsolation LoopbackExempt -a -n="ROBLOXCORPORATION.ROBLOX_55nm5eh3cm0pr"
```

- Custom `mousemove` functions
- Custom `setclipboard` function
- ~~Custom `decompiler` via [unluau](https://github.com/valencefun/unluau)[^2]~~ This likely wont be added anytime soon as unluau does not support Roblox bytecode

[^2]: this is not implemented yet