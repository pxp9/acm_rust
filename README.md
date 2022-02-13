## MIT
  2 ### The MIT License
  3 [![License: MIT](https://img.shields.io/badge/License-MIT    -yellow.svg)](https://opensource.org/licenses/MIT)

## STARTING PROYECTS IN RUST
you can copy all starting proyects by this command 

```
git clone https://github.com/pxp9/acm_rust.git
```


## INSTALL RUST

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
## RUN AND COMPILE 

```
cd some_proyect
```

```
cargo run
```

## Some Proyects need Nightly Rust to compile in order to use it 
- Install Nightly Rust 
```
rustup toolchain install nightly
```
- Set Nightly Rust only for one Proyect
```
cd some_proyect
```

```
rustup override set nightly
```
- Set Nightly Rust default
```
rustup default nightly
```
- In order to view if nightly rust is set 
```
rustc --version
```

- Should Prompt something like this if is set
```
rustc 1.60.0-nightly (498eeb72f 2022-01-31)
```

- Also you could see all Rust versions
```
rustup toolchain list
```
