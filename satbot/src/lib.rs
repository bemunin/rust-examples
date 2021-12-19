use clap::{App, Arg, ArgMatches, SubCommand};
use console::{style, Term};
use std::{thread, time::Duration};

const SATBOT_AVAILABLES: [&str; 3] = ["SB_Rob", "SB_Josh", "SB_Ann"];

// task selection
pub fn run_task(args: &ArgMatches) {
    if args.subcommand_matches("start").is_some() {
        start_satbot(args);
    } else if args.subcommand_matches("stop").is_some() {
        stop_satbot(args);
    } else if args.subcommand_matches("upload").is_some() {
        upload_to_satbot(args);
    }
}

// functionality
pub fn start_satbot(args: &ArgMatches) {
    let sub_args = args.subcommand_matches("start").unwrap();
    if sub_args.is_present("satbot_name") {
        // do some start logic here!
        println!(
            "Starting... Satbot {} is running now.",
            sub_args.value_of("satbot_name").unwrap()
        );
    } else {
        // do some start logic for all satbot here!
        println!("Starting... all satbots are running now.");
    }
}

pub fn stop_satbot(args: &ArgMatches) {
    let sub_args = args.subcommand_matches("stop").unwrap();
    if sub_args.is_present("satbot_name") {
        // do some stop logic here!
        println!(
            "Stoping... {} is shutdown.",
            sub_args.value_of("satbot_name").unwrap()
        );
    } else {
        // do some stop logic for all satbot here!
        println!("Stoping... All satbots are shutdown.");
    }
}

pub fn upload_to_satbot(args: &ArgMatches) {
    // extract args
    let sub_args = args.subcommand_matches("upload").unwrap();
    let filename = sub_args.value_of("filename").unwrap();
    let satbot_name = sub_args.value_of("satbot_name").unwrap();

    // setup pretty terminal

    let term = Term::stdout();
    let mut loading_progress: String = String::new();

    // execute
    println!("Uploading file {} to satbot {}.", filename, satbot_name);

    // do uploading logic here or inside the for loop
    for count in 0..20 {
        if count != 0 {
            term.clear_last_lines(1).unwrap();
        }
        loading_progress.push_str("#");
        term.write_line(&format!(
            "Loading: {}",
            style(loading_progress.as_str()).cyan()
        ))
        .unwrap();

        thread::sleep(Duration::from_millis(200));
    }
    println!("Finish Upload file to {} 🎉", satbot_name);
}

// Command-line builder
pub fn build_cmdline() -> App<'static, 'static> {
    App::new("SatBot")
        .version("1.0.0")
        .author("Be Munin")
        .about("Command-line app to connect SatBot in outerspace")
        .subcommand(build_subcmd_start())
        .subcommand(build_subcmd_stop())
        .subcommand(build_subcmd_upload())
}

fn arg_satbot_name(is_required: bool) -> Arg<'static, 'static> {
    Arg::with_name("satbot_name")
        .short("n")
        .long("satbot_name")
        .value_name("NAME")
        .help("Name of robot to connect")
        .required(is_required)
        .possible_values(&SATBOT_AVAILABLES)
        .takes_value(true)
}

fn build_subcmd_start() -> App<'static, 'static> {
    SubCommand::with_name("start")
        .about("start running satbot")
        .version("1.0.0")
        .arg(arg_satbot_name(false))
}

fn build_subcmd_stop() -> App<'static, 'static> {
    SubCommand::with_name("stop")
        .about("stop satbot")
        .version("1.0.0")
        .arg(arg_satbot_name(false))
}

fn build_subcmd_upload() -> App<'static, 'static> {
    SubCommand::with_name("upload")
        .about("upload script to satbot")
        .arg(arg_satbot_name(true))
        .arg(arg_filename(true))
}

fn arg_filename(is_required: bool) -> Arg<'static, 'static> {
    Arg::with_name("filename")
        .short("f")
        .long("filename")
        .help("file to upload to satbot")
        .required(is_required)
        .takes_value(true)
}
