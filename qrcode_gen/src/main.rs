use substring::Substring;
use qrcode_generator::QrCodeEcc;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let check = url_check(&args[1]);

    if check == true {
        qrcode_generator::to_png_to_file(&args[1], QrCodeEcc::Low, 1024, "file_output.png").unwrap();
    }

}

fn url_check(url: &String) -> bool{
    if url.to_string().substring(0,4) != "http"  {
        dbg!(url.to_string().substring(0,3));
        println!("Please use a valid URL");
        false
    } else {
        true
    }
}