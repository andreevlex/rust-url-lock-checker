extern crate curl;

mod urllockchecker;
use urllockchecker::UrlLockChecker;

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
	const HELP: &'static str = "Использование: url-lock-checker команда [аргументы]...
    Команды:
        -c, --check <DOMAIN_NAME> 
            проверить сайт. Параметр DOMAIN_NAME - содержит доменное имя или ip адрес
        -h, --help
            показать это сообщение.";
    
	let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(text) => {
            match text.as_ref() {
                "-c" | "--check" => {
                    if args.len() != 3 {
                            panic!("Использование: url-lock-checker --check DOMAIN_NAME");
                    }
                    check_url(&args[2])
                },
                "-h" | "--help" => {
                    println!("{}", HELP);
                },
                command @ _  => panic!(
                    format!("Неправильная команда: {}", command))
            }
        }
        None => println!("{}", HELP),
    }
}
