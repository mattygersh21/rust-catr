fn main() {
    if let Err(e) = rust_catr::get_args().and_then(rust_catr::run) {
        eprintln!("{}",e);
        std::process::exit(1);
    }
}
