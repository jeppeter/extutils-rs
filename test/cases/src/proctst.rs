#[allow(unused_imports)]
use extargsparse_codegen::{extargs_load_commandline,ArgSet,extargs_map_function};
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};
#[allow(unused_imports)]
use extargsparse_worker::namespace::{NameSpaceEx};
#[allow(unused_imports)]
use extargsparse_worker::argset::{ArgSetImpl};
use extargsparse_worker::parser::{ExtArgsParser};
use extargsparse_worker::funccall::{ExtArgsParseFunc};


use std::cell::RefCell;
use std::sync::Arc;
use std::error::Error;
use std::boxed::Box;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::any::Any;

use lazy_static::lazy_static;
use std::collections::HashMap;

#[allow(unused_imports)]
use extlog::{debug_trace,debug_buffer_trace,format_buffer_log,format_str_log};
#[allow(unused_imports)]
use extlog::loglib::{log_get_timestamp,log_output_function};
use extutils::logtrans::{init_log};


extargs_error_class!{ProcTestError}

fn waitchld_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr =ns.get_array("subnargs");
	init_log(ns.clone())?;

	let mut cmd :std::process::Command;
	if sarr.len() < 1 {
		extargs_new_error!{ProcTestError,"need at least one arg"}
	}
	cmd = std::process::Command::new(&sarr[0]);
	let mut idx :usize = 1;
	while idx < sarr.len() {
		cmd.arg(&sarr[idx]);
		idx += 1;
	}
	let mut chld :std::process::Child = cmd.spawn()?;
	let mut cnt :usize = 0;
	loop {
		println!("wait cnt {}", cnt);
		let osts :Option<std::process::ExitStatus> = chld.try_wait()?;
		if osts.is_some() {
			let sts = osts.unwrap();
			println!("{:?} run {}", sarr,sts.code().unwrap());
			break;
		}
		std::thread::sleep(std::time::Duration::from_millis(1000));
		cnt += 1;
	}

	Ok(())
}


#[extargs_map_function(waitchld_handler)]
pub fn load_proc_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"waitrun<waitchld_handler>##args ... to run cmds ##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}