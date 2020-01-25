# Kaios Rust Todo App

Based on https://github.com/kaiostech/sample-vanilla

## Build
Build the packaged app
```
cargo web deploy
```
App will be build into `target/deploy`

You need a Kaios device with debug mode enable in order to test on the device.

You can use the WebIDE and specify the `target/deploy` folder

Refer to [Kaios documentation](https://developer.kaiostech.com/getting-started/build-your-first-package-app/test-your-apps) for more informations

## Develop
You can use a web browser and cargo to make changes at a fast pace

```
cargo web start --auto-reload
```
Go to the specified URL and the viewport tool (Crtl+Shft+M on Firefox) to adjust the screensize.

Everytime you save a file the app should reload

## Limitations

- Currently only build with rust toolchain 1.39.0 (https://github.com/koute/cargo-web/issues/227)
- The todo app is buggy when the list go outside the viewport
- Todos are not stored
