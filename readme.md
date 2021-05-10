# Rust with Android Studio


## Android Studio

* Android Studio version 4.2
* Android SDK Platform 30
* Android NDK 22.1.7171670
* Android SDK Build-Tools
* Android SDK Command-line Tools
* Android SDK Platform-Tools

![img](./sdk.png)



## Rust

Install rust on your PC from [rustup](https://rustup.rs), 
then add some Android targets (arm64, arm, x86_64, x86) for rust.
```
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```


## Python

Install [Python](https://www.python.org/downloads/) on your PC.


## References
- [Rust Android Gradle Plugin](https://github.com/mozilla/rust-android-gradle)
- [Deploying Rust application on Android](https://krupitskas.github.io/posts/quest-dev-part-2/)
- [One more plugin to build Rust for Android](https://dev.to/willir/one-more-plugin-to-build-rust-for-android-125h)
- [Cargo NDK for Android projects](https://github.com/willir/cargo-ndk-android-gradle)
- [Running Rust on Android](https://blog.svgames.pl/article/running-rust-on-android)


Done!
