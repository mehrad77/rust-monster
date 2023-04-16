mod dicer;
mod utilities;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Roll Dice", about = "Drop a string and roll the dice")]
struct Opt {
    #[structopt(
        short = "d",
        long = "dicer",
        help = "The string expression of the dice roll"
    )]
    dice: String,

    #[structopt(short = "v", long = "verbose", help = "Prints more details")]
    verbose: bool,
}

fn main() {
    let opt = Opt::from_args();

    let rolled_string = dicer::roll_dice(&opt.dice);
    if opt.verbose {
        // println!("dice: {}", dicer::normalize_dice_roll(&opt.dice).unwrap());
        println!("==>: {:?}", rolled_string);
    } else {
        println!("result: {:?}", rolled_string.result);
    }
}
