use std::{process::ExitStatus, sync::Mutex};

use lazy_static::lazy_static;

pub const LICENSE: &str = include_str!("./assets/mpl-2.0.txt");

#[cfg(target_os = "macos")]
pub const INSTALL_SCRIPT_URL: &str = "https://cpc.createchstudio.com/src/scripts/macos-install.sh";

lazy_static!{
	static ref INSTALL_RESULT: Mutex<Option<ExitStatus>> = Mutex::new(None);
}

pub fn set_install_result(result: ExitStatus) {
	*INSTALL_RESULT.lock().unwrap() = Some(result);
}

pub fn get_install_result() -> Option<ExitStatus> {
	INSTALL_RESULT.lock().unwrap().clone()
}
