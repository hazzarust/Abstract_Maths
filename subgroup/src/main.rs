

//x = [1,2,3,4,5] modulo 5
//y = [1,2,3,4,5] modulo 5 
//V = [1,2,3,4,5]
//i would have to loop through x
//for each element in x i would then loop through y, 
//and see if it equaled any numbers in V

fn main() {

    let x = vec![0,1,2,3,4,5,6,7,8,9,10,11];
    addition_closure(x);
}

fn addition_closure (v: Vec<u8>) -> bool{
        let clone = v.clone();
        println!("{:?}", clone);
        for x in v.iter(){
            for y in v.iter(){
                println!("{} + {} % 12 = ", y,x);
                let num = (*y + *x) % v.len() as u8;
                println!("{}", num);
                    if clone.contains(&num) == false{
                        return false
                    }
                }
            }
            true
        }
     


    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_addition_closuer(){
            let x = vec![0,1,2,3,4,5,6,7,8,9,10,11];
            assert_eq!(true, addition_closure(x));
            let y = vec![0,1,2,4,10,11];
            assert_eq!(false, addition_closure(y))
        }
    }
