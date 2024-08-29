use std::collections::{HashMap, VecDeque};
use std::ffi::{OsStr, OsString};
use std::io;
use lazy_static::lazy_static;
use winreg::RegKey;
use winreg::types::FromRegValue;
use regex::{Regex, Replacer};

lazy_static!{
    pub static ref PATTERN_INNER_VARIABLE:Regex = Regex::new("%([A-za-z-0-9]+)%").unwrap();
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

    fn get_value_recursively<N: AsRef<OsStr>>(&self, name: N) -> Option<String>{
        let mut lookups:HashMap<OsString, String> = HashMap::new();
        let mut queue:VecDeque<OsString> = VecDeque::new();

        let name = OsString::from(name.as_ref());
        queue.push_back(name.clone());
        while ! queue.is_empty() {
            let name = queue.pop_back()?;
            let mut value:String = if let Some(cache) = lookups.get(&name){
                String::from(cache)
            }else{
                self.handle.get_value(&name).ok()?
            };

            let val = value.as_str();
            if PATTERN_INNER_VARIABLE.is_match(val){
                for m in PATTERN_INNER_VARIABLE.find_iter(val) {
                    let key = OsString::from(m.as_str());
                    if ! lookups.contains_key(&key) {
                        queue.push_back(name.clone());
                        queue.push_back(key);
                    } else {
                        let pattern = format!("%{}%", key.to_str()?);
                        let cached = lookups.get(&key)?;
                        let new_var = value.replace(pattern.as_str(), cached);
                        lookups.insert(name.clone(), new_var);
                    }
                }
            } else {
                lookups.insert(name, value);
            }
        }

        lookups.get(&name).map(|t| { String::from(t) })
    }

    pub fn get_value<N: AsRef<OsStr>>(&self, name: N, recursive: bool) -> Option<String>{
        if !recursive {
            let value:String = self.handle.get_value(name).ok()?;
            Some(value)
        } else {
            self.get_value_recursively(name)
        }
    }
}

