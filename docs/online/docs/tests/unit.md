### What are they?

Unit tests are here to show that your functions are doing what you want them to do.

### How to write them?

You must integrate your tests within your module and your functions. For each file you want to test, create a module name `tests` as follow:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_name() {
        assert_type!(...);
    }
}
```
