

//x = [1,2,3,4,5] modulo 5
//y = [1,2,3,4,5] modulo 5 
//V = [1,2,3,4,5]
//i would have to loop through x
//for each element in x i would then loop through y, 
//and see if it equaled any numbers in V

fn main() {

}

fn addition_closure (v: &Vec<i8>, modulo: i8) -> bool{
        let clone = v.clone();
        for x in v.iter(){
            for y in v.iter(){
                let num = (*y + *x) % modulo;
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
pub fn addition_inverse(v: &Vec<i8>, modulo: i8) -> bool{

    let inverse_vec: Vec<i8> = v.iter()
        .map(|y| {
            let y = 0 - y;
            y.rem_euclid(modulo) 

        })
        .collect();
    
    for x in inverse_vec.iter(){
        if v.contains(&x) == false{
            return false
        }
    }
    true
}

pub fn subgroup_verifier(vec: &Vec<i8>, subgroup_in_question: &Vec<i8>)-> bool{
    //will then see if the vector satisfies the closure and inverse.
    let modulo = vec.len() as i8;
    if addition_closure(&subgroup_in_question, modulo) == true && addition_inverse(&subgroup_in_question, modulo) == true{
        println!("{:?} is a subgroup of {:?}", subgroup_in_question, vec);
        return true
    }
    else{
        println!("{:?} is not subgroup of {:?}", subgroup_in_question, vec);
        return false
    }
}

pub fn order_of_elements(vec: &Vec<i8>, modulo: i8){
    //gets the order of every element in group 
    let mut results: Vec<Vec<i8>> = vec![];
    for x in vec.iter(){
        let computated_vec: Vec<i8> = vec.iter().enumerate()
        .map(|(a, _)|{
            let a = x * (a as i8 + 1);
            a.rem_euclid(modulo)
        })
        .collect();

        results.push(computated_vec);
        }
        
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_addition_closuer(){
            let analog_clock = vec![0,1,2,3,4,5,6,7,8,9,10,11];
            let x = vec![0,2,4,6,8,10];
            let y = vec![0,2,4,6,8];
            assert_eq!(false, subgroup_verifier(&analog_clock, &y));
            assert_eq!(true, subgroup_verifier(&analog_clock, &x));
        }
    }


    //v.iter()
    //.zip(inverse_vec.iter())
    //.for_each(|(a,b)| println!("Inverse of {} is : {}",a,b));