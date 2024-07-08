
use chrono::prelude::{DateTime,Utc};
use extargsparse_worker::{extargs_error_class,extargs_new_error};

extargs_error_class!{TimeOpError}

pub fn get_time_utc_str() -> Result<String,Box<dyn Error>> {
	let now = Utc::now();
	Ok(format!("{:4d}{:2d}{:2d}{:2d}{:2d}{:2d}",now.year(),now.month(),now.day(),now.hour24(),now.minute(),now.second()))
}


pub fn tran_time_utc_from_str(ins :&str) -> Result<DateTime<Utc>,Box<dyn Error>> {
	if ins.len() != 14 {
		extargs_new_error!{TimeOpError,"{} not valid",ins}
	}
	let bs = ins.as_bytes().to_vec();
	let syear :String;
	let smon :String;
	let sday :String;
	let shour :String;
	let smin :String;
	let ssec :String;
	syear = format!("{}{}{}{}",bs[0],bs[1],bs[2],bs[3]);
	smon = format!("{}{}",bs[4],bs[5]);
	sday = format!("{}{}",bs[6],bs[7]);
	shour = format!("{}{}",bs[8],bs[9]);
	smin = format!("{}{}",bs[10],bs[11]);
	ssec = format!("{}{}",bs[12],bs[13]);

	
}