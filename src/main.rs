use auth_service::Credentials;

fn main() {
    let cred = Credentials {
        username: "mahesh".to_owned(),
        password: "password123".to_owned(),
    };

    auth_service::authenticate(cred);
}
