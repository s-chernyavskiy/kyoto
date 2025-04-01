use kyoto::cli::start;

fn main() {
    let result = start::setup();

    match result {
        Ok(_) => {}
        Err(e) => eprintln!(" an error occured:\n{}", e),
    }
}
