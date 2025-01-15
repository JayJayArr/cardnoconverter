use std::char;

use clap::Parser;

/// Convert a Cardnumber into all kinds of formats to help with creating a card format
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// cardno in DEC format that shall be converted
    cardno: u64,

    /// optional: Data in which the cardnumber should be found, only binary is supported
    data: Option<String>,
}
//needle: decimal cardno, haystack: String of bits
fn search_in(needle: u64, haystack: &String) {
    println!(
        "\n[Search] Searching for {:?} in {:?} ... \n",
        needle, haystack
    );
    let chars: Vec<char> = needle.to_string().chars().collect();

    let reversehaystack = haystack.clone().chars().rev().collect::<String>();
    let mut asciichars: Vec<u8> = vec![];
    let bin = format_args!("{:b}", needle).to_string();
    let mut bcd: String = "".to_string();
    for char in chars.clone() {
        asciichars.push(char as u8);
        match char {
            '0' => bcd.push_str("0000"),
            '1' => bcd.push_str("0001"),
            '2' => bcd.push_str("0010"),
            '3' => bcd.push_str("0011"),
            '4' => bcd.push_str("0100"),
            '5' => bcd.push_str("0101"),
            '6' => bcd.push_str("0110"),
            '7' => bcd.push_str("0111"),
            '8' => bcd.push_str("1000"),
            '9' => bcd.push_str("1001"),
            _ => {}
        }
    }

    let asciibit: String = asciichars
        .iter()
        .map(|character| format!("{:b}", character).to_string())
        .collect();

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

    if haystack.contains(&bcd) {
        println!("Found as bcd!");
    } else if reversehaystack.contains(&asciibit) {
        println!("Found as bcd in the reverse haystack!");
    } else {
        println!("NOT Found as bcd!");
    }
}

fn main() {
    let args = Args::parse();
    let declength = args.cardno.to_string().len();
    let chars: Vec<char> = args.cardno.to_string().chars().collect();
    let mut asciichars: Vec<u8> = vec![];
    let mut hexlength = format!("{:X}", args.cardno).to_string().len();
    let orighexlength = hexlength;
    if hexlength % 2 == 1 {
        hexlength += 1;
    }
    let binlength = format!("{:b}", args.cardno).to_string().len();
    let mut bcd: String = "".to_string();
    for char in chars.clone() {
        asciichars.push(char as u8);
        match char {
            '0' => bcd.push_str("0000"),
            '1' => bcd.push_str("0001"),
            '2' => bcd.push_str("0010"),
            '3' => bcd.push_str("0011"),
            '4' => bcd.push_str("0100"),
            '5' => bcd.push_str("0101"),
            '6' => bcd.push_str("0110"),
            '7' => bcd.push_str("0111"),
            '8' => bcd.push_str("1000"),
            '9' => bcd.push_str("1001"),
            _ => {}
        }
    }

    println!("\n[Converting] Converting {:?} ...\n", args.cardno);
    println!("Dec : {}", format_args!("{:?}", args.cardno));
    println!(
        "Dec(reversed): {} ",
        format_args!(
            "{:?}",
            args.cardno.to_string().chars().rev().collect::<String>()
        )
    );
    println!("HEX: {}", format_args!("{:X}", args.cardno));
    println!(
        "HEX(reversed): {} ",
        format!("{:X}", args.cardno)
            .chars()
            .rev()
            .collect::<String>()
    );
    println!(
        "HEX(reversed bytewise): {} ",
        &format_args!("{:X}", args.cardno.swap_bytes()).to_string()[0..hexlength]
    );

    println!("Bin: {}", format_args!("{:b}", args.cardno));
    println!(
        "Bin(reversed): {} ",
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

    println!("BCD : {}", bcd);
    println!("BCD(reversed) : {}", bcd.chars().rev().collect::<String>());

    println!("\n[Length] Length of {:?} ...\n", args.cardno);
    println!(
        "Dec length (Characters): {}",
        format_args!("{:?}", declength)
    );
    if hexlength != orighexlength {
        println!(
            "Hex length (Bytes): {:?}, using complete bytes: {:?}",
            orighexlength, hexlength
        );
    } else {
        println!("Hex length: {:?}", hexlength);
    }

    println!("Bin length (Bit): {}", binlength);
    println!("BCD length (Bit): {}", bcd.to_string().len());
    //if there is a haystack, start the search
    if let Some(haystack) = args.data.as_deref() {
        search_in(args.cardno, &haystack.to_string());
    }
}
