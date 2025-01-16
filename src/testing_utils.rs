pub fn compare_output(expected: &str, source: &str) {
    let output = crate::run(source);
    assert_eq!(expected, output);
}
