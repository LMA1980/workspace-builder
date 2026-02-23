use rustup_configurator::target;
use std::{env,process::Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get all installed targets
    let installed_targets = target::installed()
        .expect("Failed to list installed targets");
    let clippy_status = Command::new("cargo")
        .args(["clippy","--","-D","warnings"])
        .status()
        .expect("cargo-clippy failed?");
    if !clippy_status.success() {
        std::process::exit(2);
    }
    println!("Installed targets:");
    let try_read_env_emsdk = env::var("ENV_EMSDK").unwrap_or("/opt/emsdk".to_string()).to_owned();
    let env_emsdk = try_read_env_emsdk.as_str();
    let env_emsdk_path = format!(
        "{env_emsdk}:{env_emsdk}/upstream/emscripten:{env_emsdk}/node/22.16.0_64bit/bin:/home/lma1980/.wasmtime/bin:/home/lma1980/.cargo/bin:/usr/lib64/ccache:/home/lma1980/.local/bin:/home/lma1980/bin:/usr/local/bin:/usr/bin:/usr/local/sbin:/usr/sbin:/opt/wasi-sdk"
    );
    let env_emsdk_node = format!("{env_emsdk}/node/22.16.0_64bit/bin/node");
    dbg!(&env_emsdk);
    dbg!(&env_emsdk_path);
    dbg!(&env_emsdk_node);
    
    for target in installed_targets {
        let mut status;
        println!("=========\t{}\t========", &target.triple);
        match target.triple.as_str() {
            "wasm32-unknown-emscripten" => {
                status = Command::new("cargo")
                    .env("PATH", &env_emsdk_path)
                    .env("EMSDK", env_emsdk)
                    .env("EMSDK_NODE", &env_emsdk_node)
                    .args(["build", "--target", target.triple.as_str()])
                    .status()
                    .expect("Failed to run cargo build");
            }
            "x86_64-pc-windows-msvc" => {
                status = Command::new("cargo")
                    .args(["xwin", "build", "--target", target.triple.as_str()])
                    .status()
                    .expect("Failed to cross-build against msvc");
            }
            _ => {
                status = Command::new("cargo")
                    .args(["build", "--target", target.triple.as_str()])
                    .status()
                    .expect("Failed to run cargo build");
            }
        }
        if !status.success() {
            std::process::exit(1);
        }
        match target.triple.as_str() {
            "wasm32-unknown-emscripten" => {
                status = Command::new("cargo")
                    .env("PATH","/opt/emsdk:/opt/emsdk/upstream/emscripten:/opt/emsdk/node/22.16.0_64bit/bin:/home/lma1980/.wasmtime/bin:/home/lma1980/.cargo/bin:/usr/lib64/ccache:/home/lma1980/.local/bin:/home/lma1980/bin:/usr/local/bin:/usr/bin:/usr/local/sbin:/usr/sbin:/opt/wasi-sdk")
                    .env("EMSDK","/opt/emsdk")
                    .env("EMSDK_NODE","/opt/emsdk/node/22.16.0_64bit/bin/node")
                    .args(["build", "--target", target.triple.as_str(),"--release"])
                    .status()
                    .expect("Failed to run cargo build");
            }
            "x86_64-pc-windows-msvc" => {
                status = Command::new("cargo")
                    .args([
                        "xwin",
                        "build",
                        "--target",
                        target.triple.as_str(),
                        "--release",
                    ])
                    .status()
                    .expect("Failed to cross-build against msvc");
            }
            _ => {
                status = Command::new("cargo")
                    .args(["build", "--target", target.triple.as_str(), "--release"])
                    .status()
                    .expect("Failed to run cargo build");
            }
        }

        if !status.success() {
            std::process::exit(1);
        }
    }
    Ok(())
}
