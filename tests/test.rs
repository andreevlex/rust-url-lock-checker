extern crate urllockcheck;

use urllockcheck::UrlLockChecker;

#[test]
fn test_source() {
	let check = UrlLockChecker::new("rutracker.org");
	match check.get_details() {
           Ok(dt) => assert_eq!("http://antizapret.info".to_string(), dt.source),
           Err(e) => panic!("{:?}", e),
           }
}

#[test]
fn test_register_len() {
	let check = UrlLockChecker::new("rutracker.org");
	let reg = match check.get_details() {
           Ok(dt) => dt.register,
           Err(e) => panic!("{:?}", e),
           };

    match reg {
        Some(vec_) => assert_eq!( 5, vec_.len()),
        None => panic!("register empty"),
        }
}

#[test]
fn test_update_time() {
	let check = UrlLockChecker::new("rutracker.org");
	match check.get_details() {
           Ok(dt) => assert_eq!(dt.get_update_time().unwrap().format("%Y-%m-%d %H:%M:%S").to_string(), dt.update_time),
           Err(e) => panic!("{:?}", e),
           }
}