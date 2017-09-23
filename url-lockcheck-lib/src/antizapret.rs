use std::io::Read;
use reqwest::{Client, IntoUrl, StatusCode};
use reqwest::header::{Headers, Accept, ContentType};

use serde::{Deserialize, Serialize};

use std::str;
use std::str::FromStr;

use models::*;

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

enum TypeResponse {
    small,
    csv,
    json,
    xml,
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

    fn build_url(&self, tp: TypeResponse) -> String {
        let mut request_str: String = String::new();
		request_str.push_str(&self.base_url);
		request_str.push_str("/get.php");
		request_str.push_str("?");
		request_str.push_str("item=");
		request_str.push_str(&self.control_url);
		request_str.push_str("&type=");

        let type_resp = match tp {
            TypeResponse::small => "small",
            TypeResponse::csv => "csv",
            TypeResponse::xml => "xml",
            TypeResponse::json => "json",
        };

        request_str.push_str(type_resp);

        request_str
    }

	pub fn is_lock(&self) -> ApiResult<bool> {
						
		let mut request_str: String = self.build_url(TypeResponse::small);
				
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

	pub fn get_details(&self) -> ApiResult<DetailInfo> {
        let full_url = self.build_url(TypeResponse::json);
        self.get_json(&full_url)
	}

    /// Returns raw data in bytes from specified url
    fn get_json<S, T>(&self, url: S) -> ApiResult<T>
        where S: IntoUrl,
              for<'de> T: Deserialize<'de> 
    {
        match self.client.get(url).unwrap().send() {
            Ok(mut response) => response.json::<T>().map_err(|e| ApiError::BadResponse(e.to_string())),
            Err(e) => Err(ApiError::BadRequest(e.to_string())),
        }
    }
}