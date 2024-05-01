use clap::{Parser, Subcommand};

mod fileops;
mod config;

#[derive(Parser)]
#[command(version, about, long_about = "An idimpotent package manager configuration manager")]
struct Args {
    /// Fully qualified path to source a valid .toml config file
    #[arg(short, long, value_name = "FILE")]
    source: Option<String>,
}

fn main() {
    env_logger::init();
    let arguments = Args::parse();
    
    /* Grab default config file and unmarshal */
    let config_path: String  = fileops::get_default_config_file_location().unwrap();
    let config_file_contents = fileops::read_config_file(config_path.clone()).unwrap();
    let config_data = config::unmarshal_toml_config_file(config_file_contents.clone()).unwrap();

 
}
