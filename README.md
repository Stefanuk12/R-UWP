# R-UWP

This program serves to fix UWP and to provide some additional fixes when it comes to exploits and their custom functions.

- This can be ran in the background as a service
  - Written in Rust takes up very little resources

## Roblox fixes

- Fixes crashes on teleport[^1]
- Fixed mouse teleporting when letting go of right click
- Makes the cursor not able to escape the window

[^1]: not fully functional yet

## Exploit fixes

To get this to work, put the [client](./Client.lua) within your `autoexec` folder.

Note: This does not work on Fluxus (the very exploit this is targeting) since they do not allow websocket connections to `localhost`

- Custom `mousemove` functions
- Custom `setclipboard` function
- ~~Custom `decompiler` via [unluau](https://github.com/valencefun/unluau)[^2]~~ This likely wont be added anytime soon as unluau does not support Roblox bytecode

[^2]: this is not implemented yet