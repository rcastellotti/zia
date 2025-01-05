mod config;
mod zia;
use clap::Parser;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    #[arg(short, long, default_value_t = String::from("zia.toml"), global=true)]
    config: String,

    #[arg(short, long)]
    json: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args: Args = Args::parse();

    let config = match config::Config::load_from_file(&args.config) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let rep = zia::run(&config);

    if args.json {
        rep.print_json();
    } else {
        rep.print_summary();
    }

    Ok(())
}
