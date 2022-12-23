use std::{collections::HashMap, path::PathBuf};

pub struct Interface;

impl Interface {
    // constants
    const HELP_TAG: &'static str = "-h";
    const PATH_TAG: &'static str = "-p";
    const OUTPUT_TAG: &'static str = "-o";

    const OUTFILE_NAME: &'static str = "out.html";
}

impl Interface {
    pub fn retrieve_program(parsed_args: HashMap<&str, PathBuf>) -> String {
        let in_file = parsed_args.get(Self::PATH_TAG).unwrap();

        std::fs::read_to_string(in_file).expect(
            format!(
                "Error: File not found at locaton {}\n",
                in_file.to_str().unwrap()
            )
            .as_str(),
        )
    }

    pub fn parse_cli_args(args: Vec<String>) -> HashMap<&'static str, PathBuf> {
        let mut parsed_args: HashMap<&str, PathBuf> = HashMap::new();

        if args.len() < 2 {
            Self::usage(Some(
                "ERROR: Error Parsing command line arguments \nNot enough arguments given",
            ));
            std::process::exit(1);
        }

        if args.contains(&Self::HELP_TAG.to_string()) {
            Self::usage(None);
            std::process::exit(0);
        }

        if args.contains(&Self::PATH_TAG.to_string()) {
            let flag_pos = args.iter().position(|a| a == Self::PATH_TAG).unwrap();

            parsed_args.insert(Self::PATH_TAG, args[flag_pos + 1].clone().into());
        } else {
            Self::usage(Some("Error: Error Parsing command line arguments \nNo input file given. Input file is required"));
            std::process::exit(1);
        }

        if args.contains(&Self::OUTPUT_TAG.to_string()) {
            let flag_pos = args.iter().position(|a| a == Self::OUTPUT_TAG).unwrap();

            parsed_args.insert(Self::OUTPUT_TAG, args[flag_pos + 1].clone().into());
        } else {
            let mut output_path = parsed_args.get(Self::PATH_TAG).unwrap().clone();

            output_path.pop();
            output_path.push(Self::OUTFILE_NAME);

            parsed_args.insert(Self::OUTPUT_TAG, output_path);
        }

        parsed_args
    }
}

impl Interface {
    fn usage(error_msg: Option<&str>) -> () {
        if error_msg.is_some() {
            eprintln!("{}", error_msg.unwrap());
        }
        eprintln!("USAGE: mlg [FLAGS]");
        eprintln!("FLAGS: -h Print this help message");
        eprintln!("       -p Designate the input file path");
        eprintln!("       -o Designate the output file path [OPTIONAL]");
    }
}
