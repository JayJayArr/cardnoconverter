use clap::Parser;

mod search;
use search::search_in;
mod print;
use print::print_info;

/// Convert a Cardnumber into all kinds of formats to help with creating a card format
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// cardno in DEC format that shall be converted
    cardno: u64,

    /// optional: Data in which the cardnumber should be found, only binary is supported
    data: Option<String>,
}

fn main() {
    let args = Args::parse();

    print_info(args.cardno);

    //if there is a haystack, start the search
    if let Some(haystack) = args.data.as_deref() {
        search_in(args.cardno, &haystack.to_string());
    }
}
