# Magik Templates Examples

This folder contains examples that demonstrate the different features of the Magik template system.

## Available Examples

### 1. `simple.rs`
Basic example showing the simplest use of the template system.

### 2. `basic_interpolation.rs`
Demonstrates how to interpolate different types of variables in templates:
- Strings (`&str`)
- Integers (`i32`)
- Floating-point numbers (`f64`)
- Booleans (`bool`)

### 3. `conditional_logic.rs`
Shows the use of the `choose()` function for conditional logic:
- Alternating between different strings based on boolean values
- Multiple conditions in the same template
- Combining conditional logic with interpolation

### 4. `component_composition.rs`
Example of component composition:
- Definition of reusable components (Button, Link, Card)
- Composition of components within other components
- Main layout using nested components

### 5. `file_template.rs` + `email_template.tmp`
Demonstrates the use of external templates:
- Template defined in separate file
- Interpolation of multiple variables
- Reusing the same template with different data

### 6. `html_generation.rs` + `html_page.tmp`
Complete HTML generation:
- Structured HTML template with CSS
- Components for specific content (Article)
- Complete web page with header, content and footer

### 7. `data_types.rs`
Comprehensive example of supported data types:
- Integers and floating-point numbers
- Booleans with conditional logic
- Strings with different formats
- Complex formatting with multiple variables

### 8. `modular_components.rs`
Modular component system:
- Small and reusable components
- Dashboard that composes multiple elements
- Demonstration of different states of the same component

## How to Run the Examples

To run any example:

```bash
cargo run --example example_name
```

For example:
```bash
cargo run --example simple
cargo run --example conditional_logic
cargo run --example html_generation
```

## Featured Characteristics

- **Type Safety**: All templates are verified at compile time
- **Composition**: Components can be nested and reused
- **External Templates**: Support for templates in separate files
- **Conditional Logic**: `choose()` function for boolean-based alternatives
- **Multiple Types**: Native support for strings, numbers and booleans
