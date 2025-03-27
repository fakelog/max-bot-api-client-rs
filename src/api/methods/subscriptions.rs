use std::collections::HashMap;

use reqwest::Method;

use crate::api::{
    ApiClient,
    models::{GetSubscriptionsResult, SubscriptionRequestBody},
    types::Result,
};

impl ApiClient {
    pub async fn get_subscriptions(&self) -> Result<GetSubscriptionsResult> {
        let response = self
            .send_request::<()>(Method::GET, "/subscriptions", None, None)
            .await?;
        Ok(response.json().await?)
    }

    pub async fn subscribe(&self, subscription: &SubscriptionRequestBody) -> Result<()> {
        self.send_request(Method::POST, "/subscriptions", None, Some(subscription))
            .await?;
        Ok(())
    }

    pub async fn unsubscribe(&self, url: &str) -> Result<()> {
        let mut query = HashMap::new();
        query.insert("url", url.to_string());

        self.send_request::<()>(Method::DELETE, "/subscriptions", Some(&query), None)
            .await?;
        Ok(())
    }
}
