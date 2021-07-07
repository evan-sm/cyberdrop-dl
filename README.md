## cyberdrop-dl - cyberdrop.me Downloader written in Rust 🦀
![cyberdrop-dl_demo](https://user-images.githubusercontent.com/4693125/124804507-0e52dd00-df63-11eb-887c-65c8da36ed53.gif)

This is my own https://cyberdrop.me album downloader, written in Rust as an exercise.
It is my new favorite language <3 .

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
Files are saved in current working directory named 'cyberdrop-dl/'.

### How to install

**Install using cargo. Get it @ https://rustup.rs/ if you don't have it installed**

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
- Download multiple albums simultaneously
- Accept list of albums.txt via URL
- Custom destination directory via -o flag
