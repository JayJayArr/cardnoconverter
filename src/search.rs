//needle: decimal cardno, haystack: String of bits
use crate::print::{chars_to_ascii, chars_to_bcd};

pub fn search_in(needle: u64, haystack: &String) {
    println!(
        "\n[Search] Searching for {:?} in {:?} ... \n",
        needle, haystack
    );

    let reversehaystack = haystack.clone().chars().rev().collect::<String>();

    // Convert to bin, bcd, ASCII
    let bin = format_args!("{:b}", needle).to_string();
    let chars: Vec<char> = needle.to_string().chars().collect();
    let bcd = chars_to_bcd(chars.clone());
    let asciichars = chars_to_ascii(chars.clone());
    let asciibit: String = asciichars
        .iter()
        .map(|character| format!("{:b}", character).to_string())
        .collect();

    // Collect cases which need to be checked
    let variants = vec![
        (&bin, "binary".to_string()),
        (&asciibit, "ASCII".to_string()),
        (&bcd, "BCD".to_string()),
    ];
    // Check Haystack for all options forward and reverse
    for (option, format) in variants {
        if haystack.contains(option) {
            print_success(format, find_position(&needle, &haystack));
        } else if reversehaystack.contains(&bin) {
            print_success_reversed(format, find_position(&needle, &haystack));
        } else {
            print_failure(format);
        }
    }
}

fn print_success(format: String, position: usize) {
    println!("Found as {} at position {}!", format, position);
}
fn print_success_reversed(format: String, position: usize) {
    println!(
        "Found as {} at postion {} in the REVERSE haystack !",
        format, position
    );
}
fn print_failure(format: String) {
    println!("NOT Found as {}!", format);
}

fn find_position(needle: &u64, haystack: &String) -> usize {
    haystack.find(&needle.to_string()).unwrap_or(0)
}
