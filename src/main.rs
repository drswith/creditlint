fn main() {
    if let Err(error) = creditlint::cli::run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
