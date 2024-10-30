@echo off
cargo build --features="windows_subsystem" --release --target x86_64-pc-windows-msvc
