# Examples
[operate.rs](operate.rs) does morphological operatin (dilation or erosion) on given image.

```bash
# this will dilate image
cargo run --release --example  operate -- benches/slice000.png d

# this will erode image
cargo run --release --example  operate -- benches/slice000.png e
```

[flip_bits.rs](flip_bits.rs) flips bits in given image

```bash
cargo run --release --example  flip_bits -- benches/slice000.png
```
