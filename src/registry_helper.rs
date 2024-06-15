use std::ffi::OsStr;
use std::io;
use lazy_static::lazy_static;
use winreg::RegKey;
use winreg::types::FromRegValue;
use regex::{Regex, Replacer};

lazy_static!{
    pub static ref PATTERN_INNER_VARIABLE:Regex = Regex::new(".*%([A-za-z-0-9]+)%.*").unwrap();
}
pub struct RegistryHelper<'h> {
    handle: &'h RegKey
}

impl <'h> RegistryHelper<'h> {

    pub fn wrap(reg_key: &'h RegKey) -> RegistryHelper{
        RegistryHelper{
            handle: reg_key
        }
    }

    fn get_value_recursively<N: AsRef<OsStr>>(&self, name: N) -> io::Result<String>{
        let variable_value: String = self.handle.get_value(name)?;
        if let Some(capture) = PATTERN_INNER_VARIABLE.captures(variable_value.as_str()){
            let (_, [var]) = capture.extract();
            let fetched_var = self.get_value(var, true)?;
            let var_name = format!("%{}%", var);
            let formatted_value = PATTERN_INNER_VARIABLE.replace_all(var_name.as_str(), fetched_var).to_string();
            return Ok(formatted_value)
        }

        Ok(variable_value)
    }

    pub fn get_value<N: AsRef<OsStr>>(&self, name: N, recursive: bool) -> io::Result<String>{
        if !recursive {
            let value:String = self.handle.get_value(name)?;
            Ok(value)
        } else {
            self.get_value_recursively(name)
        }
    }
}

