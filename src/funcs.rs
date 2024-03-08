use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::path::{Path, PathBuf};

use crate::constants::{set_install_result, get_install_result};
use crate::constants::INSTALL_SCRIPT;
use crate::constants::PYTHON;

fn save_tmp_file(content: &str) -> PathBuf {
	let p = Path::new("/tmp/cpc-install.sh");
	let mut f = File::create(p).unwrap();
	f.write(content.as_bytes()).unwrap();
	p.to_path_buf()
}

pub fn check_dependencies() -> bool {
	// 检查 git
	let git_check = Command::new("which").arg("git").output().unwrap();
	if !git_check.status.success() && String::from_utf8_lossy(&git_check.stdout).is_empty() {
		set_install_result(Some(44));
		return false;
	}

	// 检查 python
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

fn install_git() {
	#[cfg(target_os = "windows")]
	Command::new("winget")
		.arg("install")
		.arg("Git.Git")
		.arg("--accept-package-agreements")
		.arg("--accept-source-agreements")
		.spawn().unwrap();

	#[cfg(target_os = "macos")]
	Command::new("git")
		.arg("--version")
		.spawn().unwrap();
}

fn install_python() {
	#[cfg(target_os = "windows")]
	Command::new("winget")
		.arg("install")
		.arg("Python.Python.3.12")
		.arg("--accept-package-agreements")
		.arg("--accept-source-agreements")
		.spawn().unwrap();

	#[cfg(target_os = "macos")]
	Command::new("python3")
		.arg("--version")
		.spawn().unwrap();
}

pub fn dependencies_install() {
	if let Some(result) = get_install_result() {
		match result {
			44 => install_git(),
			45 => install_python(),
			_ => ()
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
