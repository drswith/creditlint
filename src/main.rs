fn main() {
    if let Err(error) = creditlint::cli::run() {
        let exit_code = error.exit_code();
        if exit_code != 1 {
            eprintln!("{error}");
        }
        std::process::exit(exit_code);
    }
}
