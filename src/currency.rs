
use napi_derive::napi;
use iso_currency::{Currency};

#[napi]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals,dead_code)]
fn isCurrency(code:String) -> bool{

    match Currency::from_code(code.as_str()){

        Some(_result) => true,
        None => false,

    }
}