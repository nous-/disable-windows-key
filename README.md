## Disable Windows Key

This program disables the Windows key on the Windows operating system, useful for gaming or other scenarios where accidental Windows key presses are disruptive.

### Download Here

You can download the latest release from the [Releases page](https://github.com/nous-/disable-windows-key/releases/latest).

These binaries are provided directly from github by compiling the code from this repo directly.

### Usage

By default, the program disables both the left and right Windows keys. You can customize this behavior using flags:

- `--disable-left`: Disables only the left Windows key.
- `--disable-right`: Disables only the right Windows key.

So for example, `disable-windows-key.exe --disable-left` would disable the left windows key only.

### To run yourself

1. [Install rust](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup)
1. Navigate to this cloned repo in the terminal
1. `cargo build --release`
1. Look in target folder for output