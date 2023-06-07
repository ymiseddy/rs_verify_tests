# verify-tests 
<div align="center">

![Build](https://seddy.com/cicd/k987e/build.svg)
![Code](https://seddy.com/cicd/k987e/code.svg)
![Test Coverage](https://seddy.com/cicd/k987e/coverage.svg)
![Awesomeness](https://seddy.com/cicd/k987e/awesome.svg)

</div>


`verify-tests` is a Rust package specially designed to streamline and simplify
your testing process using the snapshot testing strategy. This package is
inspired by the powerful C# [Verify](https://github.com/VerifyTests/Verify)
package, bringing the same level of robustness and clarity to the Rust
ecosystem.

Snapshot testing is a method that compares the current output of your code with
a "snapshot" - a previously correct result. These snapshots allow for automated
verification of code changes, significantly reducing your time
on manual inspection.

With verify-tests, you can quickly generate and maintain these snapshots,
ensuring that your tests always reflect the up-to-date behavior of your
application. The package provides utilities to serialize your data structures
into JSON snapshots, compare them in your tests, and review the differences
using tools like vimdiff.

Whether you are refactoring your code, adding new features, or debugging,
verify-tests is an invaluable tool that helps you ensure your changes do not
unintentionally alter your application's behavior. You can focus more on
building amazing things and letting verify-tests handle the grunt work of verifying
your code.

## Project Status

`verify-tests` is a relatively new project, and we are actively developing and
refining its functionalities. Therefore, as an early-stage project, some features might be
subject to change as we work to improve and expand upon the package's
capabilities.

While we strive to ensure stability and functionality, users might encounter
unexpected behaviors or limitations in certain areas. We greatly appreciate your
understanding and patience in this regard.

### Installation

- @todo: This project is not currently published as a crate. We *could* indicate
  how to install it from GitHub directly, but it might be better to wait until
  it's ready for prime time.

### How Can You Contribute?

As a growing project, we warmly welcome contributions from the community. Your
contributions can be of various forms:

- **Testing**: Use `verify-tests` in your projects and provide us with feedback
  or report any issues that you encounter which will  helps us identify and rectify
  potential bugs or shortcomings.
  
- **Feature Suggestions**: If you have ideas for new features or improvements,
  don't hesitate to share them with us. Your insights could help make
  `verify-tests` more useful to everyone.

- **Code Contributions**: If you've implemented a new feature, fixed a bug, or
  improved the code in any way, you can submit a pull request. Before doing so,
  please ensure that your code meets the following quality guidelines:
  
  - **Adherence to Rust coding standards**: Please ensure your code follows the
    standard Rust coding guidelines. An excellent way to ensure this is to use
    `cargo clippy` on your code before submitting your PR, which will help
    highlight any potential issues or non-standard usage that might need to be
    addressed.
  
  - **Passing Tests**: Please ensure all existing tests pass with your changes.
    Please include appropriate tests for any new features or modifications to
    ensure their correct functioning. You can use `cargo test` in your local
    development environment to ensure existing and new tests pass.

- **Documentation**: Good documentation is just as necessary as good code. If
  you find a mistake, something unclear, or missing information in our
  documentation, we would be glad if you could help us improve it.

By participating, you not only get to help shape `verify-tests`, but also gain
an opportunity to contribute to an open-source project which could benefit many
developers in the Rust community.

## Example Usage

Using verify-tests involves creating a VerifyTest instance and calling its
assert_snapshot method. This example illustrates basic usage of the package in
a testing context:

```rust 

#[cfg(test)] 
mod tests { 
    use super::*; 
    use verify_tests::VerifyTest;

    // Define a path where snapshots will be stored.
    static TEST_PATH: &str = "snapshot_tests";

    #[test]
    fn test_new() {

        // Define a test data structure.
        struct Rectangle {
            width: u32,
            height: u32,
        }

        // Instantiate the test data structure.
        let rectangle = Rectangle {
            width: 30,
            height: 40
        };

        // Create a new VerifyTest instance.
        // This requires a path, which determines where the snapshots will be stored.
        let verify = VerifyTest::new(TEST_PATH);

        // Assert the snapshot.
        // This method generates a new snapshot or compares it with an existing one.
        verify.assert_snapshot("test_new", &rectangle);
    }
}
```

When you run this test for the first time, `verify-tests` will create a new
snapshot as a {test_name}.accepted.json file. This file initially
remains empty. At the same time, it will make a {test_name}.received.json file,
containing the serialized form of your rectangle instance in JSON.

You can now use the verify-review command to review the test results. It
provides a vimdiff comparison between the `.accepted.json` and `.received.json`
files.

Upon the test's subsequent executions, the serialized form of your test
structure gets compared with the content of `test_name}.accepted.json`. If they
match, the test passes. If not, {test_name}.received.json gets updated with the
new serialized data for further review.

## What to Commit 

The proper management of snapshot files is crucial to the effective use of
`verify-tests`. Here are some guidelines on what to commit to your source code
repository:

- **Commit `.accepted.json` files**: All the `*.accepted.json` files should be
  committed to your source code repository. These files are the accepted
  snapshots of your tests and serve as the "source of truth" for your test
  results. By committing these files, you ensure that the correct snapshots are
  available to all team members and that the test results remain consistent
  across different environments.

- **Ignore `.received.json` files**: on the other hand, you should ignore the
  `*.received.json` files. These files contain the serialized output of your
  most recent test run and `verify-tests` uses them for comparison with the
  `.accepted.json` files. Because they can change with each test run, they
  should not be committed to your source code repository. 

Here's a sample snippet you can add to your `.gitignore` file to ensure that
Git does not track the `.received.json` files:

```bash
# Ignore snapshot test 'received' files
*.received.json
```

Remember that using version control correctly with your snapshot files ensures
your tests are reproducible across different machines and developers. Moreover,
it can help track changes in the behavior of your code over time.

<a name="verifyreview"></a>
## The `verify-review` tool 

The `verify-review` tool is a powerful companion utility with
`verify-tests`. It enables you to review the snapshot test results and manage the
snapshot files. With `verify-review`, you can compare the `.received.json` and
`.accepted.json` files using vimdiff, and decide whether to accept or reject the
changes. This tool offers great control over the snapshots, allowing
you to handle test discrepancies efficiently.

### Usage

YOu can invoke `verify-review` from the command line with or without
optional arguments:

```
    verify-review [-c] [-a]
```

If no arguments are given, `verify-review` will use `vimdiff` to open every
pair of `.received.json` and `.accepted.json` files that do not match. You can
review the changes between the two versions in this visual interface. If you
save the file, `verify-review` will verify that the `.accepted.json` file now
matches the `.received.json file`. If they match, the `.received.json` file
will be deleted, signifying that the changes have been accepted. If you make no
changes or the files still don't match, you will can choose whether to delete
the `.received.json` file.


### Other Options

You can customize the behavior of verify-review by using the following
command-line options:

- `-c`, `--clean`: Use this option to delete all `.received.json` files without
  reviewing them. This option can be useful when you want to start afresh and
  rerun all tests, discarding previous discrepancies.
- `-a', `--accept`: This option lets you accept all changes in one go. When
  used, `verify-review` will replace all existing `.accepted.json` files with
  the corresponding `.received.json` files. This option is beneficial when
  you've made a batch of changes you want to accept all at once.


## TODO

- [ ] Add support for other diff tools (beyond vimdiff)


## License

This project is licensed under the MIT License - see the
[LICENSE.md](LICENSE.md) file for details

