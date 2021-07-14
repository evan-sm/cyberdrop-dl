## cyberdrop-dl - cyberdrop.me Downloader written in Rust 🦀
![cyberdrop-dl_demo](https://user-images.githubusercontent.com/4693125/124804507-0e52dd00-df63-11eb-887c-65c8da36ed53.gif)

The fastest https://cyberdrop.me album downloader there is, written in Rust as an exercise.

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

## Docker one-liner

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

