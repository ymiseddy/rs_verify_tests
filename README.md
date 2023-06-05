# verify-tests 

A rust package for tests using snapshots.  This is inspired by the C# 
[Verify](https://github.com/VerifyTests/Verify) package.

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

You can use the [verify-review](#verifyreview) command to review tests and compare them in `vimdiff`.

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

<a name="verifyreview"></a>

## The verify-review tool

The `verify-review` tool is used to review the snapshots and compare them in `vimdiff`.

### Usage

```
    verify-review [-c] [-a]
```

If no arguments are passed, the `verify-review` tool will open `vimdiff` for each `*.received.json` file which has a non-matching `*.accepted.json` file. You can then review the changes and decide whether to accept them or not.  

If you save the file, it will check to make sure the accepted file matches.  If so, the `*.received.json` file will be deleted.  If the files do not match, or no changes are made, you will be prompted if you want to remove the `*.received.json` file.

### Options

#### -c, --clean

Passing this argument will cause the `verify-review` tool to delete all `*.received.json` files without any reviewing them.  This can be useful if you want to re-run tests.

#### -a, --accept

Passing this argument will cause the `verify-review` tool to copy all `*.received.json` files to `*.accepted.json` files.  This can be useful if you want to accept all changes.

## TODO

- [ ] Add support for other diff tools (beyond vimdiff)


## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
