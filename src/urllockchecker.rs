
use curl::easy::Easy;
use std::str;
use std::str::FromStr;

pub struct UrlLockChecker {
	base_url: String,
	control_url: String
}

impl UrlLockChecker {
	pub fn new() -> UrlLockChecker {
		UrlLockChecker {
			base_url: "http://api.antizapret.info".to_string(),
			control_url: "".to_string()
		}
	}
	
	pub fn set_url(&mut self, url: &str) {
		self.control_url = String::from_str(url).unwrap();
	}
	
	pub fn is_lock(&self) -> bool {
		
		let mut str_data: String = String::new();
		let mut handle = Easy::new();
		
		let mut item_str: String = String::new();
		item_str.push_str("item=");
		item_str.push_str(&self.control_url);
		
		let type_str = "&type=small";
		
		let mut request_str: String = String::new();
		request_str.push_str(&self.base_url);
		request_str.push_str("/get.php");
		request_str.push_str("?");
		request_str.push_str(&item_str);
		request_str.push_str(type_str);
		
		handle.url(&request_str).unwrap();
		handle.get(true).unwrap();
		{
			let mut transfer = handle.transfer();
			transfer.write_function(|new_data| {
					let new_data_utf8 = str::from_utf8(new_data).unwrap();
					str_data = new_data_utf8.to_string();
					Ok(new_data.len())
			}).unwrap();
			transfer.perform().unwrap();
		}
		return str_data == "1";
	}
}
