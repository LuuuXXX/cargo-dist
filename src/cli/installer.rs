use std::process::Command;

pub struct RustInstaller {
    project_name: String,
    rel_manifest_dir: String,
    legacy_manifest_dirs: String,
}

pub fn generate_installer(config: &RustInstaller) {
    let installer_output_name = "install.sh";

    // Initialize the installation shell for the given config
    let mut install_generator = Command::new("sh");
    install_generator
        .arg("rust-installer/gen-installer.sh")
        .arg("--project-name")
        .arg(&config.project_name)
        .arg("--rel-manifest-dir")
        .arg(&config.rel_manifest_dir)
        .arg("--success-message=RustInstall-Generator-Success")
        .arg("--output-script")
        .arg(installer_output_name)
        .arg("legacy-manifest-dirs")
        .arg(&config.legacy_manifest_dirs);

    run_generator(&mut install_generator);
}

pub fn run_generator(cmd: &mut Command) {
    cmd
        .spawn()
        .expect("failed to run installer generator"); 
}