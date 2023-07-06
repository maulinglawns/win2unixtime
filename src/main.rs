use std::env;

const NANO: u64 = 10000000;
const MAGIC_NUMBER: u64 = 11644473600;

fn validate_win(win_sec: String) -> Result<u64, &'static str> {
    // Should be an integer with 18 digits
    if win_sec.parse::<u64>().is_err() {
        return Err("Incorrect input")
    }
    if win_sec.len() != 18 {
        return Err("Incorrect Windows time")
    }

    Ok(win_sec.parse::<u64>().unwrap())
}

fn convert_win(win_time: u64) -> u64 {
    // Return Windows time as Unix time
    (win_time / NANO) - MAGIC_NUMBER
}    

fn main() {
    // Get argument
    let args: Vec<String> = env::args().collect();
    let arg = &args[1];
    let win_epoch_time: u64 = validate_win(arg.to_string()).unwrap();
    let unix_time = convert_win(win_epoch_time);

    println!("{unix_time}");
}
