
#[derive(Debug)]
pub enum Error{
    NoGenerator,
}
//Make a function that will go through the vector and find out what the generator numbers are

pub fn generator(vec: &Vec<u8>) -> Result<Vec<u8>, Error>{

    let mut results: Vec<u8> = vec![];
    let copy_vec = vec;
    for x in vec.iter(){
        let mut computated_vec: Vec<u8> = vec.iter()
        .map(|y|*y * *x % vec.len() as u8)
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
