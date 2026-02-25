# Workspace Builder (wbuilder)

An opiniated cross-compile helper for workspaces. Mainly concentrating on 
WASM32 (stable Rust toolchain). Started to also integrate partial support
for Windows binaries. 

The tool was written and tested against a relatively minimalistic Rocky 10
Linux minimal deployment. Linux dependencies to be provided once figured.

- EMSDK
- WASMSDK
- podman or docker is required for Windows MSVC target(s): using cargo-xwin.

## Milestone 1

Proof of concept.

- build: proof completed
- build release: proof completed
- test: partial, missing windows
- bench: not started
