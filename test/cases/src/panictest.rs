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

use extutils::panicop::{init_panicop};

#[allow(unreachable_code)]
fn panictest_handler(ns :NameSpaceEx,_optargset :Option<Arc<RefCell<dyn ArgSetImpl>>>,_ctx :Option<Arc<RefCell<dyn Any>>>) -> Result<(),Box<dyn Error>> {
	let sarr =ns.get_array("subnargs");
	init_log(ns.clone())?;
	init_panicop(ns.clone())?;
	if sarr.len() > 0 {
		panic!("{}",sarr[0]);
	} else {
		panic!("hello will panic");	
	}
	

	Ok(())
}


#[extargs_map_function(panictest_handler)]
pub fn load_panic_handler(parser :ExtArgsParser) -> Result<(),Box<dyn Error>> {
	let cmdline = r#"
	{
		"panictest<panictest_handler>##str ... to trans tm##" : {
			"$" : "*"
		}
	}
	"#;
	extargs_load_commandline!(parser,cmdline)?;
	Ok(())
}