
#[cfg(test)]
mod test_statements {
    use lox::testing_utils::compare_output;

    #[test]
    fn test_print_statement() {
        compare_output("hello\n", "print \"hello\";");
        compare_output("42\n", "print 42;");
    }

    #[test]
    fn test_variable_declaration() {
        compare_output("42\n", "var a = 42; print a;");
        compare_output("nil\n", "var a; print a;");
        compare_output("hello\n", "var a = \"hello\"; print a;");
    }

    #[test]
    fn test_variable_assignment() {
        compare_output("43\n", "var a = 42; a = 43; print a;");
        compare_output("hello\nworld\n", 
            "var a = \"hello\"; 
             print a; 
             a = \"world\"; 
             print a;");
    }

    #[test]
    fn test_block_scope() {
        compare_output("inner\nouter\n",
            "var a = \"outer\";
             {
                var a = \"inner\";
                print a;
             }
             print a;");
    }

    #[test]
    fn test_if_statement() {
        compare_output("yes\n", 
            "if (true) print \"yes\";");
        compare_output("no\n",
            "if (false) print \"yes\"; else print \"no\";");
    }

    #[test]
    fn test_while_loop() {
        compare_output("0\n1\n2\n",
            "var i = 0;
             while (i < 3) {
                print i;
                i = i + 1;
             }");
    }

    #[test]
    fn test_for_loop() {
        compare_output("0\n1\n2\n",
            "for (var i = 0; i < 3; i = i + 1) {
                print i;
             }");
    }

    #[test]
    fn test_nested_blocks() {
        compare_output("inner\nmiddle\nouter\n",
            "var a = \"outer\";
             {
                var a = \"middle\";
                {
                    var a = \"inner\";
                    print a;
                }
                print a;
             }
             print a;");
    }

    #[test]
    fn test_if_else_chains() {
        compare_output("second\n",
            "if (false) {
                print \"first\";
             } else if (true) {
                print \"second\";
             } else {
                print \"third\";
             }");
    }

    #[test]
    fn test_while_break() {
        compare_output("0\n1\n",
            "var i = 0;
             while (i < 5) {
                print i;
                i = i + 1;
                if (i == 2) break;
             }");
    }
}