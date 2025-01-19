pub fn print(cardno: u64) {
    let declength = cardno.to_string().len();
    let chars: Vec<char> = cardno.to_string().chars().collect();
    let mut asciichars: Vec<u8> = vec![];
    let mut hexlength = format!("{:X}", cardno).to_string().len();
    let orighexlength = hexlength;
    if hexlength % 2 == 1 {
        hexlength += 1;
    }
    let binlength = format!("{:b}", cardno).to_string().len();
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

    println!("\n[Converting] Converting {:?} ...\n", cardno);
    println!("Dec : {}", format_args!("{:?}", cardno));
    println!(
        "Dec(reversed): {} ",
        format_args!("{:?}", cardno.to_string().chars().rev().collect::<String>())
    );
    println!("HEX: {}", format_args!("{:X}", cardno));
    println!(
        "HEX(reversed): {} ",
        format!("{:X}", cardno).chars().rev().collect::<String>()
    );
    println!(
        "HEX(reversed bytewise): {} ",
        &format_args!("{:X}", cardno.swap_bytes()).to_string()[0..hexlength]
    );

    println!("Bin: {}", format_args!("{:b}", cardno));
    println!(
        "Bin(reversed): {} ",
        format!("{:b}", cardno).chars().rev().collect::<String>()
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

    println!("\n[Length] Length of {:?} ...\n", cardno);
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
}
