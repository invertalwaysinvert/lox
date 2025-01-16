#[cfg(test)]
mod test_expressions {
    use lox::testing_utils::compare_output;

    #[test]
    fn is_working() {
        compare_output("2\n", "print 1 + 1;");
    }
}
