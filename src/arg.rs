use std::env;
use std::error::Error;
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn parse_args() -> Result<Vec<String>, Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut albums = Vec::<String>::new();

    for a in args {
        if a.contains("http://") || a.contains("https://") {
            albums.push(a);
        } else {
            let file_lines: Vec<String> = read_from_file(&a).await?;
            for l in file_lines {
                albums.push(l);
            }
        }
    }
    Ok(albums)
}

async fn read_from_file(fname: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut albums = Vec::<String>::new();
    let file = tokio::fs::File::open(fname)
        .await
        .expect("Failed to open file");

    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await.expect("Failed to read file") {
        albums.push(line);
    }
    Ok(albums)
}
