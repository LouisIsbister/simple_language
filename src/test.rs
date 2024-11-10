#[cfg(test)]

mod test {

    use crate::program::{
        self, 
        ExprNode, 
        ExprValue, 
        Program
    };

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

    #[test]
    fn test_and() {
        let t = Box::new(ExprNode::Leaf(ExprValue::Bool(true)));
        let f = Box::new(ExprNode::Leaf(ExprValue::Bool(false)));
        
        let a1 = ExprNode::new_bin_op(t.clone(), f, program::logic_and);
        let a2 = ExprNode::new_bin_op(t.clone(), t, program::logic_and);

        assert_eq!(match Program::new(a1).exec() {
            ExprValue::Bool(val) => val,
            _ => panic!("Didn't find a boolean!")
        }, false);
        assert_eq!(match Program::new(a2).exec() {
            ExprValue::Bool(val) => val,
            _ => panic!("Didn't find a boolean!")
        }, true)
    }
}