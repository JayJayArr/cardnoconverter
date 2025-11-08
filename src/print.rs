pub fn print_info(cardno: u64) {
    //DECIMAL and HEX
    let declength = cardno.to_string().len();
    let chars: Vec<char> = cardno.to_string().chars().collect();
    let mut hexlength = format!("{:X}", cardno).to_string().len();
    let orighexlength = hexlength;
    if hexlength % 2 == 1 {
        hexlength += 1;
    }
    let binlength = format!("{:b}", cardno).to_string().len();

    let bcd = chars_to_bcd(chars.clone());
    let asciichars = chars_to_ascii(chars.clone());

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
    //Length of everything
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

fn chars_to_bcd(chars: Vec<char>) -> String {
    let mut bcd: String = "".to_string();
    for char in chars.clone() {
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
    bcd
}

fn chars_to_ascii(chars: Vec<char>) -> Vec<u8> {
    let mut asciichars: Vec<u8> = vec![];
    for char in chars.clone() {
        asciichars.push(char as u8);
    }
    asciichars
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(vec!['1', '2', '3', '4'], "0001001000110100")]
    #[case(vec!['4', '3', '2', '1'], "0100001100100001")]
    #[case(vec!['6', '9', '4', '2', '0'], "01101001010000100000")]
    #[case(vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'], "0001001000110100010101100111100010010000")]
    fn test_chars_to_bcd(#[case] input: Vec<char>, #[case] expected: String) {
        assert_eq!(expected, chars_to_bcd(input))
    }
}
