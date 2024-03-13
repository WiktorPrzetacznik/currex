use currex::converter::CurrencyConverter;

#[test]
fn success_fetching_pair() {
    let mut converter: CurrencyConverter = Default::default();
    let base_code = String::from("PLN");
    let target_code = String::from("USD");
    assert!(converter.get_pair_data(&base_code, &target_code).is_ok());
}

#[test]
fn fail_fetching_pair() {
    let mut converter: CurrencyConverter = Default::default();
    let base_code = String::from("P");
    let target_code = String::from("USD");
    assert!(converter.get_pair_data(&base_code, &target_code).is_err());
}

#[test]
fn success_fetching_list() {
    let mut converter: CurrencyConverter = Default::default();
    let base_code = String::from("PLN");
    assert!(converter.get_list_data(&base_code).is_ok());
}

#[test]
fn fail_fetching_list() {
    let mut converter: CurrencyConverter = Default::default();
    let base_code = String::from("P");
    assert!(converter.get_list_data(&base_code).is_err());
}

#[test]
fn success_save_cache() {
    let converter: CurrencyConverter = Default::default();
    assert!(converter.save().is_ok())
}

#[test]
fn success_load_cache() {
    let converter: CurrencyConverter = Default::default();
    assert!(converter.save().is_ok() && CurrencyConverter::load().is_ok())
}