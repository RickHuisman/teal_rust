#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::fs;
    use std::io::Cursor;
    use walkdir::WalkDir;
    use teal::run_with_output;

    #[derive(PartialEq, Debug)]
    enum TestResult {
        Ok,
        CompileError,
        RuntimeError,
    }

    fn parse_expects(source: &str, regex: Regex, field: usize) -> Vec<String> {
        let mut results = vec![];
        for line in source.lines() {
            let caps = regex.captures(line);
            if let Some(caps) = caps {
                results.push(caps[field].to_owned());
            }
        }

        results
    }

    fn extract_expects(source: &str) -> TestResult {
        if !parse_expects(source, Regex::new(r"\[line (\d+)\] (Error.+)").unwrap(), 2).is_empty() {
            return TestResult::CompileError;
        }

        if !parse_expects(source, Regex::new(r"// (Error.*)").unwrap(), 1).is_empty() {
            return TestResult::CompileError;
        }

        if !parse_expects(
            source,
            Regex::new(r"// expect runtime error: (.+)").unwrap(),
            1,
        )
            .is_empty()
        {
            return TestResult::RuntimeError;
        }

        TestResult::Ok
    }

    fn execute(source: &str) -> (Vec<String>, TestResult) {
        match run_with_output(source) {
            Ok(output) => {
                (output, TestResult::Ok)
            },
            Err(err) => {
                println!("Runtime error: {:?}", err);
                (vec![], TestResult::RuntimeError)
            }
        }
    }

    fn harness(source: &str) {
        let expects = parse_expects(source, Regex::new(r"// expect: ?(.*)").unwrap(), 1);

        let expected_result = extract_expects(source);

        println!("Test case: {}", source);
        let (output, result) = execute(source);
        assert_eq!(expects, output);
        assert_eq!(expected_result, result);
    }

    fn run_test_file(path: String) {
        match fs::read_to_string(path) {
            Ok(source) => harness(&source),
            Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn run_tests() {
        // Runs every test in the test folder.
        for f in WalkDir::new("./tests/lang_tests").into_iter().filter_map(|e| e.ok()) {
            if f.metadata().unwrap().is_file() {
                let path = f.path().display().to_string();
                run_test_file(path);
            }
        }
    }
}