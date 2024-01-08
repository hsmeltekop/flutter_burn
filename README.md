# mnist_burn

# rust in flutter:
## tutorials
* https://www.zaynetro.com/post/flutter-rust-bridge-2023#global-state-and-tokio
* https://blog.logrocket.com/using-flutter-rust-bridge-cross-platform-development/
  
## create model to use with flutter:
* inside `create_mnist_burn_model` run: `src/bin/create_model.rs`
* get `model.mpk.gz` and 

## setup flutter
* `dart create local`  # test api with dart locally
* `flutter pub add ffi`
* `flutter pub add ffigen --dev`
* `dart pub add flutter_rust_bridge:1.78.0`

## setup rust
* `cargo new native`
* `cargo add flutter_rust_bridge`
* `cargo install flutter_rust_bridge_codegen --force --version 1.78.0`
* in cargo.toml add: 
    > [lib]<br>
    > name = "native"<br>
    > crate-type = ["cdylib"]<br>
* `cargo install cargo-ndk`
* `echo "ANDROID_NDK=~/Android/Sdk/ndk/26.1.10909125" >> ~/.gradle/gradle.properties` 

## local build rust and dart:
* `./local_build.sh`
* test with dart: `dart ./local/bin/test_predictions.dart` or in one-go: `./build.sh && dart ./local/bin/test_predictions.dart`

## android build rust and flutter:
* `./cargo.sh` 
* `flutter clean && flutter build apk && flutter install`


## linker problems with android
*  `~/Android/Sdk/ndk/26.1.10909125/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/33` contains all libs that android uses, e.g. libnativewindow.so
*  linker in: `~/Android/Sdk/ndk/26.1.10909125/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android33-clang`

## create icons:
`dart run flutter_launcher_icons -f flutter_launcher_icons.yaml`

## Create splashscreen:
```bash
dart run flutter_native_splash:create --path=flutter_native_splash.yaml
``` 


## Burn related stuff
https://stackoverflow.com/questions/52353764/how-do-i-get-the-assets-file-path-in-flutter
convert logo.png -resize 48x48!  icon.png
convert logo.png -background "#a9a9a9" -alpha remove -alpha off -resize 1024x1024! gray_icon.png
