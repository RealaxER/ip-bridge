use clap::Parser;
use system_intergration::SystemIntergration;
pub mod devices;
pub mod error;
pub mod json;
pub mod logic;
pub mod sql;
pub mod system_intergration;
pub mod transport;
use env_logger::Builder;
use std::fs::File;
use log::LevelFilter;
use chrono::Local;
use std::io::Write;



#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    log: String,
    
    #[clap(short, long)]
    db: String,
}


#[tokio::main]
async fn main() {
    let args = Args::parse();
    log::info!("args: {:?}", args);

    let target = Box::new(File::create(args.log).expect("Can't create file"));
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(target))
        .filter(None, LevelFilter::Info)
        .init();

    let mut system_intergration = SystemIntergration::new(args.db).await;
    log::info!("Bui Dinh Hien");
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
