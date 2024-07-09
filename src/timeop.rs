
use chrono::prelude::{DateTime,Utc};
use chrono::{Datelike,Timelike,TimeZone};
use extargsparse_worker::{extargs_error_class,extargs_new_error};
//use extlog::*;
//use extlog::loglib::*;

use std::error::Error;

extargs_error_class!{TimeOpError}

pub fn get_time_utc_str() -> Result<String,Box<dyn Error>> {
	let now = Utc::now();
	Ok(format!("{:4}{:2}{:2}{:2}{:2}{:2}",now.year(),now.month(),now.day(),now.hour(),now.minute(),now.second()))
}

fn _get_ival_bytes(bs :&[u8],sidx :usize,eidx:usize) -> Result<u32,Box<dyn Error>> {
	if bs.len() < eidx {
		extargs_new_error!{TimeOpError,"inner error for eidx {}",eidx}
	}
	let c = bs[sidx..eidx].to_vec().clone();
	let ores = std::str::from_utf8(&c);
	if ores.is_err() {
		extargs_new_error!{TimeOpError,"sidx {} eidx {} utf8 error {:?}",sidx,eidx,ores.err().unwrap()}
	}
	let s = format!("{}",ores.unwrap());
	match u32::from_str_radix(&s,10) {
		Ok(v) => {
			return Ok(v);
		},
		Err(e) => {
			extargs_new_error!{TimeOpError,"{} error {:?}",s,e}
		}
	}
}


pub fn tran_time_utc_from_str(ins :&str) -> Result<DateTime<Utc>,Box<dyn Error>> {
	let bs = ins.as_bytes().to_vec();
	if bs.len() != 14 {
		extargs_new_error!{TimeOpError,"{} not valid len {}",ins,bs.len()}
	}
	let bs = ins.as_bytes().to_vec();
	let iyear = _get_ival_bytes(&bs,0,4)?;
	let imon = _get_ival_bytes(&bs,4,6)?;
	let iday = _get_ival_bytes(&bs,6,8)?;
	let ihour = _get_ival_bytes(&bs,8,10)?;
	let imin = _get_ival_bytes(&bs,10,12)?;
	let isec = _get_ival_bytes(&bs,12,14)?;
	let odt = Utc.with_ymd_and_hms(iyear as i32,imon,iday,ihour,imin,isec);
	match odt {
		chrono::LocalResult::None => {
			extargs_new_error!{TimeOpError,"{} not valid",ins}
		},
		chrono::LocalResult::Single(e) => {
			return Ok(e);
		},
		chrono::LocalResult::Ambiguous(_b,_b2) => {
			extargs_new_error!{TimeOpError,"{} not valid",ins}
		}
	}
}