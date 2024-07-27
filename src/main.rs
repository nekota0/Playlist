use std::io;
use std::process;
use todo::*;

#[tokio::main]
async fn main() {
    let mut input = String::new();
    clear_screen();
    let _title = title().unwrap_or_else(|err| {
        eprintln!("Couldn't read the title of the program: {}", err);
    });
    let _initial_read = read_line_from_todo();

    
    loop {
        clear_screen();

        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let args: Vec<String> = input.split_whitespace()
                            .map(String::from)
                            .collect();

        let config = Config::new(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing argument: {}", err);
            process::exit(1);
        });

        if let Err(e) = run(config) {
            eprint!("error: {}", e);
            process::exit(1);
        }
    }
}


