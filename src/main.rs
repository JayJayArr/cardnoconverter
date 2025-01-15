use std::char;

use clap::Parser;

/// Convert a Cardnumber into all kinds of formats to help with creating a card format
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// cardno in DEC format that shall be converted
    cardno: u128,

    /// optional: Data in which the cardnumber should be found, only binary is supported
    data: Option<String>,
}
//needle: decimal cardno, haystack: String of bits
fn search_in(needle: u128, haystack: &String) {
    println!("Searching for {:?} in {:?} ", needle, haystack);
    let chars: Vec<char> = needle.to_string().chars().collect();

    let reversehaystack = haystack.clone().chars().rev().collect::<String>();
    let needlestring: String = chars.iter().collect();
    let mut asciichars: Vec<u8> = vec![];
    let bin = format_args!("{:b}", needle).to_string();
    for char in chars.clone() {
        asciichars.push(char as u8);
    }

    let asciibit: String = asciichars
        .iter()
        .map(|character| format!("{:b}", character).to_string())
        .collect();

    let ascii: String = format_args!("{:X?}", asciichars.to_ascii_uppercase()).to_string();

    println!(
        "chars: {:?}, needlestring: {:?}, asciibit: {:?}, ascii: {:?}, bin: {:?}",
        chars, needlestring, asciibit, ascii, bin
    );

    if haystack.contains(&bin) {
        println!("Found as binary!");
    } else if reversehaystack.contains(&bin) {
        println!("Found as binary in the reverse haystack!");
    } else {
        println!("NOT Found as binary!");
    }
    if haystack.contains(&asciibit) {
        println!("Found as ascii!");
    } else if reversehaystack.contains(&asciibit) {
        println!("Found as ascii in the reverse haystack!");
    } else {
        println!("NOT Found as ascii!");
    }
}

fn main() {
    let args = Args::parse();
    let declength = args.cardno.to_string().len();
    let chars: Vec<char> = args.cardno.to_string().chars().collect();
    let mut asciichars: Vec<u8> = vec![];
    for char in chars {
        asciichars.push(char as u8);
    }
    let mut hexlength = format!("{:X}", args.cardno).to_string().len();
    let orighexlength = hexlength;
    if hexlength % 2 == 1 {
        hexlength += 1;
    }
    let binlength = format!("{:b}", args.cardno).to_string().len();

    println!("Dec: {}", format_args!("{:?}", args.cardno));
    println!(
        "Dec(reversed): {}",
        format_args!(
            "{:?}",
            args.cardno.to_string().chars().rev().collect::<String>()
        )
    );
    println!("HEX: {}", format_args!("{:X}", args.cardno));
    println!(
        "HEX(reversed): {}",
        format!("{:X}", args.cardno)
            .chars()
            .rev()
            .collect::<String>()
    );
    println!(
        "HEX(reversed bytewise): {}",
        &format_args!("{:X}", args.cardno.swap_bytes()).to_string()[0..hexlength]
    );

    println!("Bin: {}", format_args!("{:b}", args.cardno));
    println!(
        "Bin: {}",
        format!("{:b}", args.cardno)
            .chars()
            .rev()
            .collect::<String>()
    );
    println!(
        "ASCII (decimal): {}",
        format_args!("{:?}", asciichars.to_ascii_lowercase())
    );
    println!(
        "ASCII (hex): {}",
        format_args!("{:X?}", asciichars.to_ascii_uppercase())
    );

    println!("Dec length: {}", format_args!("{:?}", declength));
    if hexlength != orighexlength {
        println!(
            "Hex length: {:?}, using complete bytes: {:?}",
            orighexlength, hexlength
        );
    } else {
        println!("Hex length: {:?}", hexlength);
    }

    println!("Bin length: {}", binlength);
    //if there is a haystack, start the search
    if let Some(haystack) = args.data.as_deref() {
        println!("Haystack: {:?}", haystack);
        search_in(args.cardno, &haystack.to_string());
    }
}
