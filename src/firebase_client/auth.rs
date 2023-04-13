use super::FirebaseClient;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FirebaseClientAuthenticationLoginWithRefreshTokenRequest {
    grant_type: String,
    refresh_token: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirebaseClientAuthenticationLoginWithRefreshTokenResponse {
    expires_in: String,
    token_type: String,
    refresh_token: String,
    id_token: String,
    user_id: String,
    project_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirebaseClientAuthenticationLoginWithCustomTokenRequest {
    token: String,
    return_secure_token: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirebaseClientAuthenticationLoginWithCustomTokenResponse {
    id_token: String,
    refresh_token: String,
    expires_in: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FirebaseClientAuthenticationSignUpWithEmailPasswordRequest {
    pub email: String,
    pub password: String,
    pub return_secure_token: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FirebaseClientAuthenticationSignUpWithEmailPasswordResponse {
    id_token: String,
    email: String,
    refresh_token: String,
    expires_in: String,
    local_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FirebaseClientAuthenticationSignInWithEmailPasswordRequest {
    pub email: String,
    pub password: String,
    pub return_secure_token: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FirebaseClientAuthenticationSignInWithEmailPasswordResponse {
    id_token: String,
    email: String,
    refresh_token: String,
    expires_in: String,
    local_id: String,
    registered: bool,
}

/*
* Firebase Documentation: https://firebase.google.com/docs/reference/rest/auth
*/
#[async_trait]
pub trait FirebaseClientAuthentication {
    async fn sign_in_with_custom_token(
        &self,
        request: FirebaseClientAuthenticationLoginWithCustomTokenRequest,
    ) -> Result<FirebaseClientAuthenticationLoginWithCustomTokenResponse, Box<dyn std::error::Error>>;

    async fn sign_in_with_refresh_token(
        &self,
        request: FirebaseClientAuthenticationLoginWithRefreshTokenRequest,
    ) -> Result<FirebaseClientAuthenticationLoginWithRefreshTokenResponse, Box<dyn std::error::Error>>;

    async fn sign_up_with_email_password(
        &self,
        request: FirebaseClientAuthenticationSignUpWithEmailPasswordRequest,
    ) -> Result<FirebaseClientAuthenticationSignUpWithEmailPasswordResponse, Box<dyn std::error::Error>>;

    async fn sign_in_with_email_password(
        &self,
        request: FirebaseClientAuthenticationSignInWithEmailPasswordRequest,
    ) -> Result<FirebaseClientAuthenticationSignInWithEmailPasswordResponse, Box<dyn std::error::Error>>;
}

#[async_trait]
impl FirebaseClientAuthentication for FirebaseClient {
    async fn sign_in_with_custom_token(
        &self,
        request: FirebaseClientAuthenticationLoginWithCustomTokenRequest,
    ) -> Result<FirebaseClientAuthenticationLoginWithCustomTokenResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/accounts:signInWithCustomToken?key={}", self.base_url, self.api_key);
        let response = self.client.post(&url).json(&request).send().await?;
        let json = response.json::<FirebaseClientAuthenticationLoginWithCustomTokenResponse>().await?;
        Ok(json)
    }

    async fn sign_in_with_refresh_token(
        &self,
        request: FirebaseClientAuthenticationLoginWithRefreshTokenRequest,
    ) -> Result<FirebaseClientAuthenticationLoginWithRefreshTokenResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/token?key={}", self.base_url, self.api_key);
        let response = self.client.post(&url).json(&request).send().await?;
        let json = response.json::<FirebaseClientAuthenticationLoginWithRefreshTokenResponse>().await?;
        Ok(json)
    }

    async fn sign_up_with_email_password(
        &self,
        request: FirebaseClientAuthenticationSignUpWithEmailPasswordRequest,
    ) -> Result<FirebaseClientAuthenticationSignUpWithEmailPasswordResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/accounts:signUp?key={}", self.base_url, self.api_key);
        let response = self.client.post(&url).json(&request).send().await?;
        let json = response.json::<FirebaseClientAuthenticationSignUpWithEmailPasswordResponse>().await?;
        Ok(json)
    }

    async fn sign_in_with_email_password(
        &self,
        request: FirebaseClientAuthenticationSignInWithEmailPasswordRequest,
    ) -> Result<FirebaseClientAuthenticationSignInWithEmailPasswordResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/accounts:signInWithPassword?key={}", self.base_url, self.api_key);
        let response = self.client.post(&url).json(&request).send().await?;
        let json = response.json::<FirebaseClientAuthenticationSignInWithEmailPasswordResponse>().await?;
        Ok(json)
    }
}
