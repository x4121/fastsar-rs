use rusoto_core::Region;
use rusoto_sts::{AssumeRoleRequest, Credentials, Sts, StsClient};
use std::env;

#[tokio::main]
async fn main() {
    let account = "517411888864";
    let role = "OrganizationAccountAccessRole";
    let session = "fastsar_temp_session";
    let request = AssumeRoleRequest {
        role_arn: format!("arn:aws:iam::{}:role/{}", account, role),
        role_session_name: String::from(session),
        ..AssumeRoleRequest::default()
    };
    let client = StsClient::new(Region::EuWest1);

    match client.assume_role(request).await {
        Ok(output) => match output.credentials {
            Some(credentials) => {
                set_credentials(credentials);
                println!("OK:");
            }
            None => println!("No tables in database!"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}

fn set_credentials(credentials: Credentials) {
    env::set_var("AWS_ACCESS_KEY_ID", credentials.access_key_id);
    env::set_var("AWS_SECRET_ACCESS_KEY", credentials.secret_access_key);
    env::set_var("AWS_SESSION_TOKEN", credentials.session_token);
}
