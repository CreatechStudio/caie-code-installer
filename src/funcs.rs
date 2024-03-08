use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::path::{Path, PathBuf};

use crate::constants::{set_install_result, OS_TYPE};
use crate::constants::INSTALL_SCRIPT;
use crate::constants::PYTHON;

fn save_tmp_file(content: &str) -> PathBuf {
	let p = Path::new("/tmp/macos-install.sh");
	let mut f = File::create(p).unwrap();
	f.write(content.as_bytes()).unwrap();
	p.to_path_buf()
}

pub fn check_dependencies() -> bool {
	// Check if Xcode Command Line Tools is installed
	let git_check = Command::new("which").arg("git").output().unwrap();
	if !git_check.status.success() && String::from_utf8_lossy(&git_check.stdout).is_empty() {
		set_install_result(Some(44));
		return false;
	}
	// Check if commands in PYTHON can be run
	for py in PYTHON {
		let python_check = Command::new("which").arg(py).output().unwrap();
		if python_check.status.success() && !String::from_utf8_lossy(&python_check.stdout).is_empty() {
			break;
		} else {
			set_install_result(Some(45));
			return false;
		}
	}

	true
}

pub fn dependencies_install() {
    use crate::constants::get_install_result;

	fn install_git() {
		if  OS_TYPE == "windows" {
			Command::new("winget")
			.arg("install")
			.arg("Git.Git")
			.arg("--accept-package-agreements")
			.arg("--accept-source-agreements");
		} else if cfg!(target_os = "macos"){
			Command::new("git")
			.arg("--version");
		}
		
	}

	fn install_python() {
		if OS_TYPE == "windows" {
			Command::new("winget")
			.arg("install")
			.arg("Python.Python.3.12")
			.arg("--accept-package-agreements")
			.arg("--accept-source-agreements");
		} else if cfg!(target_os = "macos"){
			Command::new("python3")
			.arg("--version");
		}
	}

	if let Some(result) = get_install_result() {
		if result == 44 {
			install_git();
		} else if result == 45{
			install_python();
		}
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
