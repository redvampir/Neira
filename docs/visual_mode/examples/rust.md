# Rust

```rust
// @neyra:visual_block id="sum" display="Add" i18n.es="Suma" category="math" translator="Ana"
fn add(a: i32, b: i32) -> i32 {
    // @neyra:var id="a" display="First number"
    // @neyra:var id="b" display="Second number"
    let result = a + b;
    // @neyra:connection from="a" to="sum" category="data"
    // @neyra:connection from="b" to="sum" category="data"
    result
}
```
