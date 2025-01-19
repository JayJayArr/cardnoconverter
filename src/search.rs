//needle: decimal cardno, haystack: String of bits

pub fn search_in(needle: u64, haystack: &String) {
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
