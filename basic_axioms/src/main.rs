use num_traits::Num;

//ASSOCIATION
//If the equation a * (b * c) = (a * b) * c, is true then I can 
//call the * operation associative. Please note * != Multiplication when refered to in Algebra terms

pub enum Op{
    Multiplication,
    Addition,
    Division,
    Subtraction,
}

pub fn association<T>(x: &[T], operator: Op) -> bool 
    where T: Copy + Num{

        if x.len() > 3{
            return false
        }
        
        let count = 0;
        match operator{
            Op::Multiplication => 
                if x[count] * (x[count+1] * x[count+2])
                == (x[count] * x[count+1]) * x[count+2]{
                    return true
                }
                else{
                    return false
                }
            Op::Addition => 
                if x[count] + (x[count+1] + x[count+2])
                == (x[count] + x[count+1]) + x[count+2]{
                    return true
                }
                else{
                    return false
                }
            Op::Division => 
                if x[count] / (x[count+1] / x[count+2])
                == (x[count] / x[count+1]) / x[count+2]{
                    return true
                }
                else{
                    return false
                }
            Op::Subtraction => 
                if x[count] - (x[count+1] - x[count+2])
                == (x[count] - x[count+1]) - x[count+2]{
                    return true
                }
                else{
                    return false
                }
            }
        }


fn main() {
    println!("Hello, world!");
    println!("test");
}

#[cfg(test)]
mod tests {
    use crate::association;

    use super::*;

    #[test]
    fn association_test(){
        let x = vec![2,4,6];
        assert!(association(&x, Op::Multiplication));
        assert!(association(&x, Op::Addition));
    }
    #[test]
    #[should_panic]
    fn association_test_div(){
        let x = vec![2,4,6];
        assert!(association(&x, Op::Division));
    }
    #[test]
    #[should_panic]
    fn association_test_sub(){
        let x = vec![2,4,6];
        assert!(association(&x, Op::Subtraction));
    }

}
