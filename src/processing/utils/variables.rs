use crate::processing::lexer::{Error, Result};
use crate::processing::syntax_elements::Token;
use crate::processing::types::LineTokenizedBody;
use crate::processing::{
    syntax_elements::{DataTypes, Variable},
    types::Line,
};

pub fn look_variable(
    var_name: &str,
    variables: &Vec<Variable>,
    expected_type: DataTypes,
) -> Option<Variable> {
    for variable in variables {
        if var_name != variable.name {
            continue;
        }
        let same_type = match (&variable.value, &expected_type) {
            (DataTypes::Int(_), DataTypes::Int(_)) => true,
            (DataTypes::Char(_), DataTypes::Char(_)) => true,
            (DataTypes::Bool(_), DataTypes::Bool(_)) => true,
            (DataTypes::Float(_), DataTypes::Float(_)) => true,
            (DataTypes::Str(_), DataTypes::Str(_)) => true,
            _ => false,
        };

        if same_type {
            return Some(variable.clone());
        }
    }

    None
}

pub fn var_exists(variables: &Vec<Variable>, name: &str) -> bool {
    for variable in variables {
        if variable.name == name.to_string() {
            return true 
        }
    }

    false
}

pub fn get_var_name(line: &Line) -> Result<String> {
    let variable_declaration_position = line.tokenized_body.iter().position(|token| {
        matches!(
            token,
            Token::MutVarDeclaration(_) | Token::ImmutVarDeclaration(_)
        )
    });

    if variable_declaration_position.is_none() {
        return Err(Error::UnableToFindVarName);
    }

    let variable_name_position = variable_declaration_position.unwrap() + 1;
    let variable_name_token = &line.tokenized_body[variable_name_position];

    let variable_name = match variable_name_token {
        Token::Variable(name, _) => name,
        other => return Err(Error::ExpectedVarName(other.clone())),
    };

    Ok(variable_name.to_string())
}

pub fn is_var_mutable(line: &Line) -> bool {
    match line.tokenized_body[0] {
        Token::ImmutVarDeclaration(_) => true,
        Token::MutVarDeclaration(_) => true,
        _ => false,
    }
}

/// Slices out the variable declaration out the line
pub fn slice_variable_dec(line: &Line) -> LineTokenizedBody {
    let mut new = line.tokenized_body.clone();

    for i in (0..3).rev() {
        new.remove(i);
    }

    new
}
