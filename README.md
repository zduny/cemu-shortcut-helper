# Shortcut helper for Cemu
Create Windows shortcuts (with icons) to all games installed in Cemu with a single double-click.

## Donate
If you like this tool and would like to see more emulation related projects, please consider [buying me a coffee](https://ko-fi.com/zduny).

## Usage
Download the archive from releases tab, unpack its contents into your Cemu directory (make sure to not create a containing directory - 
`shortcut-helper.exe` has to be in the same directory as `Cemu.exe`).

Internet connection is required for program to work correctly - icons are downloaded from [GameFAQs](https://gamefaqs.gamespot.com/) website.

Now you have two choices:
 - starting `Create shortcuts - windowed mode.bat` will create shortcuts that will start game in windowed mode,
 - starting `Create shortcuts - fullscreen mode.bat` will create shortcuts that will start game in fullscreen mode.
 
Icons are downloaded into `icons` directory (it will be created in Cemu directory).

Created shortcuts will be placed in `shortcuts` directory (it will be created in Cemu directory).
Yoy may then move them to any location you like (for example: desktop).

Starting program again will overwrite existing files (icons and shortcuts in directories mentioned above).

## Development
If you plan to fork this project, to make debugging easier I recommend copying `Cemu.exe` and `settings.xml`
from your Cemu directory into project root - otherwise program will error out very early on `cargo run`.

Also, piece of advice: don't start resulting shortcuts as they will launch `Cemu.exe` and pollute your project directory.
For final testing it's better to just move `cemu-shortcut-helper.exe` from `target` to your Cemu directory and launch it there. 

## Disclaimer
This project is no way affiliated with either Cemu Emulator project or Cemu development team. 

## See also
[Cemu Emulator](https://cemu.info/)
