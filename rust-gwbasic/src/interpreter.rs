//! Interpreter for GW-BASIC

use crate::error::{Error, Result};
use crate::parser::{AstNode, BinaryOperator, UnaryOperator};
use crate::value::Value;
use std::collections::HashMap;
use std::io::{self, Write};

/// The GW-BASIC interpreter
pub struct Interpreter {
    /// Variable storage
    variables: HashMap<String, Value>,
    
    /// Program lines indexed by line number
    lines: HashMap<u32, Vec<AstNode>>,
    
    /// Current execution position
    current_line: Option<u32>,
    
    /// Call stack for GOSUB/RETURN
    call_stack: Vec<u32>,
    
    /// FOR loop stack
    for_stack: Vec<ForLoopState>,
}

#[derive(Debug, Clone)]
struct ForLoopState {
    variable: String,
    end_value: f64,
    step: f64,
    #[allow(dead_code)]
    return_line: u32,
}

impl Interpreter {
    /// Create a new interpreter
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            lines: HashMap::new(),
            current_line: None,
            call_stack: Vec::new(),
            for_stack: Vec::new(),
        }
    }

    /// Execute a program AST
    pub fn execute(&mut self, ast: AstNode) -> Result<()> {
        match ast {
            AstNode::Program(nodes) => {
                for node in nodes {
                    self.execute_node(node)?;
                }
            }
            _ => {
                self.execute_node(ast)?;
            }
        }
        Ok(())
    }

    /// Execute a single AST node
    fn execute_node(&mut self, node: AstNode) -> Result<()> {
        match node {
            AstNode::Program(nodes) => {
                // Execute all nodes in sequence
                for n in nodes {
                    self.execute_node(n)?;
                }
                Ok(())
            }
            AstNode::Line(num, statements) => {
                self.lines.insert(num, statements);
                Ok(())
            }
            AstNode::Print(exprs) => self.execute_print(exprs),
            AstNode::Let(name, expr) => self.execute_let(name, *expr),
            AstNode::If(condition, then_stmts, else_stmts) => {
                self.execute_if(*condition, then_stmts, else_stmts)
            }
            AstNode::For(var, start, end, step) => {
                self.execute_for(var, *start, *end, step.map(|s| *s))
            }
            AstNode::Next(var) => self.execute_next(var),
            AstNode::While(condition, statements) => {
                self.execute_while(*condition, statements)
            }
            AstNode::Goto(line) => self.execute_goto(line),
            AstNode::Gosub(line) => self.execute_gosub(line),
            AstNode::Return => self.execute_return(),
            AstNode::End => Err(Error::ProgramEnd),
            AstNode::Input(vars) => self.execute_input(vars),
            AstNode::Dim(name, dimensions) => self.execute_dim(name, dimensions),
            AstNode::Rem(_) => Ok(()), // Comments are no-ops
            _ => Err(Error::RuntimeError(format!("Cannot execute node: {:?}", node))),
        }
    }

    fn execute_print(&mut self, exprs: Vec<AstNode>) -> Result<()> {
        for (i, expr) in exprs.iter().enumerate() {
            let value = self.evaluate_expression(expr)?;
            print!("{}", value);
            
            if i < exprs.len() - 1 {
                print!(" ");
            }
        }
        println!();
        Ok(())
    }

    fn execute_let(&mut self, name: String, expr: AstNode) -> Result<()> {
        let value = self.evaluate_expression(&expr)?;
        self.variables.insert(name, value);
        Ok(())
    }

    fn execute_if(
        &mut self,
        condition: AstNode,
        then_stmts: Vec<AstNode>,
        else_stmts: Option<Vec<AstNode>>,
    ) -> Result<()> {
        let condition_value = self.evaluate_expression(&condition)?;
        let is_true = match condition_value {
            Value::Integer(i) => i != 0,
            Value::Single(f) => f != 0.0,
            Value::Double(d) => d != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Nil => false,
        };

        if is_true {
            for stmt in then_stmts {
                self.execute_node(stmt)?;
            }
        } else if let Some(else_statements) = else_stmts {
            for stmt in else_statements {
                self.execute_node(stmt)?;
            }
        }

        Ok(())
    }

    fn execute_for(
        &mut self,
        var: String,
        start: AstNode,
        end: AstNode,
        step: Option<AstNode>,
    ) -> Result<()> {
        let start_val = self.evaluate_expression(&start)?.as_double()?;
        let end_val = self.evaluate_expression(&end)?.as_double()?;
        let step_val = if let Some(s) = step {
            self.evaluate_expression(&s)?.as_double()?
        } else {
            1.0
        };

        // Initialize loop variable
        self.variables.insert(var.clone(), Value::Double(start_val));

        // Store loop state (in real implementation, would need to handle nested loops properly)
        let state = ForLoopState {
            variable: var,
            end_value: end_val,
            step: step_val,
            return_line: self.current_line.unwrap_or(0),
        };
        self.for_stack.push(state);

        Ok(())
    }

    fn execute_next(&mut self, var: String) -> Result<()> {
        if let Some(state) = self.for_stack.last() {
            if !var.is_empty() && state.variable != var {
                return Err(Error::RuntimeError(format!(
                    "NEXT variable mismatch: expected {}, got {}",
                    state.variable, var
                )));
            }

            let current = self.variables
                .get(&state.variable)
                .ok_or_else(|| Error::UndefinedError(format!("Variable {} not defined", state.variable)))?
                .as_double()?;

            let new_value = current + state.step;
            self.variables.insert(state.variable.clone(), Value::Double(new_value));

            // Check if loop should continue
            let should_continue = if state.step > 0.0 {
                new_value <= state.end_value
            } else {
                new_value >= state.end_value
            };

            if !should_continue {
                self.for_stack.pop();
            }
        } else {
            return Err(Error::RuntimeError("NEXT without FOR".to_string()));
        }

        Ok(())
    }

    fn execute_while(&mut self, condition: AstNode, statements: Vec<AstNode>) -> Result<()> {
        loop {
            let condition_value = self.evaluate_expression(&condition)?;
            let is_true = match condition_value {
                Value::Integer(i) => i != 0,
                Value::Single(f) => f != 0.0,
                Value::Double(d) => d != 0.0,
                Value::String(s) => !s.is_empty(),
                Value::Nil => false,
            };

            if !is_true {
                break;
            }

            for stmt in &statements {
                self.execute_node(stmt.clone())?;
            }
        }

        Ok(())
    }

    fn execute_goto(&mut self, line: u32) -> Result<()> {
        if self.lines.contains_key(&line) {
            self.current_line = Some(line);
            Ok(())
        } else {
            Err(Error::LineNumberError(format!("Line {} not found", line)))
        }
    }

    fn execute_gosub(&mut self, line: u32) -> Result<()> {
        if let Some(current) = self.current_line {
            self.call_stack.push(current);
        }
        self.execute_goto(line)
    }

    fn execute_return(&mut self) -> Result<()> {
        if let Some(return_line) = self.call_stack.pop() {
            self.current_line = Some(return_line);
            Ok(())
        } else {
            Err(Error::RuntimeError("RETURN without GOSUB".to_string()))
        }
    }

    fn execute_input(&mut self, vars: Vec<String>) -> Result<()> {
        for var in vars {
            print!("? ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .map_err(|e| Error::IoError(e.to_string()))?;

            let input = input.trim();

            // Try to parse as number first, then as string
            let value = if let Ok(i) = input.parse::<i32>() {
                Value::Integer(i)
            } else if let Ok(f) = input.parse::<f64>() {
                Value::Double(f)
            } else {
                Value::String(input.to_string())
            };

            self.variables.insert(var, value);
        }

        Ok(())
    }

    fn execute_dim(&mut self, _name: String, _dimensions: Vec<AstNode>) -> Result<()> {
        // DIM implementation would require array support
        // For now, just acknowledge it
        Ok(())
    }

    /// Evaluate an expression and return its value
    fn evaluate_expression(&mut self, node: &AstNode) -> Result<Value> {
        match node {
            AstNode::Literal(val) => Ok(val.clone()),
            AstNode::Variable(name) => {
                self.variables
                    .get(name)
                    .cloned()
                    .ok_or_else(|| Error::UndefinedError(format!("Variable {} not defined", name)))
            }
            AstNode::BinaryOp(op, left, right) => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.evaluate_binary_op(op, left_val, right_val)
            }
            AstNode::UnaryOp(op, expr) => {
                let val = self.evaluate_expression(expr)?;
                self.evaluate_unary_op(op, val)
            }
            AstNode::FunctionCall(name, args) => {
                self.evaluate_function_call(name, args)
            }
            _ => Err(Error::RuntimeError(format!("Cannot evaluate node: {:?}", node))),
        }
    }

    fn evaluate_binary_op(&mut self, op: &BinaryOperator, left: Value, right: Value) -> Result<Value> {
        match op {
            BinaryOperator::Add => {
                if left.is_string() || right.is_string() {
                    Ok(Value::String(format!("{}{}", left.as_string(), right.as_string())))
                } else {
                    Ok(Value::Double(left.as_double()? + right.as_double()?))
                }
            }
            BinaryOperator::Subtract => {
                Ok(Value::Double(left.as_double()? - right.as_double()?))
            }
            BinaryOperator::Multiply => {
                Ok(Value::Double(left.as_double()? * right.as_double()?))
            }
            BinaryOperator::Divide => {
                let right_val = right.as_double()?;
                if right_val == 0.0 {
                    Err(Error::DivisionByZero)
                } else {
                    Ok(Value::Double(left.as_double()? / right_val))
                }
            }
            BinaryOperator::Power => {
                Ok(Value::Double(left.as_double()?.powf(right.as_double()?)))
            }
            BinaryOperator::Equal => {
                Ok(Value::Integer(if left.as_double()? == right.as_double()? { -1 } else { 0 }))
            }
            BinaryOperator::NotEqual => {
                Ok(Value::Integer(if left.as_double()? != right.as_double()? { -1 } else { 0 }))
            }
            BinaryOperator::LessThan => {
                Ok(Value::Integer(if left.as_double()? < right.as_double()? { -1 } else { 0 }))
            }
            BinaryOperator::GreaterThan => {
                Ok(Value::Integer(if left.as_double()? > right.as_double()? { -1 } else { 0 }))
            }
            BinaryOperator::LessEqual => {
                Ok(Value::Integer(if left.as_double()? <= right.as_double()? { -1 } else { 0 }))
            }
            BinaryOperator::GreaterEqual => {
                Ok(Value::Integer(if left.as_double()? >= right.as_double()? { -1 } else { 0 }))
            }
            BinaryOperator::And => {
                let l = left.as_integer()?;
                let r = right.as_integer()?;
                Ok(Value::Integer(l & r))
            }
            BinaryOperator::Or => {
                let l = left.as_integer()?;
                let r = right.as_integer()?;
                Ok(Value::Integer(l | r))
            }
        }
    }

    fn evaluate_unary_op(&mut self, op: &UnaryOperator, val: Value) -> Result<Value> {
        match op {
            UnaryOperator::Negate => {
                Ok(Value::Double(-val.as_double()?))
            }
            UnaryOperator::Not => {
                Ok(Value::Integer(!val.as_integer()?))
            }
        }
    }

    fn evaluate_function_call(&mut self, name: &str, args: &[AstNode]) -> Result<Value> {
        // Implement built-in functions
        match name.to_uppercase().as_str() {
            "ABS" => {
                if args.len() != 1 {
                    return Err(Error::RuntimeError("ABS requires 1 argument".to_string()));
                }
                let val = self.evaluate_expression(&args[0])?;
                Ok(Value::Double(val.as_double()?.abs()))
            }
            "INT" => {
                if args.len() != 1 {
                    return Err(Error::RuntimeError("INT requires 1 argument".to_string()));
                }
                let val = self.evaluate_expression(&args[0])?;
                Ok(Value::Integer(val.as_double()? as i32))
            }
            "SQR" => {
                if args.len() != 1 {
                    return Err(Error::RuntimeError("SQR requires 1 argument".to_string()));
                }
                let val = self.evaluate_expression(&args[0])?;
                Ok(Value::Double(val.as_double()?.sqrt()))
            }
            _ => Err(Error::UndefinedError(format!("Function {} not defined", name))),
        }
    }

    /// Run a stored program starting from the first line
    pub fn run(&mut self) -> Result<()> {
        let mut line_numbers: Vec<u32> = self.lines.keys().copied().collect();
        line_numbers.sort();

        for line_num in line_numbers {
            self.current_line = Some(line_num);
            if let Some(statements) = self.lines.get(&line_num).cloned() {
                for stmt in statements {
                    if let Err(e) = self.execute_node(stmt) {
                        if matches!(e, Error::ProgramEnd) {
                            return Ok(());
                        }
                        return Err(e);
                    }
                }
            }
        }

        Ok(())
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_interpreter_creation() {
        let interp = Interpreter::new();
        assert_eq!(interp.variables.len(), 0);
    }

    #[test]
    fn test_execute_print() {
        let mut interp = Interpreter::new();
        let mut lexer = Lexer::new("PRINT 42");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        // Should not error
        assert!(interp.execute(ast).is_ok());
    }

    #[test]
    fn test_execute_let() {
        let mut interp = Interpreter::new();
        let mut lexer = Lexer::new("LET A = 42");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        interp.execute(ast).unwrap();
        assert_eq!(interp.variables.get("A").unwrap().as_integer().unwrap(), 42);
    }

    #[test]
    fn test_evaluate_expression() {
        let mut interp = Interpreter::new();
        let mut lexer = Lexer::new("LET A = 2 + 3 * 4");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        interp.execute(ast).unwrap();
        // 2 + 3 * 4 = 2 + 12 = 14
        assert_eq!(interp.variables.get("A").unwrap().as_integer().unwrap(), 14);
    }

    #[test]
    fn test_division_by_zero() {
        let mut interp = Interpreter::new();
        let mut lexer = Lexer::new("PRINT 1 / 0");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let result = interp.execute(ast);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::DivisionByZero);
    }

    #[test]
    fn test_variable_undefined() {
        let mut interp = Interpreter::new();
        let mut lexer = Lexer::new("PRINT X");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let result = interp.execute(ast);
        assert!(result.is_err());
    }
}