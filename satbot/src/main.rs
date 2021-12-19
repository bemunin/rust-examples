fn main() {
    let matches = satbot::build_cmdline().get_matches();

    satbot::run_task(&matches);
}
