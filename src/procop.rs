

use extargsparse_worker::{extargs_error_class,extargs_new_error};
use std::error::Error;

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

