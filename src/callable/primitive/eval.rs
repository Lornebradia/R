use lazy_static::lazy_static;
use r_derive::*;

use crate::callable::core::*;
use crate::err;
use crate::lang::*;
use crate::object::*;

lazy_static! {
    pub static ref FORMALS: ExprList = ExprList::from(vec![
        (Some("x".to_string()), Expr::Missing),
        (
            Some("envir".to_string()),
            Expr::Call(
                Box::new(Expr::Symbol("parent".to_string())),
                ExprList::new()
            )
        )
    ]);
}

#[derive(Debug, Clone, PartialEq)]
#[builtin(sym = "eval")]
pub struct PrimitiveEval;
impl Callable for PrimitiveEval {
    fn formals(&self) -> ExprList {
        FORMALS.clone()
    }

    fn call(&self, args: ExprList, stack: &mut CallStack) -> EvalResult {
        let (args, _ellipsis) = self.match_arg_exprs(args, stack)?;
        let mut args = Obj::List(args);

        let Obj::Expr(expr) = args.try_get_named("x")?.force(stack)? else {
            return err!("Argument 'x' should be a quoted expression.");
        };

        let Obj::Environment(envir) = args.try_get_named("envir")?.force(stack)? else {
            return err!("Argument 'envir' should be an environment or data context.");
        };

        Obj::Closure(expr, envir.clone()).force(stack)
    }
}
