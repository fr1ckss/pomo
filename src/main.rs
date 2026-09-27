use std::env::args;
use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let minutes = args().nth(1).unwrap_or_else(|| "25".to_string());
    let cycles = args().nth(2).unwrap_or_else(|| "1".to_string());
    let time = parse(&minutes, "число.");

    let cycle = parse(&cycles, "количество циклов");
    match cycle {
        Ok(value) => {
            let cycle = value;
            for i in 1..=value {
                println!("Цикл {} из {}", i, value);
                match time {
                    Ok(value) => {
                        timer(value, "работы");
                        if i != cycle {
                            timer(5, "перерыва");
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
        Err(format!("Пожалуйста введите корректное {}", phrase))
    }
}

fn timer(time: u64, phrase: &str) {
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
        let notification = format!("Время {} вышло.", phrase);
        std::process::Command::new("notify-send").arg(notification).spawn().unwrap();
    } else {
        println!("Пожалуйста введите корректное число.")
    }
}
