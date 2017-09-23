extern crate urllockcheck;

use urllockcheck::UrlLockChecker;

#[test]
fn test1() {
	let check = UrlLockChecker::new("rutracker.org");
	match check.get_details() {
           Ok(dt) => assert_eq!("http://antizapret.info".to_string(), dt.source),
           Err(e) => panic!("{:?}", e),
           }
}

#[test]
fn test2() {
	let check = UrlLockChecker::new("rutracker.org");
	let reg = match check.get_details() {
           Ok(dt) => dt.register,
           Err(e) => panic!("{:?}", e),
           };

    let reg1 = match reg {
        Some(vec_) => assert_eq!( 5, vec_.len()),
        None => panic!("register empty"),
        }
}