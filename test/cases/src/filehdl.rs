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

use extutils::fileop::{read_file_bytes,write_file_bytes,read_file,touch_file,delete_file,exists_file,append_file_bytes};
use extutils::strop::{encode_base64,split_lines};

extargs_error_class!{FileHdlError}


fn fileencbase64_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let output :String;
	let sarr :Vec<String>;

	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");
	output = ns.get_string("output");

	for f in sarr.iter() {
		let data = read_file_bytes(f)?;
		let outs = encode_base64(&data);
		write_file_bytes(&output,outs.as_bytes())?;
	}

	Ok(())
}

fn splitlines_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;

	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");

	for f in sarr.iter() {
		let sdata : String = read_file(f)?;
		let rdata :Vec<String> = split_lines(&sdata);
		let mut idx :usize = 0;
		while idx < rdata.len() {
			println!("[{}][{}]",idx,rdata[idx]);
			idx += 1;
		}

	}

	Ok(())
}

fn touch_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;

	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");

	for f in sarr.iter() {
		if !exists_file(f) {
			let _ = touch_file(f)?;	
		} else {
			println!("exist {}",f);
		}
		
	}

	Ok(())
}

fn delfile_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;

	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");

	for f in sarr.iter() {
		if exists_file(f) {
			let _ = delete_file(f)?;	
		} else {
			println!("not exist {}", f);
		}
		
	}

	Ok(())
}

fn writefile_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	let ifile :String;
	let ibytes :Vec<u8>;

	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");

	if sarr.len() < 1 {
		extargs_new_error!{FileHdlError,"need one file"}
	}

	ifile = ns.get_string("input");
	ibytes = read_file_bytes(&ifile)?;
	let _ = write_file_bytes(&sarr[0],&ibytes)?;
	Ok(())
}


fn appendfile_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr :Vec<String>;
	let ifile :String;
	let ibytes :Vec<u8>;

	init_log(ns.clone())?;

	sarr = ns.get_array("subnargs");

	if sarr.len() < 1 {
		extargs_new_error!{FileHdlError,"need one file"}
	}

	ifile = ns.get_string("input");
	ibytes = read_file_bytes(&ifile)?;
	let _ = append_file_bytes(&sarr[0],&ibytes)?;
	Ok(())
}


#[extargs_map_function(fileencbase64_handler,splitlines_handler,touch_handler,delfile_handler,writefile_handler,appendfile_handler)]
pub fn load_file_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"fileencbase64<fileencbase64_handler>##fname ... to encode base64##" : {
			"$" : "+"
		},
		"splitlines<splitlines_handler>##fname ... to split lines##" : {
			"$" : "+"
		},
		"touch<touch_handler>##files ... to touch file##" : {
			"$" : "+"
		},
		"delfile<delfile_handler>##files ... to delete file##" : {
			"$" : "+"
		},
		"writefile<writefile_handler>##file to write file from input##" : {
			"$" : 1
		},
		"appendfile<appendfile_handler>##file to append from input##" : {
			"$" : 1
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}