use crate::processing::syntax_elements::{DataTypes, Variable};

#[test]
fn test_infer_type_int() {
    let value = 42;
    let inferred = Variable::convert_to_data_types(&value);
    assert_eq!(inferred, Some(DataTypes::Int(42)));
}

#[test]
fn test_infer_type_string() {
    let value = String::from("hello");
    let inferred = Variable::convert_to_data_types(&value);
    assert_eq!(inferred, Some(DataTypes::Str(value)));
}

#[test]
fn test_infer_type_bool() {
    let value = true;
    let inferred = Variable::convert_to_data_types(&value);
    assert_eq!(inferred, Some(DataTypes::Bool(true)));
}

#[test]
fn test_infer_type_float() {
    let value = 3.14;
    let inferred = Variable::convert_to_data_types(&value);
    assert_eq!(inferred, Some(DataTypes::Float(3.14)));
}

#[test]
fn test_infer_type_char() {
    let value = 'a';
    let inferred = Variable::convert_to_data_types(&value);
    assert_eq!(inferred, Some(DataTypes::Char('a')));
}

#[test]
fn test_infer_type_none() {
    struct CustomType;
    let value = CustomType;
    let inferred = Variable::convert_to_data_types(&value);
    assert_eq!(inferred, None);
}

#[test]
fn test_new_variable_int() {
    let name = String::from("x");
    let value = 42;
    let variable = Variable::new(name.clone(), &value, true).unwrap();
    assert_eq!(variable.name, name);
    assert_eq!(variable.value, DataTypes::Int(42));
}

#[test]
fn test_new_variable_string() {
    let name = String::from("y");
    let value = String::from("hello");
    let variable = Variable::new(name.clone(), &value, false).unwrap();
    assert_eq!(variable.name, name);
    assert_eq!(variable.value, DataTypes::Str(value));
}

#[test]
fn test_new_variable_bool() {
    let name = String::from("z");
    let value = true;
    let variable = Variable::new(name.clone(), &value, true).unwrap();
    assert_eq!(variable.name, name);
    assert_eq!(variable.value, DataTypes::Bool(true));
}

#[test]
fn test_new_variable_float() {
    let name = String::from("w");
    let value = 3.14;
    let variable = Variable::new(name.clone(), &value, false).unwrap();
    assert_eq!(variable.name, name);
    assert_eq!(variable.value, DataTypes::Float(3.14));
}

#[test]
fn test_new_variable_char() {
    let name = String::from("a");
    let value = 'a';
    let variable = Variable::new(name.clone(), &value, true).unwrap();
    assert_eq!(variable.name, name);
    assert_eq!(variable.value, DataTypes::Char('a'));
}
