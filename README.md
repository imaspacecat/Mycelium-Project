# Mycelium Project

A now named framework for logging across coprocessors, plugin extensions, & a front end for users with proper support for
user plugins without a need to rebuild.

To be built in rust & kotlin for the robot side, and rust & TS/TSX with Next & Tauri for the user interface.

## Basic Todos

- [X] Create rust NT handler
- [ ] Create basic UI
- [ ] Create basic plugin system
- [ ] Begin rust rio logger
- [X] Begin rust coprocessor logger
- [ ] Create logger JNIs
- [ ] Create Kotlin user side for logger
- [ ] Properly handle multithreaded asynchronous multi NT handlers T-T

## Plugins

- [ ] Network Tables UI plugin
- [ ] Line Graph plugin
- [ ] Dashboard plugin
- [ ] Conductor DS UI plugin
- [ ] Drive base plugin
- [ ] PID/LQR/Controller Tuner plugin
- [ ] OxConfig plugin
- [ ] Scouting Data plugin

## Developing
[Docs](docs/developing.md)

### Developers
- Autumn Ouellette
- Nick Sharp
- Bowan Foryt
