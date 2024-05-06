# T5 Stack

- Install dependencies using `bun install`
- Watch and build TailwindCSS using `bun tailwind`

## API

- Run API server using `bun api`

## Web

> BUG: To compile this you must switch on the "web" feature of the `dioxus` package and remove the "mobile" one & also comment out the explicit `openssl` dependency from the `Cargo.toml`.

- Compile and run web app using `bun web`

## Android

> Works only with `openssl` installed with the `vendor` option (requires `perl` of the UNIX flavour).

You have to setup a `.cargo/config.toml` file in your home directory or in the project folder with the following content:

```toml
[target.aarch64-linux-android]
linker = "/<absolute-path-to-home>/<path-to-sdk>/ndk/<ndk-version>/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android<api-version>-clang"
```

Also you should setup all these environment variables in your terminal or in your `.bashrc` or `.zshrc` file:

```sh
export JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64
export ANDROID_HOME=$HOME/android
export ANDROID_SDK_ROOT=$ANDROID_HOME
export NDK_HOME=$ANDROID_HOME/ndk/26.3.11579264
export ANDROID_NDK_HOME=$NDK_HOME

export TOOLCHAIN=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64

# These two variables depend on the architecture of the device and the API version you are targeting
export TARGET=aarch64-linux-android
export API=33

export AR=$TOOLCHAIN/bin/llvm-ar
export CC=$TOOLCHAIN/bin/$TARGET$API-clang
export AS=$CC
export CXX=$TOOLCHAIN/bin/$TARGET$API-clang++
export LD=$TOOLCHAIN/bin/ld
export RANLIB=$TOOLCHAIN/bin/llvm-ranlib
export STRIP=$TOOLCHAIN/bin/llvm-strip

export PATH=$JAVA_HOME/bin:$ANDROID_HOME/cmdline-tools/latest/bin:$TOOLCHAIN/bin:$NDK_HOME:$ANDROID_HOME/platform-tools:$ANDROID_HOME/tools:$ANDROID_HOME/tools/bin:$PATH
```

- Compile and run Android app using `bun android`
- Connect to the local API server using `adb reverse tcp:8000 tcp:8000`

## Desktop

> Works with `openssl` installed with the `vendor` option and also without it.

- Compile and run desktop app using `bun desktop`
