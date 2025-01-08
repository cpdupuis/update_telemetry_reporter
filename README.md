# update-telemetry-reporter

A standalone tool for reporting on the status of updates, intended to be run
as a periodic task such as a cronjob, scheduled task, etc.

Currently a work in progress.

## Viewing uploaded telemetry

Telemetry is tagged as debug in order to be visible here: https://debug-ping-preview.firebaseapp.com/

Documentation on debug view tag: https://mozilla.github.io/glean/book/reference/debug/debugViewTag.html

## License

[MPL 2.0](http://mozilla.org/MPL/2.0/)

## How to build a statically-linked executable

Rust executables are "almost" statically linked by default, except for
the C runtime library. Here is how to build with static linkage of the runtime library:

```
$Env:RUSTFLAGS='-C target-feature=+crt-static'
cargo build --target x86_64-pc-windows-msvc --release
```

Reference: https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes

On Windows 11, here is a size comparison. 

Statically linked: 3593216 bytes
Dynamically linked: 3426304 bytes

## Shrinking the binary further

With these optimizations:

```
strip = true  # Automatically strip symbols from the binary.
opt-level = "z"  # Optimize for size.
lto = true
```

Statically-linked size gets down to 3408896 bytes

Then using UPX reduces size to 1246208 bytes


Removed dependency on tempfile New stats: raw exe is 3403776 bytes, packed with UPX to 1244672 bytes.

