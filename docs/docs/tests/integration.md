### What are they?

Integration tests aim to prove that a new module integrates well with others. It ensures that this module doesn't break the software.

### How to write?

To write integration tests in Rust you must write your tests in `tests/`. Create a new file with the name of the module you want to tests.

Then follow the example of the `hello.rs` file.

```rust
#[tests]
fn test_name {
    assert_test!(...);
}
```
