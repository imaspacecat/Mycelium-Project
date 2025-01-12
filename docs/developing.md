# Basic Development Setup
### Tool Installations
1. Install rustup from [rustup.rs](https://rustup.rs)
2. Install NodeJS from [nodejs.org](https://nodejs.org)
3. Install yarn from [yarnpkg.com](https://yarnpkg.com/getting-started/install)

### Setting Up Enoki for Development
1. Open Enoki or run `cd Enoki` in your terminal
2. Install node dependencies by using `yarn`
3. Install tauri-cli by using `cargo install tauri-cli`
4. Run `cargo tauri dev` in your terminal to start the app

### Building Enoki for Production
1. Open Enoki or run `cd Enoki` in your terminal
2. Install node dependencies by using `yarn`
3. Run `cargo tauri build` in your terminal to build the app
4. The built installer will be in the `src-tauri/target/release/bundle`

### Recommended Development Environment
OS:
- Windows 11
- Ubuntu 20.04 LTS
- MacOS Ventura

IDE:
- WebStorm, for frontend work
- CLion, for work on the rust core
- VSCode, general purpose & free

### Recommended Side Software
- [Conductor](https://github.com/Redrield/Conductor) for DS control on any OS