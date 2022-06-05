use napi_derive::napi;

#[napi]
#[allow(
  non_snake_case,
  non_camel_case_types,
  non_upper_case_globals,
  dead_code
)]
fn isJson(obj: String) -> bool {
  match json::parse(&obj) {
    Ok(_result) => true,
    Err(_e) => false,
  }
}
