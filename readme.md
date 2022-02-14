# Rust with Android Studio or Xcode iOS


## Android Studio

* Android Studio Arctic Fox | 2020.3.1 Patch 2
* Android SDK Platform 31
* Android NDK 23.1.7779620
* Android SDK Build-Tools 31.0.0
* Android SDK Command-line Tools
* Android SDK Platform-Tools

#### Config: Tools >> SDK Manager >>  SDK Tools (middle tab):

![img](./sdk.png)

#### Fixing build error for Above NDK 23 and more

find out all the 4 folders containing file `libunwind.a`, in my PC, it's `C:\Users\Administrator\AppData\Local\Android\Sdk\ndk\23.1.7779620\toolchains\llvm\prebuilt\windows-x86_64\lib64\clang\12.0.8\lib\linux\x86_64\` and more.
create 4 `text` files named `libgcc.a` in the folders with 
```
INPUT(-lunwind)
```

![image](https://user-images.githubusercontent.com/30760636/153782719-458e0054-a800-4a62-87b7-c3fa80a7008c.png)

[reference link](https://github.com/rust-windowing/android-ndk-rs/pull/189/files#diff-9c2f0d812ce52310e2784d3ba203437e6f318230e4e4f701b321a0f656dfa416R180-R187)

## Rust

Install rust on your PC from [rustup](https://rustup.rs), 
then add some Android targets (arm64, arm, x86_64, x86) for rust.
```
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```
Uses [rust-android-gradle](https://github.com/mozilla/rust-android-gradle) plugin, so is built with the command:
```cli
gradlew cargoBuild
```

### Function naming convention

In `src/lib.rs` you need to name the function according to the following naming convention in order to make it available in `Java`.

If the _Java_ function is called `greeting` and it is saved in a file named `RustBindings.java` pulled from package `com.krupitskas.pong` then in _Rust_ the function name is:

| Java |    package name     |   filename   | function name |
| :--: | :-----------------: | :----------: | :-----------: |
| Java | com_krupitskas_pong | RustBindings |   greeting    |

Which would look like this:

`Java_com_krupitskas_pong_RustBindings_greeting(...)`

## Python

Install [Python](https://www.python.org/downloads/) on your PC.


## iOS

- macOS / Xcode
- `curl https://sh.rustup.rs -sSf | sh`
- `rustup target add aarch64-apple-ios x86_64-apple-ios`
- `cargo install cargo-lipo`
- `cargo lipo --release`


## References
- [Rust Android Gradle Plugin](https://github.com/mozilla/rust-android-gradle)
- [Workshop: Use Rust in iOS Apps](https://github.com/thombles/dw2019rust)
- [One more plugin to build Rust for Android](https://dev.to/willir/one-more-plugin-to-build-rust-for-android-125h)
- [Cargo NDK for Android projects](https://github.com/willir/cargo-ndk-android-gradle)
- [Running Rust on Android](https://blog.svgames.pl/article/running-rust-on-android)
- [Building and Deploying a Rust library on iOS](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html)
- [Building and Deploying a Rust library on Android](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html)


Done!
