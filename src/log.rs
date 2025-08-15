use chrono::prelude::*;
use colored::Colorize;
use std::error::Error;

const TIMEZONE_OFFSET_SECONDS: i32 = -3 * 3600;

fn get_time_now() -> String {
    let time = Utc::now();
    let fixed_offset =
        FixedOffset::east_opt(TIMEZONE_OFFSET_SECONDS).unwrap();

    time.with_timezone(&fixed_offset)
        .format("%d/%m/%Y %H:%M:%S")
        .to_string()
}

pub fn info(msg: &str) {
    let time = get_time_now();
    println!("[{}][{time}] {msg}", "INFO".bright_cyan());
}

pub fn warn(msg: &str) {
    let time = get_time_now();
    let msg = format!("[{}][{time}] {msg}", "AVISO".yellow());
    println!("{msg}");
    write_to_log(&msg);
}

pub fn err(msg: &str, err: Box<dyn Error>) -> Box<dyn Error> {
    let time = get_time_now();
    let msg = format!("[{}][{time}] ({err}) {msg}", "ERRO".bright_red());
    eprintln!("{msg}");
    write_to_log(&msg);

    err
}

pub fn debug(msg: &str) {
    let time = get_time_now();
    let msg = format!("[DEBUG][{time}] {msg}").bright_black();
    eprintln!("{msg}");
}

fn write_to_log(msg: &str) {}