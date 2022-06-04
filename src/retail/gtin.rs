

use napi_derive::napi;
use gtin_validate::{gtin12, gtin13};


#[napi]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals,dead_code)]
fn isUPC(num:String) -> bool{

    gtin12::check(&num)

}

#[napi]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals,dead_code)]
fn isGTIN13(num:String) -> bool{

    gtin13::check(&num)
}

