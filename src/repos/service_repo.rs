use sqlx::PgPool;
use crate::models::{ServiceCode, Services};

pub async fn fetch_by_category(
    pool: &PgPool,
    code: ServiceCode,
) -> sqlx::Result<Vec<Services>> {
    let services = sqlx::query_as::<_, Services>(
        r#"
        SELECT service_id, name_service, code, price, created_at
        FROM "Services"
        WHERE code = $1
        ORDER BY service_id
        "#
    )
    .bind(code) 
    .fetch_all(pool)
    .await?;

    Ok(services)
}


pub async fn fetch_service_by_id(pool: &PgPool, service_id: i32) -> sqlx::Result<Services> {
    let service = sqlx::query_as::<_, Services>(
        r#"
        SELECT service_id, name_service, code, price, created_at
        FROM "Services"
        WHERE service_id = $1
        "#
    )
    .bind(service_id)
    .fetch_one(pool)
    .await?;

    Ok(service)
}
