# verify-tests - A rust package for snapshot testing

This is inspired by the C# [Verify](https://github.com/VerifyTests/Verify) library.

## Example Usage

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use verify_tests::VerifyTest;

    static TEST_PATH: &str = "snapshot_tests";

    #[test]
    fn test_new() {

        struct Rectangle {
            width: u32,
            height: u32,
        }

        let rectangle = Rectangle {
            width: 30,
            height: 40
        };

        // Instantiate a new VerifyTest - passing path where snapshots will be stored.
        let verify = VerifyTest::new(TEST_PATH);

        // Verify the snapshot.
        verify.assert_snapshot("test_new", &rectangle);
    }
}
```

On the first execution, an empty `{test_name}.accepted.json` will be created along with
a `{test_name}.received.json` file containing the serialization as `json`.

You can either replace the entire accepted file, or use your diff tool of choice to compare
the differences. 

On subsequent executions of test, the structure will be serialized and compared with 
`{test_name}.accepted.json}` - if they match, the test will pass. If not, a new 
`{test_name}.received.json` file will contain the new serialization.


## What to commit 

You sould commit all of the `*.accepted.json` to your source code repository, but ignore
the `*.received.json` files.

Sample to add to .gitignore:
```
*.received.json
```

## TODO's and wishlist
- [ ] It would be helpful to include a tool to automate the diff/verification process?
- [ ] Derive macro to automate the test name and verification process?
