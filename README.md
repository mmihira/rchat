# rchat

A simple single server multiple client chat program I made to explore asynchronous programming in rust.

### Features

- Create chatrooms
- Join different chatrooms
- List available chatrooms

Multiple client single server.

### Usage

--help to show what cli commands are visible

#### Host

```
cargo build --release
./target/release/rchat -h localhost:20000
```

With logging enabled 

```
RUST_LOG=rchat=info ./target/release/rchat -h localhost:20000
```

#### Client

```
cargo build --release
./target/release/rchat-client -h localhost:20000 -n Bob
```

### Preview

![sdfsfd](https://thumbs.gfycat.com/TerribleLameAndeancat-size_restricted.gif)
