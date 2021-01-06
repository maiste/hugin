/// This Source Code Form is subject to the terms of the Mozilla Public
/// License, v. 2.0. If a copy of the MPL was not distributed with this
/// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use std::fs;
use std::path::Path;

// Import from crate
use crate::rest::get_config;

pub fn get_json_as_string(file: String) -> Option<String> {
    let repo_path = get_config();
    let path = Path::new(&repo_path).join(file);
    fs::read_to_string(path).ok()
}
