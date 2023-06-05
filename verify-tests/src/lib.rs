use std::{fs, path::PathBuf};

pub struct VerifyTest {
    path: PathBuf,
}

impl VerifyTest {

    /**
     * Create a new VerifyTest instance
     *
     * @param path Directory where test results are stored
     */
    pub fn new(path: &str) -> Self {

        // Create test path if it doesn't exist.
        if !PathBuf::from(path).exists() {
            fs::create_dir_all(path).expect("Unable to create test directory");
        }

        VerifyTest {
            path: PathBuf::from(path),
        }
    }

    /**
     * Verify that the test result matches the accepted result
     *
     * @param fname Name of the test
     * @param result Result of the test
     * @return bool
     */
    pub fn assert_snapshot<T>(self: &VerifyTest, fname: &str, result: &T) -> bool
        where T: std::cmp::PartialEq + 
            serde::ser::Serialize {

        let received_text = serde_json::to_string_pretty(&result).unwrap()
            .trim().to_string();
    
        let accepted_path  = self.path.join(format!("{}.accepted.json", fname));
        let accepted_text = if !accepted_path.exists() {
            fs::write(&accepted_path, "").expect("Unable to write file");
            "".to_string()
        } else {
            fs::read_to_string(&accepted_path).expect("Unable to read accepted file")
        };

        // Don't count any whitespace around the JSON
        let accepted_text = accepted_text.trim();

        let matched = received_text.eq(&accepted_text);
        if !matched {
            let received_path  = self.path.join(format!("{}.received.json", fname));
            fs::write(&received_path, &received_text).expect("Unable to write received file");
        }
        assert!(matched, "Test result does not match accepted result - use verify-review to review.");
        matched
    }
}
