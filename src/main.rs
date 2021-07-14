use cyberdrop_dl::arg::parse_args;
use cyberdrop_dl::download_albums;
use std::error::Error;
pub mod arg;

const HELP: &str = r#"Usage:
cyberdrop-dl https://cyberdrop.me/a/album1
cyberdrop-dl https://cyberdrop.me/a/album1 https://cyberdrop.me/a/album2
cyberdrop-dl albums.txt
cyberdrop-dl album.txt https://cyberdrop.me/a/album album2.txt
"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let albums = parse_args().await?;
    if albums.len() == 0 {
        println!("{}", HELP);
        std::process::exit(1);
    }
    println!("Albums to download: {}", albums.len());

    let job = tokio::spawn(async move {
        let _ = download_albums(albums).await;
    });
    let _ = tokio::join!(job);
    println!("Done!");
    Ok(())
}
