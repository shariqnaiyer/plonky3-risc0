## Instructions

```bash
cargo build --target=riscv32im-risc0-zkvm-elf
```

## Error

```
error: target is not supported. You may need to define a custom backend see: https://docs.rs/getrandom/0.3.4/#custom-backend
   --> /home/snaiyer/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends.rs:204:9
    |
204 | /         compile_error!(concat!(
205 | |             "target is not supported. You may need to define a custom backend see: \
206 | |             https://docs.rs/getrandom/", env!("CARGO_PKG_VERSION"), "/#custom-backend"
207 | |         ));
    | |__________^

error[E0425]: cannot find function `fill_inner` in module `backends`
  --> /home/snaiyer/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/lib.rs:99:19
   |
99 |         backends::fill_inner(dest)?;
   |                   ^^^^^^^^^^ not found in `backends`

error[E0425]: cannot find function `inner_u32` in module `backends`
   --> /home/snaiyer/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/lib.rs:123:15
    |
123 |     backends::inner_u32()
    |               ^^^^^^^^^ not found in `backends`
    |
help: consider importing this function
    |
33  + use crate::util::inner_u32;
    |
help: if you import `inner_u32`, refer to it directly
    |
123 -     backends::inner_u32()
123 +     inner_u32()
    |

error[E0425]: cannot find function `inner_u64` in module `backends`
   --> /home/snaiyer/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/lib.rs:137:15
    |
137 |     backends::inner_u64()
    |               ^^^^^^^^^ not found in `backends`
    |
help: consider importing this function
    |
33  + use crate::util::inner_u64;
    |
help: if you import `inner_u64`, refer to it directly
    |
137 -     backends::inner_u64()
137 +     inner_u64()
    |
```
