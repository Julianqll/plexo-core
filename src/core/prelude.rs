use plexo_sdk::resources::members::extensions::{CreateMemberFromEmailInputBuilder, MembersExtensionOperations};

use crate::core::config::ADMIN_PHOTO_URL;

use super::{
    app::Core,
    config::{ADMIN_EMAIL, ADMIN_NAME, ADMIN_PASSWORD},
};

impl Core {
    pub async fn prelude(&self) -> Result<(), Box<dyn std::error::Error>> {
        let default_admin_email = (*ADMIN_EMAIL).clone();
        let default_admin_password = (*ADMIN_PASSWORD).clone();
        let default_admin_name = (*ADMIN_NAME).clone();
        let default_admin_photo_url = (*ADMIN_PHOTO_URL).clone();

        match self.engine.get_member_by_email(default_admin_email.clone()).await {
            Ok(Some(_admin)) => {
                println!("Default admin user already exists: {}", default_admin_email);
                return Ok(());
            }
            Err(e) => {
                println!("Error checking for default admin user: {}", e);
                return Err(Box::new(e));
            }
            _ => {}
        }

        println!("Creating default admin user: {}", default_admin_email);

        let hashed_password = self.auth.hash_password(default_admin_password.as_str());

        self.engine
            .create_member_from_email(
                CreateMemberFromEmailInputBuilder::default()
                    .email(default_admin_email)
                    .name(default_admin_name)
                    .password_hash(hashed_password)
                    .photo_url(default_admin_photo_url)
                    .build()?,
            )
            .await?;

        Ok(())
    }
}
