# smol egui skia

This is a drawing backend for [egui](https://github.com/emilk/egui) that uses [skia-safe](https://crates.io/crates/skia-safe). This library is NOT intended to enable interactive UI applications, it is only intended to allow for the creation of raster images using egui as layout engine. It is a fork of [egui_skia](https://github.com/lucasmerlin/egui_skia), which is a full-featured interactive UI library.

## Run the examples

```bash
cargo run --example rasterize --features cpu_fix
```

## Status

Rendering on the gpu works great, only the dancing strings example doesn't work for some reason.

For rendering on the cpu to look correct, the cpu_fix feature needs to be enabled. See https://github.com/lucasmerlin/egui_skia/issues/1 for more information.
