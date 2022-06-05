#![deny(clippy::all)]

pub(crate) mod finance;
pub(crate) mod healthcare;
pub(crate) mod retail;
pub(crate) mod general;


#[cfg(test)]
mod tests{

    use crate::general::postalcode::isPostalCode;

    use super::*;

    #[test]
    
    fn test_postalcode(){

        assert_eq!(isPostalCode("1234".to_string(),"AT".to_string()), true);
        assert_eq!(isPostalCode("12334".to_string(),"AT".to_string()), false);
    }
}