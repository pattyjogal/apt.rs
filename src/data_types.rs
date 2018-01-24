use std::process::Command;

// I may or may not use these someday; for now they were just a test

/// An Aptitude Dependency
#[derive(Debug)]
pub struct Dependency {
    /// The name for the package in the repository
    pub package_name: String,
    /// The version code (could be a string of some semver number, or
    /// None if the user specified no version preference with a `"*"`
    pub version_code: Option<String>,
}

impl Dependency {
    pub fn new(package_name: String, version_code: String) -> Dependency {
        // If the specified version code is equal to "*", then specify None
        // for the internal version code.
        Dependency {
            package_name, // Side note: <- that's dirty and I love it :)
            version_code: match version_code.as_ref() {
                "*" => None,
                _   => Some(version_code),
            },
        }
    }

    pub fn install(&self) -> Vec<u8> {
        // Hey, I can have a little fun
        if cfg!(target_os = "windows") {
            panic!("Uh... I think you might be in the wong place.")
        }

        Command::new("sudo")
            .arg("apt")
            .arg("install")
            .arg(self.to_apt_syntax())
            .output()
            .expect("Process failed to execute")
            .stdout
    }

    pub fn to_apt_syntax(&self) -> String {
        match self.version_code {
            Some(ref v_code) => format!("{}={}", self.package_name, v_code),
            None         => self.package_name.to_string()
        }
    }
}
