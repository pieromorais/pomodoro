#![allow(unused)]

use std::{io, process::abort};
use std::thread::sleep;
use std::time::Duration;

struct Clock {
    minutes: usize,
    seconds: usize,
}
impl Clock {
    fn minutes_down(&mut self){
        self.minutes = if self.seconds == 0 || self.seconds == 60 {self.minutes - 1} else {self.minutes};
    }

    fn seconds_down(&mut self){
        self.seconds = if self.seconds == 0 {59} else {self.seconds - 1};
    }

    fn exit_check(&self) -> bool{
        if self.seconds == 0 && self.minutes == 0 {
            println!("Congratulations 🎉🎉🎉");
            print!("\x07");
            return true;
        }
        false
    }
    fn format_time(number : usize) -> String{
        if number < 10usize{
            format!("0{}", number)
        }else {
            format!("{}", number)
        }

    }

    fn print_clock(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("Pomodoro🍎: {0}:{1}", Clock::format_time(self.minutes), Clock::format_time(self.seconds));
    }
}

fn main() {
    let initial_minutes : usize = menu();
    let mut clock = Clock{minutes: initial_minutes, seconds: 60};

    loop {
        sleep(Duration::new(1,0));
        clock.minutes_down();
        clock.seconds_down();
        clock.print_clock();
        if clock.exit_check() {
            break; 
        }
    } 
}



fn menu () -> usize {
    let mut option = String::new();

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear screen and send cursor to first col and row
        println!("1 - 15 min");
        println!("2 - 30 min");
        println!("3 - 45 min");
        println!("0 - Exit");

        io::stdin()
        .read_line(&mut option)
        .expect("Unable to read line");
        //change types - from string to usize
        //using the same variable name

        let option : usize = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            1 => {
                return 15
            }
            2 => {
                return 30
            }
            3 => {
                return 45
            }
            _ => break,
        }
     
    }    
   0 // return 0 for exit option
}

