use std::sync::Arc;

use anyhow::Result;
use sqlx::{types::time::OffsetDateTime, FromRow, Pool, QueryBuilder, Sqlite};
use tokio::task::JoinSet;

use crate::{
    models::{
        event::Event,
        generic::IdRow,
        notifier::{CreateNotifier, Notifier},
    },
    state::AppState,
};

#[derive(FromRow)]
pub struct NotifierRow {
    id: i64,
    created_at: OffsetDateTime,
    params: String,
}

impl TryFrom<NotifierRow> for Notifier {
    type Error = anyhow::Error;

    fn try_from(value: NotifierRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            created_at: value.created_at.to_string(),
            params: serde_json::from_str(value.params.as_str())?,
        })
    }
}

const SELECT: &str = r#"SELECT
    n."id",
    n."created_at",
    n."params"
FROM notifier n"#;

#[derive(Clone)]
pub struct NotifierService {
    pool: Pool<Sqlite>,
}

impl NotifierService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn find(&self, id: i64) -> Result<Notifier> {
        let mut qb = QueryBuilder::new(SELECT);
        qb.push(r#" WHERE n."id" = "#).push_bind(id);
        let row: NotifierRow = qb.build_query_as().fetch_one(&self.pool).await?;
        Ok(Notifier::try_from(row)?)
    }

    pub async fn find_all(&self) -> Result<Vec<Notifier>> {
        let rows: Vec<NotifierRow> = sqlx::query_as(SELECT).fetch_all(&self.pool).await?;
        Ok(rows
            .into_iter()
            .map(Notifier::try_from)
            .collect::<Result<_, _>>()?)
    }

    pub async fn create(&self, data: &CreateNotifier) -> Result<i64> {
        let params_string = serde_json::to_string(&data.params)?;
        let row = sqlx::query_as!(
            IdRow,
            r#"INSERT INTO notifier (
                "params"
            ) VALUES (
                $1
            ) RETURNING "id""#,
            params_string
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row.id)
    }

    pub async fn update(&self, id: i64, data: &CreateNotifier) -> Result<i64> {
        let params_string = serde_json::to_string(&data.params)?;
        let row = sqlx::query_as!(
            IdRow,
            r#"UPDATE notifier SET
                "params" = $1
            WHERE
                "id" = $2
            RETURNING "id""#,
            params_string,
            id,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row.id)
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query!(
            r#"DELETE FROM notifier
            WHERE "id" = $1"#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn handle(&self, state: Arc<AppState>, event: Event) -> Result<()> {
        let notifiers = self.find_all().await?;
        let mut join_set = JoinSet::new();
        let event = Arc::new(event);
        for notifier in notifiers {
            let state = state.clone();
            let event = event.clone();
            join_set.spawn(async move { notifier.params.send(state, event).await });
        }
        join_set.join_all().await;
        Ok(())
    }

    pub async fn start_worker(&self, state: Arc<AppState>) {
        let mut receiver = state.event_service.subscribe();
        while let Ok(event) = receiver.recv().await {
            if let Err(e) = self.handle(state.clone(), event).await {
                println!("Failed to send notification: {:?}", e)
            }
        }
    }
}
