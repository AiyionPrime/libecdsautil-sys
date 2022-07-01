# libecdsautil-sys

Unsafe rust FFI bindings generated using the bindgen crate.

## usage

`cargo test`

`cargo docs`

## notes

Generally unsafe to use;
Better than nothing, but barely.

Uses a workaround to get rid of two erroneous re-exports of `stdint.h`.

Furthermore exports some typedefs of libuecc, which do not neccessarily belong in this crate and might be replaced at some pint.
