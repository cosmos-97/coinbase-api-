mod prices;

use clap::Parser;
use prices::spot_prices;

#[derive(Parser, Default, Debug)]
struct Arguments {
    #[arg(short, long)]
    function_name: String,
    #[arg(short, long)]
    currency: String,
    #[arg(short, long)]
    base: String,
}

fn main() {
    let args = Arguments::parse();

    let base = args.base.to_uppercase();
    let currency = args.currency.to_uppercase();

    match &args.function_name[..] {
        "spot_prices" => spot_prices(base, currency),
        _ => panic!("Invalid Function Name!"),
    }
}
