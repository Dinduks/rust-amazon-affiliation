use ini::Ini;
use std::env;

mod links;

fn main() {
    let links_file_name = env::args().nth(1);

    match links_file_name {
        Some(file_name) => {
            let maybe_links = Ini::load_from_file(file_name);

            match maybe_links {
                Ok(links_from_file) => {
                    let _links = links::ini_to_links(&links_from_file);
                },
                Err(_) => {
                    println!("Could not read the links file.");
                    std::process::exit(-1);
                },
            }
        },
        None => {
            println!("No links file specified.");
            std::process::exit(-1);
        },
    }
}
