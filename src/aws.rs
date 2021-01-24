use crate::arguments::Arguments;
use anyhow::*;
use rusoto_core::Region;
#[cfg(test)]
use rusoto_mock::*;
#[cfg(test)]
use rusoto_sts::AssumeRoleResponse;
use rusoto_sts::{AssumeRoleRequest, Credentials, Sts, StsClient};
use std::str::FromStr;

pub const ACCESS_KEY_ID: &str = "AWS_ACCESS_KEY_ID";
pub const SECRET_ACCESS_KEY: &str = "AWS_SECRET_ACCESS_KEY";
pub const SESSION_TOKEN: &str = "AWS_SESSION_TOKEN";

const SESSION_NAME: &str = "skastsar_temp_session";

async fn assume_role_exec(
    account: &String,
    role: &String,
    mfa_id: Option<String>,
    mfa_token: Option<String>,
    client: &StsClient,
) -> Result<Credentials> {
    let request = AssumeRoleRequest {
        role_arn: format!("arn:aws:iam::{}:role/{}", account, role),
        role_session_name: String::from(SESSION_NAME),
        serial_number: mfa_id,
        token_code: mfa_token,
        ..AssumeRoleRequest::default()
    };

    let output = client.assume_role(request).await?;
    if let Some(credentials) = output.credentials {
        Ok(credentials)
    } else {
        bail!("no credentials")
    }
}

pub async fn assume_role(
    account: &String,
    role: &String,
    region: Region,
    arguments: &Arguments,
) -> Result<Credentials> {
    let client = StsClient::new(region);
    let mfa_id = arguments.mfa_id.clone();
    let mfa_token = arguments.mfa_token.clone();
    assume_role_exec(account, role, mfa_id, mfa_token, &client).await
}

pub fn get_region(preselect: &Option<String>) -> Result<Region> {
    if let Some(region) = preselect {
        let region = Region::from_str(region)?;
        Ok(region)
    } else {
        Ok(Region::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_from_string() {
        assert_eq!(
            get_region(&Some(String::from("EuCentral1"))).unwrap(),
            Region::EuCentral1
        );
        assert_eq!(
            get_region(&Some(String::from("UsEast1"))).unwrap(),
            Region::UsEast1
        );
        assert_eq!(get_region(&None).unwrap(), Region::default());
    }

    #[test]
    fn region_error_fallback() {
        assert!(get_region(&Some(String::from(""))).is_err());
        assert!(get_region(&Some(String::from("foobar"))).is_err());
    }

    // I think this test should pass, but it doesn't
    // see: https://github.com/rusoto/rusoto/issues/1895
    #[tokio::test]
    async fn mock_sts_call() {
        let account = String::from("123456789123");
        let role = String::from("user");
        let credentials = Credentials {
            access_key_id: String::from("ASIA1231231231231234"),
            secret_access_key: String::from("123123123"),
            session_token: String::from("00000000000000"),
            ..Credentials::default()
        };
        let response = AssumeRoleResponse {
            credentials: Some(credentials.clone()),
            ..AssumeRoleResponse::default()
        };
        let client = StsClient::new_with(
            MockRequestDispatcher::default().with_json_body(response),
            MockCredentialsProvider,
            Default::default(),
        );

        let result = assume_role_exec(&account, &role, None, None, &client).await;
        assert_eq!(result.unwrap(), credentials);
    }
}
