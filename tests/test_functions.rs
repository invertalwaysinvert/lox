#[cfg(test)]
mod test_functions {
    use lox::testing_utils::compare_output;

    #[test]
    fn test_function_declaration() {
        compare_output(
            "hi\n",
            "fun sayHi() {
                print \"hi\";
             }
             sayHi();",
        );
    }

    #[test]
    fn test_function_parameters() {
        compare_output(
            "3\n",
            "fun add(a, b) {
                print a + b;
             }
             add(1, 2);",
        );
    }

    #[test]
    fn test_function_return() {
        compare_output(
            "3\n",
            "fun add(a, b) {
                return a + b;
             }
             print add(1, 2);",
        );
    }

    #[test]
    fn test_function_closure() {
        compare_output(
            "1\n2\n",
            "fun makeCounter() {
                var i = 0;
                fun count() {
                    i = i + 1;
                    print i;
                }
                return count;
            }
            var counter = makeCounter();
            counter();
            counter();",
        );
    }
}
