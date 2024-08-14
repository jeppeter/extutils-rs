

use extargsparse_worker::{extargs_error_class,extargs_new_error};
use std::error::Error;
use sysinfo;

extargs_error_class!{PropOpError}

pub fn get_exec_name() -> Result<String,Box<dyn Error>> {
	let ores = std::env::current_exe();
	if ores.is_err() {
		extargs_new_error!{PropOpError,"can not get current_exe {:?}",ores.err().unwrap()}
	}
	let ename = ores.unwrap();
	Ok(format!("{}",ename.display()))
}

pub fn get_exec_dir() -> Result<String,Box<dyn Error>> {
	let ename = get_exec_name()?;
	let oparent = std::path::Path::new(&ename).parent();
	if oparent.is_none() {
		extargs_new_error!{PropOpError,"can not get parent for [{}]",ename}
	}
	let parentd = oparent.unwrap();
	Ok(format!("{}",parentd.display()))
}

pub fn get_pid_by_exact_name(n :&str) -> Vec<u64> {
	let mut retv :Vec<u64> = vec![];
	let mut system = sysinfo::System::new();
	system.refresh_all();
	let nostr = std::ffi::OsStr::new(n);
	for p in system.processes_by_name(nostr) {
		retv.push(p.pid().as_u32() as u64);
	}

	return retv;
}