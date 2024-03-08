use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::path::{Path, PathBuf};

use crate::constants::set_install_result;
use crate::constants::INSTALL_SCRIPT;
use crate::constants::PYTHON;

fn save_tmp_file(content: &str) -> PathBuf {
	let p = Path::new("/tmp/macos-install.sh");
	let mut f = File::create(p).unwrap();
	f.write(content.as_bytes()).unwrap();
	p.to_path_buf()
}

#[cfg(target_os = "macos")]
pub fn check_dependencies() -> bool {
	// Check if Xcode Command Line Tools is installed
	let git_check = Command::new("git").arg("--version").output().unwrap();
	let git_result = String::from_utf8(git_check.stdout).unwrap();
	if !git_result.contains("git version") {
		set_install_result(Some(44));
		return false;
	}
	// Check if commands in PYTHON can be run
	let mut python_check_flag = false;
	for py in PYTHON {
		if Command::new(py).arg("--version").output().is_ok() {
			python_check_flag = true;
			break;
		}
	}

	if !python_check_flag {
		set_install_result(Some(45));
		return false;
	}

	true
}

#[cfg(target_os = "windows")]
pub fn check_dependencies() {
	use std::process::ExitCode;
	// Check if Xcode Command Line Tools is installed
	let git_check = Command::new("git").arg("--version").output().unwrap();
	let git_result = String::from_utf8(git_check.stdout).unwrap();
	if !git_result.contains("git version") {
		set_install_result(Some());
		return;
	}
	// Check if commands in PYTHON can be run
	let mut python_check_flag = false;
	for py in PYTHON {
		if Command::new(py).arg("--version").output().is_ok() {
			python_check_flag = true;
			break;
		}
	}

	if !python_check_flag {
		set_install_result(Some(ExitStatus::from_raw(45)));
		return;
	}
}

#[cfg(target_os = "macos")]
pub fn install() {
	let tmp_file = save_tmp_file(INSTALL_SCRIPT);
	let mut command = Command::new("sh");
	command.arg(tmp_file);

	let output = command.output().unwrap();
	set_install_result(Some(output.status.code().unwrap()));
}

#[cfg(target_os = "windows")]
pub fn install() {

}

#[cfg(target_os = "linux")]
pub fn install() {

}
