use crate::models::stream_model::Stream;
use crate::types::stream_enum::StreamStatus;
use sqlx::{Error, PgPool, Result};
use uuid::Uuid;

pub struct StreamRepository {
    pool: PgPool,
}

impl StreamRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // 1. Tạo mới một Stream (INSERT)
    pub async fn create(
        &self,
        app: &str,
        stream_name: &str,
        stream_key: &str,
    ) -> Result<Stream, Error> {
        let stream = sqlx::query_as!(
            Stream,
            r#"
            INSERT INTO streams (app, stream_name, stream_key)
            VALUES ($1, $2, $3)
            RETURNING id, app, stream_name, stream_key, status as "status: StreamStatus", created_at, updated_at
            "#,
            app,
            stream_name,
            stream_key
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(stream)
    }

    // 2. Lấy thông tin Stream theo ID (SELECT ONE)
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Stream>, Error> {
        let stream = sqlx::query_as!(
            Stream,
            r#"
            SELECT id, app, stream_name, stream_key, status as "status: StreamStatus", created_at, updated_at
            FROM streams
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(stream)
    }

    // 3. Lấy danh sách các Stream đang Live (SELECT ALL with Filter)
    pub async fn find_all_live(&self) -> Result<Vec<Stream>, Error> {
        let streams = sqlx::query_as!(
            Stream,
            r#"
            SELECT id, app, stream_name, stream_key, status as "status: StreamStatus", created_at, updated_at
            FROM streams
            WHERE status = $1
            ORDER BY created_at DESC
            "#,
            StreamStatus::Live as StreamStatus
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(streams)
    }

    // 4. Cập nhật trạng thái Stream (UPDATE)
    pub async fn update_status(
        &self,
        id: Uuid,
        status: StreamStatus,
    ) -> Result<Option<Stream>, Error> {
        let stream = sqlx::query_as!(
            Stream,
            r#"
            UPDATE streams
            SET status = $1, updated_at = CURRENT_TIMESTAMP
            WHERE id = $2
            RETURNING id, app, stream_name, stream_key, status as "status: StreamStatus", created_at, updated_at
            "#,
            status as StreamStatus,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(stream)
    }

    // 5. Xóa Stream (DELETE)
    pub async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM streams
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn find_by_key(&self, stream_key: &str) -> Result<Option<Stream>, Error> {
        let stream = sqlx::query_as!(
            Stream,
            r#"
            SELECT id, app, stream_name, stream_key, status as "status: StreamStatus", created_at, updated_at
            FROM streams
            WHERE stream_key = $1
            "#,
            stream_key
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(stream)
    }
}
