use rusoto_core::Region;
use rusoto_sts::{AssumeRoleRequest, Credentials, Sts, StsClient};

pub const ACCESS_KEY_ID: &str = "AWS_ACCESS_KEY_ID";
pub const SECRET_ACCESS_KEY: &str = "AWS_SECRET_ACCESS_KEY";
pub const SESSION_TOKEN: &str = "AWS_SESSION_TOKEN";

const SESSION_NAME: &str = "skastsar_temp_session";
const REGION: Region = Region::EuWest1;

pub async fn assume_role(account: &String, role: &String) -> Result<Credentials, String> {
    let request = AssumeRoleRequest {
        role_arn: format!("arn:aws:iam::{}:role/{}", account, role),
        role_session_name: String::from(SESSION_NAME),
        ..AssumeRoleRequest::default()
    };
    let client = StsClient::new(REGION);

    match client.assume_role(request).await {
        Ok(output) => match output.credentials {
            Some(credentials) => Ok(credentials),
            None => Err(String::from("no credentials")),
        },
        Err(error) => Err(format!("Error: {:?}", error)),
    }
}
