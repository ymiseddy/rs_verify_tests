#[cfg(test)]
mod tests {
    use verify_tests::VerifyTest;
    use serde::{Serialize, Deserialize};
    use std::{fs, path::PathBuf};

    const VERIFY_DIR: &str = "snapshot_results/";
    const MISSING_DIR: &str = "snapshot_results/missing_dir";

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct SampleStruct {
        name: String,
        age: u32,
    }

    #[test]
    fn ensure_missing_path_is_created() {
        let missing_dir = PathBuf::from(MISSING_DIR);
        if missing_dir.exists() {
            fs::remove_dir_all(MISSING_DIR).unwrap();
        }
        
        VerifyTest::new(MISSING_DIR);
        assert!(missing_dir.exists(), "Snapshot directory should be created.");
    }
    

    #[test]
    fn ensure_matching_snapshot_file_works() {
        let verify = VerifyTest::new(VERIFY_DIR);
        let sample = SampleStruct {
            name: "Leroy".to_string(),
            age: 20,
        };

        verify.assert_snapshot("ensure_matching_snapshot_file_works", &sample);
    }
    
    #[test]
    #[should_panic]
    fn ensure_mismatched_snapshot_panics() {
        let verify = VerifyTest::new(VERIFY_DIR);
        let sample = SampleStruct {
            name: "Leroy".to_string(),
            age: 23,
        };

        verify.assert_snapshot("ensure_mismatched_snapshot_panics", &sample);
    }

    #[test]
    fn ensure_accepted_and_received_files_created_on_new_test() {
        let verify = VerifyTest::new(VERIFY_DIR);
        let sample = SampleStruct {
            name: "Leroy".to_string(),
            age: 23,
        };

        let accepted_file = PathBuf::from(format!("{}ensure_accepted_and_received_files_created_on_new_test.accepted.json", VERIFY_DIR));
        if accepted_file.exists() {
            fs::remove_file(&accepted_file).unwrap();
        }
        let received_file = PathBuf::from(format!("{}ensure_accepted_and_received_files_created_on_new_test.received.json", VERIFY_DIR));
     

        if received_file.exists() {
            fs::remove_file(&received_file).unwrap();
        }
        
        assert!(!accepted_file.exists(), "Accepted file should be deleted.");
        assert!(!received_file.exists(), "Received file should be deleted.");

        let _res = std::panic::catch_unwind(|| {
            verify.assert_snapshot("ensure_accepted_and_received_files_created_on_new_test", &sample);
        });

        assert!(accepted_file.exists(), "Accepted file should be created.");
        assert!(received_file.exists(), "Received file should be created.");
    }

}
