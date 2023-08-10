use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

use serde::Deserialize;

use crate::CompileOptions;
use crate::Mode;
use crate::cwd;
use crate::default_triple;

use super::installer::InstallerGenerator;

#[derive(Debug, Deserialize)]
pub struct Output {
    packages: Vec<Package>,
}

#[allow(unused)]
#[derive(Debug, Default, Deserialize)]
pub struct Package {
    name: String,
    source: Option<String>,
    manifest_path: String,
    dependencies: Vec<Dependency>,
    targets: Vec<Target>
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Dependency {
    name: String,
    source: Option<String>,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Target {
    kind: Vec<String>,
}

#[derive(Debug)]
pub struct Builder {
    compile_options: CompileOptions,
    current_manifest: String,
    current_package: Package,
    current_path: PathBuf,
    cmd: Command,
    triple: String,
}

impl Builder<> {
    pub fn default() -> Self {
        Self { 
            compile_options: CompileOptions::default(), 
            current_manifest: String::new(), 
            current_package: Package::default(), 
            current_path: PathBuf::new(),
            cmd: Command::new(""), 
            triple: String::new(),
        }
    }

    pub fn new(cmd: Command, compile_opts: &CompileOptions) -> Self {
        let mut builder = Builder::default();
        builder.compile_options = compile_opts.clone();
        
        // Manifest
        match &compile_opts.manifest_dir {
            Some(manifest_path) => {
                builder.current_manifest = manifest_path.to_string();
            },
            None => {
                let current_manifest_path = cwd().join("Cargo.toml");
                if current_manifest_path.exists() {
                    match current_manifest_path.to_str() {
                        Some(manifest_path) => {
                            builder.current_manifest = manifest_path.to_string();
                        },
                        None => {
                            panic!("failed to find Cargo.toml");
                        },
                    }
                    
                } else {
                    panic!("failed to find Cargo.toml")
                }
            },
        }

        let packages = cargo_metadata(&builder.current_manifest);

        match Self::current_package(&builder.current_manifest, packages) {
            Some(package) => builder.current_package = package,
            None => panic!(
                "Unable to find the corresponding Packge for the current Cargo.tomls"
            ),
        };
        builder.current_path = cwd();
        // Save the command
        builder.cmd = cmd;
        // triple
        match &compile_opts.target_triple {
            Some(triple) => {
                // TODO: check that triple is useable or not
                builder.triple = triple.to_string()
            },
            None => builder.triple = default_triple(),
        };

        builder
    }

    pub fn run(self) {
        let installer = InstallerGenerator::default();
        let current_package = self.current_package;

        // work directory for rust-installer
        let working_dir = self.current_path.join("target/dist/workspace");
        let work_dir = match working_dir.to_str() {
            Some(path) => {
                let _ = clear_or_create_directory(path);
                path
            },
            None => panic!("Error `work-dir` path is provided"),
        };

        // output directory for rust-installer
        let output_dir = self.current_path.join("target/dist/output");
        let output_dir  = match output_dir.to_str() {
            Some(path) => {
                let _ = clear_or_create_directory(path);
                path
            },
            None => panic!("Error `output-dir` path is provided"),
        };

        // image directory for rust-installer: this is the directory where the build artifacts should be
        let image_dir = match self.compile_options.release {
            true => {
                self.current_path.join("target").join(self.triple).join("release")
            },
            false => {
                self.current_path.join("target").join(self.triple).join("debug")
            },
        };

        let image_dir = match image_dir.to_str() {
            Some(path) => {
                path.to_string()
            },
            None => panic!("Error `image-dir` path is provided"),
        };
            
        execute_cli(self.cmd);

        installer
            .project_name("Rust".to_string())
            .component_name("bin".to_string())
            .package_name(current_package.name)
            .work_dir(work_dir.to_string())
            .output_dir(output_dir.to_string())
            .image_dir(image_dir)
            .generate();
    }

    fn current_package(current_manifest_path: &String, packages: Vec<Package>) -> Option<Package> {
        for package in packages {
            if package.manifest_path.eq(current_manifest_path) {
                return Some(package);
            }
        }
        None
    }
}

fn clear_or_create_directory(path: &str) -> Result<(), std::io::Error> {
    let path = Path::new(path);
    
    // 判断路径是否存在
    if path.exists() {
        // 判断路径是否为目录
        if path.is_dir() {
            // 清空目录中的内容
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                fs::remove_file(entry.path())?;
            }
        } else {
            // 路径存在但不是目录，抛出错误
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Path is not a directory",
            ));
        }
    } else {
        // 路径不存在，创建目录
        fs::create_dir_all(path)?;
    }
        
    Ok(())
}

// General options for cargo
pub fn bare_cargo() -> Command{
    let cargo = Command::new("cargo");
    cargo
}

pub fn cargo(
    compile_opts: &CompileOptions,
) -> Command {
    // Initialize the bare cargo
    let mut cargo = bare_cargo();
    let target_triple = match &compile_opts.target_triple {
        Some(triple) => {
            // TODO: check that triple is useable or not
            triple.to_string()
        },
        None => default_triple(),
    };

    match compile_opts.mode {
        Mode::Dist => {
            cargo.arg("build");
            cargo.arg("--target").arg(target_triple);

            if let Some(target_dir) = &compile_opts.output_dir {
                cargo.arg("--target-dir").arg(target_dir);
            }
            if let Some(manifest_path) = &compile_opts.manifest_dir {
                cargo.arg("--manifest-path").arg(manifest_path);
            }
            if compile_opts.release {
                cargo.arg("--release");
            }
        },
        Mode::Install => todo!(),
    }    cargo
}

/// Invokes `cargo metadata` to get package metadata of project
pub fn cargo_metadata(manifest_path: &String) -> Vec<Package> {
    let mut cargo = bare_cargo();
    cargo
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--no-deps")
        .arg("--manifest-path")
        .arg(manifest_path);
    let metadata_output = output(&mut cargo);
    let packages = match serde_json::from_str(&metadata_output) {
        Ok(Output{ packages, .. }) => { packages },
        Err(_) => panic!("failed to parse metadata output"),
    };
    packages
}

#[track_caller]
pub fn output(cmd: &mut Command) -> String {
    let output = match cmd.stderr(Stdio::inherit()).output() {
        Ok(status) => status,
        Err(err) => panic!("{}", format!("failed to execute command : {:?} \n error {}", cmd, err)),
    };

    if !output.status.success() {
        panic!("command did not execute successfully");
    }
    String::from_utf8(output.stdout).unwrap()
}

// Run commands with the standard output
pub fn execute_cli(mut cmd: Command) {
    let output = cmd
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .unwrap();

    if output.status.success() {
        let _result = String::from_utf8_lossy(&output.stdout);
        println!("Execute successfully");
    } else {
        let _error = String::from_utf8_lossy(&output.stderr);
        println!("Command failed");
    }
}