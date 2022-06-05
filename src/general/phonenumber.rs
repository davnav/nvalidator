use napi_derive::napi;
use phonenumber::country::Id;

#[napi]
#[allow(
  non_snake_case,
  non_camel_case_types,
  non_upper_case_globals,
  dead_code
)]

fn isPhonenumber(number: String, country_code: String) -> bool {
  let ph = number.parse::<String>();
  match ph {
    Ok(phnum) => {
      let id = country_code.parse::<Id>();

      match id {
        Ok(idnum) => {
          let phfull = phonenumber::parse(Some(idnum), phnum);

          match phfull {
            Ok(phfullnumber) => phonenumber::is_valid(&phfullnumber),
            Err(_e) => false,
          }
        }

        Err(_e) => false,
      }
    }

    Err(_e) => false,
  }
}
