// src/db.rs
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use crate::models::*;
use crate::StdErr;

#[derive(Clone)]
pub struct Db {
    pool: Pool<Postgres>,
}

impl Db {
    pub async fn connect() -> Result<Self, StdErr> {
        let db_url = std::env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new()
            .max_connections(100)
            .min_connections(50)
            .connect(&db_url)
            .await?;
        Ok(Db { pool })
    }

    // authentication

    pub async fn validate_token<T: AsRef<str>>(&self, token_id: T) -> Result<Token, StdErr> {
        let token_id = token_id.as_ref();
        let token = sqlx::query_as("SELECT * FROM tokens WHERE id = $1 AND expired_at > current_timestamp")
            .bind(token_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(token)
    }

    // board data

    pub async fn boards(&self) -> Result<Vec<Board>, StdErr> {
        let boards = sqlx::query_as("SELECT * FROM boards")
            .fetch_all(&self.pool)
            .await?;
        Ok(boards)
    }

    pub async fn board_summary(&self, board_id: i64) -> Result<BoardSummary, StdErr> {
        let counts: Vec<(i64, Status)> = sqlx::query_as(
            "SELECT count(*), status FROM cards WHERE board_id = $1 GROUP BY status",
        )
        .bind(board_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(counts.into())
    }

    pub async fn create_board(&self, create_board: CreateBoard) -> Result<Board, StdErr> {
        let board = sqlx::query_as("INSERT INTO boards (name) VALUES ($1) RETURNING *")
            .bind(&create_board.name)
            .fetch_one(&self.pool)
            .await?;
        Ok(board)
    }

    pub async fn destroy_board(&self, board_id: i64) -> Result<(), StdErr> {
        sqlx::query("DELETE FROM boards WHERE id = $1")
            .bind(board_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // card data

    pub async fn cards(&self, board_id: i64) -> Result<Vec<Card>, StdErr> {
        let cards = sqlx::query_as("SELECT * FROM cards WHERE board_id = $1")
            .bind(board_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(cards)
    }

    pub async fn create_card(&self, create_card: CreateCard) -> Result<Card, StdErr> {
        let card =
            sqlx::query_as("INSERT INTO cards (board_id, description) VALUES ($1, $2) RETURNING *")
                .bind(&create_card.board_id)
                .bind(&create_card.description)
                .fetch_one(&self.pool)
                .await?;
        Ok(card)
    }

    pub async fn update_card(&self, card_id: i64, update_card: UpdateCard) -> Result<Card, StdErr> {
        let card = sqlx::query_as(
            "UPDATE cards SET description = $1, status = $2 WHERE id = $3 RETURNING *",
        )
        .bind(&update_card.description)
        .bind(&update_card.status)
        .bind(card_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(card)
    }

    pub async fn destroy_card(&self, card_id: i64) -> Result<(), StdErr> {
        sqlx::query("DELETE FROM cards WHERE id = $1")
            .bind(card_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
