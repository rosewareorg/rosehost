# rosehost
website hosting program and an example ekerö app.

### setup
Pre-build make sure the directory "www" is in the "src" directory and that all resources using a "include_[...]!()" macro is already present.

Post-build make sure the "www" directory is in the same directory as the executable.

### usage
Pre-build:
```bash
cargo run "./src/www/"
```
Post-build:
```bash
./rosehost
```

### note
ekerö currently doesn't support: 
- https
