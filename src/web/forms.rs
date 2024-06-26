use serde::Deserialize;
use tracing::error;

use crate::data::{interfaces::db::Manager, models::customer::PotentialCustomer};

#[derive(Deserialize, Debug)]
pub struct PotentialCustomerForm {
    pub name: String,
    pub email: String,
    pub message: String,
    pub agent: String,
    pub language: String,
    pub url: String,
}

impl PotentialCustomerForm {
    pub async fn create(&self) {
        // TODO: handle possible errors and correct response
        match PotentialCustomer::create(
            "name, email, message, agent, language, url",
            format!(
                "'{}', '{}', '{}', '{}', '{}', '{}'",
                self.name, self.email, self.message, self.agent, self.language, self.url
            )
            .as_str(),
        )
        .await
        {
            Ok(_result) => {}
            Err(err) => error!("error executing sql: {:?}", err),
        };
    }
}
