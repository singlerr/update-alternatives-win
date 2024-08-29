use std::io;
use std::io::Error;
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;
use crate::registry_helper::RegistryHelper;

mod registry_helper;


fn main() -> io::Result<()>{
    let key = RegKey::predef(HKEY_LOCAL_MACHINE);
    let handle = key.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment")?;
    let helper = RegistryHelper::wrap(&handle);
    let value = helper.get_value("Path", true).unwrap();

    let tokens = value.split(";");
    Ok(())
}
