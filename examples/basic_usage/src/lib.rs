
static TEST_PATH: &str = "snapshot_tests";


// Sample struct to test - requires serde::Serialize
#[derive(serde::Serialize, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use verify_tests::VerifyTest;

    #[test]
    fn test_new() {
        let verify = VerifyTest::new(TEST_PATH);
        let rect = Rectangle::new(10, 20);
        verify.verify("test_new", &rect);
    }
}
