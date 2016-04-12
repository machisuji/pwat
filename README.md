# pwat

[![travis ci](https://travis-ci.org/machisuji/pwat.svg?branch=master)](https://travis-ci.org/machisuji/pwat)

Prints bits of a password at the given 1-based indices.

## Example

```
> pwat 2 5 8
Enter password: hallowelt
lwt
```

The password not being displayed of course.

## Build

Built using cargo.

```
cargo build --release
```

Whereupon the executable will be created at `target/release/pwat`.
