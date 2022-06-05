
use napi_derive::napi;
use regex::Regex;
enum Country {
    CA,
    IN,
    None
    
}

impl From<String> for Country{
    fn from(c:String) -> Self{
        if c == "CA" {
            Country::CA
        }else if c == "IN" {
            Country::IN
        }else{
            Country::None
        }
    } 
}

#[napi]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals,dead_code)]
fn isPostalCode(code:String,country:String) -> bool{
    match country.into() {
        
        Country::CA=> {
            match Regex::new(r"/^[ABCEGHJKLMNPRSTVXY]\d[ABCEGHJ-NPRSTV-Z][\s\-]?\d[ABCEGHJ-NPRSTV-Z]\d$/i"){
                Ok(_result)=>true,
                Err(_e)=>false,
	    }
	},
       	Country::IN => { true },
        Country::None => { false },
   } 
   
}
