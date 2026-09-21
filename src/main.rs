//use clap::Parser;
use std::env::args;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let minutes = args().nth(1).unwrap_or_else(|| "25".to_string());
    let cycles = args().nth(2).unwrap_or_else(|| "1".to_string());

    let cycle = parse(&cycles, "количество циклов");
    match cycle {
        Ok(value) => {
            for i in 1..=value {
                println!("Цикл {} из {}", i, value);
                let time = parse(&minutes, "число.");
                match time {
                    Ok(value) => {
                        timer(value, "работы");
                        timer(5, "перерыва");
                    }
                    Err(error) => {
                        println!("{}", error);
                    }
                }
            }
        }
        Err(error) => {
            println!("{}", error);
        }

    }
    // if let Ok(cycle) = cycles.parse::<u64>() {
    //     for i in 1..=cycle {
    //         println!("Цикл {} из {}", i, cycle);
    //         if let Ok(time) = value.parse::<u64>() {
    //             timer(time, "работы");
    //             timer(5, "перерыва")
    //         } else {
    //             println!("Пожалуйста введите корректное число.")
    //         }
    //     }
    //     println!("Все циклы завершены.")
    // } else {
    //     println!("Пожалуйста введите корректное количество циклов.")
    // }
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
        let seconds: u64 = time * 60;
        for sec in (1..=seconds).rev() {
            println!("До конца {}: {}", phrase, sec);
            sleep(Duration::from_secs(1));
        }
        println!("Время {} вышло.", phrase);
    } else {
        println!("Пожалуйста введите корректное число.")
    }
}
