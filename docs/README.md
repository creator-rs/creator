# Creator

This file will cover some of the basics of Android and iOS development with `creator-rs`.

## Android development

## Build with Docker

Pre-requirements:

* Cloned `creator-rs` repository
* Docker

To run example Android application with `docker` you will need to run following command in `examples/app` folder of this project:

```sh
docker run --rm -it -v "$(pwd)/../../:/src" -w /src/examples/app docker.pkg.github.com/creator-rs/creator/creator cargo apk build
```

## Build with installed Android NDK

Run this command in `examples/app` folder of this project:

```sh
cargo apk build
```

## Run it on Android smartphone

To watch logs of the Android application run this:

```sh
adb logcat RustStdoutStderr:D '*:S'
```