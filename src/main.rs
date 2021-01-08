use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(short = "v", long = "verbose", help = "Be verbose")]
    verbose: bool,
}

fn main() {
    let opt = Options::from_args();
    println!("Hello, world! {}", opt.verbose);
}
