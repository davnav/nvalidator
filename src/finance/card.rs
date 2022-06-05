use card_validate::Validate;
use napi_derive::napi;

#[napi]
#[allow(
  non_snake_case,
  non_camel_case_types,
  non_upper_case_globals,
  dead_code
)]
// payment card will be validated based on the crate function implementation - https://crates.io/crates/card-validate
// Supported providers
//  Debit cards:
//
//    Visa Electron
//    Maestro
//    Forbrugsforeningen
//    Dankort
//    Credit cards:
//
//   Visa
//
//    MasterCard
//    American Express
//    Diners Club
//    Discover
//    UnionPay
//    JCB

fn isValidCard(card: String) -> bool {
  match Validate::from(card.as_str()) {
    Ok(_result) => true,
    Err(_e) => false,
  }
}
