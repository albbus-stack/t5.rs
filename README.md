# t5.rs

<p align="center">
    <img height="30" src="https://img.shields.io/badge/Rust-f75208?style=for-the-badge&logo=rust&logoColor=white">&nbsp;
    <img height="30" src="https://img.shields.io/badge/Dioxus-00a8d6?style=for-the-badge&logo=rust">&nbsp;
    <img height="30" src="https://img.shields.io/badge/Diesel-9b0000?style=for-the-badge&logo=rust&logoColor=white">&nbsp;
    <img height="30" src="https://img.shields.io/badge/Supabase-3ecf8e?style=for-the-badge&logo=supabase&logoColor=white">&nbsp;
    <img height="30" src="https://img.shields.io/badge/Bun-14151a?style=for-the-badge&logoColor=fbf0df&logo=bun">&nbsp;
    <img height="30" src="https://img.shields.io/badge/Tailwind%20CSS-38b2ac?style=for-the-badge&logo=tailwind-css&logoColor=white">&nbsp;
    <img height="30" src="https://img.shields.io/badge/Fly.io-7c3aed?style=for-the-badge&logo=rust&logoColor=white">
    </br> </br>
    <img width="90%" src="https://github.com/albbus-stack/t5.rs/assets/57916483/481515a9-e6e6-4293-b649-58238e2707a2">
    </br> </br>
    An opinionated cross-platform full-stack application template developed with Rust,</br> Cargo Mobile 2, Dioxus, Warp, Diesel, PostgreSQL, Supabase Auth, Bun and TailwindCSS.
    </br>
    Inspired by an attempt at <i>rewriting in Rust</i> the <a href="https://github.com/timothymiller/t4-app">t4-app</a> project.
 </p>

## Development

- Install cargo-mobile2 using `cargo install --git https://github.com/tauri-apps/cargo-mobile2`
- Install cargo-watch using `cargo install cargo-watch`
- Install bun using `curl -fsSL https://bun.sh/install | bash` or `powershell -c "irm bun.sh/install.ps1 | iex"`
- Install `@material-tailwind/html` using `bun install`
- Watch and build TailwindCSS using `bun tailwind`

### Supabase Auth

- Create a new project on [Supabase](https://supabase.io/), this will be used for authentication, database integration & media storage.
- To setup Supabase Auth copy the `.env.example` file in a new `.env` file and fill in the `SUPABASE_URL`, `SUPABASE_API_KEY` and `SUPABASE_JWT_SECRET` fields with your Supabase credentials (found in the Supabase dashboard under project settings).

### Database

- To setup the database integration fill in the `DATABASE_URL` in the `.env` file with a PostgreSQL connection string (you can use the one provided by Supabase):

```sh
DATABASE_URL="postgres://postgres.<name>:<password>@<domain>:<port>/<database>"
```

- Install the Diesel CLI using `cargo install diesel_cli --no-default-features --features postgres`. You could run into some issues linking the postgres library, in that case you should install the `libpq-dev` package (or [Postgres](https://www.postgresql.org/download/windows/) for Windows) on your system and setup the correct `rustc` linker path search in the `.cargo/config.toml` file:

```toml
[target.x86_64-unknown-linux-gnu.pq]
rustc-link-search = ["/path/to/postgres/15/lib"]
rustc-link-lib = ["libpq"]

# or for Windows

[target.x86_64-pc-windows-msvc.pq]
rustc-link-search = ["C:\\path\\to\\postgres\\15\\lib"]
rustc-link-lib = ["libpq"]
```

- Run migrations using `bun migrate` or call directly the `diesel` CLI inside the `api` folder.

### Supabase Storage

- To setup Supabase Storage you should create two new buckets in the Supabase dashboard (e.g. a private bucket `images` and a public one `public-images`), then upload a file to them (e.g. `t5.png`).

- Make sure to allow read and/or write access to the private bucket for authenticated users, using Supabase RLS as follows:

<img src="https://github.com/albbus-stack/t5.rs/assets/57916483/c283ba4a-4b44-4a26-adfc-e2d8907a7f9c" width="70%">

- The example images (both public and authed) are instantiated inside the [`about` page](https://github.com/albbus-stack/t5.rs/blob/master/src/pages/about.rs).

## Platforms

### API

- Run the API server using `bun api` (or `cargo run` inside the `api` folder).

> All the below notes on various package versions and features are handled by the `bun web`, `bun android`, and `bun desktop` commands by automatically copying the correct `Cargo.toml` file to the project root so **beware of running multiple platorms at the same time**.

### Web

> To compile this you must switch on the `web` feature of the `dioxus` package and remove the `mobile` one. Works only without `openssl` installed with the `vendored` option in the `Cargo.toml`.

- Compile and run the web app using `bun web` (or `dx serve`).

### Desktop

> Works with `openssl` installed with the `vendored` option and also without it (builds significantly faster both on Windows and Linux).

- Compile and run the desktop app using `bun desktop` (or `dx serve --platform desktop`).

### Android

> Works only with `openssl` installed with the `vendored` option (requires `perl` of the UNIX flavour to build it).

- You have to create a `.cargo/config.toml` file in the project folder with the following content to setup the linker for the Android targets:

```toml
[target.aarch64-linux-android]
linker = "/<absolute-path-to-home>/<path-to-sdk>/ndk/<ndk-version>/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android<api-version>-clang"

[target.armv7-linux-androideabi]
linker = "/<absolute-path-to-home>/<path-to-sdk>/ndk/<ndk-version>/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7-linux-android<api-version>-clang"

[target.i686-linux-android]
linker = "/<absolute-path-to-home>/<path-to-sdk>/ndk/<ndk-version>/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android<api-version>-clang"

[target.x86_64-linux-android]
linker = "/<absolute-path-to-home>/<path-to-sdk>/ndk/<ndk-version>/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android<api-version>-clang"
```

- You should also setup all the below environment variables in your terminal or in your `.bashrc`/`.zshrc` file (or in the system environment for Windows) to compile the Android app:

```sh
# These two variables depend on the architecture of the device 
# and the API version you are targeting
export TARGET=aarch64-linux-android
export API=33

export JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64
export ANDROID_HOME=$HOME/android
export ANDROID_SDK_ROOT=$ANDROID_HOME
export NDK_HOME=$ANDROID_HOME/ndk/<your-ndk-version>
export ANDROID_NDK_HOME=$NDK_HOME

export TOOLCHAIN=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64
export AR=$TOOLCHAIN/bin/llvm-ar
export CC=$TOOLCHAIN/bin/$TARGET$API-clang
export AS=$CC
export CXX=$TOOLCHAIN/bin/$TARGET$API-clang++
export LD=$TOOLCHAIN/bin/ld
export RANLIB=$TOOLCHAIN/bin/llvm-ranlib
export STRIP=$TOOLCHAIN/bin/llvm-strip

export PATH=$JAVA_HOME/bin:$ANDROID_HOME/cmdline-tools/latest/bin:\
    $TOOLCHAIN/bin:$NDK_HOME:$ANDROID_HOME/platform-tools:\
    $ANDROID_HOME/tools:$ANDROID_HOME/tools/bin:$PATH
```

- Compile and run the Android app using `bun android` (or `cargo android run`).
- You can also debug the app wirelessly using `adb tcpip 5555` and `adb connect <device-ip>:5555` with the device temporarily connected via USB (this works great on WSL).
- Connect to the local API server using `adb reverse tcp:8000 tcp:8000` from the local machine.

## Production

- To produce optimized builds add the following code in the `.cargo/config.toml` file introduced above:

```toml
[profile.release]
opt-level = "z"
debug = false
lto = true
codegen-units = 1
panic = "abort"
strip = true
incremental = false
```

### API

- Install the `flyctl` CLI using `curl -L https://fly.io/install.sh | sh` and login with your [Fly.io account](https://fly.io) using `fly auth login`.
- Create a new Fly app inside the project folder using `fly launch` and following the prompts, using the existing `Dockerfile` and `fly.toml` files.
- Deploy the api using `fly deploy` or `bun deploy:api`. You can then change the `API_URL` in the `.env` file to the one deployed by Fly.

### Web

- To deploy the web app using Github Pages you have to add all the secrets in the repository settings (`APP_URL`, `API_URL`, `SUPABASE_URL`, `SUPABASE_API_KEY`, `SUPABASE_JWT_SECRET`, `DATABASE_URL`).
- Configure the `.github/workflows/web.yaml` file to run on every push to the `main` branch or manually with the `workflow_dispatch` event.
