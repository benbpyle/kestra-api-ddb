use aws_sdk_dynamodb::{types::AttributeValue, Client};

use chrono::Utc;
use svix_ksuid::{Ksuid, KsuidLike};
use tracing::info;

use crate::models::ReadModel;
use crate::models::WriteModel;

pub async fn create_item(client: &Client, table_name: &str, item: WriteModel) -> ReadModel {
    let ksuid = Ksuid::new(None, None);
    let dt = Utc::now();

    let result = client
        .put_item()
        .item("id".to_string(), AttributeValue::S(ksuid.to_string()))
        .item("name".to_string(), AttributeValue::S(item.name.to_string()))
        .item(
            "description".to_string(),
            AttributeValue::S(item.description.to_string()),
        )
        .item("created_at".to_string(), AttributeValue::S(dt.to_string()))
        .item("updated_at".to_string(), AttributeValue::S(dt.to_string()))
        .table_name(table_name)
        .send()
        .await;

    info!("(Result)={:?}", result);
    ReadModel {
        id: ksuid.to_string(),
    }
}
