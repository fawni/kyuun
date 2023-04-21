# kyuun 🍒

A simple Spotify playlists analyzer

## Motivation

Sometimes i want to a quick summary about a playlist to ~~judge~~ get a general idea of what it offers. For me it's particularily interesting for the unique playlists that Spotify taylors to each user!

![scrot](assets/scrot.png)

> ###### i don't use spotify that much do NOT judge...

It simply lists the artists and how many times they appear in the playlist along with some other basic stats but i might add more stuff in the future!

i would also like to note that this was initially written as a quick and dirty little program for me but i ended up making it more accessible; the performance/code quality was never a concern, hence the plethora of `clone()`s.

## Installation

Static binaries are automatically released [here](https://github.com/fawni/kyuun/releases)

<!-- ### crates.io

```
cargo install kyuun
``` -->

### Build from source

```
cargo install --git https://github.com/fawni/kyuun
```

### Usage

1. Create a new app in the [Spotify dev dashboard](https://developer.spotify.com/dashboard)
2. Add `http://localhost:1410` as a Redirect URI and make note of `Client ID` and `Client secret`
3. Run `kyuun setup` and follow the instructions
4. Now you can use kyuun by running `kyuun` or `kyuun --id <playlist id>`

### License

[ISC](LICENSE)
