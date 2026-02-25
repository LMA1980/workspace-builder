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

## Current usage (m1)
Execute at the root of your workspace or from workspace-builder context.

| Hard-coded to exploit ~/workspace for now.

For continious development while editing your projects:
`cargo watch -x 'run --release --bin wbuilder'`

Or ad-hoc execution:
`cargo run --release --bin wbuilder`

Or using the tool if you placed it on your PATH:
`wbuilder`
