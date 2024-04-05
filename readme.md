# Melody Plus

Can be used on a Raspberry Pi (Zero 2 W), Windows, or in theory any OpenGL 2 supporting system.

Currently, it embeds the used refresh token for getting access tokens at compile time along with the client id and secret, so no binary releases at the moment.

## Compiling From Source
### Prerequisites
- A [Rust](https://www.rust-lang.org/tools/install) development environment
- A Spotify Premium Account
- A Spotify Developer Account

### Compiling
1. Clone this repo
2. Create the client.id and client.secret files in the src directory, these should contain the Spotify App Client ID and Client Secret you are going to be using
3. Generate a refresh token and place in a refresh.token file in the src directory
   - Currently, this has to be done before compiling, the long term plan is to have this be off device (Probably using PKCE and an intermediate server/web app, this would also negate needing to embed the client secret as well)
   - You can either generate this yourself on your build machine or use something like [this](https://spotify-refresh-token-generator.netlify.app/). 
   - You will need the user-read-playback-state scope.
4. Run `cargo build` this will compile for your current system
   - If cross compiling I recommend [Cross](https://github.com/cross-rs/cross)

### Target Systems
#### Raspberry Pi
Can be cross-compiled with the target "armv7-unknown-linux-gnueabihf", should be able to be compiled on device as this target is a Tier 2 Target with Host Tools, however it is probably better to compile on a more powerful machine
#### Windows
Has been tested to compile targeting "x86_64-pc-windows-msvc" on Windows 11.
Uses [winres](https://crates.io/crates/winres) to embed the icon for windows.


## Current Features
- Track Info (Title, Album, Artist, Current Progress)
- Dynamic colour based on Album cover
- Battery level indicator for use with the [WaveShare UPS Hat (B)](https://www.waveshare.com/wiki/UPS_HAT_(B))

## Hardware Version
Although this projects works fine as just a software application, it does not really offer much that the official spotify Mini-Player (as part of the Spotify Desktop/Web Client) does not.
Where this project really shines is being used on a dedicated hardware device. The hardware list below is for a battery operated version, as such the UPS hat and 18650 cells are optional if you are happy to have it tethered to a wall.

### Parts List
- [Raspberry Pi Zero 2 W](https://thepihut.com/products/raspberry-pi-zero-2)
- [Waveshare 5" HDMI LCD (H)](https://thepihut.com/products/5-capacitive-touchscreen-lcd-slimmed-down-version-800x480) ([wiki](https://www.waveshare.com/wiki/5inch_HDMI_LCD_(H)_V4))
- [Waveshare UPS hat (B)](https://thepihut.com/products/uninterruptible-power-supply-ups-hat-b-for-raspberry-pi?variant=41409963425987) ([wiki](https://www.waveshare.com/wiki/UPS_HAT_(B)))
- 2x 18650 cells
- USB Micro-B to USB Micro-B (USB OTG cable) - this is for touch support with the above display
- Mini-HDMI to HDMI cable
- Micro SD Card
### Setup
1. Install Raspberry Pi OS onto SD card ([Raspberry Pi Imager](https://www.raspberrypi.com/software/))
   - I have been using the Bullseye based Raspberry Pi OS Legacy (32 bit)
   - Ensure AutoLogin, and SSH Sever are enabled
   - Also follow the setup instructions for the display
   - It is worth setting up the Wi-Fi network in the installer ([Instructions](https://www.raspberrypi.com/documentation/computers/configuration.html#connect-to-a-wireless-network))
2. Build with the armv7-unknown-linux-gnueabihf target and copy onto the Pi, ensuring the execute permission is set. 
   - Add the INA219 feature flag when building if you have the UPS Hat from above
3. Create `~/.xinitrc` file and have it run the program
4. Edit `~/.profile` and add `[[ -z "$DISPLAY" && "$XDG_VTNR" -eq 1 ]] && exec startx` to automatically start x11 on login
5. Stop and disable the lightdm service
   - This will stop the standard desktop environment from starting and combined with steps 3&4 will cause the program to be the exclusive GUI program running

## Future Features
- WPS based Wi-Fi setup for systems without a keyboard
- QRCode based process for getting Spotify authentication
