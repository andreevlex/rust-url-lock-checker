extern crate reqwest;

use std::io::Read;
use reqwest::{Client,IntoUrl,StatusCode};
use reqwest::header::{Headers, Accept, ContentType};
use std::str;
use std::str::FromStr;
use std::time::Duration;

pub struct UrlLockChecker {
	base_url: String,
	control_url: String,
	client: Client,
}

#[derive(Debug)]
pub enum ApiError {
	EmptyResponse,
	BadResponse(String),
	BadRequest(String),
	Unknown(String),
}

type ApiResult<T> = Result<T, ApiError>;

impl UrlLockChecker {
	pub fn new(c_url: &str) -> UrlLockChecker {
		let client = Client::new().unwrap();
		
		UrlLockChecker {
			base_url: "http://api.antizapret.info".to_string(),
			control_url: String::from_str(c_url).unwrap(),
			client: client,
		}
	}

	pub fn is_lock(&self) -> ApiResult<bool> {
						
		let mut request_str: String = String::new();
		request_str.push_str(&self.base_url);
		request_str.push_str("/get.php");
		request_str.push_str("?");
		request_str.push_str("item=");
		request_str.push_str(&self.control_url);
		request_str.push_str("&type=small");
				
		match self.client.get(&request_str).unwrap().send() {
			Ok(mut resp) => {
				let mut content: String = String::new();		
				resp.read_to_string(&mut content);

				match content.as_ref() {
					"piff-paff" => Err(ApiError::BadRequest(format!("Bad request {}", request_str))),
					"1" => Ok(true),
					"0" => Ok(false),
					_ => Err(ApiError::Unknown("Unknown value".to_string())),
				}
			},
			Err(e) => Err(ApiError::BadRequest(e.to_string())),	
		}
	}
}
