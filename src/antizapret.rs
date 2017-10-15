use std::io::Read;
use reqwest::{Client, IntoUrl};

use serde::{Deserialize};

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
    Small,
    Csv,
    Json,
    Xml,
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
        let type_resp = match tp {
            TypeResponse::Small => "small",
            TypeResponse::Csv => "csv",
            TypeResponse::Xml => "xml",
            TypeResponse::Json => "json",
        };

        let request_str = format!("{}/get.php?item={}&type={}", self.base_url, self.control_url, type_resp);

        request_str
    }

	pub fn is_lock(&self) -> ApiResult<bool> {
						
		let request_str: String = self.build_url(TypeResponse::Small);
				
		match self.client.get(&request_str).unwrap().send() {
			Ok(mut resp) => {
				let mut content: String = String::new();		
				if let Err(e) = resp.read_to_string(&mut content) {
					return Err(ApiError::BadResponse(e.to_string()));
				};

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
        let full_url = self.build_url(TypeResponse::Json);
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