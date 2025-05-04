use chitchatrustserver::utils::env_provider;

#[test]
fn test_env() {
    println!("{}", env_provider::MARIADB_USER.to_string());
}
