use mmsdm::sql_server;
use mmsdm::GetTable;
use std::fs;
use tiberius;
use tokio::net;
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let mut config = tiberius::Config::new();

    config.host("127.0.0.1");
    config.port(1433);
    config.authentication(tiberius::AuthMethod::sql_server("SA", "12345Abcde"));
    config.encryption(tiberius::EncryptionLevel::Off);
    config.trust_cert();
    config.database("mmsdm");

    let tcp = net::TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    println!("TCP connected");

    let mut client = tiberius::Client::connect(config, tcp.compat_write()).await?;
    let paths = fs::read_dir("./data")?
        .into_iter()
        .map(|entry| entry.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()?;
    for path in paths {
        println!("Loading file at path: {:?}", path);
        let file = fs::File::open(path)?;
        let mut zip = zip::ZipArchive::new(file)?;
        let inner_file = zip.by_index(0)?;
        let mut aemo = mmsdm::AemoFile::from_reader(inner_file)?;
        dbg!(aemo.file_keys());
        sql_server::save_all(&mut aemo, None, &mut client, Some(10_000)).await?;
    }
    Ok(())
}
