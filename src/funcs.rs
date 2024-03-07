use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::path::{Path, PathBuf};

use crate::constants::set_install_result;
use crate::constants::INSTALL_SCRIPT;

fn save_tmp_file(content: &str) -> PathBuf {
	let p = Path::new("/tmp/macos-install.sh");
	let mut f = File::create(p).unwrap();
	f.write(content.as_bytes()).unwrap();
	p.to_path_buf()
}

#[cfg(target_os = "macos")]
pub fn install() {
	let tmp_file = save_tmp_file(INSTALL_SCRIPT);
	let mut command = Command::new("sh");
	command.arg(tmp_file);

	let output = command.output().unwrap();
	set_install_result(Some(output.status));
}

#[cfg(target_os = "windows")]
pub fn install() {

}

#[cfg(target_os = "linux")]
pub fn install() {

}
