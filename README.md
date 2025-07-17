# Magik 🪄

A templating library for Rust that allows creating compile-time safe templates with type-safe syntax and powerful features.

## Table of Contents

- [Magik 🪄](#magik-)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
    - [Running the Project](#running-the-project)
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

### Running the Project

To run the example project:

```bash
cargo run
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
magik = { git = "https://github.com/darilrt/magik/tree/master/magik" }
magik-macro = { git = "https://github.com/darilrt/magik/tree/master/magik-macro" }
```

## Basic Usage

### 1. Define a structure with template

```rust
use magik_macro::template;

#[template(path = "pages/greeting.tmp")]
pub struct GreetingPage {
    name: &'static str,
    is_greeting: bool,
}
```

### 2. Create the template file

**`pages/greeting.tmp`**:
```html
<h1>{{ props.is_greeting.choose("Hello", "Bye") }}, {{ props.name }}!</h1>
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
    // Output: <h1>Hello, World!</h1>
}
```

## Template Syntax

Templates use the `{{ }}` syntax to embed **pure Rust code**. The behavior depends on whether the code returns a value:

- **Expressions that return a value**: The returned value is converted to a string (via the `Renderable` trait) and inserted into the template
- **Statements that don't return a value**: These are executed in the global scope of the template file, allowing you to declare variables, import modules, or perform setup logic

**Example demonstrating scope sharing:**

```html
<!-- Global imports and setup -->
{{ use std::collections::HashMap; }}
{{
    let mut config = HashMap::new();
    config.insert("theme", "dark");
    config.insert("lang", "en");
}}

<html data-theme="{{ config.get("theme").unwrap_or(&"light") }}">
<head>
    <title>{{ format!("App - {}", config.get("lang").unwrap_or(&"unknown")) }}</title>
</head>
<body>
    <!-- The config HashMap is still available here -->
    <div class="status">{{ config.len() }} settings loaded</div>
</body>
</html>
```

### Variable Interpolation

```html
<!-- Access to struct properties -->
<p>{{ props.name }}</p>
<p>{{ props.age }}</p>

<!-- Any Rust expression that returns a value -->
<p>{{ props.items.len() }}</p>
<p>{{ format!("User: {}", props.username) }}</p>
```

### Global Scope Statements

```html
<!-- Import statements (global scope) -->
{{ use std::collections::HashMap; }}

<!-- Variable declarations (global scope) -->
{{
    let user_count = props.users.len();
    let is_empty = user_count == 0;
}}

<!-- These variables can now be used in expressions -->
<p>Total users: {{ user_count }}</p>
<p>{{ is_empty.choose("No users found", "Users available") }}</p>
```

### Conditional Logic with Choosable

```html
<!-- Elegant ternary operator -->
<span>{{ props.is_active.choose("Active", "Inactive") }}</span>

<!-- With functions -->
<div>{{ props.show_content.choose_with(|| "Visible content", || "Hidden content") }}</div>
```

### Complex Rust Logic

```html
<!-- Control flow that returns values -->
<div class="status">
{{
    if props.age >= 18 {
        format!("Adult ({})", props.age)
    } else {
        format!("Minor ({})", props.age)
    }
}}
</div>

<!-- Pattern matching -->
<p>Status: {{
    match props.status {
        "active" => "🟢 Online",
        "away" => "🟡 Away", 
        _ => "🔴 Offline"
    }
}}</p>
```

### Using Other Components

```html
{{ use crate::components::Button; }}
<div>
  {{ Button { text: "Click me", disabled: false } }}
</div>
```

## System Components

### 1. Parser (`magik::Parser`)

Analyzes templates and separates static text from Rust code:

```rust
use magik::Parser;

let mut parser = Parser::new("<h1>Hello, {{ name }}!</h1>");
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
#[template(path = "pages/user.tmp")]
pub struct UserPage {
    username: String,
    email: String,
}
```

### `#[template(source = "template")]` or `#[template_str("template")]`

Uses an inline template:

```rust
#[template(source = "<h1>Hello, {{ props.name }}!</h1>")]
pub struct InlineGreeting<'a> {
    name: &'a str,
}

#[template_str("<h1>Hello, {{ props.name }}!</h1>")]
pub struct InlineGreetingStr<'a> {
    name: &'a str,
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

You can create a `build.rs` file or copy the file from the repository [`build.rs`](https://github.com/darilrt/magik/blob/master/build.rs) and place it in your project root. This script will watch for changes in the `pages/` directory (or any specified directory) and trigger a recompilation when necessary.

**Add to your `Cargo.toml`:**

```toml
[package]
name = "your-project"
version = "0.1.0"
edition = "2021"
build = "build.rs"  # Enable the build script

...
```

With this setup, Cargo will automatically recompile your project whenever you modify any template file in the `pages/` directory.

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
├── magik-macro/        # Procedural macros
│   ├── src/
│   │   ├── lib.rs
│   │   ├── utils.rs    # Compilation utilities
│   │   └── check_return.rs # Return analysis
│   └── Cargo.toml
├── pages/              # Example templates
├── src/                # Example application
|-- build.rs            # Example build script for automatic recompilation
└── Cargo.toml
```

---

*Magik makes String generation in Rust magical* ✨