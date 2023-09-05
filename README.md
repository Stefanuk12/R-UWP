# R-UWP

This program serves to fix UWP and to provide some additional fixes when it comes to exploits and their custom functions.

- This can be ran in the background as a service
  - Written in Rust takes up very little resources

## How to install and running

- Grab the latest release from [here](https://github.com/Stefanuk12/R-UWP/releases/latest), it should be the `.zip` file
- Extract the `.zip`
- Run the executable that you just extracted
  - If you wish, you can customise the settings. View [usage](#usage)

Note: You must use a third party application if you want to minimise to system tray or "hide" it.

## Usage

```
General and exploit fixes for UWP Roblox

Usage: r-uwp.exe [OPTIONS]

Options:
  -w, --disable-ws
          Disable the websocket server
  -p, --port <PORT>
          The port the websocket server is attached to [default: 8080]
  -k, --shift-lock-key <SHIFT_LOCK_KEY>
          The shift lock key to use. Provide the key id [default: 160]
  -c, --disable-clip-mouse
          Disables clipping the mouse to the window during right click
  -l, --disable-clip-shift
          Disables clipping the mouse to the window during shift lock
  -m, --disable-mouse-tp
          Disables the mouse teleport fix
  -t, --disable-tp-crash
          Disables the teleport crash fix
  -s, --silent
          Supresses all output messages
  -h, --help
          Print help
  -V, --version
          Print version
```
## Roblox fixes

- Fixes crashes on teleport[^1]
- Fixed mouse teleporting when letting go of right click
- Makes the cursor not able to escape the window

[^1]: not fully functional yet

## Exploit fixes

### Installation

- Make sure you have done [this](#how-to-install)
- Put the [client](./Client.lua) within your `autoexec` folder

### Features

- Custom `mousemove` functions
- Custom `setclipboard` function
- ~~Custom `decompiler` via [unluau](https://github.com/valencefun/unluau)[^2]~~ This likely wont be added anytime soon as unluau does not support Roblox bytecode

[^2]: this is not implemented yet

## Failed to install the loopback exemption / Custom functions not working

- Make sure you put the [client](./Client.lua) in autoexec
- Open the Commmand Prompt (`cmd`) and execute the following command and if successful, you will see `OK.`:
  ```
  CheckNetIsolation LoopbackExempt -a -n="ROBLOXCORPORATION.ROBLOX_55nm5eh3cm0pr"
  ```