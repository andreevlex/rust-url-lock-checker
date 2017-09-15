extern crate curl;
extern crate clap;

mod urllockchecker;
use urllockchecker::UrlLockChecker;
use clap::{Arg, App};

fn check_url( s: &str ) {
	let checker = UrlLockChecker::new(s);
	if checker.is_lock() {
		println!("yes");
	}
	else {
		println!("no");
	}
}

fn main() {
	let app_m = App::new("rust-url-lock-checker")
        .version("0.3.0")
        .author("Andreev Alexander")
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
