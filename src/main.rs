use rustup_configurator::target;
use std::{env::{self, set_current_dir},path::PathBuf,process::{Command, ExitStatus}};

fn set_workspace(location: PathBuf) {
    let cd_status = set_current_dir(location);
    if cd_status.is_err() {
        std::process::exit(3);
    }
}

fn execute_clippy() {
    let clippy_status = Command::new("cargo")
        .args(["clippy","--","-D","warnings"])
        .status()
        .expect("cargo-clippy failed?");
    if !clippy_status.success() {
        std::process::exit(2);
    }
}

fn with_emsdk() -> (String, String, String) {
    let try_read_env_emsdk = env::var("EMSDK").unwrap_or("/opt/emsdk".to_string()).to_owned();
    let with_env_emsdk = try_read_env_emsdk.as_str();
    dbg!(&with_env_emsdk);
    let with_env_emsdk_path = format!(
        "{with_env_emsdk}:{with_env_emsdk}/upstream/emscripten:{with_env_emsdk}/node/22.16.0_64bit/bin:/home/lma1980/.wasmtime/bin:/home/lma1980/.cargo/bin:/usr/lib64/ccache:/home/lma1980/.local/bin:/home/lma1980/bin:/usr/local/bin:/usr/bin:/usr/local/sbin:/usr/sbin:/opt/wasi-sdk"
    );
    dbg!(&with_env_emsdk_path);
    let with_env_emsdk_node = env::var("EMSDK_NODE").unwrap_or(format!("{with_env_emsdk}/node/22.16.0_64bit/bin/node")).to_owned();
    dbg!(&with_env_emsdk_node);
    ( with_env_emsdk.to_owned(), with_env_emsdk_path.to_owned(), with_env_emsdk_node.to_owned() )
    
}

fn execute_build_with_emsdk(target: &str, is_release: bool) -> ExitStatus {
    let (env_emsdk, env_emsdk_path, env_emsdk_node) = with_emsdk();
    let mut bind_cmd = Command::new("cargo");
    let cmd = &mut bind_cmd
        .env("PATH", &env_emsdk_path)
        .env("EMSDK", &env_emsdk)
        .env("EMSDK_NODE", &env_emsdk_node);
    if !is_release {
        *cmd = cmd.args(["build", "--target", target]);
    } else {
        *cmd = cmd.args(["build", "--target", target, "--release"]);
    }
    cmd.status().expect("Failed to run cargo build").to_owned()
}

fn execute_build_with_xwin(target: &str, is_release: bool) -> ExitStatus {
    let mut bind_cmd = Command::new("cargo");
    let cmd: &mut Command = &mut bind_cmd;
    if !is_release {
        cmd.args(["xwin", "build", "--target", target]);
    } else  {
        cmd.args(["xwin", "build", "--target", target, "--release"]);
    }
    cmd.status().expect("Failed to cross-build against msvc")
}

fn execute_build_default(target: &str, is_release: bool) -> ExitStatus {
    let mut bind_cmd = Command::new("cargo");
    let cmd: &mut Command = &mut bind_cmd;
    if !is_release {
        cmd.args(["build", "--target", target]);
    } else  {
        cmd.args(["build", "--target", target, "--release"]);
    }
    cmd.status().expect("Failed to cross-build against msvc")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Change directory to current user workspace folder. Currently hard-coded... 
    let home = env::var("HOME").unwrap_or(".".to_string()).to_owned();
    set_workspace(format!("{home}/workspace").into());
    execute_clippy();
    // Get all installed targets
    let installed_targets = target::installed()
        .expect("Failed to list installed targets");
    println!("Installed targets:");
    for target in installed_targets {
        let mut status;
        println!("=========\t{}\t========", &target.triple);
        match target.triple.as_str() {
            "wasm32-unknown-emscripten" => {
                status = execute_build_with_emsdk(target.triple.as_str(), false);
            }
            "x86_64-pc-windows-msvc" => {
                status = execute_build_with_xwin(target.triple.as_str(), false);
            }
            _ => {
                status = execute_build_default(target.triple.as_str(), false);
            }
        }
        if !status.success() {
            std::process::exit(1);
        }
        match target.triple.as_str() {
            "wasm32-unknown-emscripten" => {
                status = execute_build_with_emsdk(target.triple.as_str(), true);
            }
            "x86_64-pc-windows-msvc" => {
                status = execute_build_with_xwin(target.triple.as_str(), true);
            }
            _ => {
                status = execute_build_default(target.triple.as_str(), true);
            }
        }

        if !status.success() {
            std::process::exit(1);
        }
    }
    Ok(())
}
