// Store user settings in somewhere(or just fetch JAVA_HOME to detect?)
// Or using "where java" or system env?

use crate::jdk::{get_jdks, JDK};
use crate::registry_helper::RegistryHelper;
use regex::Regex;
use std::ffi::OsString;
use std::fmt::format;
use std::path::PathBuf;
use std::process::ExitStatus;

const JAVA_HOME: &'static str = "_JAVA_HOME_";
pub fn detect_current_jdk() -> std::io::Result<String> {
    if !cfg!(windows) {
        panic!("This is only for Windows!");
    }

    let out = std::process::Command::new("where")
        .args(["java"])
        .output()?;

    if !out.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            String::from_utf8(out.stderr).unwrap(),
        ));
    }

    Ok(String::from_utf8(out.stdout)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?)
}

pub fn set_java_home(handle: &RegistryHelper, jdk: &JDK) -> std::io::Result<()> {
    handle.set_value(JAVA_HOME, jdk.path.as_os_str())
}

pub fn get_path_vars(
    handle: &RegistryHelper,
    leave_variable: bool,
) -> std::io::Result<Vec<String>> {
    let path_vars = handle.get_value("Path", !leave_variable)?;
    let vars: Vec<String> = path_vars.split(";").map(|s| String::from(s)).collect();
    Ok(vars)
}

/// Check for _JAVA_HOME_ is set in System Environment Variables
/// If it is correct, then returns Ok(None)
/// else, its value was not found or jdk version does not match with actual working
/// returns Ok(Some), which consists of env value to set
pub fn validate_java_home(handle: &RegistryHelper) -> std::io::Result<Option<String>> {
    let java_home = handle.get_value(JAVA_HOME, false);

    // There is env var assigned already
    if let Ok(java_home) = java_home {
        let jdk_path = format!("{}\\bin\\java.exe", java_home);
        let actual_path = detect_current_jdk()?;

        // Actual JDK does not match with env
        if jdk_path != actual_path {
            // Tell the caller the need of correcting java home
            return Ok(Some(java_home));
        }

        return Ok(None);
    }

    // No java home due to either first install or env var removed in some reason
    // We need to assign new val
    // just return jdk path, with policy that sets jdk found in last
    let mut jdks = get_jdks();
    let jdk = jdks.pop().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Any JDK not installed",
    ))?;
    let jdk = jdk.path.to_str().unwrap();

    Ok(Some(String::from(jdk)))
}

pub fn validate_env_path(handle: &RegistryHelper) -> std::io::Result<Option<String>> {
    let var = format!("%{}%\\bin", JAVA_HOME);
    let path_vars = get_path_vars(handle, true)?;

    for (index, path) in path_vars.iter().enumerate() {
        if path.starts_with(&var) {
            let mut result = String::new();
            result.push_str(var.clone().as_str());
            result.push_str(";");
            for (i, p) in path_vars.iter().enumerate() {
                if index == i {
                    continue;
                }
                &result.push_str(p.as_str());
                &result.push_str(";");
            }
            result.remove(result.len() - 1); // remove last ;
                                             // If there is index pointing _JAVA_HOME_, just take its priority to the highest
                                             // and it may not be safe to put env variable, return new value so that caller can handle it
                                             // handle.set_value("Path", new_path_var)?;
            return Ok(Some(result));
        }
    }

    // There isn't env variable, let's create one
    let var = format!("{};{}", &var, path_vars.join(";").as_str());
    Ok(Some(var))
}

fn get_jdk_root(path: &String) -> Option<PathBuf> {
    let mut path = PathBuf::from(path);
    while path.is_dir() && path.file_name()? == "bin" {
        path.pop();
    }

    if !path.pop() {
        return None;
    }

    Some(path)
}
