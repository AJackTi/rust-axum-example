use async_trait::async_trait;
use mockall::automock;
use anyhow::Result;

use crate::{ db::postgres::Db, models::part::{ NewPart, Part, PartList, PartQuery } };

pub struct PartRepositoryImpl {
    pool: Db,
}

impl PartRepositoryImpl {
    pub fn new(pool: Db) -> Self {
        Self { pool }
    }
}

#[automock]
#[async_trait]
pub trait PartRepository {
    async fn find_all(&self, conditions: &PartQuery) -> Result<PartList>;
    async fn create(&self, part_data: &NewPart) -> Result<Part>;
    async fn update(&self, part_data: &Part) -> Result<Part>;
    async fn delete(&self, part_id: i32) -> Result<u64>;
    async fn find_by_id(&self, part_id: i32) -> Result<Part>;
}

#[async_trait]
impl PartRepository for PartRepositoryImpl {
    async fn find_all(&self, conditions: &PartQuery) -> Result<PartList> {
        let mut query = sqlx::query_as::<_, Part>("SELECT * FROM parts");
        if let Some(name) = &conditions.name {
            query = sqlx
                ::query_as::<_, Part>("SELECT * FROM parts WHERE name LIKE $1")
                .bind(format!("%{}%", name));
        }
        let result = query.fetch_all(&*self.pool).await?;
        Ok(result)
    }

    async fn create(&self, part_data: &NewPart) -> Result<Part> {
        let created_part = sqlx
            ::query_as::<_, Part>(
                r#"
            INSERT INTO parts (name, car_id)
            VALUES ($1, $2)
            RETURNING id, name, car_id
            "#
            )
            .bind(&part_data.name)
            .bind(&part_data.car_id)
            .fetch_one(&*self.pool).await?;
        Ok(created_part)
    }

    async fn update(&self, part_data: &Part) -> Result<Part> {
        let updated_part = sqlx
            ::query_as::<_, Part>(
                r#"
            UPDATE parts
            SET name = $2, car_id = $3
            WHERE id = $1
            RETURNING id, name, car_id
            "#
            )
            .bind(&part_data.id)
            .bind(&part_data.name)
            .bind(&part_data.car_id)
            .fetch_one(&*self.pool).await?;
        Ok(updated_part)
    }

    async fn delete(&self, part_id: i32) -> Result<u64> {
        let query = sqlx
            ::query("DELETE FROM parts WHERE id = $1")
            .bind(part_id)
            .execute(&*self.pool).await?;
        Ok(query.rows_affected())
    }

    async fn find_by_id(&self, part_id: i32) -> Result<Part> {
        let row = sqlx
            ::query_as::<_, Part>("SELECT * FROM parts WHERE id = $1")
            .bind(part_id)
            .fetch_one(&*self.pool).await?;
        Ok(row)
    }
}
