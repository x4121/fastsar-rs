extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
struct T {
    name: String,
    id: String,
    roles: Vec<String>,
}

fn main() {
    let json_str = r#"
    [
    {
        "name": "admin",
        "id": "547382574378",
        "roles": ["ryte-user-admin"]
    },
    {
        "name": "production-new",
        "id": "338378888882",
        "roles": ["OrganizationAccountAccessRole"]
    }
    ]
    "#;
    let res: Vec<T> = serde_json::from_str(json_str).unwrap();
    println!(
        "name: {}, id: {}, role: {}",
        res[0].name, res[0].id, res[0].roles[0]
    )
}
