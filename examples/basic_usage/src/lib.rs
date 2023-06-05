
#[cfg(test)]
mod tests {
    use serde::Serialize;
    use verify_tests::VerifyTest;

    static TEST_PATH: &str = "snapshot_tests";

    #[test]
    fn test_new() {

        #[derive(Serialize,PartialEq)]
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

        // Assert snapshot - passing name of the test and the object to be serialized.
        // This will create an empty {test_name}.accepted.json if it doesn't exist.
        // If it does exist, it will compare the serialized object to the contents of the file.
        // If they are different, it will fail the test and a {test_name}.recieved.json 
        // will be created.
        verify.assert_snapshot("test_new", &rectangle);
    }
}
