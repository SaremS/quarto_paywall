use std::collections::HashMap;

pub struct EnvVarLoader {
    pub env_vars: HashMap<String, String>,
}

impl EnvVarLoader {


    pub fn new() -> EnvVarLoader {
    let variable_names: Vec<&str> = vec![
        "ADMIN_EMAIL",
        "ADMIN_PASSWORD",
        "PATH_STATIC_FILES",
        "STRIPE_SECRET_KEY",
        "STRIPE_WEBHOOK_KEY"
    ];
        let mut env_vars: HashMap<String, String> = HashMap::new();

        for key in variable_names.iter() {
            let value = std::env::var(key).expect(&format!("{:?} not defined.", key));
            env_vars.insert(key.to_string(), value);
        }

        return EnvVarLoader { env_vars };
    }

    pub fn get_admin_email(&self) -> String {
        return self.env_vars.get("ADMIN_EMAIL").unwrap().clone();
    }

    pub fn get_admin_password(&self) -> String {
        return self.env_vars.get("ADMIN_PASSWORD").unwrap().clone();
    }

    pub fn get_path_static_files(&self) -> String {
        return self.env_vars.get("PATH_STATIC_FILES").unwrap().clone();
    }

    pub fn get_stripe_secret_key(&self) -> String {
        return self.env_vars.get("STRIPE_SECRET_KEY").unwrap().clone();
    }

    pub fn get_stripe_webhook_key(&self) -> String {
        return self.env_vars.get("STRIPE_WEBHOOK_KEY").unwrap().clone();
    }

}
