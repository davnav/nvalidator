
use napi_derive::napi;

#[napi]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals,dead_code)]
fn isUSzipCode(code:String) -> bool{

    match zip_codes::map().get(code.as_str()){
        Some(_) => true,
        None => false, 
    }
}