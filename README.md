# 📦🌏 cyberdrop-dl - cyberdrop.me Downloader written in Rust 🦀
![cyberdrop-dl_demo](https://user-images.githubusercontent.com/4693125/125909983-6306d4e3-e377-41f4-aaf6-f03134203613.gif)

 > The fastest https://cyberdrop.me album downloader there is, written in Rust as an exercise.

### Usage

- Download **single album**
```
$ cyberdrop-dl https://cyberdrop.me/a/album1
```
- Download **multiple albums**
```
$ cyberdrop-downloader albums.txt
```
- or
```
$ cyberdrop-dl https://cyberdrop.me/a/album1 https://cyberdrop.me/a/album2
```
Files are saved in current working directory named './cyberdrop-dl'.

### How to install

**Recomended. Install using cargo. You need Rust toolchain installed, get it here @ https://rustup.rs/**

It's that simple
```
$ cargo install cyberdrop-dl
```

## Docker

No need to build and install via Docker

```
$ docker run -it --rm -v "$(pwd)"/cyberdrop-dl:/cyberdrop-dl:rw wmw9/cyberdrop-dl cyberdrop-dl https://cyberdrop.me/a/album
```
### TODO
- [x] Download multiple albums simultaneously
- [x] Download multiple album files in parallel
- [ ] Accept list of albums.txt via remote URL
- [ ] Custom destination directory via -o flag
- [ ] Integrate with Telegram Bot for easier usage
- [ ] Detect dublicate albums

# What I Learned 🧠
- Tokio runtime (using channels, green threads, Arc<>, Semaphore limits)
- HTML scraping
- Terminal UI (multiple progress bars, spinners)
- Async/Await
- Async I/O
- Working with filesystem
- Rust basics (HTTP requests, args parsing, error handling)
