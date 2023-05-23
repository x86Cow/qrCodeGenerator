use substring::Substring;
use qrcode_generator::QrCodeEcc;
use clap::Parser;

#[derive(Parser, Debug)]

#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,
    
    #[arg(short, long)]
    filename: String,

}

fn main() {
    let args = Args::parse();
    
    if args.url != "" {
        if url_check(&args.url) {
            qrcode_generator::to_png_to_file(&args.url, QrCodeEcc::Low, 1024, args.filename).unwrap();
        }
    } else {
        println!("Please enter a URL");
    }
}

fn url_check(url: &String) -> bool{
    if url.to_string().substring(0,4) != "http"  {
        println!("Please use a valid URL");
        false
    } else {
        true
    }
}