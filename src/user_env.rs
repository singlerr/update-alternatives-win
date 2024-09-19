// Store user settings in somewhere(or just fetch JAVA_HOME to detect?)
// Or using "where java" or system env?

use std::ffi::OsString;
use std::fmt::format;
use std::path::PathBuf;
use std::process::ExitStatus;
use regex::Regex;
use windows::core::h;
use crate::jdk::JDK;
use crate::registry_helper::RegistryHelper;

const JAVA_HOME: &'static str = "_JAVA_HOME_";
pub fn detect_current_jdk() -> std::io::Result<String>{
    if ! cfg!(windows){
        panic!("This is only for Windows!");
    }

    let out = std::process::Command::new("where")
        .args(["java"])
        .output()?;

    if ! out.status.success(){
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, String::from_utf8(out.stderr).unwrap()));
    }

    Ok(String::from_utf8(out.stdout).map_err(|e| { std::io::Error::new(std::io::ErrorKind::Other, e.to_string()) })?)
}

fn set_java_home(handle: &RegistryHelper,jdk: &JDK) -> std::io::Result<()>{
    handle.set_value(JAVA_HOME, jdk.path.as_os_str())
}

pub fn get_path_vars(handle: &RegistryHelper, leave_variable: bool) -> std::io::Result<Vec<String>>{
    let path_vars = handle.get_value("Path", ! leave_variable)?;
    let vars:Vec<String> = path_vars.split(";").map(|s| { String::from(s)}).collect();
    Ok(vars)
}

fn validate_java_home(handle: &RegistryHelper) -> std::io::Result<()>{

}

pub fn validate_env_path(handle: &RegistryHelper) -> std::io::Result<()>{
    let var = format!("%{}%\\bin", JAVA_HOME);
    let path_vars = get_path_vars(handle, true)?;

    for (index, path) in path_vars.iter().enumerate() {
        if path.starts_with(&var){
            let mut result = String::new();
            result.push_str(";");
            for (i, p) in path_vars.iter().enumerate() {
                if i == index{
                    continue;
                }
                &result.push_str(p.as_str());
            }
            // If there is index pointing _JAVA_HOME_, just take its priority to the highest
            let new_path_var = format!("{0};{1}", &var, &result);
            println!("testr: {}", new_path_var);
            // handle.set_value("Path", new_path_var)?;
            return Ok(())
        }
    }

    // There isn't env variable, let's create one

    Ok(())
}

fn get_jdk_root(path: &String) -> Option<PathBuf>{
    let mut path = PathBuf::from(path);
    while path.is_dir() && path.file_name()? == "bin" {
        path.pop();
    }

    if ! path.pop(){
        return None
    }

    Some(path)
}