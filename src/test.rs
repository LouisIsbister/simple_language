#[cfg(test)]

mod test {

    use crate::program::{
        self, 
        Expr, 
        Value, 
        Program
    };

    #[test]
    fn test_add() {
        let left = Box::new(Expr::Leaf(Value::Int(42)));
        let right = Box::new(Expr::Leaf(Value::Int(42)));
        let add = Expr::new_bin_op(left, right, program::add);

        assert_eq!(match Program::new(add).exec() {
            Value::Int(val) => val,
            _ => panic!("Didn't find a number!")
        }, 84)
    }

    #[test]
    fn test_eq() {
        let left = Box::new(Expr::Leaf(Value::Int(42)));
        let right = Box::new(Expr::Leaf(Value::Int(42)));
        let eq = Expr::new_bin_op(left, right, program::eq);

        assert_eq!(match Program::new(eq).exec() {
            Value::Bool(val) => val,
            _ => panic!("Didn't find a boolean!")
        }, true)
    }

    #[test]
    fn test_and() {
        let t = Box::new(Expr::Leaf(Value::Bool(true)));
        let f = Box::new(Expr::Leaf(Value::Bool(false)));
        
        let a1 = Expr::new_bin_op(t.clone(), f.clone(), program::logic_and);
        let a2 = Expr::new_bin_op(t.clone(), t, program::logic_and);
        let a3 = Expr::new_bin_op(f.clone(), f, program::logic_and);

        let tests = vec![
            (Program::new(a1).exec(), false), 
            (Program::new(a2).exec(), true),
            (Program::new(a3).exec(), false),
        ];
        for (value, expected) in tests {
            assert_eq!(match value {
                Value::Bool(val) => val,
                _ => panic!("Didn't find a boolean!")
            }, expected);
        }
    }

    #[test]
    fn test_not() {
        let val = Box::new(Expr::Leaf(Value::Bool(false)));
        let not = Expr::new_unary_op(val, program::not);

        assert_eq!(match Program::new(not).exec() {
            Value::Bool(val) => val,
            _ => panic!("Didn't find a boolean!")
        }, true)
    }
}