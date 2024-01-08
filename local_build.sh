#!/bin/bash
cd native; 
cargo build --release --lib;
cd ../; 
flutter_rust_bridge_codegen -r "native/src/api.rs" -d "./local/bin/gen_api.dart" --rust-output "native/src/gen_api.rs" --class-name apiClass; 
cd native; 
cargo build --release --lib; 
cd ..;

cp ./local/bin/gen_api.dart ./lib
