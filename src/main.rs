use kyomu_regex::KyomuRegex;
use proconio::input;
use colored::*;

fn main() {
    println!("Input a pattern (e.g., {} ):", "a|b* (c|d)".cyan().bold());
    input! { pattern: String }
    println!("{}", "Input a string to match:");
    input! { text: String }
    match KyomuRegex::compile(&pattern) {
        Ok(regex) => {
            if regex.whole_match(&text) {
                println!("{}", "Matched!".green());
            } else {
                println!("{}", "Not matched.".red());
            }
        }
        Err(e) => {
            eprintln!("Error compiling regex: {}", e);
        }
    }
 }
