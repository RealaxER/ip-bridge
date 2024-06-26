use clap::Parser;
use system_intergration::SystemIntergration;
pub mod devices;
pub mod error;
pub mod json;
pub mod logic;
pub mod sql;
pub mod system_intergration;
pub mod transport;
// use env_logger::Builder;
// use std::fs::File;
// use log::LevelFilter;
// use chrono::Local;
// use std::io::Write;



#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    db: String,
}


#[tokio::main]
async fn main() {

    env_logger::builder().format_timestamp_millis().init();

    let args = Args::parse();
    log::info!("args: {:?}", args);

    let mut system_intergration = SystemIntergration::new(args.db).await;
    system_intergration.init().await;

    loop {
        match system_intergration.recv().await {
            Ok(_) => {}
            Err(e) => {
                log::error!("{:?}", e);
                break;
            }
        }
    }
}
