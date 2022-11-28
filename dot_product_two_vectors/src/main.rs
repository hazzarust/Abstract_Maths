use num_traits::{Num, Zero};

#[derive(Debug)]
enum Error{
    LengthError,
}

fn dot_product_two_vectors<T>(x: &[T], y: &[T]) -> Result<T, Error>
    where T: Num + Clone{
        if x.len() !=  y.len(){
            return Err(Error::LengthError)
        }

        let x = x.iter()
        .zip(y.iter())
        .map(|(a, b)| a.clone() * b.clone())
        .fold(T::zero(), |x, acc| x + acc);

    Ok(x)
    }

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_elements(){
        let x = vec![4, -7];
        let y = vec![-2, 3];

    }

    #[test]
    fn three_elements(){
        let x = vec![5, -4, 3];
        let y = vec![7, 2, -8];
        
    }
}