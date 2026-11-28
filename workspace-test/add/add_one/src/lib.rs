pub struct Operation;

impl Operation {
    pub fn add(left: u64, right: u64) -> u64 {
        left + right
    }

    pub fn subs(left: u64, right: u64) -> u64 {
        left - right
    }

    pub fn mul(left: u64, right: u64) -> u64 {
        left * right
    }

    pub fn div(left: u64, right: u64) -> u64 {
        if right != 0 {
            return left / right
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = Operation::add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sub() {
        let result = Operation::subs(2, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_mul() {
        let result = Operation::mul(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_div_non_zero() {
        let result = Operation::div(4, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_div_zero() {
        let result = Operation::div(4, 0);
        assert_eq!(result, 0);
    }
}
