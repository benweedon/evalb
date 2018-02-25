#[macro_use]
extern crate clap;
extern crate evalb;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

use clap::App;
use evalb::Tree;

#[derive(Debug)]
enum DebugMode {
    None,
    Normal,
    Plus,
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .name(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches();

    let debug_mode = if matches.is_present("debug_mode") {
        DebugMode::Normal
    } else if matches.is_present("debug_mode_plus") {
        DebugMode::Plus
    } else {
        DebugMode::None
    };

    let _cut_len = matches.value_of("cut_off");
    let _max_error = matches.value_of("num_errors");

    if matches.is_present("param_file") {
        // read_parameter_file(matches.value_of("param_file"));
    }

    let gold_file = File::open(matches.value_of("gold_file").unwrap()).unwrap();
    let mut gold_lines = BufReader::new(gold_file).lines();
    let test_file = File::open(matches.value_of("test_file").unwrap()).unwrap();
    let mut test_lines = BufReader::new(test_file).lines();

    // init_global();
    // print_head();

    loop {
        // init();
        let gold_line = gold_lines.next();
        let test_line = test_lines.next();
        match (gold_line, test_line) {
            (Some(gold_line), Some(test_line)) => {
                let _gold_tree = Tree::from_string(&gold_line.unwrap());
                let _test_tree = Tree::from_string(&test_line.unwrap());
                // calc_result(gold_line, test_line);
                match debug_mode {
                    DebugMode::Normal | DebugMode::Plus => {
                        // dsp_info();
                    }
                    DebugMode::None => (),
                }
            }
            (None, None) => break,
            (Some(_), None) => {
                eprintln!("Number of lines mismatch (too many lines in gold file)");
                process::exit(1);
            }
            (None, Some(_)) => {
                eprintln!("Number of lines mismatch (too many lines in test file)");
                process::exit(1);
            }
        }
    }

    // print_total();
}
