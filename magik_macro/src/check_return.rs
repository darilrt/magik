use syn::Stmt;

/// Checks if the block returns a value.
/// this functions is used to determine if any branch of the block returns a value.
pub fn is_block_returning_value(block: &syn::Block) -> bool {
    if let Some(Stmt::Expr(expr, semi)) = block.stmts.last() {
        if semi.is_some() {
            return false; // If the last statement is an expression without a semicolon, which means it returns a value
        }

        if expr_has_return_value(expr) {
            return true; // If the last expression returns a value
        }

        if !semi.is_some() {
            return true; // If the last statement is an expression without a semicolon, which means it returns a value
        }
    }
    false // No return statement found
}

fn expr_has_return_value(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Break(expr) => expr.expr.is_some(),

        syn::Expr::Block(block) => is_block_returning_value(&block.block),

        syn::Expr::If(expr) => {
            is_block_returning_value(&expr.then_branch)
                || expr
                    .else_branch
                    .as_ref()
                    .is_some_and(|else_branch| expr_has_return_value(&else_branch.1))
        }

        syn::Expr::Match(expr) => expr
            .arms
            .iter()
            .any(|arm| expr_has_return_value(arm.body.as_ref())),

        // syn::Expr::While(expr) Would not return a value, so we ignore it
        // syn::Expr::ForLoop(expr) Would not return a value, so we ignore it
        syn::Expr::Loop(expr) => is_block_returning_value(&expr.body),

        syn::Expr::Try(expr) => expr_has_return_value(&expr.expr),

        syn::Expr::Unsafe(expr) => is_block_returning_value(&expr.block),

        syn::Expr::Async(expr) => is_block_returning_value(&expr.block),

        _ => false, // Other expressions do not return a value
    }
}

// tests to check the functionality
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_block_returning_value() {
        let code: syn::Block = syn::parse_str("{ let x = 5; x }").unwrap();
        assert!(is_block_returning_value(&code));

        let code: syn::Block = syn::parse_str("{ let x = 5; }").unwrap();
        assert!(!is_block_returning_value(&code));

        let code: syn::Block = syn::parse_str("{ if true { 1 } else { 2 } }").unwrap();
        assert!(is_block_returning_value(&code));
    }

    #[test]
    fn test_expr_has_return_value() {
        let expr: syn::Expr = syn::parse_str("if true { 1 } else { 2 }").unwrap();
        assert!(expr_has_return_value(&expr));

        let expr: syn::Expr = syn::parse_str("let x = 5").unwrap();
        assert!(!expr_has_return_value(&expr));
    }
}
