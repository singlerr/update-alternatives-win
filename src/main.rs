use std::io;
mod registry_helper;


fn main() -> io::Result<()>{
    Ok(())
}
#[cfg(test)]

mod tests{
    use winreg::enums::HKEY_LOCAL_MACHINE;
    use winreg::RegKey;
    use crate::registry_helper::RegistryHelper;

    #[test]
    fn get_java_home(){
        let key = RegKey::predef(HKEY_LOCAL_MACHINE);
        let handle = key.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment").expect("Failed to get handle!");
        let helper = RegistryHelper::wrap(&handle);
        let value = helper.get_value("JAVA_HOME", true).expect("Failed to fetch JAVA_HOME env var!");
    }
}