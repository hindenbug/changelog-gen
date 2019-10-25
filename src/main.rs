use gitjournal::GitJournal;
use failure::Error;

fn run() -> Result<(), Error> {
    let mut journal = GitJournal::new("/Users/duilioruggiero/Documents/github/test/").unwrap();
    journal
        .parse_log("v1..v2", "nc", &0, &true, &true, None)
        .expect("Could not parse log.");
    journal
        .print_log(true, None, None)
        .expect("Could not print short log.");
    journal
        .print_log(false, None, None  )
        .expect("Could not print detailed log.");
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => println!("End!"),
        Err(e) => println!("error: {}", e),
    }
}
