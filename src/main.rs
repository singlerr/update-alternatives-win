extern crate core;

use std::io;
use clap::Parser;
mod registry_helper;
mod jdk;

/// Switch JDK in one command
/// Inspired by Linux tool, update-java-alternatives
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{

    #[arg(short, long)]
    list: bool,

    #[arg(short, long)]
    set: u8
}

fn main() -> io::Result<()>{

    Ok(())
}
#[cfg(test)]

mod tests{
    use winreg::enums::HKEY_LOCAL_MACHINE;
    use winreg::RegKey;
    use crate::jdk::{get_jdks, get_search_path_candidates};
    use crate::registry_helper::RegistryHelper;

    #[test]
    fn get_java_home(){
        let key = RegKey::predef(HKEY_LOCAL_MACHINE);
        let handle = key.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment").expect("Failed to get handle!");
        let helper = RegistryHelper::wrap(&handle);
        let value = helper.get_value("JAVA_HOME", true).expect("Failed to fetch JAVA_HOME env var!");

    }

    #[test]
    fn get_roaming_path(){
        unsafe {
            let p = get_jdks();
            println!("{:?}", &p);
        }
    }

}