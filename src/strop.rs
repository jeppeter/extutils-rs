
#[allow(unused_imports)]
use extargsparse_worker::{extargs_error_class,extargs_new_error};

use base64;
use std::error::Error;
use num_bigint::{BigInt};
use num_traits::{zero};
use std::ffi::{OsString};
use regex::Regex;


extargs_error_class!{StrOpError}

pub fn encode_base64(bb :&[u8]) -> String {
	return base64::encode(bb);
}

pub fn decode_base64(instr :&str) -> Result<Vec<u8>,Box<dyn Error>> {
	let res = base64::decode(instr);
	if res.is_err() {
		let err = res.err().unwrap();
		extargs_new_error!{StrOpError,"can not parse [{}] for base64 error [{:?}]", instr,err}
	}
	let bv = res.unwrap();
	Ok(bv)
}

pub fn split_lines(s :&str) -> Vec<String> {
	let c : Vec<&str> = s.split("\n").collect();
	let mut retv :Vec<String> = Vec::new();
	for l in c.iter() {
		let cs :String = format!("{}",l);
		retv.push(format!("{}",cs.trim_end_matches('\r')));
	}
	return retv;
}

pub fn parse_u64(instr :&str) -> Result<u64,Box<dyn Error>> {
	let mut cparse = format!("{}",instr);
	let mut base :u32 = 10;
	let retv :u64;
	if cparse.starts_with("0x") || cparse.starts_with("0X") {
		cparse = cparse[2..].to_string();
		base = 16;
	} else if cparse.starts_with("x") || cparse.starts_with("X") {
		cparse = cparse[1..].to_string();
		base = 16;
	}

	match u64::from_str_radix(&cparse,base) {
		Ok(v) => {
			retv = v;
		},
		Err(e) => {
			extargs_new_error!{StrOpError, "parse [{}] error [{:?}]", instr, e}
		}
	}
	Ok(retv)
}

pub fn parse_to_bigint(instr :&str) -> Result<BigInt,Box<dyn Error>> {
	let mut _cparse = format!("{}",instr);
	let mut base :u32 = 10;
	let mut retv :BigInt = zero();
	let mut curv :BigInt ;
	let mut curvi :i32;
	let mut addi :i32 = 0;
	let mut bbchar :String;
	let mut negv :bool = false;
	let cparse :Vec<u8>;
	if _cparse.starts_with("0x") || _cparse.starts_with("0X") {
		_cparse = _cparse[2..].to_string();
		base = 16;
		addi += 2;
	} else if _cparse.starts_with("x") || _cparse.starts_with("X") {
		_cparse = _cparse[1..].to_string();
		base = 16;
		addi += 1;
	}

	if _cparse.starts_with("-") {
		_cparse = _cparse[1..].to_string();
		negv = true;
		addi += 1;
	}

	cparse = _cparse.as_bytes().to_vec();

	if cparse.len() == 0 {
		extargs_new_error!{StrOpError,"not valid [{}]",instr};
	}

	let mut lasti :usize = 0;
	let mut idx :i32 = 0;
	while lasti < cparse.len() {
		if base == 10 {
			if cparse[lasti] >= ('0' as u8) && cparse[lasti] <= ('9' as u8) {
				curvi = (cparse[lasti] - ('0' as u8)) as i32;
			} else {
				bbchar = "".to_string();
				if cparse[lasti] >= 0x20 && cparse[lasti] <= 0x7e {
					bbchar.push(cparse[lasti] as char);
				} else {
					bbchar.push_str(&format!("char[0x{:x}]",cparse[lasti]));
				}
				
				extargs_new_error!{StrOpError,"[{}] character not valid [{}]", idx + addi,bbchar}
			}
			curv = curvi.into();
			retv *= 10;
			retv += curv;
		} else {
			if cparse[lasti] >= ('0'  as u8) && cparse[lasti] <= ('9' as u8) {
				curvi = (cparse[lasti] - ('0' as u8)) as i32;
			} else if cparse[lasti] >= ('a'  as u8) && cparse[lasti] <= ('f' as u8){
				curvi = (cparse[lasti] - ('a' as u8)) as i32 + 10;
			} else if cparse[lasti] >= ('A' as u8 ) && cparse[lasti] <= ('F' as u8)  { 
				curvi = (cparse[lasti] - ('A' as u8)) as i32 + 10;
			} else {
				bbchar = "".to_string();
				if cparse[lasti] >= 0x20 && cparse[lasti] <= 0x7e {
					bbchar.push(cparse[lasti] as char);
				} else {
					bbchar.push_str(&format!("char[0x{:x}]",cparse[lasti]));
				}

				extargs_new_error!{StrOpError,"[{}] character not valid [{}]", idx + addi,bbchar}
			}
			curv = curvi.into();
			retv *= 16;
			retv += curv;
		}
		lasti += 1;
		idx += 1;
	}

	if negv {
		retv = -retv;
	}
	Ok(retv)
}


pub fn quote_string(ins :&str) -> Result<String,Box<dyn Error>> {
	let mut outs = ins.replace("\\","\\\\");
	outs = format!("\"{}\"",outs);
	Ok(outs)
}

pub fn os_str_to_str(oss :&OsString) -> Result<String,Box<dyn Error>> {
	let oret = oss.to_str();
	if oret.is_none() {
		extargs_new_error!{StrOpError,"{:?} to str error",oss}
	}
	Ok(oret.unwrap().to_string())
}


pub fn parse_to_u8_array(narr :&[String]) -> Result<Vec<u8>,Box<dyn Error>> {
	let mut retv :Vec<u8> = Vec::new();
	for v in narr.iter() {
		retv.push( parse_u64(v)? as u8);
	}
	Ok(retv)
}

pub fn parse_to_u8_array_str(narr :&[&str]) -> Result<Vec<u8>,Box<dyn Error>> {
	let mut retv :Vec<u8> = Vec::new();
	for v in narr.iter() {
		retv.push( parse_u64(v)? as u8);
	}
	Ok(retv)
}

pub fn parse_computer_size_u64(s :&str) -> Result<u64,Box<dyn Error>> {
	let mut retv :u64 ;
	let sbytes = s.as_bytes().to_vec();
	let regs = format!("^([0-9]+).*$");
	let ores = Regex::new(&regs);
	if ores.is_err() {
		extargs_new_error!{StrOpError,"{} compile regex error {:?}",regs,ores.err().unwrap()}
	}

	let reg = ores.unwrap();
	let caps = reg.captures(s);
	if caps.is_none() {
		extargs_new_error!{StrOpError,"{} not valid computer size", s}
	}
	let v = caps.unwrap();
	let c = format!("{}",v.get(1).map_or("", |m| m.as_str()));
	let cbytes = c.as_bytes().to_vec();

	match u64::from_str_radix(&c,10) {
		Ok(v) => {
			retv = v;
		},
		Err(e) => {
			extargs_new_error!{StrOpError,"parse [{}] error {:?}", s,e}
		}
	}

	let idx :usize = cbytes.len();
	if idx == sbytes.len() {
		return Ok(retv);
	} else if idx < (sbytes.len() - 1) {
		extargs_new_error!{StrOpError,"{} not valid in computer size", s}
	} else {
		if sbytes[idx] == b'k' || sbytes[idx] == b'K' {
			retv *= 1024;
		} else if sbytes[idx] == b'm' || sbytes[idx] == b'M' {
			retv *= 1024 * 1024;
		} else if sbytes[idx] == b'g' || sbytes[idx] == b'G' {
			retv *= 1024 * 1024 * 1024;
		} else if sbytes[idx] == b't' || sbytes[idx] == b'T' {
			retv *= 1024 * 1024 * 1024 * 1024;
		} else if sbytes[idx] == b'p' || sbytes[idx] == b'P' {
			retv *= 1024 * 1024 * 1024 * 1024 *1024;
		} else if sbytes[idx] == b'e' || sbytes[idx] == b'E' {
			retv *= 1024 * 1024 * 1024 * 1024 *1024 * 1024;
		} else {
			extargs_new_error!{StrOpError,"{} not valid computer size",s}
		}
	}
	Ok(retv)
}

pub fn parse_computer_size_i64(s :&str) -> Result<i64,Box<dyn Error>> {
	let mut retv :i64 ;
	let sbytes = s.as_bytes().to_vec();
	let regs = format!("^(\\-?[0-9]+).*$");
	let ores = Regex::new(&regs);
	if ores.is_err() {
		extargs_new_error!{StrOpError,"{} compile regex error {:?}",regs,ores.err().unwrap()}
	}

	let reg = ores.unwrap();
	let caps = reg.captures(s);
	if caps.is_none() {
		extargs_new_error!{StrOpError,"{} not valid computer size", s}
	}
	let v = caps.unwrap();
	let c = format!("{}",v.get(1).map_or("", |m| m.as_str()));
	let cbytes = c.as_bytes().to_vec();

	match i64::from_str_radix(&c,10) {
		Ok(v) => {
			retv = v;
		},
		Err(e) => {
			extargs_new_error!{StrOpError,"parse [{}] error {:?}", s,e}
		}
	}

	let idx :usize = cbytes.len();
	if idx == sbytes.len() {
		return Ok(retv);
	} else if idx < (sbytes.len() - 1) {
		extargs_new_error!{StrOpError,"{} not valid in computer size", s}
	} else {
		if sbytes[idx] == b'k' || sbytes[idx] == b'K' {
			retv *= 1024;
		} else if sbytes[idx] == b'm' || sbytes[idx] == b'M' {
			retv *= 1024 * 1024;
		} else if sbytes[idx] == b'g' || sbytes[idx] == b'G' {
			retv *= 1024 * 1024 * 1024;
		} else if sbytes[idx] == b't' || sbytes[idx] == b'T' {
			retv *= 1024 * 1024 * 1024 * 1024;
		} else if sbytes[idx] == b'p' || sbytes[idx] == b'P' {
			retv *= 1024 * 1024 * 1024 * 1024 *1024;
		} else if sbytes[idx] == b'e' || sbytes[idx] == b'E' {
			retv *= 1024 * 1024 * 1024 * 1024 *1024 * 1024;
		} else {
			extargs_new_error!{StrOpError,"{} not valid computer size",s}
		}
	}
	Ok(retv)
}