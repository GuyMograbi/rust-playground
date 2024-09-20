
# Installation


```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Will add ~/.cargo/bin to PATH for the session, but you should really also add it through bashrc/profile files.
# ```export PATH=$PATH:~/.cargo/bin```

rustup update

rustc --version

rustup self uninstall
```


# Running

normally - you would compile `rustc hello-world.rs` and then run the binary `hello-world` program.

To make life easier - I defined Cargo.toml - and then we can simply run `cargo run --bin hello-world` (use the target name, not the path)



# TODO

 - [ ] Decide on structure convention. (use lib.rs, mod.rs or what?)
    - [ ] KPI - no mod in main.rs?
 - [X] Do POST request with body
 - [ ] Do DB calls (postgres?)
 - [x] Add middleware
 - [ ] Decide on unwrap_or_else etc functions. Best pattern.
 - [ ] upload a file
 - [X] Write a custom attribute (procedural macro)
 - [ ] Write a custom derive like #[derive(Debug)]
 - [ ] Write a macro


Resources
===
 - [Create your own macro: proc, dec and function](https://l4p1n.ch/2021/rust-writing-procedural-macros/)
 - [Create your own derive](https://users.rust-lang.org/t/how-do-i-create-a-custom-attribute-that-will-be-consumed-by-a-custom-derive-macro/43116)
