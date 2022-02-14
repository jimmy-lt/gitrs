/* build.rs
 * ========
 *
 * Copying
 * -------
 *
 * Copyright (c) 2022 gitrs authors and contributors.
 *
 * This file is part of the *gitrs* project.
 *
 * gitrs is a free software project. You can redistribute it and/or modify it
 * following the terms of the MIT License.
 *
 * This software project is distributed *as is*, WITHOUT WARRANTY OF ANY KIND;
 * including but not limited to the WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
 * PARTICULAR PURPOSE and NONINFRINGEMENT.
 *
 * You should have received a copy of the MIT License along with *gitrs*. If
 * not, see <http://opensource.org/licenses/MIT>.
 */
use std::env;
use std::fmt;
use std::collections::HashMap;

/// Instruct Cargo to define the provided environment variable for the current
/// build.
macro_rules! cargo_env {
    ($var:expr) => ({
        println!("cargo:rustc-env={}", $var);
    });

    ($key:tt, $value:tt) => ({
        let var = EnvVar::
        println!("cargo:rustc-env={}={}", $key, $value);
    });
}

/// Name of the environment variable provided by Cargo to specify the CPU
/// pointer width.
const CARGO_CFG_TARGET_POINTER_WIDTH: &'static str = "CARGO_CFG_TARGET_POINTER_WIDTH";

/// The default prefix used in constructing the path to the different
/// installation directories.
const DEFAULT_PREFIX: &'static str = "/usr/local";

/// Name of the environment variable to a prefix used in constructing the
/// default values for other installation directories.
const PREFIX: &'static str = "PREFIX";
/// Name of the environment variable to a prefix used in constructing the
/// default values for executable installation directories.
const EXEC_PREFIX: &'static str = "EXEC_PREFIX";

/// Name of the environment variable to the directory for installing executable
/// programs that users can run.
const BINDIR: &'static str = "BINDIR";
/// Name of the environment variable to the directory for installing
/// idiosyncratic read-only architecture-independent data files for this
/// program.
const DATADIR: &'static str = "DATADIR";
/// Name of the environment variable to the root of the directory tree for
/// read-only architecture-independent data files.
const DATAROOTDIR: &'static str = "DATAROOTDIR";
/// Name of the environment variable to the directory for installing
/// documentation files (other than Info or Man) for this package.
const DOCDIR: &'static str = "DOCDIR";
/// Name of the environment variable to the directory for installing header
/// files to be included by user programs with the C ‘`#include`’ preprocessor
/// directive.
const INCLUDEDIR: &'static str = "INCLUDEDIR";
/// Name of the environment variable to the directory for installing the
/// Info files for this package.
const INFODIR: &'static str = "INFODIR";
/// Name of the environment variable to the directory for installing executable
/// programs to be run by other programs rather than by users.
const LIBEXECDIR: &'static str = "LIBEXECDIR";
/// Name of the environment variable to the directory for object files and
/// libraries of object code.
const LIBDIR: &'static str = "LIBDIR";
/// Name of the environment variable to the directory for installing data
/// files which the programs modify while they run, and that pertain to one
/// specific machine.
const LOCALSTATEDIR: &'static str = "LOCALSTATEDIR";
/// Name of the environment variable to the top-level directory for installing
/// the man pages (if any) for this package.
const MANDIR: &'static str = "MANDIR";
/// Name of the environment variable to the directory for installing data files
/// which the programs modify while they run, that pertain to one specific
/// machine, and which need not persist longer than the execution of the
/// program.
const RUNSTATEDIR: &'static str = "RUNSTATEDIR";
/// Name of the environment variable to the directory for installing executable
/// programs that can be run from the shell, but are only generally useful to
/// system administrators.
const SBINDIR: &'static str = "SBINDIR";
/// Name of the environment variable to the directory for installing
/// architecture-independent data files which the programs modify while they
/// run.
const SHAREDSTATEDIR: &'static str = "SHAREDSTATEDIR";
/// Name of the environment variable to the directory for installing read-only
/// data files that pertain to a single machine–that is to say, files for
/// configuring a host.
const SYSCONFDIR: &'static str = "SYSCONFDIR";

/// List of directories to be installed under the provided prefix.
const PREFIX_DIRS: [(&'static str, &'static str); 5] = [
    // The root of the directory tree for read-only architecture-independent
    // data files.
    (DATAROOTDIR, "/share"),
    // The directory for installing header files to be included by user programs
    // with the C ‘`#include`’ preprocessor directive.
    (INCLUDEDIR, "/include"),
    // The directory for installing data files which the programs modify while
    // they run, and that pertain to one specific machine.
    (LOCALSTATEDIR, "/var"),
    // The directory for installing architecture-independent data files which
    // the programs modify while they run.
    (SHAREDSTATEDIR, "/var/lib"),
    // The directory for installing read-only data files that pertain to a
    // single machine–that is to say, files for configuring a host.
    (SYSCONFDIR, "/etc"),
];

const DATA_DIRS: [(&'static str, &'static str); 3] = [
    // The directory for installing documentation files (other than info or man)
    // for this package.
    (DOCDIR, "/doc"),
    // The directory for installing the Info files for this package.
    (INFODIR, "/info"),
    // The top-level directory for installing the man pages (if any) for this
    // package.
    (MANDIR, "/man"),
];

/// List of directories to be installed under the executable prefix.
const EXEC_DIRS: [(&'static str, &'static str); 3] = [
    // The directory for installing executable programs that users can run.
    (BINDIR, "/bin"),
    // The directory for installing executable programs to be run by other
    // programs rather than by users.
    (LIBEXECDIR, "/libexec"),
    // The directory for installing executable programs that can be run from the
    // shell, but are only generally useful to system administrators.
    (SBINDIR, "/sbin"),
];

/// List of library directories which are CPU pointer size dependent.
const LIB_DIRS: [(&'static str, &'static str); 1] = [
    // The directory for object files and libraries of object code.
    (LIBDIR, "/lib"),
];

/// List of directories not impacted by the provided prefix.
const ROOT_DIRS: [(&'static str, &'static str); 1] = [
    // The directory for installing data files which the programs modify while
    // they run, that pertain to one specific machine, and which need not
    // persist longer than the execution of the program.
    (RUNSTATEDIR, "/run"),
];

/// Data structure to store the key name and value of an environment variable.
#[derive(Clone, Debug)]
struct EnvVar {
    /// The key name of the environment variable.
    pub key: String,
    /// The value of the environment variable.
    pub value: Option<String>,
}

impl EnvVar {
    /// Get the environment variable with provided `key` name.
    fn get(key: &str) -> Self {
        Self {
            key: String::from(key),
            value: env::var(key).ok(),
        }
    }

    /// Set the value of the environment variable when its current value is
    /// [`None`].
    fn or (mut self, value: &str) -> Self {
        if let Some(_) = self.value {
            self
        } else {
            self.value = Some(String::from(value));
            self
        }
    }

    /// Set the value of the environment variable when its current value is
    /// [`None`].
    fn or_from (mut self, other: &Self) -> Self {
        if let Some(_) = self.value {
            self
        } else {
            self.value = other.value.clone();
            self
        }
    }
}

impl fmt::Display for EnvVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(x) = &self.value {
            write!(f, "{}={}", self.key, *x)
        } else {
            write!(f, "{}=", self.key)
        }
    }
}

/// Installation directories should always be named by variables, so it is easy
/// to install in a nonstandard place. This function emit installation
/// directories environment variables so the built code can use them to define
/// the location of the various files under its management.
///
/// The directories are based on a standard Unix file system layout as
/// specified by the
/// [Filesystem Hierarchy Standard](https://refspecs.linuxfoundation.org/fhs.shtml).
///
/// Emitted variables follow the
/// [GNU Variables for Installation Directories](https://www.gnu.org/prep/standards/html_node/Directory-Variables.html)
/// guideline. If such environment variable is already existing in the build
/// environment its value is reissued otherwise, the default computed value is
/// provided.
fn install_dirs() {
    let mut dir: HashMap<&str, EnvVar> = HashMap::new();
    dir.insert(PREFIX, EnvVar::get(PREFIX).or(DEFAULT_PREFIX));
    dir.insert(
        EXEC_PREFIX,
        EnvVar::get(EXEC_PREFIX).or_from(&dir[PREFIX])
    );

    // The `/lib` folder will vary based on the pointer size. For example,
    // libraries targeting a 64-bit CPU will be installed under `/lib64`.
    let qual = match EnvVar::get(CARGO_CFG_TARGET_POINTER_WIDTH).value.unwrap() {
        x => if x.parse::<i32>().unwrap() >= 64 { x } else { String::from("") },
    };

    // Set prefix variables.
    cargo_env!(dir[PREFIX]);
    cargo_env!(dir[EXEC_PREFIX]);

    // Define root variables.
    for (k, v) in ROOT_DIRS.iter() {
        dir.insert(k, EnvVar::get(k).or(v));
        cargo_env!(dir[k]);
    }

    // Define prefix dependent variables.
    let prefix = dir[PREFIX].value.clone().unwrap();
    let exec_prefix = dir[EXEC_PREFIX].value.clone().unwrap();

    for (k, v) in PREFIX_DIRS.iter() {
        dir.insert(k, EnvVar::get(k).or(&format!("{}{}", prefix, v)));
        cargo_env!(dir[k]);
    }

    for (k, v) in EXEC_DIRS.iter() {
        cargo_env!(EnvVar::get(k).or(&format!("{}{}", exec_prefix, v)));
    }

    for (k, v) in LIB_DIRS.iter() {
        cargo_env!(EnvVar::get(k).or(&format!("{}{}{}", exec_prefix, v, qual)));
    }

    // `DATADIR` equals to `DATAROOTDIR` if not set already.
    dir.insert(DATADIR, EnvVar::get(DATADIR).or_from(&dir[DATAROOTDIR]));
    cargo_env!(dir[DATADIR]);

    // Define `DATAROOTDIR` dependent variables.
    let datarootdir = dir[DATAROOTDIR].value.clone().unwrap();
    for (k, v) in DATA_DIRS.iter() {
        dir.insert(k, EnvVar::get(k).or(&format!("{}{}", datarootdir, v)));
        cargo_env!(dir[k]);
    }
}

fn main() {
    install_dirs();
}
