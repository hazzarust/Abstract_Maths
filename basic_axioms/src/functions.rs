use num_traits::Num;
pub enum Op{
    Multiplication,
    Addition,
    Division,
    Subtraction,
}

#[derive(PartialEq, Debug)]
pub enum Error{
    NoValueFound,
}

pub fn association_mul<T>(x: &[T]) -> bool
    where T: Copy + Num{
        let count = 0;
        if x[count] * (x[count+1] * x[count+2])
                == (x[count] * x[count+1]) * x[count+2]{
                    return true
                }
                else{
                    return false
                }
    }

pub fn association_div<T>(x: &[T]) -> bool
    where T: Copy + Num{
        let count = 0;
        if x[count] / (x[count+1] / x[count+2])
                == (x[count] / x[count+1]) / x[count+2]{
                    return true
                }
                else{
                    return false
                }
    }

pub fn association_sub<T>(x: &[T]) -> bool
    where T: Copy + Num{
        let count = 0;
        if x[count] - (x[count+1] - x[count+2])
        == (x[count] - x[count+1]) - x[count+2]{
            return true
        }
        else{
            return false
        }
    }

pub fn association_add<T>(x: &[T]) -> bool
    where T: Copy + Num{
    let count = 0;
    if x[count] + (x[count+1] + x[count+2])
    == (x[count] + x[count+1]) + x[count+2]{
        return true
    }
    else{
        return false
    }
}

pub fn identity_mul<T>(x: &[T]) -> Result<&T, Error> 
    where T: Copy + Num{
        for top_vector_x in x.iter(){
            if *top_vector_x == T::zero(){
                continue;
            }
            for bottom_vector_x in x.iter(){
                if *bottom_vector_x * *top_vector_x == *top_vector_x{
                    return Ok(bottom_vector_x)
                }
            }
    }
    Err(Error::NoValueFound)
}

pub fn identity_div<T>(x: &[T]) -> Result<&T, Error> 
    where T: Copy + Num{
        for top_vector_x in x.iter(){
            if *top_vector_x == T::zero(){
                continue;
            }
            for bottom_vector_x in x.iter(){
                if *bottom_vector_x / *top_vector_x == *top_vector_x{
                    return Ok(bottom_vector_x)
                }
            }
        }
        Err(Error::NoValueFound)
    }

pub fn identity_add<T>(x: &[T]) -> Result<&T, Error> 
    where T: Copy + Num{
        for top_vector_x in x.iter(){
            for bottom_vector_x in x.iter(){
                if *bottom_vector_x + *top_vector_x == *top_vector_x{
                    return Ok(bottom_vector_x)
                }
            }
        }
        Err(Error::NoValueFound)

    }

pub fn identity_sub<T>(x: &[T]) -> Result<&T, Error> 
    where T: Copy + Num{
        for top_vector_x in x.iter(){
            for bottom_vector_x in x.iter(){
                if *top_vector_x - *bottom_vector_x == *top_vector_x{
                    return Ok(bottom_vector_x)
                }
            }
        }
        Err(Error::NoValueFound)
    }