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
use extutils::procop::{get_pid_by_exact_name,get_pid_children_tree};
use extutils::strop::{parse_u64};

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

//use sysinfo::SystemExt;

fn getpid_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr =ns.get_array("subnargs");
	let mut idx :usize;
	init_log(ns.clone())?;

	for n in sarr.iter() {
		let pids = get_pid_by_exact_name(n);
		print!("pids of {}", n);
		idx = 0;
		while idx < pids.len() {
			if (idx % 5) == 0 {
				print!("\n    ");
			}
			print!(" {:05}",pids[idx]);
			idx += 1;
		}
		print!("\n");

	}

	Ok(())
}

fn getchild_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr =ns.get_array("subnargs");
	init_log(ns.clone())?;

	for n in sarr.iter() {
		let pid = parse_u64(n)?;
		let ptree = get_pid_children_tree(pid);
		print!("{}",ptree.to_string(0));
	}

	Ok(())
}


#[extargs_map_function(waitchld_handler,getpid_handler,getchild_handler)]
pub fn load_proc_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"waitrun<waitchld_handler>##args ... to run cmds ##" : {
			"$" : "+"
		},
		"getpid<getpid_handler>##exename ... to filter##" : {
			"$" : "+"
		},
		"getchild<getchild_handler>##pid ... to get children##" : {
			"$" : "+"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}