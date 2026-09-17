use std::sync::Arc;

use crate::error::AppError;
use crate::libs::{jwt, password};
use crate::model::auth::{
    AuthResponseDto, AuthUserDto, InfoChangeRequestDto, PasswardChangeRequestDto, SigninRequestDto,
    SignupRequestDto,
};
use common::config;
use domain::{DomainError, Repositories};
use validator::Validate;

pub struct AuthUseCase {
    repositories: Arc<dyn Repositories>,
}

impl AuthUseCase {
    pub fn new(repositories: Arc<dyn Repositories>) -> Self {
        Self { repositories }
    }

    pub async fn signup(&self, dto: SignupRequestDto) -> Result<(), AppError> {
        dto.custom_validate()?;

        self.repositories
            .user()
            .save(dto.to_model().await?)
            .await
            .map_err(|error| match error {
                DomainError::Conflict(_) => {
                    AppError::BadRequest("Incorrect account or password.".to_string())
                }
                error => AppError::from(error),
            })?;

        Ok(())
    }

    pub async fn signin(&self, dto: SigninRequestDto) -> Result<AuthResponseDto, AppError> {
        let mut user = self
            .repositories
            .user()
            .find(&dto.account)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Incorrect account or password.".to_string()))?;

        if !password::verify(dto.password, user.password.clone()).await? {
            return Err(AppError::Unauthorized(
                "Incorrect account or password.".to_string(),
            ));
        }

        let jwt_id = uuid25::gen_v4().to_string();

        let token = jwt::generate(
            user.account.clone(),
            config().security.jwt_issuer.clone(),
            config().security.jwt_expires_minutes,
            jwt_id.clone(),
            config().security.jwt_secret.clone(),
        )?;

        user.jwt_id = Some(jwt_id);

        let user = self
            .repositories
            .user()
            .replace(user)
            .await
            .map_err(|ex| AppError::Unexpected(ex.into()))?;

        Ok(AuthResponseDto {
            account: user.account,
            email: user.email,
            name: user.name,
            token,
        })
    }

    pub async fn authenticate(&self, token: String) -> Result<AuthUserDto, AppError> {
        let (account, jwt_id) = jwt::verify(
            token,
            config().security.jwt_issuer.clone(),
            config().security.jwt_secret.clone(),
        )
        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

        let user = self
            .repositories
            .user()
            .find(&account)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid token".to_string()))?;

        if user.jwt_id != Some(jwt_id) {
            return Err(AppError::Unauthorized("Invalid token".to_string()));
        }

        Ok(AuthUserDto::from(user))
    }

    pub async fn signout(&self, token: String) -> Result<(), AppError> {
        let dto = match self.authenticate(token).await {
            Ok(dto) => dto,
            Err(AppError::Unauthorized(_)) => return Ok(()),
            Err(error) => return Err(AppError::Unexpected(error.into())),
        };

        if let Some(mut user) = self.repositories.user().find(&dto.account).await? {
            user.jwt_id = None;
            self.repositories
                .user()
                .replace(user)
                .await
                .map_err(|ex| AppError::Unexpected(ex.into()))?;
        }

        Ok(())
    }

    pub async fn password_change(
        &self,
        dto: PasswardChangeRequestDto,
        account: &str,
    ) -> Result<(), AppError> {
        dto.custom_validate()?;

        let mut user = self
            .repositories
            .user()
            .find(account)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Incorrect account or password.".to_string()))?;

        if !password::verify(dto.now_password, user.password.clone()).await? {
            return Err(AppError::Unauthorized(
                "Incorrect account or password.".to_string(),
            ));
        }

        user.password = password::hash(dto.password).await?;

        self.repositories
            .user()
            .replace(user)
            .await
            .map_err(|ex| AppError::Unexpected(ex.into()))?;

        Ok(())
    }

    pub async fn info_change(
        &self,
        dto: InfoChangeRequestDto,
        account: &str,
    ) -> Result<(), AppError> {
        dto.validate()?;

        let mut user = self
            .repositories
            .user()
            .find(account)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Incorrect account or password.".to_string()))?;

        if !password::verify(dto.password, user.password.clone()).await? {
            return Err(AppError::Unauthorized(
                "Incorrect account or password.".to_string(),
            ));
        }

        user.email = dto.email;
        user.name = dto.name;

        self.repositories
            .user()
            .replace(user)
            .await
            .map_err(|ex| AppError::Unexpected(ex.into()))?;

        Ok(())
    }
}
