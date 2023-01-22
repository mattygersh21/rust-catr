fn main() {
    if let Err(e) = rust_catr::run() {
        eprintln!("{}",e);
        std::process::exit(1);
    }
}
