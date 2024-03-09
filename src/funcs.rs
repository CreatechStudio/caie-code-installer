use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::path::{Path, PathBuf};

use crate::constants::{set_install_result, get_install_result};
use crate::constants::INSTALL_SCRIPT;
use crate::constants::PYTHON;

fn save_tmp_file(content: &str) -> PathBuf {
	let tmp_path = "/tmp/cpc-install.sh".to_string();

	#[cfg(target_os = "windows")]
	let tmp_path = format!(
		"{}\\AppData\\Local\\Temp\\cpc-install.sh",
		dirs::home_dir().unwrap().to_str().unwrap()
	);

	let p = Path::new(&tmp_path);

	let mut f = File::create(p).unwrap();
	f.write(content.as_bytes()).unwrap();
	p.to_path_buf()
}

fn check_dependence_exist<T>(command_name: T) -> bool
where T: ToString {
	let output = Command::new("where").arg(command_name.to_string()).output().unwrap();
	let output_str = String::from_utf8_lossy(&output.stdout).to_string();
	!output.status.success() || output_str.is_empty() || output_str.contains("not found")
}

pub fn check_dependencies() -> bool {
	// 检查 git
	if check_dependence_exist("git") {
		set_install_result(Some(44));
		return false;
	}

	// 检查 python
	for py in PYTHON {
		if check_dependence_exist(py) {
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
	opener::open("https://git-scm.com/").unwrap();

	#[cfg(target_os = "macos")]
	Command::new("git")
		.arg("--version")
		.spawn().unwrap();
}

fn install_python() {
	#[cfg(target_os = "windows")]
	opener::open("https://python.org/").unwrap();

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
	let tmp_file = save_tmp_file(INSTALL_SCRIPT);
	let mut command = Command::new("start");
	command.arg(tmp_file);

	let output = command.output().unwrap();
	set_install_result(Some(output.status.code().unwrap()));
}

#[cfg(target_os = "linux")]
pub fn install() {

}
