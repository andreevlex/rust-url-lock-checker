#[macro_use]
extern crate clap;
extern crate urllockcheck;

use urllockcheck::UrlLockChecker;
use clap::{Arg, App};

fn check_url( s: &str ) {
	let checker = UrlLockChecker::new(s);
	match checker.is_lock() {
		Ok(result) => println!("{}", result),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn main() {
	let app_m = App::new("url-lock-checker")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Консольная программа для проверки сайта на наличие в запрещенном списке через API сайта https://antizapret.info/")
        .arg(Arg::with_name("check")
                .short("c")
                .long("check")
                .help("проверить сайт по доменному имени или ip адресу")
                .takes_value(true)
                .value_name("DOMAIN_NAME"))
        .get_matches();
    if let Some(url) = app_m.value_of("check") {
        check_url(&url);
    }
    else {
        println!("{}", "Используйте ключ -h для получения справки");
    }
}
