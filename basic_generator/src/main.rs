

#[derive(Debug)]
pub enum Error{
    NoGenerator,
}

//Make a function that will go through the vector and find out what the generator numbers are
//Takes a vector of size u8, makes a copy
//iterates through the vec we passed into function, the element of current loop is stored under x
//I will then iterate again for each element x of our inital first loop
//This time I will perform computation using the enumeration of the current loop and the element x from our first loop
//(current enumeration of this loop) * x (from our initial loop) % 12 for every enumeration element in our 2nd loop
//The result is then stored under computated vec.
//I then sort it, and compare to the original copy vec, to see if x element generated the inital vector using the computation above
//If they match, then I will push x into a vector and return all possible generators after x has finished looping through the whole vector

pub fn generator(vec: &Vec<u8>) -> Result<Vec<u8>, Error>{

    let mut results: Vec<u8> = vec![];
    let copy_vec = vec;
    for x in vec.iter(){
        let mut computated_vec: Vec<u8> = vec.iter().enumerate()
        .map(|(a, _) |(a as u8 + 1) * *x % vec.len() as u8)
        .collect();
        computated_vec.sort();
        if computated_vec == *copy_vec{
            results.push(*x);
        }else{
            continue;

        }
    }
    if results.len() == 0{
        Err(Error::NoGenerator)
    }else{
        println!("{:?}", results);
        Ok(results)
    }
    
}
       
        

    

fn main() {
    
    let z = vec![0,1,2,3,4,5,6,7,8,9,10,11];
    generator(&z).unwrap();
    let b = vec![0,1,2,3];
    generator(&b).unwrap();
}
