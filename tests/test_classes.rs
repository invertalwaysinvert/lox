#[cfg(test)]
mod test_classes {
    use lox::testing_utils::compare_output;

    #[test]
    fn test_class_declaration() {
        compare_output(
            "<loxClass Bagel>\n",
            "class Bagel {}
             print Bagel;",
        );
    }

    #[test]
    fn test_class_instantiation() {
        compare_output(
            "<loxInstance Bagel>\n",
            "class Bagel {}
             var bagel = Bagel();
             print bagel;",
        );
    }

    #[test]
    fn test_class_methods() {
        compare_output(
            "hello\n",
            "class Greeter {
                sayHi() {
                    print \"hello\";
                }
             }
             var greeter = Greeter();
             greeter.sayHi();",
        );
    }

    #[test]
    fn test_class_this() {
        compare_output(
            "hello world\n",
            "class Greeter {
                init(name) {
                    this.name = name;
                }
                sayHi() {
                    print \"hello \" + this.name;
                }
             }
             var greeter = Greeter(\"world\");
             greeter.sayHi();",
        );
    }

    #[test]
    fn test_class_init() {
        compare_output(
            "initialized\n",
            "class Foo {
                init() {
                    print \"initialized\";
                }
             }
             var foo = Foo();",
        );
    }
}
