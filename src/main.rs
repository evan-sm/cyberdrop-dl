use cyberdrop_dl::arg::parse_args;
use cyberdrop_dl::download_album;
use std::error::Error;
mod arg;
//pub mod scrape;

const HELP: &str = r#"Usage:
cyberdrop-dl https://cyberdrop.me/a/album1
cyberdrop-dl https://cyberdrop.me/a/album1 https://cyberdrop.me/a/album2
cyberdrop-dl albums.txt
cyberdrop-dl album.txt https://cyberdrop.me/a/album album2.txt
"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args().await?;
    if args.len() == 0 {
        println!("{}", HELP);
        std::process::exit(1);
    }
    println!("Albums to download: {}", args.len());

    let job = tokio::spawn(async move {
        for a in args {
            let _ = download_album(a).await;
        }
    });
    tokio::join!(job);
    println!("Done!");
    Ok(())
}
