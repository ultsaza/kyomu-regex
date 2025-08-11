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
            let start = std::time::Instant::now();
            let is_matched = regex.whole_match(&text);
            let duration = start.elapsed();
            if is_matched {
                println!("{}", "Matched!".green());
            } else {
                println!("{}", "Not matched.".red());
            }
            println!("Duration: {:.8?}", duration.as_secs_f64());
        }
        Err(e) => {
            eprintln!("Error compiling regex: {}", e);
        }
    }
 }
