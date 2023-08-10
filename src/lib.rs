use std::path::PathBuf;
use std::env;

pub mod cli;
pub mod util;

use anyhow::Context;
use clap::ArgMatches;

#[derive(Clone, Debug, Default)]
pub enum Mode {
    #[default]
    Dist,
    Install,
}

impl Mode {
    pub fn from_str(cmd: &str) -> Mode {
        match cmd {
            "dist" => Mode::Dist,
            "install" => Mode::Install,
            _ => panic!("Unknown mode, Run `cargo help` for more information"),
        }
    }
}

// The main configuration for the command completion
#[derive(Clone, Debug, Default)]
pub struct CompileOptions {
    // The mode to execute
    mode: Mode,
    // Build triple
    target_triple: Option<String>,
    // Output file path 
    output_dir: Option<String>,
    // Manifest file path. (path to Cargo.toml)
    manifest_dir: Option<String>,
    // using the release mode or debug mode
    release: bool,
}

pub trait ArgMatchesExt {
    fn target_triple(&self) -> Option<&str> {
        self._value_of("target")
    }

    fn output_dir(&self) -> Option<&str> {
        self._value_of("target-dir")
    }

    fn manifest_dir(&self) -> Option<&str> {
        self._value_of("manifest-path")
    }

    fn is_release(&self) -> bool {
        if !self.flag("release") {
            return false;
        }
        true
    }

    fn compile_options(&self, mode: Mode) -> CompileOptions {
        CompileOptions {
            mode: mode,
            target_triple: match self.target_triple() {
                Some(str) => Some(str.to_string()),
                None => None,
            },
            output_dir: match self.output_dir() {
                Some(str) => Some(str.to_string()),
                None => None,
            },
            manifest_dir: match self.manifest_dir() {
                Some(str) => Some(str.to_string()),
                None => None,
            },
            release: self.is_release(),
        }
    }

    fn flag(&self, name: &str) -> bool;

    fn _value_of(&self, name: &str) -> Option<&str>;
}

impl<'a> ArgMatchesExt for ArgMatches {
    fn flag(&self, name: &str) -> bool {
        ignore_unknown(self.try_get_one::<bool>(name))
            .copied()
            .unwrap_or(false)
    }

    fn _value_of(&self, name: &str) -> Option<&str> {
        ignore_unknown(self.try_get_one::<String>(name))
            .map(String::as_str)
    }
}

#[track_caller]
pub fn ignore_unknown<T: Default>(r: Result<T, clap::parser::MatchesError>) -> T {
    match r {
        Ok(t) => t,
        Err(clap::parser::MatchesError::UnknownArgument { .. }) => Default::default(),
        Err(e) => {
            panic!("Mismatch between definition and access: {}", e);
        }
    }
}

pub fn cwd() -> PathBuf {
    let cwd = env::current_dir()
        .with_context(|| "Couldn't get the current directory of the process")
        .unwrap();

    cwd
}

pub fn default_triple() -> String {
    const TARGET_FIELD: &str = "host: ";
    // Query rustc for defaults
    let output = std::process::Command::new("rustc")
        .arg("-vV")
        .output()
        .map_err(Box::new)
        .unwrap();
    // Decode stdout
    let stdout = std::str::from_utf8(&output.stdout).map_err(Box::new).unwrap();
    // Parse the default target from stdout.
    let host = stdout
        .lines()
        .find(|l| l.starts_with(TARGET_FIELD))
        .map(|l| &l[TARGET_FIELD.len()..])
        .unwrap()
        .to_owned();
    
    host
}

fn resolve_crate(krate: &String) -> Result<(String, String), anyhow::Error> {
    let mut name = String::new();
    let mut version = String::new();

    if let Some((k, v)) = krate.split_once('@') {
        if k.is_empty() {
            anyhow::bail!("missing crate name");
        }
        name = k.to_owned();
        version = v.to_owned();
    }
    Ok((name, version))
}