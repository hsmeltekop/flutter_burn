use std::{
    env,
    path::{Path, PathBuf},
};

fn msg(msg: String) {
    println!("cargo:warning=BUILD.RS: {msg}");
}

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "android" {
        android();
    }
}

fn android() {
    println!("cargo:rustc-link-lib=nativewindow");

    let p1 = env::var("CARGO_NDK_OUTPUT_PATH");
    // println!("cargo:warning=CARGO_NDK_OUTPUT_PATH: {:?}", p1);
    msg(p1.unwrap());

    //   env::set_var("CARGO_NDK_SYSROOT_LIBS_PATH", "/home/n1/Android/Sdk/ndk/26.1.10909125/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/33/");
    env::set_var("CARGO_NDK_SYSROOT_LIBS_PATH", "/home/n1/Android/Sdk/ndk/26.1.10909125/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/arm-linux-androideabi/33");
    msg(env::var_os("CARGO_NDK_SYSROOT_LIBS_PATH")
        .unwrap()
        .to_string_lossy()
        .to_string());

    if let Ok(output_path) = env::var("CARGO_NDK_OUTPUT_PATH") {
        let sysroot_libs_path = PathBuf::from(env::var_os("CARGO_NDK_SYSROOT_LIBS_PATH").unwrap());
        let lib_path = sysroot_libs_path.join("libnativewindow.so");

        msg(lib_path.to_string_lossy().to_string());

        msg(env::var("CARGO_NDK_ANDROID_TARGET").unwrap());

        std::fs::copy(
            lib_path,
            Path::new(&output_path)
                .join(&env::var("CARGO_NDK_ANDROID_TARGET").unwrap())
                .join("libnativewindow.so"),
        )
        .unwrap();
    }
}
