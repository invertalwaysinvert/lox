use std::{collections::HashMap, fmt::Write};

use crate::{
    callable::{LoxCallable, LoxFunction},
    class::LoxClass,
    environment::Environment,
    exceptions::Return,
    expr::{Expr, ExprVisitor, ExprVisitorAcceptor},
    stmt::{Stmt, StmtVisitor, StmtVisitorAcceptor},
    tokens::{LoxObject, Token, TokenType},
};

#[derive(Clone)]
pub struct Interpreter {
    pub environment: Environment,
    pub locals: HashMap<String, usize>,
    pub output: String,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Environment::new();
        Interpreter {
            environment: Environment::new_with_enclosing(globals),
            locals: HashMap::new(),
            output: String::new(),
        }
    }

    fn evaluate_expr(&mut self, expr: Expr) -> LoxObject {
        match expr {
            Expr::Binary(x) => self.evaluate(x),
            Expr::Grouping(x) => self.evaluate(x),
            Expr::Literal(x) => self.evaluate(x),
            Expr::Unary(x) => self.evaluate(x),
            Expr::Variable(x) => self.evaluate(x),
            Expr::Assign(x) => self.evaluate(x),
            Expr::Logical(x) => self.evaluate(x),
            Expr::Call(x) => self.evaluate(x),
            Expr::Get(x) => self.evaluate(x),
            Expr::Set(x) => self.evaluate(x),
            Expr::This(x) => self.evaluate(x),
            Expr::Super(x) => self.evaluate(x),
        }
    }

    fn execute_stmt(&mut self, statement: Stmt) -> Result<LoxObject, Return> {
        match statement {
            Stmt::Expression(x) => self.execute(x),
            Stmt::Print(x) => self.execute(x),
            Stmt::Var(x) => self.execute(x),
            Stmt::Block(x) => self.execute(x),
            Stmt::If(x) => self.execute(x),
            Stmt::While(x) => self.execute(x),
            Stmt::Fun(x) => self.execute(x),
            Stmt::Return(x) => self.execute(x),
            Stmt::Class(x) => self.execute(x),
        }
    }

    fn is_truthy(&self, object: LoxObject) -> LoxObject {
        match object {
            LoxObject::None => LoxObject::Bool(false),
            LoxObject::Bool(b) => LoxObject::Bool(b),
            _ => LoxObject::Bool(true),
        }
    }
    pub fn interpret(mut self, statements: Vec<Stmt>) -> String {
        for statement in statements {
            let _ = self.execute_stmt(statement);
        }
        self.output
    }

    pub fn resolve(&mut self, expr: Token, depth: usize) {
        self.locals.insert(expr.to_string(), depth);
    }

    pub fn execute_block(
        &mut self,
        statements: Vec<Stmt>,
        environment: Environment,
    ) -> Result<(), Return> {
        let previous = self.environment.clone();
        self.environment = environment;
        let mut response = Ok(());
        for stmt in statements {
            if let Err(x) = self.execute_stmt(stmt) {
                response = Err(x);
                break;
            }
        }
        self.environment = previous;
        response
    }

    fn lookup_variable(&mut self, expr: crate::expr::VariableExpr) -> LoxObject {
        match self.locals.get(&expr.to_string()) {
            Some(x) => {
                self.environment.get_at(*x, expr.name.lexeme)
                // self
                //             .environment
                //             .get(expr.name.lexeme.clone())
                //             .unwrap_or_else(|_| panic!("Undefined variable found: {}", &expr.name.lexeme))
            }
            None => self.environment.get(expr.name.lexeme).unwrap(),
        }
    }

    fn lookup_this(&mut self, expr: crate::expr::ThisExpr) -> LoxObject {
        match self.locals.get(&expr.name.to_string()) {
            Some(x) => {
                self.environment.get_at(*x, expr.name.to_string())
                // self
                //             .environment
                //             .get(expr.name.lexeme.clone())
                //             .unwrap_or_else(|_| panic!("Undefined variable found: {}", &expr.name.lexeme))
            }
            None => self.environment.get(expr.name.to_string()).unwrap(),
        }
    }
}

impl Interpreter
where
    Interpreter: ExprVisitor<LoxObject>,
{
    pub fn evaluate<A: ExprVisitorAcceptor<LoxObject>>(&mut self, expr: A) -> LoxObject {
        expr.accept(self)
    }
}

impl ExprVisitor<LoxObject> for Interpreter {
    fn visit_assign_expr(&mut self, expr: crate::expr::AssignExpr) -> LoxObject {
        let value = self.evaluate_expr(*expr.value.clone());

        if let Some(distance) = self.locals.get(&expr.to_string()) {
            self.environment
                .assign_at(*distance, expr.name, value.clone())
        }

        value
    }

    fn visit_literal_expr(&mut self, expr: crate::expr::LiteralExpr) -> LoxObject {
        expr.value
    }

    fn visit_variable_expr(&mut self, expr: crate::expr::VariableExpr) -> LoxObject {
        self.lookup_variable(expr)
    }

    fn visit_grouping_expr(&mut self, expr: crate::expr::GroupingExpr) -> LoxObject {
        self.evaluate_expr(*expr.expression)
    }

    fn visit_unary_expr(&mut self, expr: crate::expr::UnaryExpr) -> LoxObject {
        let right = self.evaluate_expr(*expr.right);

        match expr.operator.token_type {
            TokenType::Bang => match self.is_truthy(right) {
                LoxObject::Bool(true) => LoxObject::Bool(false),
                LoxObject::Bool(false) => LoxObject::Bool(true),
                _ => panic!("Not possible"),
            },
            TokenType::Minus => match right {
                LoxObject::Number(n) => LoxObject::Number(-1.0 * n),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    fn visit_binary_expr(&mut self, expr: crate::expr::BinaryExpr) -> LoxObject {
        let left = self.evaluate_expr(*expr.left);
        let right = self.evaluate_expr(*expr.right);
        match expr.operator.token_type {
            TokenType::Greater => LoxObject::Bool(left > right),
            TokenType::GreaterEqual => LoxObject::Bool(left >= right),
            TokenType::Less => LoxObject::Bool(left < right),
            TokenType::LessEqual => LoxObject::Bool(left <= right),
            TokenType::Minus => left - right,
            TokenType::Plus => left + right,
            TokenType::Slash => left / right,
            TokenType::Star => left * right,
            TokenType::BangEqual => LoxObject::Bool(left != right),
            TokenType::EqualEqual => LoxObject::Bool(left == right),
            _ => panic!(),
        }
    }

    fn visit_logical_expr(&mut self, expr: crate::expr::LogicalExpr) -> LoxObject {
        let left = self.evaluate_expr(*expr.left);
        if expr.operator.token_type == TokenType::Or {
            if let LoxObject::Bool(true) = self.is_truthy(left.clone()) {
                return left;
            }
        } else if let LoxObject::Bool(false) = self.is_truthy(left.clone()) {
            return left;
        }
        self.evaluate_expr(*expr.right)
    }

    fn visit_call_expr(&mut self, expr: crate::expr::CallExpr) -> LoxObject {
        let callee = self.evaluate_expr(*expr.callee);

        let mut arguments = Vec::new();
        for argument in expr.arguments {
            arguments.push(self.evaluate_expr(argument));
        }

        if let LoxObject::Callable(function) = callee {
            if arguments.len() != function.arity() {
                panic!("Unexpected number of arguments received");
            }
            function.call(self, arguments)
        } else if let LoxObject::Class(class) = callee {
            class.call(self, arguments)
        } else {
            panic!("Expression not of type LoxCallable")
        }
    }

    fn visit_get_expr(&mut self, expr: crate::expr::GetExpr) -> LoxObject {
        let object = self.evaluate_expr(*expr.object);

        if let LoxObject::Instance(instance) = object {
            return instance.get(expr.name);
        }

        panic!("Only instances have properties.")
    }

    fn visit_set_expr(&mut self, expr: crate::expr::SetExpr) -> LoxObject {
        let object = self.evaluate_expr(*expr.object);

        if let LoxObject::Instance(mut instance) = object {
            let value = self.evaluate_expr(*expr.value);
            instance.set(expr.name, value.clone());
            value
        } else {
            panic!("Only instance have fields!");
        }
    }

    fn visit_this_expr(&mut self, expr: crate::expr::ThisExpr) -> LoxObject {
        self.lookup_this(expr)
    }

    fn visit_super_expr(&mut self, expr: crate::expr::SuperExpr) -> LoxObject {
        if let Some(distance) = self.locals.get(&expr.keyword.lexeme) {
            let superclass = self
                .environment
                .get_at(*distance, "Super super ".to_string());

            let object = self
                .environment
                .get_at(distance - 1, "This this ".to_string());
            if let LoxObject::Class(func) = superclass {
                if let Some(method) = func.find_methods(&expr.method.lexeme) {
                    if let LoxObject::Instance(instance) = object {
                        return LoxObject::Callable(Box::new(method.bind(instance)));
                    }
                } else {
                    panic!("Undefined property '{}'", expr.keyword.lexeme);
                }
            }
        }
        LoxObject::None
    }
}

impl Interpreter
where
    Interpreter: StmtVisitor<LoxObject>,
{
    fn execute<A: StmtVisitorAcceptor<LoxObject>>(&mut self, stmt: A) -> Result<LoxObject, Return> {
        stmt.accept(self)
    }
}

impl StmtVisitor<LoxObject> for Interpreter {
    fn visit_expression_stmt(
        &mut self,
        stmt: crate::stmt::ExpressionStmt,
    ) -> Result<LoxObject, Return> {
        Ok(self.evaluate_expr(stmt.expression))
    }

    fn visit_print_stmt(&mut self, stmt: crate::stmt::PrintStmt) -> Result<LoxObject, Return> {
        let value = self.evaluate_expr(stmt.expression);
        self.output.push_str(&format!("{}\n", value));
        Ok(LoxObject::None)
    }

    fn visit_var_stmt(&mut self, stmt: crate::stmt::VarStmt) -> Result<LoxObject, Return> {
        let value = match stmt.initializer {
            Some(f) => self.evaluate_expr(f),
            None => LoxObject::None,
        };

        self.environment.define(stmt.name.lexeme, value);
        Ok(LoxObject::None)
    }

    fn visit_block_stmt(&mut self, stmt: crate::stmt::BlockStmt) -> Result<LoxObject, Return> {
        self.execute_block(stmt.statements, self.environment.clone())?;
        // TODO: Cloning the environment here is leading to weird behaviour where assign values to
        // variables inside a while block is not being reflected outside it
        Ok(LoxObject::None)
    }

    fn visit_if_stmt(&mut self, stmt: crate::stmt::IfStmt) -> Result<LoxObject, Return> {
        let value = self.evaluate_expr(stmt.condition);
        match self.is_truthy(value) {
            LoxObject::Bool(true) => self.execute_stmt(*stmt.then_branch)?,
            LoxObject::Bool(false) => {
                if let Some(s) = *stmt.else_branch {
                    self.execute_stmt(s)?
                } else {
                    LoxObject::None
                }
            }
            _ => panic!(),
        };
        Ok(LoxObject::None)
    }

    fn visit_while_stmt(&mut self, stmt: crate::stmt::WhileStmt) -> Result<LoxObject, Return> {
        loop {
            let value = self.evaluate_expr(stmt.condition.clone());
            match self.is_truthy(value) {
                LoxObject::Bool(true) => self.execute_stmt(*stmt.body.clone()).unwrap(),
                LoxObject::Bool(false) => break Ok(LoxObject::None),
                _ => panic!(),
            };
        }
    }

    fn visit_fun_stmt(&mut self, stmt: crate::stmt::FunStmt) -> Result<LoxObject, Return> {
        let fun_name = stmt.name.lexeme.clone();
        let function = LoxFunction::new(stmt, self.environment.clone(), false);
        self.environment
            .define(fun_name, LoxObject::Callable(Box::new(function)));
        Ok(LoxObject::None)
    }

    fn visit_return_stmt(&mut self, stmt: crate::stmt::ReturnStmt) -> Result<LoxObject, Return> {
        let mut output = LoxObject::None;
        if let Some(value) = *stmt.value {
            output = self.evaluate_expr(value)
        }
        Err(Return { value: output })
    }

    fn visit_class_stmt(&mut self, stmt: crate::stmt::ClassStmt) -> Result<LoxObject, Return> {
        let mut superclass = None;
        if let Some(superinit) = *stmt.superclass.clone() {
            let super_exp = self.evaluate_expr(superinit);
            match super_exp {
                LoxObject::Class(x) => superclass = Some(x),
                _ => panic!("Superclass must be a class."),
            }
        }
        self.environment
            .define(stmt.name.lexeme.clone(), LoxObject::None);
        if let Some(superinit) = *stmt.superclass.clone() {
            let super_exp = self.evaluate_expr(superinit);
            self.environment = Environment::new_with_enclosing(self.environment.clone());
            self.environment
                .define("Super super ".to_string(), super_exp);
        }
        let mut methods = HashMap::new();
        for method in stmt.methods {
            if let Stmt::Fun(stmt) = method {
                let function = LoxFunction::new(
                    stmt.clone(),
                    self.environment.clone(),
                    stmt.name.lexeme.eq("init"),
                );
                methods.insert(stmt.name.lexeme, function);
            } else {
                panic!("Invalid method found {}", stmt.name.lexeme);
            }
        }
        let class = LoxClass::new(stmt.name.lexeme.clone(), superclass, methods);

        if let Some(_superinit) = *stmt.superclass.clone() {
            if let Some(enclosing) = self.environment.enclosing.clone() {
                self.environment = *enclosing;
            }
        }

        self.environment
            .assign(stmt.name.lexeme, LoxObject::Class(class));
        Ok(LoxObject::None)
    }
}
