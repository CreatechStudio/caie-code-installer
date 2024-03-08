use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

pub const LICENSE: &str = include_str!("./assets/mpl-2.0.txt");
pub const PYTHON : &[&str] = &[
    "pypy3",
    "pypy",
    "python",
    "python3",
];
pub const OS_TYPE: &str = std::env::consts::OS;

#[cfg(target_os = "macos")]
pub const INSTALL_SCRIPT: &str = include_str!("./scripts/macos-install.sh");

#[cfg(target_os = "linux")]
pub const INSTALL_SCRIPT: &str = include_str!("./scripts/linux-install.sh");

#[cfg(target_os = "windows")]
pub const INSTALL_SCRIPT: &str = include_str!("./scripts/install.bat");

lazy_static!{
	static ref INSTALL_RESULT: Mutex<Option<i32>> = Mutex::new(None);
}

pub fn set_install_result(result: Option<i32>) {
	*INSTALL_RESULT.lock().unwrap() = result;
}

pub fn get_install_result() -> Option<i32> {
	INSTALL_RESULT.lock().unwrap().clone()
}

pub fn get_desc_from_exit_code(code: i32) -> Option<String> {
	let mut exit_code_map = HashMap::new();
	exit_code_map.insert(1, "osacript error");
	exit_code_map.insert(2, "Git is unable to clone repo from remote");
	exit_code_map.insert(3, "Failed to set git config");
	exit_code_map.insert(4, "Failed to set repo permission");
	exit_code_map.insert(44, "User did not install git");
	exit_code_map.insert(45, "User did not install any python or it is not include in PATH");

	if let Some(v) = exit_code_map.get(&code) {
		Some(v.to_string())
	} else {
		None
	}
}
