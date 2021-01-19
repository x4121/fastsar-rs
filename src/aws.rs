use crate::arguments::Arguments;
use rusoto_core::Region;
use rusoto_sts::{AssumeRoleRequest, Credentials, Sts, StsClient};
use std::str::FromStr;

pub const ACCESS_KEY_ID: &str = "AWS_ACCESS_KEY_ID";
pub const SECRET_ACCESS_KEY: &str = "AWS_SECRET_ACCESS_KEY";
pub const SESSION_TOKEN: &str = "AWS_SESSION_TOKEN";

const SESSION_NAME: &str = "skastsar_temp_session";

pub async fn assume_role(
    account: &String,
    role: &String,
    region: Region,
    arguments: &Arguments,
) -> Result<Credentials, String> {
    let request = AssumeRoleRequest {
        role_arn: format!("arn:aws:iam::{}:role/{}", account, role),
        role_session_name: String::from(SESSION_NAME),
        serial_number: arguments.mfa_id.clone(),
        token_code: arguments.mfa_token.clone(),
        ..AssumeRoleRequest::default()
    };
    let client = StsClient::new(region);

    match client.assume_role(request).await {
        Ok(output) => match output.credentials {
            Some(credentials) => Ok(credentials),
            None => Err(String::from("no credentials")),
        },
        Err(error) => Err(format!("Error: {:?}", error)),
    }
}

pub fn get_region(preselect: &Option<String>) -> Region {
    match preselect {
        Some(region) => Region::from_str(region).unwrap(),
        _ => Region::default(),
    }
}
