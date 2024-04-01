# Melody Plus

Can be used on a Raspberry Pi (Zero 2 W), Windows, or in theory any OpenGL 2 supporting system.

Currently, it embeds the used refresh token for getting access tokens at compile time along with the client id and secret, so no binary releases at the moment.

## Compiling From Source
### Raspberry Pi
Can be cross-compiled with the target "armv7-unknown-linux-gnueabihf", should be able to be compiled on device as this target is a Tier 2 Target with Host Tools, however it is probably better to compile on a more powerful machine
### Windows
Has been tested to compile targeting "x86_64-pc-windows-msvc" on Windows 11.
Uses [winres](https://crates.io/crates/winres) to embed the icon for windows.


## Future Features
- WPS based WiFi setup for systems without a keyboard
- QRCode based process for getting Spotify authentication
- Battery level indicator for use with the [WaveShare UPS Hat (B)](https://www.waveshare.com/wiki/UPS_HAT_(B))