# Rust

```rust
// @neyra:visual_block id="sum" display="Add" i18n.ru="Сложение" category="math" translator="Иван"
fn add(a: i32, b: i32) -> i32 {
    // @neyra:var id="a" display="First number" i18n.ru="Первое число"
    // @neyra:var id="b" display="Second number" i18n.ru="Второе число"
    let result = a + b;
    // @neyra:connection from="a" to="sum" category="data"
    // @neyra:connection from="b" to="sum" category="data"
    result
}
```
