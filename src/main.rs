#![allow(unused)]

use std::process::exit;
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

    fn exit_check(&self) -> bool{ // return true if the pomodoro finish count down
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

    fn print_rest(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("Pomodoro😴: {0}:{1}", Clock::format_time(self.minutes), Clock::format_time(self.seconds));
    }
}

fn main() {

    loop {
        let initial_minutes : usize = menu();

        if initial_minutes == 0 {exit(0)}

        let mut clock = Clock{minutes: initial_minutes, seconds: 60};

        loop {

            sleep(Duration::new(1,0));
            clock.minutes_down();
            clock.seconds_down();
            clock.print_clock();

            let mut control : bool = false;
            if clock.exit_check() {
                break;
            }
        } 

        let rest_minutes : usize = menu_rest();
        if rest_minutes == 0 {exit(0)}
                
        let mut rest_clock = Clock{
            minutes: rest_minutes, seconds: 60
        };
        loop {                    

        sleep(Duration::new(1,0));
        rest_clock.minutes_down();
        rest_clock.seconds_down();
        rest_clock.print_rest();

        //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            if rest_clock.exit_check() {

                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                break;

            }
        }
    }
}



fn menu () -> usize {
    let mut option = String::new();

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear screen and send cursor to first col and row
        println!("Choose your pomodoro time:");
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
                return 1
            }
            2 => {
                return 30
            }
            3 => {
                return 45
            }
            0 => return 0,
            _ => continue,
        }
     
    }    
   0 // return 0 for exit option
}

fn menu_rest () -> usize {
    loop {
    println!("Choose your rest time!");
    println!("1 - 5 min");
    println!("2 - 10 min");
    println!("3 - 15 min");
    println!("0 - Exit");
    
    let  mut rest_option = String::new();
    io::stdin()
    .read_line(&mut rest_option)
    .expect("Unable to read option");

    let rest_option : usize = match rest_option.trim().parse() {
        Ok(num) => num,
        Err(_) => continue
    };

    match rest_option {
        1 => {
            return 1
        }
        2 => {
            return 10
        }
        3 => {
            return 15
        }
        0 => return 0,
        _ => continue,
        }
    }
}
