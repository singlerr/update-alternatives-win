extern crate core;

use crate::jdk::JDK;
use clap::Parser;
use std::io;

mod jdk;
mod registry_helper;
mod user_env;

/// Switch JDK in one command
/// Inspired by Linux tool, update-java-alternatives
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    list: bool,

    #[arg(short, long)]
    set: u8,
}

fn main() -> io::Result<()> {
    Ok(())
}

fn print_jdk_list(jdk_list: &Vec<JDK>) {
    println!("\t{0: <10} | {1: <10} | {2: <10}", "index", "info", "path");
    for (index, jdk) in jdk_list.iter().enumerate() {
        println!(
            "\t{0: <10}  {1: <10}  {2: <10}",
            index,
            jdk.version,
            jdk.path.to_str().unwrap()
        );
    }
}
#[cfg(test)]
mod tests {
    use crate::jdk::get_jdks;
    use crate::print_jdk_list;
    use crate::registry_helper::RegistryHelper;
    use crate::user_env::{detect_current_jdk, get_path_vars, validate_env_path};
    use windows::core::h;
    use winreg::enums::HKEY_LOCAL_MACHINE;
    use winreg::RegKey;

    #[test]
    fn cli_jdk_list() {
        print_jdk_list(&get_jdks());
    }

    #[test]
    fn get_current_jdk() {
        println!(
            "{}",
            detect_current_jdk().expect("Failed to get current jdk!")
        );
    }

    #[test]
    fn get_java_home() {
        let key = RegKey::predef(HKEY_LOCAL_MACHINE);
        let handle = key
            .open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment")
            .expect("Failed to get handle!");
        let helper = RegistryHelper::wrap(&handle);
        let value = helper
            .get_value("JAVA_HOME", true)
            .expect("Failed to fetch JAVA_HOME env var!");
    }

    #[test]
    fn get_env_path() {
        let key = RegKey::predef(HKEY_LOCAL_MACHINE);
        let handle = key
            .open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment")
            .expect("Failed to get handle!");
        let helper = RegistryHelper::wrap(&handle);
        let value = helper
            .get_value("Path", true)
            .expect("Failed to fetch Path env var!");
        println!("{}", value);
    }

    #[test]
    fn get_env_path_list() {
        let key = RegKey::predef(HKEY_LOCAL_MACHINE);
        let handle = key
            .open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment")
            .expect("Failed to get handle!");
        let helper = RegistryHelper::wrap(&handle);
        println!("{:?}", get_path_vars(&helper, true).expect("Error!"));
    }

    #[test]
    fn validate_env() {
        let key = RegKey::predef(HKEY_LOCAL_MACHINE);
        let handle = key
            .open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment")
            .expect("Failed to get handle!");
        let helper = RegistryHelper::wrap(&handle);
        validate_env_path(&helper).expect("Error!");
    }

    #[test]
    fn get_roaming_path() {
        let p = get_jdks();
        println!("{:?}", &p);
    }
}
