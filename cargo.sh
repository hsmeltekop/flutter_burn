#RUST_BACKTRACE=full
#CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_DEBUG=true

# update these variables as you see fit
ANDROID_NDK_HOME=~/Android/Sdk/ndk;
export CARGO_NDK_ANDROID_PLATFORM=33

cd native;

targets=("armv7-linux-androideabi" "aarch64-linux-android")
for target in "${targets[@]}"
do
    cargo ndk -t $target -p $CARGO_NDK_ANDROID_PLATFORM  -o ../android/app/src/main/jniLibs build --release;
done

reset;