#[cfg(test)]
mod test_expressions {
    use lox::testing_utils::compare_output;

    #[test]
    fn test_arithmetic() {
        compare_output("4\n", "print 2 + 2;");
        compare_output("0\n", "print 2 - 2;");
        compare_output("6\n", "print 2 * 3;");
        compare_output("2\n", "print 4 / 2;");
    }

    #[test]
    fn test_grouping() {
        compare_output("7\n", "print (2 + 3) + 2;");
        compare_output("10\n", "print 2 * (3 + 2);");
    }

    #[test]
    fn test_literals() {
        compare_output("true\n", "print true;");
        compare_output("false\n", "print false;");
        compare_output("nil\n", "print nil;");
        compare_output("3.14\n", "print 3.14;");
        compare_output("hello\n", "print \"hello\";");
    }

    #[test]
    fn test_comparison() {
        compare_output("true\n", "print 1 < 2;");
        compare_output("true\n", "print 2 > 1;");
        compare_output("true\n", "print 1 <= 1;");
        compare_output("true\n", "print 1 >= 1;");
        compare_output("true\n", "print 1 == 1;");
        compare_output("false\n", "print 1 != 1;");
    }

    #[test]
    fn test_unary() {
        compare_output("false\n", "print !true;");
        compare_output("true\n", "print !false;");
        compare_output("-1\n", "print -1;");
    }

    #[test]
    fn test_precedence() {
        compare_output("7\n", "print 1 + 2 * 3;");
        compare_output("9\n", "print (1 + 2) * 3;");
        compare_output("true\n", "print !false == true;");
        compare_output("false\n", "print 2 < 1 == true;");
    }
}