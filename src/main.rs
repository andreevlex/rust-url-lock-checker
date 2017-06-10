extern crate curl;

mod urllockchecker;

fn check_url( s: &str ) {
	let mut checker = urllockchecker::UrlLockChecker::new();
	checker.set_url(s);
	if checker.is_lock() {
		println!("yes");
	}
	else {
		println!("no");
	}
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
