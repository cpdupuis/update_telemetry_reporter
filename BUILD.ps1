$Env:RUSTFLAGS='-C target-feature=+crt-static'
cargo build --release

upx --best --lzma .\target\release\main.exe
