# ferbot
6dof robot with rust


## Prepare:

* *__On Linux__*:

  ```commandline
  sudo apt-get install binutils gcc-avr avr-libc libudev-dev avrdude
  ```
* *__On Windows__*:

  Install avr-gcc toolkit (Example, from Arduino Studio).  
  Add in PATH avr-gcc.

```commandline
rustup override set nightly
rustup component add --toolchain nightly rust-src
cargo +stable install ravedude
```

## Build

```commandline
cargo build --release
```

## Install
```commandline
cargo run --release
```