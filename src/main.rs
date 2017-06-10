extern crate curl;

use curl::easy::Easy;
use std::str;

fn check_url(url:&str) {
	let mut str_data: String = String::new();
	let mut handle = Easy::new();
	let main_url = "http://api.antizapret.info/get.php";
	let mut item_str: String = String::new();
	item_str.push_str("item=");
	item_str.push_str(url);
	
	let type_str = "&type=small";
	
	let mut request_str: String = String::new();
	request_str.push_str(main_url);
	request_str.push_str("?");
	request_str.push_str(&item_str);
	request_str.push_str(type_str);
	
	handle.url(&request_str).unwrap();
	handle.get(true).unwrap();
	{
    	let mut transfer = handle.transfer();
    	transfer.write_function(|new_data| {
    			//data.extend_from_slice(new_data);
    			let new_data_utf8 = str::from_utf8(new_data).unwrap();
    			str_data = new_data_utf8.to_string();
    			Ok(new_data.len())
    	}).unwrap();
    	transfer.perform().unwrap();
    }
    println!("{}", str_data);
}

fn main() {
	const HELP: &'static str = "Использование: url-lock-checker команда [аргументы]...
    Команды:
        check DOMAIN_NAME - проверить сайт. Параметр DOMAIN_NAME - содержит доменное имя или ip адрес
        help  - показать это сообщение.";
    
	let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(text) => {
            match text.as_ref() {
                "check" => {
                    if args.len() != 3 {
                            panic!("Использование: url-lock-checker check DOMAIN_NAME");
                    } 
                    check_url(&args[2])
                },
                "help" => {
                    println!("{}", HELP);
                },
                command @ _  => panic!(
                    format!("Неправильная команда: {}", command))
            }
        }
        None => println!("{}", HELP),
    }
}
