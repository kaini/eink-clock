# E-Ink Clock

For details read [the articles on my website](https://me.pushrax.com/e-ink-clock/).

## Build Instructions

To be able to successfully build the project you need the `inkscape` and
`magick` executable and a ARM GCC toolchain in your path. The resulting binary
is ready to be flashed onto the controller.

```
cd timekeeper
xargo build --target lpc1227-none-eabi --release
```

To execute the tests, run `cargo test`.

There are some features you might want to activate.

* `semihosted`: Enable debug output via the semihosting mechanism.
* `fake_time`: Do not wait for a valid DCF77 time, but just assume a hardcoded time value.
* `stay_awake`: Panic instead of going into Deep Power-Down mode.

## Warning

The PCB layout is not up to date!
