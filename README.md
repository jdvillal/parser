# Expression Parser written in Rust

A simple mathematical expression parser and evaluator written in Rust using the **Pratt Parsing Algorithm**. It supports arithmetic operations, variable assignment, operator precedence, and a custom REPL loop.

---

## Features
- [x] Variable definitions and lookup
- [x] Arithmetic expressions
- [x] Assignment operator `=`
- [x] Operator precedence
- [x] Parentheses and grouping
- [x] Digit constants (`0`–`9`)
- [x] REPL with `exit`
- [x] Error handling for undefined variables and invalid input

---

## Parsing: Pratt Parsing Algorithm

This parser uses **Pratt parsing** (top-down operator precedence parsing), which is ideal for evaluating binary operations with different precedences.

The parser uses binding powers to:
- Determine precedence and associativity
- Resolve when to evaluate operators and sub-expressions
- Enable recursive expression construction

---

## Operator Precedence Table

| Operator | Binding Power (L, R) | Associativity |
| -------- | -------------------- | ------------- |
| `=`      | (0.2, 0.1)           | Right         |
| `+`, `-` | (1.0, 1.1)           | Left          |
| `*`, `/` | (2.0, 2.1)           | Left          |
| `^`, `√` | (3.1, 3.0)           | Right         |

---

## Technical Overview

This Rust program is a **simple recursive descent parser** and evaluator for **mathematical expressions** with support for variable assignments, operator precedence, and basic arithmetic operations.

---

### Top-Level Summary
- **Functionality:** Parses and evaluates mathematical expressions with variables
- **Supports:**
    - Variables (`a` to `z`, `A` to `Z`)
    - Assignments (`=`)
    - Operators: `+`, `-`, `*`, `/`, `^`, `√`
    - Operator precedence and associativity
    - Parentheses for grouping
    - REPL loop with `"exit"` command
---

### Main Functionality (`main`)

```rust
let mut variables: HashMap<char, f32> = HashMap::new();
```

- Stores variable assignments (`x = 3`, etc.)

```rust
loop {
    ...
}
```

- Infinite loop serving as a **Read-Eval-Print Loop (REPL)**.
- Prompts user, reads input, evaluates, and prints result.

```rust
if input.trim() == "exit" {
    break;
}
```

- Exits REPL on `"exit"`

```rust
let expr = Expression::from_str(&input);
```

- Parses input into AST using `Expression::from_str`

```rust
if let Some((var_name, lhs)) = expr.is_asign() {
    ...
}
```

- Handles assignments like `(= x 2)` by storing evaluated value

```rust
let value = expr.eval(&variables);
println!("{}", value);
```

- Evaluates and prints the result of the expression

---

### Token Representation: `Token` Enum

```rust
enum Token {
    Atom(char),  // Digits or variable names
    Op(char),    // Operators like +, -, *, etc.
    Eof,         // End of input
}
```

---

### Lexical Analyzer: `Lexer`

Converts a string input into a stream of tokens:
- Skips whitespace
- Categorizes characters into `Atom` or `Op`

```rust
fn new(input: &str) -> Lexer
fn next(&mut self) -> Token
fn peek(&mut self) -> Token
```

---

### Abstract Syntax Tree: `Expression`

```rust
enum Expression {
    Atom(char),                      // A digit or variable
    Operation(char, Vec<Expression>) // Operator and operands
}
```

- Represents parsed expressions as a `Expression` itself
- Display implementation prints LISP-style format

---

### Parser: `Expression::from_str` & `parse_expression`

- Uses **Pratt Parsing** with precedence climbing  
- `min_bp` ensures correct left/right associativity  
- Parses:
    - Atoms
    - Parentheses
    - Binary operations using `infix_binding_power`

---

### Binding Power Function

```rust
fn infix_binding_power(op: char) -> (f32, f32)
```

Defines operator precedence and associativity via left/right binding powers

---

### Assignment Detection: `is_asign`

```rust
fn is_asign(&self) -> Option<(char, &Expression)>
```

- Detects and validates expressions like `(x = 3)`
- Ensures valid variable and returns its name and value expression

---

### Evaluation: `eval`

```rust
fn eval(&self, variables: &HashMap<char, f32>) -> f32
```

- Recursively evaluates expressions
- Resolves:
    - Variables and digits
    - Arithmetic operators
- Operators:
    - `+`, `-`, `*`, `/` for basic arithmetic
    - `^` for exponentiation
    - `√` for nth root

---

## Limitations and Future Scope

- [ ] No support for multi-digit numbers or floating-point literals
- [ ] No unary operators
- [ ] Variables limited to one character
- [ ] All expressions must be parenthesized

---

## File Structure

```
├── src/
│   ├── main.rs       # REPL and evaluator
│   └── test.rs       # tests
├── .gitignore
├── Cargo.lock
├── Cargo.toml

```

---

## Getting Started
- Build and run the REPL:
```bash
cargo run
```

---

## Author(s)

### Expression Parser
- [Jorge Villalta @CoreDumpped](https://github.com/jdvillal/)

### README
- [Mohd Shamoon](https://github.com/MOHD-SHAMOON-04)

---
