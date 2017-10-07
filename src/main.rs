#[macro_use]
extern crate clap;
extern crate urllockcheck;

use urllockcheck::{UrlLockChecker,DetailInfo};

use clap::{Arg, App};

fn check_url( s: &str ) {
	let checker = UrlLockChecker::new(s);
	match checker.is_lock() {
		Ok(result) => println!("{}", result),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn print_ip_addresses(dt: DetailInfo) {
    let result = dt.get_ip_addresses();
    for el in result.iter() {
        println!("{}", el);
    }
}

fn print_block_ip( s: &str ) {
	let checker = UrlLockChecker::new(s);
    match checker.get_details() {
        Ok(dt) => print_ip_addresses(dt),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn print_urls(dt: DetailInfo) {
    let result = dt.get_urls();
    println!("{:?}", result);
    for el in result.iter() {
        println!("{}", el);
    }
}

fn print_block_urls( s: &str ) {
	let checker = UrlLockChecker::new(s);
    match checker.get_details() {
        Ok(dt) => print_urls(dt),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn print_update_date( s: &str ) {
	let checker = UrlLockChecker::new(s);
	match checker.get_details() {
		Ok(dt) => println!("{}", dt.update_time.to_string()),
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
        .arg(Arg::with_name("print-ip")
                .short("p")
                .long("print-ip")
                .help("получить список заблокированных ip адресов по доменному имени")
                .takes_value(true)
                .value_name("DOMAIN_NAME"))
        .arg(Arg::with_name("update-date")
                .short("d")
                .long("update-date")
                .help("показать дату обновления данных по доменному имени")
                .takes_value(true)
                .value_name("DOMAIN_NAME"))
        .arg(Arg::with_name("print-url")
                .short("u")
                .long("print-url")
                .help("получить список заблокированных url адресов по доменному имени")
                .takes_value(true)
                .value_name("DOMAIN_NAME"))
        .get_matches();
    if let Some(url) = app_m.value_of("check") {
        check_url(&url);
    }
    else if let Some(url) = app_m.value_of("print-ip") {
        print_block_ip(&url);
    }
    else if let Some(url) = app_m.value_of("update-date") {
        print_update_date(&url);
    }
    else if let Some(url) = app_m.value_of("print-url") {
        print_block_urls(&url);
    }
    else {
        println!("{}", "Используйте ключ -h для получения справки");
    }
}