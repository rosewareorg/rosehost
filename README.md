# rosehost

Website hosting program and an example Ekerö app.

### Note:
Ekerö (v. 0.1.4) currently doesn't support: 
- search queries (in v. 0.1.5)
- https

### Setup:
Pre-build make sure the directory "www" is in the "src" directory and that all resources using a "include_[...]!()" macro is already present.

Post-build make sure the "www" directory is in the same directory as the executable.

### Useage:
Pre-build:
```bash
cargo run "./src/www/"
```
Post-build:
```bash
./rosehost
```

Read through the main script [here](https://github.com/rosewareorg/rosehost/blob/main/src/main.rs).
