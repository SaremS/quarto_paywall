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
            "STRIPE_WEBHOOK_KEY",
            "JWT_SECRET_KEY",
            "DOMAIN_URL",
            "SMTP_MAIL_ADDRESS",
            "SMTP_SENDER_NAME",
            "SMTP_HOST",
            "SMTP_USERNAME",
            "SMTP_PASSWORD",
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

    pub fn get_jwt_secret_key(&self) -> String {
        return self.env_vars.get("JWT_SECRET_KEY").unwrap().clone();
    }

    //TODO: Use separate mail key
    pub fn get_mail_secret_key(&self) -> String {
        return self.env_vars.get("JWT_SECRET_KEY").unwrap().clone();
    }

    //TODO: Use separate deletion key
    pub fn get_deletion_secret_key(&self) -> String {
        return self.env_vars.get("JWT_SECRET_KEY").unwrap().clone();
    }

    pub fn get_domain_url(&self) -> String {
        return self.env_vars.get("DOMAIN_URL").unwrap().clone();
    }

    pub fn get_smtp_mail_address(&self) -> String {
        return self.env_vars.get("SMTP_MAIL_ADDRESS").unwrap().clone();
    }

    pub fn get_smtp_sender_name(&self) -> String {
        return self.env_vars.get("SMTP_SENDER_NAME").unwrap().clone();
    }

    pub fn get_smtp_host(&self) -> String {
        return self.env_vars.get("SMTP_HOST").unwrap().clone();
    }

    pub fn get_smtp_username(&self) -> String {
        return self.env_vars.get("SMTP_USERNAME").unwrap().clone();
    }

    pub fn get_smtp_password(&self) -> String {
        return self.env_vars.get("SMTP_PASSWORD").unwrap().clone();
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use temp_env::with_vars;


    #[test]
    fn test_set_and_get() {
        with_vars([
            ("ADMIN_EMAIL", Some("admin_email_test")),
            ("ADMIN_PASSWORD", Some("admin_password_test")),
            ("PATH_STATIC_FILES", Some("path_static_files_test")),
            ("STRIPE_SECRET_KEY", Some("stripe_secret_key_test")),
            ("STRIPE_WEBHOOK_KEY", Some("stripe_webhook_key_test")),
            ("JWT_SECRET_KEY", Some("jwt_secret_key_test")),
            ("DOMAIN_URL", Some("domain_url_test")),
            ("SMTP_MAIL_ADDRESS", Some("smtp_mail_address_test")),
            ("SMTP_SENDER_NAME", Some("smtp_sender_name_test")),
            ("SMTP_HOST", Some("smtp_host_test")),
            ("SMTP_USERNAME", Some("smtp_username_test")),
            ("SMTP_PASSWORD", Some("smtp_password_test"))
        ],|| {
            let loader = EnvVarLoader::new();

            assert_eq!(loader.get_admin_email(), "admin_email_test");
            assert_eq!(loader.get_admin_password(), "admin_password_test");
            assert_eq!(loader.get_path_static_files(), "path_static_files_test");
            assert_eq!(loader.get_stripe_secret_key(), "stripe_secret_key_test");
            assert_eq!(loader.get_jwt_secret_key(), "jwt_secret_key_test");
            assert_eq!(loader.get_domain_url(), "domain_url_test");
            assert_eq!(loader.get_smtp_mail_address(), "smtp_mail_address_test");
            assert_eq!(loader.get_smtp_sender_name(), "smtp_sender_name_test");
            assert_eq!(loader.get_smtp_host(), "smtp_host_test");
            assert_eq!(loader.get_smtp_username(), "smtp_username_test");
            assert_eq!(loader.get_smtp_password(), "smtp_password_test");
        }); 
    }
}
