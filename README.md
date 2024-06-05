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
    <img width="95%" src="https://github.com/albbus-stack/t5.rs/assets/57916483/481515a9-e6e6-4293-b649-58238e2707a2">
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
- Database access is gated to authenticated users but you can change that easily in the `api/src/handlers.rs` file by removing the JWT verification.

### Supabase Storage

- To setup Supabase Storage you should create two new buckets in the Supabase dashboard (e.g. a private bucket `images` and a public one `public-images`), then upload a file to them (e.g. `t5.png`).

- Make sure to allow read and/or write access to the private bucket for authenticated users, using Supabase RLS as follows:

<p align="center">
    <img src="https://github.com/albbus-stack/t5.rs/assets/57916483/c283ba4a-4b44-4a26-adfc-e2d8907a7f9c" width="80%">
</p>

- The example images (both public and authed) are instantiated inside the [about page](https://github.com/albbus-stack/t5.rs/blob/master/src/pages/about.rs).

## Platforms

### API

- Run the API server using `bun api` (or `cargo run` inside the `api` folder).

> All the below notes on various package versions and features are handled by the `bun web`, `bun desktop`, and `bun android` commands by automatically copying the correct `Cargo.toml` (`.web`, `.desktop` or `.mobile`) file to the project root so **beware of running multiple platorms at the same time**.

### Web

> To compile this you must switch on the `web` feature of the `dioxus` package and remove the `mobile` one. Works only without `openssl` installed with the `vendored` option in the `Cargo.toml`.

- Compile and run the web app using `bun web` (or `dx serve`).

### Desktop

> Works with `openssl` installed with the `vendored` option and also without it (builds significantly faster both on Windows and Linux).

- Install all the necessary prerequisites for the `dioxus-cli`, you can find them [here](https://dioxuslabs.com/learn/0.5/getting_started).
- You should install `llvm` on your system with `sudo apt install llvm-dev libclang-dev clang lib` on Linux, `winget install LLVM.LLVM` on Windows or `brew install llvm` on MacOS to compile the desktop app.
- You will also need to setup the `LIBCLANG_PATH` environment variable for Windows to point to the LLVM installation folder (e.g. `C:\Program Files\LLVM\bin`)
- Compile and run the desktop app using `bun desktop` (or `dx serve --platform desktop`).

### Android

> Works only with `openssl` installed with the `vendored` option (requires `perl` of the UNIX flavour to build it, this means you cannot use the Windows `perl` version directly). Avoid spaces in the absolute path of the project folder, this will cause a linking error while building `openssl`.

- You have to run `cargo mobile init` to generate only the `.cargo/config.toml` file and discard the other changes. If that errors out just create it with the following content to setup the linker for the Android targets:

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

- You should also setup all the below environment variables in your terminal or in your `.bashrc`/`.zshrc` file (or in the system environment for Windows) and install `gcc-multilib` with apt to compile the Android app:

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

- If you want to use Github Actions to deploy the app (API, Web & Desktop) you should add the following secrets in the repository settings (the above optimizations are already included in the workflows):

```text
API_URL (for the web app) -> update this after your API deployment
APP_URL (for the API) -> update this after your web deployment
SUPABASE_URL
SUPABASE_API_KEY
SUPABASE_JWT_SECRET
DATABASE_URL
```

### API

- Install the `flyctl` CLI using `curl -L https://fly.io/install.sh | sh` and login with your [Fly.io account](https://fly.io) using `fly auth login`.
- Create a new Fly app inside the project folder using `fly launch` and following the prompts, using the existing `Dockerfile` and `fly.toml` files.
- Deploy the API using `fly deploy` or `bun deploy:api`. You can then change the `API_URL` in the `.env` file to the one deployed by Fly.
- You can also deploy the API though Github Actions using the `./github/workflows/deploy-api.yml` workflow, adding to the repository secrets the `FLY_API_TOKEN` key, generated with `fly tokens create deploy -x 999999h`.

### Web

- To deploy the web app using [Github Pages](https://pages.github.com/) you have to choose in the repository settings under Pages the `gh-pages` branch. You should also set workflow permissions under the Actions settings to `Read and write permissions`.
- Change the `base_path` in the `Dioxus.pages.toml` file to match the name of your repository.
- Remember to deploy your api with `APP_URL` set to your Github Pages domain (e.g. `https://albbus-stack.github.io`) otherwise there will be CORS errors.
- Configure the `.github/workflows/deploy-web.yaml` workflow to deploy on every push to the `master` branch or manually with the `workflow_dispatch` event.

### Desktop

> The Github action for MacOS is not yet implemented. Linux is barely functioning since the built bundle has some bugs (e.g the installed .deb package searches assets in the folder where you call it and not globally). Windows is the only platform that bundles correctly for now.

- For Windows you can configure the `.github/workflows/windows.yaml` workflow to build and upload the `.msi` installer in the action artifacts on every push to the `master` branch or manually with the `workflow_dispatch` event.
- For Linux you can configure the `.github/workflows/linux.yaml` workflow to build and upload the `.deb` package along with the dist folder in the action artifacts on every push to the `master` branch or manually with the `workflow_dispatch` event.
- Currently `dx bundle` is being actively developed and is not yet ready for production use, so you should use `dx build` if bundling fails on other platforms (this is not good since you need attach the entire `dist` folder to your release build).

### Android

- You can build the Android app using the `bun build:android` command, this will generate various `.apk` files in subfolders of `gen/android/app/build/outputs/apk/` for each ABI.
- Generate a keystore file to sign the app using the `keytool` CLI as follows:

```sh
keytool -genkey -keystore release.keystore -alias alias_name -keyalg RSA -validity 10000
```

- You can then sign each generated `.apk` file using the `jarsigner` CLI as follows:

```sh
jarsigner -keystore release.keystore ./path/to/apk alias_name
```

- After signing an `.apk` you can install the app on your device physically or using `adb install ./path/to/apk` (you should sign and install an `.apk` compatible with the ABI of your device).

<!-- WIP Github Action -->
<!-- encrypt keystore locally with a password (ANDROID_KEYSTORE_PASSWORD): gpg -c --armor release.keystore => copy release-keystore.asc into ANDROID_KEYSTORE -->
<!-- secrets: ANDROID_KEYSTORE, ANDROID_KEYSTORE_PASSWORD, ANDROID_KEYSTORE_ALIAS -->
