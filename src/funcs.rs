use std::process::Command;

use crate::constants::{set_install_result, INSTALL_SCRIPT_URL};

#[cfg(target_os = "macos")]
pub fn install() {
	let mut command = Command::new("curl");
	command.arg(INSTALL_SCRIPT_URL);
	command.args(vec!["|", "bash"]);

	let output = command.output().unwrap();
	set_install_result(output.status);
	println!("{:?}", output);
}

#[cfg(target_os = "windows")]
pub fn install() {

}

#[cfg(target_os = "linux")]
pub fn install() {

}
