#[cfg(test)]

mod test {

    use crate::program;
    use crate::program::{ExprNode, ExprValue, Program};

    #[test]
    fn test_add() {
        let left = Box::new(ExprNode::Leaf(ExprValue::Int(42)));
        let right = Box::new(ExprNode::Leaf(ExprValue::Int(42)));
        let add = ExprNode::new_bin_op(left, right, program::add);

        let res = Program::new(add).exec();
        assert_eq!(match res {
            ExprValue::Int(val) => val,
            _ => panic!("Didn't find a number!")
        }, 84)
    }

    #[test]
    fn test_eq() {
        let left = Box::new(ExprNode::Leaf(ExprValue::Int(42)));
        let right = Box::new(ExprNode::Leaf(ExprValue::Int(42)));
        let eq = ExprNode::new_bin_op(left, right, program::eq);

        let res = Program::new(eq).exec();
        assert_eq!(match res {
            ExprValue::Bool(val) => val,
            _ => panic!("Didn't find a boolean!")
        }, true)
    }
}