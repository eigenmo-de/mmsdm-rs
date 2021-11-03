use std::{
    fs,
    io::{self, BufRead, Read},
};
use tiberius;
use tokio::net;
use tokio_util::compat::Tokio02AsyncWriteCompatExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let mut config = tiberius::Config::new();

    config.host("192.168.145.92");
    config.port(1433);
    config.authentication(tiberius::AuthMethod::sql_server("SA", "12345Abcde"));
    config.encryption(tiberius::EncryptionLevel::Off);
    config.trust_cert();
    config.database("mmsdm");

    let tcp = net::TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = tiberius::Client::connect(config, tcp.compat_write()).await?;
    for entry in fs::read_dir("./data")? {
        let dir = entry?;
        let file = fs::File::open(dir.path())?;
        let br = io::BufReader::new(file);

        let aemo = aemo_rs::AemoFile::from_bufread(br)?;
        dbg!(aemo.file_keys());
        // dbg!(&aemo.header);
        // for (k, v) in &aemo.data {
        //     // dbg!(k);
        //     // dbg!(v.len());
        // }
        aemo.load_data(&mut client).await?;
    }
    Ok(())
}
