

//x = [1,2,3,4,5] modulo 5
//y = [1,2,3,4,5] modulo 5 
//V = [1,2,3,4,5]
//i would have to loop through x
//for each element in x i would then loop through y, 
//and see if it equaled any numbers in V

fn main() {

    let y = vec![0,2,4,6,8,10];
    addition_inverse(y);
}

fn addition_closure (v: Vec<u8>) -> bool{
        let clone = v.clone();
        println!("{:?}", clone);
        for x in v.iter(){
            for y in v.iter(){
                let num = (*y + *x) % v.len() as u8;
                println!("{}", num);
                    if clone.contains(&num) == false{
                        return false
                    }
                }
            }
            true
        }

//make a function that displays every element in a vector and its inverses next to it
//to find a inverse we can do n - the number 
//[0,2,4,6,8,10] for mod 12
fn addition_inverse(v: Vec<i8>){
    let modulo_vec: Vec<i8> = v.iter()
    .map(|y| y % 12 as i8)
    .collect();

    
    let inverse_vec: Vec<i8> = modulo_vec.iter()
        .map(|y| {
            let y = 0 - y;
            y.rem_euclid(12) 

        })
        .collect();

        
    modulo_vec.iter()
    .zip(inverse_vec.iter())
    .for_each(|(a,b)| println!("Inverse of {} is : {}",a,b));
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
