use clap::Parser;

/// Simple program to convert a Cardnumber into all kinds of formats to help with creating a card format
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// cardno in DEC format that shall be converted
    cardno: i128,
}

fn main() {
    let args = Args::parse();
    let declength = args.cardno.to_string().len();
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
}
