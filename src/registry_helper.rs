use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::ffi::{OsStr, OsString};
use std::io;
use std::io::ErrorKind;
use windows::core::{h, s, w, Param};
use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{SendMessageA, SendMessageTimeoutA, HWND_BROADCAST, WM_SETTINGCHANGE};
use winreg::types::ToRegValue;
use winreg::RegKey;

lazy_static! {
    pub static ref PATTERN_INNER_VARIABLE: Regex = Regex::new("%([A-za-z-0-9]+)%").unwrap();
}
pub struct RegistryHelper<'h> {
    handle: &'h RegKey,
}

impl<'h> RegistryHelper<'h> {
    pub fn wrap(reg_key: &'h RegKey) -> RegistryHelper {
        RegistryHelper { handle: reg_key }
    }

    pub fn set_value<N: AsRef<OsStr>, T: ToRegValue>(&self, name: N, value: T) -> io::Result<()> {
        self.handle.set_value(name, &value)?;
        Ok(())
    }

    fn get_value_recursively<N: AsRef<OsStr>>(&self, name: N) -> io::Result<String> {
        let mut lookups: HashMap<OsString, String> = HashMap::new();
        let mut queue: VecDeque<OsString> = VecDeque::new();

        let name = OsString::from(name.as_ref());
        queue.push_back(name.clone());
        while !queue.is_empty() {
            let name = queue.pop_back().unwrap();
            let mut value: String = if let Some(cache) = lookups.get(&name) {
                String::from(cache)
            } else {
                match self.handle.get_value(&name) {
                    Ok(val) => val,
                    Err(_) => name.clone().into_string().unwrap(),
                }
            };

            let val = value.as_str();
            if PATTERN_INNER_VARIABLE.is_match(val) {
                for (i, m) in PATTERN_INNER_VARIABLE.find_iter(val).enumerate() {
                    let match_val = &m.as_str()[1..m.len() - 1];
                    let key = OsString::from(match_val);
                    if !lookups.contains_key(&key) {
                        queue.push_back(name.clone());
                        queue.push_back(key);
                    } else {
                        let pattern = format!("%{}%", key.to_str().unwrap());
                        let cached = lookups.get(&key).unwrap();
                        let new_var = value.replace(pattern.as_str(), cached);
                        lookups.insert(name.clone(), new_var);
                    }
                }
            } else {
                lookups.insert(name, value);
            }
        }
        match lookups.get(&name).map(|t| String::from(t)) {
            None => Err(io::Error::from(ErrorKind::NotFound)),
            Some(val) => Ok(val),
        }
    }

    pub fn get_value<N: AsRef<OsStr>>(&self, name: N, recursive: bool) -> io::Result<String> {
        if !recursive {
            let value: String = self.handle.get_value(name)?;
            Ok(String::from(&value[1..(value.len()-1)]))
        } else {
            self.get_value_recursively(name)
        }
    }
}
