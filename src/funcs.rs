use std::process::Command;

#[cfg(target_os = "macos")]
pub fn install() {
    use crate::constants::{set_install_result, INSTALL_SCRIPT_URL};

	let mut command = Command::new("osascript");
	command.arg("-e");
	command.arg(format!("sudo curl \"{}\" | bash", INSTALL_SCRIPT_URL));
	command.args(vec!["with", "administrator", "privileges"]);

	let output = command.output().unwrap();
	set_install_result(output.status);
}

#[cfg(target_os = "windows")]
pub fn install() {

}

#[cfg(target_os = "linux")]
pub fn install() {

}
