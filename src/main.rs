use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echoer")
        .version("0.1.0")
        .author("John Doe")
        .about("Does awesome things")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("The text to echo")
                .required(true),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Omit the trailing newline"),
        )
        .get_matches();

    println!("{:#?}", matches);
}
