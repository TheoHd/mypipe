extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("mypipe")
        .version("1.0")
        .about("Pipe program")
        .author("Théo H")
        .arg(
            Arg::with_name("in")
                .short("i")
                .long("in")
                .value_name("INPUT")
                .help("INPUT CMD")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .value_name("OUTPUT")
                .help("OUTPUT CMD")
                .takes_value(true),
        )
        .get_matches();

    let input_cmd = match matches.value_of("in"){
        Some(x) => x,
        None => panic!("No input command given")
    };
    let output_cmd = match matches.value_of("out"){
        Some(x) => x,
        None => panic!("No output command given")
    };

    println!("{:?}\n{:?}", input_cmd, output_cmd);
}
