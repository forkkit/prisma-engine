use crate::{ast, dml, error::DatamodelError};

/// State error message. Seeing this error means something went really wrong internally. It's the datamodel equivalent of a bluescreen.
pub (crate) const STATE_ERROR: &str = "Failed lookup of model or field during internal processing. This means that the internal representation was mutated incorrectly.";
pub (crate) const ERROR_GEN_STATE_ERROR: &str = "Failed lookup of model or field during generating an error message. This often means that a generated field or model was the cause of an error.";

impl ast::WithDirectives for Vec<ast::Directive> {
    fn directives(&self) -> &Vec<ast::Directive> {
        self
    }
}

pub fn field_validation_error(
    message: &str,
    model: &dml::Model,
    field: &dml::Field,
    ast: &ast::SchemaAst,
) -> DatamodelError {
    DatamodelError::new_model_validation_error(
        message,
        &model.name,
        ast.find_field(&model.name, &field.name)
            .expect(ERROR_GEN_STATE_ERROR)
            .span,
    )
}
