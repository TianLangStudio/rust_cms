#! /bin/bash
cargo build --release --target x86_64-unknown-linux-gnu
# scp rust_cms_20200806p1.zip root@www.tianlang.tech:/home/www/web/rust_cms