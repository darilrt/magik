# Magik 🪄

A templating library for Rust that allows creating compile-time safe templates with type-safe syntax and powerful features.

## Table of Contents

- [Magik 🪄](#magik-)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
    - [Running the Examples](#running-the-examples)
  - [Installation](#installation)
  - [Basic Usage](#basic-usage)
    - [1. Define a structure with template](#1-define-a-structure-with-template)
    - [2. Create the template file](#2-create-the-template-file)
    - [3. Render the template](#3-render-the-template)
  - [Template Syntax](#template-syntax)
    - [Variable Interpolation](#variable-interpolation)
    - [Global Scope Statements](#global-scope-statements)
    - [Conditional Logic with Choosable](#conditional-logic-with-choosable)
    - [Complex Rust Logic](#complex-rust-logic)
    - [Using Other Components](#using-other-components)
  - [System Components](#system-components)
    - [1. Parser (`magik::Parser`)](#1-parser-magikparser)
    - [2. TemplateData](#2-templatedata)
    - [3. Trait Renderable](#3-trait-renderable)
    - [4. Trait Choosable](#4-trait-choosable)
  - [Macros](#macros)
    - [`#[template(path = "path")]`](#templatepath--path)
    - [`#[template(source = "template")]` or `#[template_str("template")]`](#templatesource--template-or-template_strtemplate)
  - [Advantages](#advantages)
  - [Limitations](#limitations)
  - [Development](#development)
    - [Automatic Template Recompilation](#automatic-template-recompilation)
  - [Project Structure](#project-structure)

## Features

- **Compile-time compilation**: Templates are compiled at compile time, detecting errors before execution
- **Familiar syntax**: Uses `{{ }}` syntax to insert Rust code
- **Renderable trait**: Automatic implementation for common types
- **Choosable trait**: Enables elegant conditional logic in templates
- **Procedural macros**: Facilitates template component creation

### Running the Examples

All examples are located inside the `examples/` folder of the `magik_macro` crate.

To run a specific example, use the following command from the root of the repository:


```bash
cargo run -p magik-macro --example hello_magik
```

Expected output:
```
Hello from Magik!
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
magik = { git = "https://github.com/darilrt/magik", package = "magik" }
magik-macro = { git = "https://github.com/darilrt/magik", package = "magik_macro" }
```

## Basic Usage

### 1. Define a structure with template

```rust
use magik_macro::template;

#[template(path = "templates/greeting.tmp")]
pub struct GreetingPage {
    name: &'static str,
    is_greeting: bool,
}
```

### 2. Create the template file

**`templates/greeting.tmp`**:
```
{{ props.is_greeting.choose("Hello", "Goodbye") }}, {{ props.name }}!
Welcome to our application.
```

### 3. Render the template

```rust
use magik::Renderable;

fn main() {
    let page = GreetingPage {
        name: "World",
        is_greeting: true,
    };
    
    println!("{}", page.render());
    // Output: Hello, World!
    //         Welcome to our application.
}
```

## Template Syntax

Templates use the `{{ }}` syntax to embed **pure Rust code**. The behavior depends on whether the code returns a value:

- **Expressions that return a value**: The returned value is converted to a string (via the `Renderable` trait) and inserted into the template
- **Statements that don't return a value**: These are executed in the global scope of the template file, allowing you to declare variables, import modules, or perform setup logic

**Example demonstrating scope sharing:**

```
# Configuration Report
{{ use std::collections::HashMap; }}
{{
    let mut config = HashMap::new();
    config.insert("theme", "dark");
    config.insert("lang", "en");
    config.insert("debug", "true");
}}

Application Theme: {{ config.get("theme").unwrap_or(&"light") }}
Language: {{ config.get("lang").unwrap_or(&"unknown") }}
Debug Mode: {{ config.get("debug").unwrap_or(&"false") }}

Total settings loaded: {{ config.len() }}
```

### Variable Interpolation

```
User: {{ props.name }}
Age: {{ props.age }}
Items count: {{ props.items.len() }}
Formatted: {{ format!("User: {}", props.username) }}
```

### Global Scope Statements

```
{{ use std::collections::HashMap; }}

{{
    let user_count = props.users.len();
    let is_empty = user_count == 0;
}}

Total users: {{ user_count }}
Status: {{ is_empty.choose("No users found", "Users available") }}
```

### Conditional Logic with Choosable

```
Status: {{ props.is_active.choose("Active", "Inactive") }}
Content: {{ props.show_content.choose_with(|| "Visible content", || "Hidden content") }}
```

### Complex Rust Logic

```
User Type: {{
    if props.age >= 18 {
        format!("Adult ({})", props.age)
    } else {
        format!("Minor ({})", props.age)
    }
}}

Connection Status: {{
    match props.status {
        "active" => "🟢 Online",
        "away" => "🟡 Away", 
        _ => "🔴 Offline"
    }
}}
```

### Using Other Components

```
{{ use crate::components::Button; }}
Actions:
{{ Button { text: "Submit", disabled: false } }}
{{ Button { text: "Cancel", disabled: true } }}
```

## System Components

### 1. Parser (`magik::Parser`)

Analyzes templates and separates static text from Rust code:

```rust
use magik::Parser;

let mut parser = Parser::new("Hello, {{ name }}! Welcome to {{ app }}.");
while let Some(data) = parser.next() {
    println!("{:?}", data);
}
```

### 2. TemplateData

Enum that represents different types of content in a template:

```rust
pub enum TemplateData {
    String(String),  // Static text
    Code(String),    // Rust code to evaluate
}
```

### 3. Trait Renderable

Converts types to strings for display in templates:

```rust
pub trait Renderable {
    fn render(self) -> String;
}

// Automatically implemented for:
// - String, &str
// - i32, f64, bool
// - Option<T: Renderable>
// - Vec<T: Renderable>
// - ()
```

### 4. Trait Choosable

Enables elegant conditional logic based on boolean values:

```rust
pub trait Choosable<T> {
    fn choose(&self, if_true: T, if_false: T) -> T;
    fn choose_with<F>(&self, if_true: F, if_false: F) -> T
    where F: FnOnce() -> T;
}

// Usage example
let is_admin = true;
let message = is_admin.choose("Admin Panel", "User Panel");
```

## Macros

### `#[template(path = "path")]`

Applies a template from an external file:

```rust
#[template(path = "templates/user.tmp")]
pub struct UserPage {
    username: String,
    email: String,
}
```

### `#[template(source = "template")]` or `#[template_str("template")]`

Uses an inline template:

```rust
#[template(source = "Hello, {{ props.name }}! Welcome to {{ props.app_name }}.")]
pub struct InlineGreeting<'a> {
    name: &'a str,
    app_name: &'a str,
}

#[template_str("Hello, {{ props.name }}! Welcome to {{ props.app_name }}.")]
pub struct InlineGreetingStr<'a> {
    name: &'a str,
    app_name: &'a str,
}
```

## Advantages

1. **Compile-time safety**: Syntax errors detected before execution
2. **Performance**: Templates compile to native Rust code
3. **Perfect integration**: Uses Rust's type system without compromises
4. **No runtime dependencies**: Pure Rust implementation with no external libraries
5. **Rust integration**: Templates are pure Rust code, allowing full access to the language's features

## Limitations

- Requires templates to be available at compile time
- Types must implement the `Renderable` trait
- Syntax is limited to valid Rust expressions

## Development

### Automatic Template Recompilation

To automatically recompile your project when template files change, it's highly recommended to use a `build.rs` script. This ensures that any changes to your `.tmp` template files will trigger a rebuild.

You can create a `build.rs` file or copy the file from the repository [`build.rs`](https://github.com/darilrt/magik/blob/master/build.rs) and place it in your project root. This script will watch for changes in the `templates/` directory (or any specified directory) and trigger a recompilation when necessary.

**Add to your `Cargo.toml`:**

```toml
[package]
name = "your-project"
version = "0.1.0"
edition = "2021"
build = "build.rs"  # Enable the build script

...
```

With this setup, Cargo will automatically recompile your project whenever you modify any template file in the `templates/` directory.

## Project Structure

```
magik/
├── magik/              # Main library
│   ├── src/
│   │   ├── lib.rs
│   │   ├── parser.rs   # Template parser
│   │   ├── template.rs # Type definitions
│   │   ├── renderable.rs # Renderable trait
│   │   └── choosable.rs  # Choosable trait
│   └── Cargo.toml
├── magik_macro/        # Procedural macros
│   │── examples/*      # Examples
│   ├── src/
│   │   ├── lib.rs
│   │   ├── utils.rs    # Compilation utilities
│   │   └── check_return.rs # Return analysis
│   └── Cargo.toml
│── build.rs            # Example build script for automatic recompilation
└── Cargo.toml
```

---

*Magik makes text generation in Rust magical* ✨