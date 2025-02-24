# ip-msg

A very much WIP peer-to-peer chat app. Written for fun and learning purposes.

## Running

To build and run you need to have both [Deno](https://docs.deno.com/runtime/getting_started/installation/) and the [Rust toolchain](https://www.rust-lang.org/tools/install) installed.

> Before running the launch commands, it might be neccesary to run ``deno install`` in the root project directory.

### Development

To build and run during development, you can use the ```deno task tauri dev`` command in the root directory to run the app with support for hot reload.

### Production

To build the app in production mode, you can use the ```deno task tauri build``` command. The output will be an .msi and NSIS .exe installer, you can use either one.
