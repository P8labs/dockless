use std::path::Path;
use std::process::Command;

fn main() {
    let portal_dir = Path::new("./portal/build");
    let console_index = portal_dir.join("index.html");

    if !console_index.exists() {
        println!("cargo:warning=Portal not found, building it now...");

        let portal_root = Path::new("./portal");

        let pnpm_check = Command::new("pnpm").arg("--version").output();

        if pnpm_check.is_err() {
            panic!(
                "pnpm is not installed. Please install pnpm and try again, or build the console manually: cd console && pnpm install && pnpm build"
            );
        }

        println!("cargo:warning=Running pnpm install...");
        let install_status = Command::new("pnpm")
            .arg("install")
            .arg("--frozen-lockfile")
            .current_dir(portal_root)
            .status()
            .expect("Failed to run pnpm install");

        if !install_status.success() {
            panic!(
                "Failed to install console dependencies. Try running: cd console && pnpm install"
            );
        }

        println!("cargo:warning=Running pnpm build...");
        let build_status = Command::new("pnpm")
            .arg("build")
            .current_dir(portal_root)
            .status()
            .expect("Failed to run pnpm build");

        if !build_status.success() {
            panic!("Failed to build portal. Try running: cd portal && pnpm build");
        }

        println!("cargo:warning=Portal built successfully!");
    }

    if !portal_dir.exists() {
        std::fs::create_dir_all(portal_dir).expect("Failed to create portal build directory");
    }

    println!("cargo:rerun-if-changed=./portal/src");
    println!("cargo:rerun-if-changed=./portal/package.json");
    println!("cargo:rerun-if-changed=./portal/build");
}
