use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};

use crate::libs::{
    password,
    validation::{is_valid_account, is_valid_password},
};
use crate::model::custom_deserializers::{option_trim_string, trim_string};
use common::BoxError;
use domain::model::UserModel;

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignupRequestDto {
    #[serde(deserialize_with = "trim_string")]
    #[validate(custom(function = "validate_account"))]
    pub account: String,
    #[serde(deserialize_with = "option_trim_string")]
    #[validate(email)]
    #[serde(default)]
    pub email: Option<String>,
    #[serde(deserialize_with = "option_trim_string")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(deserialize_with = "trim_string")]
    #[validate(custom(function = "validate_password"))]
    pub password: String,
    #[serde(deserialize_with = "trim_string")]
    pub confirm_password: String,
}

impl SignupRequestDto {
    pub fn custom_validate(&self) -> Result<(), ValidationErrors> {
        let _ = &self.validate()?;

        if self.password != self.confirm_password {
            let mut errors = ValidationErrors::new();
            errors.add("confirm_password", ValidationError::new("password_match"));
            return Err(errors);
        }

        Ok(())
    }

    pub async fn to_model(&self) -> Result<UserModel, BoxError> {
        Ok(UserModel {
            account: self.account.clone(),
            email: self.email.clone(),
            name: self.name.clone(),
            password: password::hash(self.password.clone()).await?,
            jwt_id: None,
        })
    }
}

pub struct AuthUserDto {
    pub account: String,
    pub email: Option<String>,
    pub name: Option<String>,
}

impl From<UserModel> for AuthUserDto {
    fn from(user: UserModel) -> Self {
        Self {
            account: user.account,
            email: user.email,
            name: user.name,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SigninRequestDto {
    #[serde(deserialize_with = "trim_string")]
    pub account: String,
    #[serde(deserialize_with = "trim_string")]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PasswardChangeRequestDto {
    #[serde(deserialize_with = "trim_string")]
    pub now_password: String,
    #[serde(deserialize_with = "trim_string")]
    #[validate(custom(function = "validate_password"))]
    pub password: String,
    #[serde(deserialize_with = "trim_string")]
    pub confirm_password: String,
}

impl PasswardChangeRequestDto {
    pub fn custom_validate(&self) -> Result<(), ValidationErrors> {
        let _ = &self.validate()?;

        if self.password != self.confirm_password {
            let mut errors = ValidationErrors::new();
            errors.add("confirm_password", ValidationError::new("password_match"));
            return Err(errors);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InfoChangeRequestDto {
    #[serde(deserialize_with = "trim_string")]
    pub password: String,
    #[serde(deserialize_with = "option_trim_string")]
    pub name: Option<String>,
    #[serde(deserialize_with = "option_trim_string")]
    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponseDto {
    pub account: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub token: String,
}

// region: custom deserializers
// endregion: custom deserializers

// region: custom validators
fn validate_account(s: &str) -> Result<(), ValidationError> {
    if is_valid_account(s) {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid value."))
    }
}

fn validate_password(s: &str) -> Result<(), ValidationError> {
    if is_valid_password(s) {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid value."))
    }
}
// endregion: custom validators
