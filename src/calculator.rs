pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn substract(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide(a: f64, b: f64) -> f64 {
    a / b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_add_five() {
        assert_eq!(add(5.0, 5.0), 5.0 + 5.0);
    }
    #[test]
    fn five_add_zero() {
        assert_eq!(add(5.0, 0.0), 5.0 + 0.0);
    }
    #[test]
    fn five_add_neg_five() {
        assert_eq!(add(5.0, -5.0), 5.0 + (-5.0));
    }

    #[test]
    fn five_substract_five() {
        assert_eq!(substract(5.0, 5.0), 5.0 - 5.0);
    }
    #[test]
    fn five_substract_zero() {
        assert_eq!(substract(5.0, 0.0), 5.0 - 0.0);
    }
    #[test]
    fn five_substract_neg_five() {
        assert_eq!(substract(5.0, -5.0), 5.0 - (-5.0));
    }

    #[test]
    fn five_multiply_five() {
        assert_eq!(multiply(5.0, 5.0), 5.0 * 5.0);
    }
    #[test]
    fn five_multiply_zero() {
        assert_eq!(multiply(5.0, 0.0), 5.0 * 0.0);
    }
    #[test]
    fn five_multiply_neg_five() {
        assert_eq!(multiply(5.0, -5.0), 5.0 * (-5.0));
    }

    #[test]
    fn five_divide_five() {
        assert_eq!(divide(5.0, 5.0), 5.0 / 5.0);
    }
    #[test]
    fn five_divide_zero() {
        assert_eq!(divide(5.0, 0.0), 5.0 / 0.0);
    }
    #[test]
    fn five_divide_neg_five() {
        assert_eq!(divide(5.0, -5.0), 5.0 / (-5.0));
    }
}