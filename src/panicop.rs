
use std::panic::{set_hook,PanicInfo};
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
use extargsparse_codegen::{extargs_load_commandline,extargs_map_function};
use extargsparse_worker::namespace::NameSpaceEx;
use extargsparse_worker::funccall::ExtArgsParseFunc;
use crate::fileop::{mkdir_safe,write_file};
use crate::timeop::{get_time_utc_str};
use extargsparse_worker::parser::{ExtArgsParser};
use lazy_static::lazy_static;
use std::collections::HashMap;
//use std::backtrace::{Backtrace};
use backtrace::{Backtrace};

use std::error::Error;
use extlog::*;
use extlog::loglib::*;
use crate::procop::{get_exec_dir};
use crate::strop::{str_to_quoted};

extargs_error_class!{PanicOpError}

static mut PANIC_DIR:Option<String> = None;
static mut PANIC_VERBOSE :i32 = 0;


fn set_panic_dir(dname :&str) -> Result<(),Box<dyn Error>> {
	unsafe {
		PANIC_DIR = Some(format!("{}",dname));
	}
	Ok(())
}

fn panic_hook_fn(info :&PanicInfo<'_>) {
	let mut outs = "".to_string();
	let mut outok : bool = false;
	let mut verbose :bool = false;
	if unsafe {PANIC_VERBOSE >= 3} {
		verbose= true;
	}
	if verbose {
		let bk = Backtrace::new();
		outs.push_str(&format!("{:?}\n",bk));
	}
	outs.push_str(&format!("{}",info));
	if unsafe {PANIC_DIR.is_some()} {
		let dname :String = unsafe{format!("{}",PANIC_DIR.as_ref().unwrap())};
		let ores = mkdir_safe(&dname);
		if ores.is_ok() {
			let ores = get_time_utc_str();
			if ores.is_ok() {
				let times = ores.unwrap();
				let fname = format!("{}/panic_{}.log",dname,times);
				let ores = write_file(&fname,&outs);
				if ores.is_ok() {
					outok = true;
				}
			}
		}
	}
	if !outok {
		println!("{}",outs);
	}
	return;
}

fn set_panic_hook(dname :&str) -> Result<(),Box<dyn Error>> {
	let _ = set_panic_dir(dname)?;
	set_hook(Box::new(|s| {
		panic_hook_fn(s);
	}));
	Ok(())
}

fn set_panic_verbose(verbse :i32) -> Result<(),Box<dyn Error>> {
	unsafe {
		PANIC_VERBOSE = verbse;
	}
	debug_trace!("PANIC_VERBOSE {}",unsafe{PANIC_VERBOSE});
	Ok(())
}

pub fn init_panicop(ns :NameSpaceEx) -> Result<(),Box<dyn Error>> {
	let dname = ns.get_string("panicdir");
	let enable = ns.get_bool("panicenable");
	if !enable {
		return Ok(());
	}
	if dname.len() == 0 {
		return Ok(());
	}

	let _ = set_panic_hook(&dname)?;
	let iv = ns.get_int("panicverbose") as i32;
	let _ = set_panic_verbose(iv)?;
	Ok(())
}

#[extargs_map_function()]
pub fn load_panicop_commandline(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let sdir = str_to_quoted(&get_exec_dir()?)?;
	let cmdline = format!(r#"
	{{
		"panicdir" : {},
		"panicenable" : true,
		"panicverbose" : 3
	}}
	"#,sdir);
	extargs_load_commandline!(parser,&cmdline)?;
	Ok(())
}

