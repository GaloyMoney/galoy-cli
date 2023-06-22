use crate::app::{token, App};

impl App {
    pub async fn user_login(&self, phone: String, code: String) -> anyhow::Result<()> {
        let result = self.client.user_login(phone, code).await?;
        match token::save_token(&result) {
            Ok(()) => println!("User logged in successfully!"),
            Err(err) => eprintln!("Failed to log in: {}", err),
        }
        Ok(())
    }
}
