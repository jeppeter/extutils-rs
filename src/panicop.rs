
use std::panic::{set_hook};
use extargsparse_worker::{extargs_error_class,extargs_new_error};
use crate::fileop::{mkdir_safe};

extargs_error_class!{PanicOpError}

static mut PANIC_DIR:Option<String> = None;


fn set_panic_dir(dname :&str) -> Result<(),Box<dyn Error>> {
	unsafe {
		PANIC_DIR = Some(format!("{}",dname));
	}
	Ok(())
}

fn panic_hook_fn(info :&PanicInfo<'_>) {
	let outs = format!("{:?}",info);
	let mut outok : bool = false;
	if unsafe {PANIC_DIR.is_some()} {
		let dname :String = unsafe{format!("{}",PANIC_DIR.as_ref().unwrap())};
		let ores = mkdir_safe(&dname);
		if ores.is_ok() {
			
		}
	}
	if !outok {
		println!("{}",outs);
	}
	return;
}

fn set_panic_hook(dname :&str) -> Result<(),Box<dyn Error>> {
	set_hook(Box::new(|s| {
		panic_hook_fn(s);
	}));
}

