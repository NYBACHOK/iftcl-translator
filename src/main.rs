use clap::Parser;
use eyre::Result;
use iftcl_translator::{translate, Language};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(long)]
    text: String,

    #[arg(short, long, default_value = Option::None)]
    from: Option<Language>,

    #[arg(short, long, default_value = Option::None)]
    to: Option<Language>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let res = translate(
        args.text,
        args.from.unwrap_or(Language::English),
        args.to.unwrap_or(Language::Ukrainian),
    );

    println!("{res}");

    Ok(())
}
