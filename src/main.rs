mod fileops;

fn main() {
    println!("alwayspacking v{}", env!("CARGO_PKG_VERSION"));

    env_logger::init();
    let val: String  = fileops::get_default_config_file_location().unwrap();
    let config_file_contents = fileops::read_config_file(val.clone()).unwrap();

    println!("{}", config_file_contents);
}
