use std::env::args;
use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use std::process::Command;
use clap;
use clap::Arg;

fn main() {

    clap::Command::new("pomo")
        .version("0.3.0")
        .about("pomo — терминальный Pomodoro-таймер")
        .arg(Arg::new("Время работы").help("Длительность рабочей сессии в минутах   (по умолчанию: 25)"))
        .arg(Arg::new("Циклы").help("Сколько раз повторить работа+отдых   (по умолчанию: 1)"))
        .arg(Arg::new("Время перерыва").help("Длительность перерыва в минутах   (по умолчанию: 5)"))
        .get_matches();

    let notify_send = notify_send_is_installed();
    if !notify_send { println!("notify-send не найден, уведомления не будут работать, установи пакет libnotify (или его аналог).") }
    let minutes = args().nth(1).unwrap_or_else(|| "25".to_string());
    let cycles = args().nth(2).unwrap_or_else(|| "1".to_string());
    let break_time_str = args().nth(3).unwrap_or_else(|| "5".to_string());
    let time = parse(&minutes, "число минут");
    let break_time = parse(&break_time_str, "время перерыва");

    let cycle= parse(&cycles, "количество циклов");
    match cycle {
        Ok(value) => {
            let cycle = value;
            for i in 1..=value {
                println!("Цикл {} из {}.", i, value);
                match time {
                    Ok(value) => {
                        timer(value, "работы", &notify_send);
                        if i != cycle {
                            match break_time {
                                Ok(value) => { timer(value, "перерыва", &notify_send) },
                                Err(ref error) => { println!("{}", error); },
                            }
                        }
                    }
                    Err(ref error) => {
                        println!("{}", error);
                    }
                }
            }
        }
        Err(error) => {
            println!("{}", error);
        }

    }
}

fn parse(num: &str, phrase: &str) -> Result<u64, String> {
    if let Ok(n) = num.parse::<u64>() {
        Ok(n)
    } else {
        Err(format!("Пожалуйста введите корректное {}.", phrase))
    }
}

fn timer(time: u64, phrase: &str, notify_send: &bool) {
    if time != 0 {
        println!("{} минут, время {}.", time, phrase);
        let total_seconds: u64 = time * 60;

        for remaining in (0..=total_seconds).rev() {
            let minutes = remaining / 60;
            let seconds = remaining % 60;

            print!("\r{}:{}", minutes, seconds);
            io::stdout().flush().unwrap();
            sleep(Duration::from_secs(1));
        }

        println!("\nВремя {} вышло.", phrase);
        if *notify_send {
            let notification = format!("Время {} вышло.", phrase);
            let _ = Command::new("notify-send").arg(notification).spawn();
        }
        let _ = Command::new("paplay")
            .arg("/usr/share/sounds/freedesktop/stereo/complete.oga")
            .spawn();
    } else {
        println!("Пожалуйста введите корректное число.")
    }
}

fn notify_send_is_installed() -> bool{
    match Command::new("sh")
        .args(["-c", "command -v \"$1\"", "--", "notify-send"])
        .status()
    {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}