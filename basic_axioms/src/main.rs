use functions::*;
use num_traits::Num;
mod functions;
//ASSOCIATION
//If the equation a * (b * c) = (a * b) * c, is true then I can 
//call the * operation associative. Please note * != Multiplication when refered to in Algebra terms



pub fn association<T>(x: &[T], operator: Op) -> bool 
    where T: Copy + Num{

        if x.len() > 3{
            return false
        }
        match operator{
            Op::Multiplication => 
            return association_mul(x),
            Op::Addition => 
            return association_add(x),
            Op::Division => 
            return association_div(x),
            Op::Subtraction => 
            return association_sub(x),
            };
        
        }

//IDENTITY
//if e * a = a or e * a = e, we call the e element the identity element
//for example, in addition, the identity element would be 0 as 4 + 0 = 4 AND 0 + 4 = 4
//This function will let us know what our identity element is from the vector provided for each
//operation in the Op enum. Please note I don't find this function very practical; I built it to 
//grasp the Identity concept better.

pub fn identity<T>(x: &[T], operation: Op) -> Result<&T, Error> 
        where T: Copy + Num {
            match operation{
                Op::Multiplication =>
                return identity_mul(x),
                Op::Addition => 
                return identity_add(x),
                Op::Subtraction => 
                return identity_sub(x),
                Op::Division =>
                return identity_div(x),
            };
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

    #[test]
    fn identity_test_add(){
        let x = vec![3,7,1,6,0,-3];
        assert_eq!(identity(&x, Op::Addition).unwrap(), &0);
    }
    #[test]
    fn identity_test_sub(){
        let x = vec![3,7,1,6,0,-3];
        assert_eq!(identity(&x, Op::Subtraction).unwrap(), &0);
    }
    #[test]
    fn identity_test_mul(){
        let x = vec![3,7,1,6,0,-3];
        assert_eq!(identity(&x, Op::Multiplication).unwrap(), &1);
    }
    #[test]
    fn identity_test_div(){
        let x = vec![3,7,1,6,0,-3];
        assert_eq!(identity(&x, Op::Division).unwrap(), &1);
    }
    

}
