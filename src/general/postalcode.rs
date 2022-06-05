use napi_derive::napi;
use regex::Regex;

const THREEDIGIT: &str = r"^\d{3}$";
const FOURDIGIT: &str = r"^\d{4}$";

enum Country {
  CA,
  IN,
  AT,
  None,
}

impl From<String> for Country {
  fn from(c: String) -> Self {
    match c.as_str() {
      "CA" => Country::CA,
      "IN" => Country::IN,
      "AT" => Country::AT,
      _ => Country::None,
    }
  }
}

#[napi]
#[allow(
  non_snake_case,
  non_camel_case_types,
  non_upper_case_globals,
  dead_code
)]
pub(crate) fn isPostalCode(code: String, country: String) -> bool {
  match country.into() {
    Country::CA => {
      let pattern =
        Regex::new(r"^[ABCEGHJKLMNPRSTVXY]\d[ABCEGHJ-NPRSTV-Z][\s\-]?\d[ABCEGHJ-NPRSTV-Z]\d$");
      match pattern {
        Ok(ref patt) => patt.is_match(code.as_str()),
        Err(_e) => false,
      }
    }
    Country::IN => {
      let pattern = Regex::new(r"^((?!10|29|35|54|55|65|66|86|87|88|89)[1-9][0-9]{5})$");
      match pattern {
        Ok(ref patt) => patt.is_match(code.as_str()),
        Err(_e) => false,
      }
    }
    Country::AT => {
      let pattern = Regex::new(FOURDIGIT);

      match pattern {
        Ok(ref patt) => patt.is_match(code.as_str()),
        Err(_e) => false,
      }
    }

    Country::None => false,
  }
}
