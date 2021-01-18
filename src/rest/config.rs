/// This Source Code Form is subject to the terms of the Mozilla Public
/// License, v. 2.0. If a copy of the MPL was not distributed with this
/// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    static ref CONFIG_PATH: Mutex<String> = Mutex::new(String::from("content"));
}

pub fn get_config() -> String {
    CONFIG_PATH.lock().unwrap().to_string()
}

pub fn set_config(new_path: &String) -> Option<()> {
    if Path::new(&new_path).exists() {
        let mut path = CONFIG_PATH.lock().expect("Cannot unwrap config mutex");
        *path = String::from(new_path);
        Some(())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use std::env;

    #[test]
    fn test_get_config() {
        assert_eq!("content", super::get_config());
    }

    #[test]
    fn test_wrong_set_config() {
        let new_path = String::from("path");
        let old_path = super::get_config();
        match super::set_config(&new_path) {
            None => assert!(true),
            Some(()) => assert!(false),
        };
        assert_eq!(old_path, super::get_config());
    }

    #[test]
    fn test_good_set_config() {
        let new_path = match env::current_dir().unwrap().to_str() {
            Some(str) => String::from(str),
            None => String::from("failed path"),
        };
        match super::set_config(&new_path) {
            None => assert!(false),
            Some(()) => assert!(true),
        };
        assert_eq!(new_path, super::get_config());
    }
}
