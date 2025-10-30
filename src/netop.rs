use extargsparse_worker::{extargs_error_class,extargs_new_error};
use std::error::Error;

extargs_error_class!{NetOpError}

pub fn request_url_get_string(url :&str,bcheck :bool,timeout :u32) -> Result<(i32,String),Box<dyn Error>> {
	//let client = reqwest::Client::new();
	let oclient :reqwest::Result<reqwest::blocking::Client>;
	let mut clibuilder :reqwest::blocking::ClientBuilder;
	clibuilder = reqwest::blocking::ClientBuilder::new();
	if timeout != 0 {
		clibuilder = clibuilder.timeout(std::time::Duration::from_millis(timeout as u64));
	}
	if url.starts_with("https:") {
		if !bcheck {
			clibuilder = clibuilder.danger_accept_invalid_certs(true);			
		}
		clibuilder = clibuilder.use_rustls_tls();
	}
	oclient = clibuilder.build();

	if oclient.is_err() {
		extargs_new_error!{NetOpError,"can not build client error {:?}",oclient.err().unwrap()}
	}
	let client = oclient.unwrap();
	let ores = client.get(url).send();
	if ores.is_err() {
		extargs_new_error!{NetOpError,"can not get [{}] error {:?}",url, ores.err().unwrap()}
	}
	let resp = ores.unwrap();
	let sts = resp.status().as_u16() as i32;
	let obody = resp.text();
	if obody.is_err() {
		extargs_new_error!{NetOpError,"get [{}] body error {:?}",url,obody.err().unwrap()}
	}
	let rs = obody.unwrap();
	return Ok((sts,rs));
	//Ok(format!(""))
}
