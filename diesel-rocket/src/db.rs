// src/db.rs

use std::env;

use diesel::prelude::*;
use diesel::{r2d2, PgConnection};

use crate::models::*;
use crate::schema::*;
use crate::StdErr;

type PgPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub struct Db {
    pool: PgPool,
}

impl Db {
    pub fn connect() -> Result<Self, StdErr> {
        let db_url = env::var("DATABASE_URL")?;
        let manager = r2d2::ConnectionManager::new(db_url);
        let pool = r2d2::Builder::new()
            .max_size(100)
            .min_idle(Some(50))
            .build(manager)?;
        Ok(Db { pool })
    }

    // token methods

    pub fn validate_token(&self, token_id: &str) -> Result<Token, StdErr> {
        let conn = self.pool.get()?;
        let token = tokens::table
            .filter(tokens::id.eq(token_id))
            .filter(tokens::expired_at.ge(diesel::dsl::now))
            .first(&conn)?;
        Ok(token)
    }

    // board methods

    pub fn boards(&self) -> Result<Vec<Board>, StdErr> {
        let conn = self.pool.get()?;
        Ok(boards::table.load(&conn)?)
    }

    pub fn board_summary(&self, board_id: i64) -> Result<BoardSummary, StdErr> {
        let conn = self.pool.get()?;
        let counts: Vec<StatusCount> = diesel::sql_query(format!(
            "select count(*), status from cards where cards.board_id = {} group by status",
            board_id
        ))
        .load(&conn)?;
        Ok(counts.into())
    }

    pub fn create_board(&self, create_board: CreateBoard) -> Result<Board, StdErr> {
        let conn = self.pool.get()?;
        let board = diesel::insert_into(boards::table)
            .values(&create_board)
            .get_result(&conn)?;
        Ok(board)
    }

    pub fn delete_board(&self, board_id: i64) -> Result<(), StdErr> {
        let conn = self.pool.get()?;
        diesel::delete(boards::table.filter(boards::id.eq(board_id))).execute(&conn)?;
        Ok(())
    }

    // card methods

    pub fn cards(&self, board_id: i64) -> Result<Vec<Card>, StdErr> {
        let conn = self.pool.get()?;
        let cards = cards::table
            .filter(cards::board_id.eq(board_id))
            .load(&conn)?;
        Ok(cards)
    }

    pub fn create_card(&self, create_card: CreateCard) -> Result<Card, StdErr> {
        let conn = self.pool.get()?;
        let card = diesel::insert_into(cards::table)
            .values(create_card)
            .get_result(&conn)?;
        Ok(card)
    }

    pub fn update_card(&self, card_id: i64, update_card: UpdateCard) -> Result<Card, StdErr> {
        let conn = self.pool.get()?;
        let card = diesel::update(cards::table.filter(cards::id.eq(card_id)))
            .set(update_card)
            .get_result(&conn)?;
        Ok(card)
    }

    pub fn delete_card(&self, card_id: i64) -> Result<(), StdErr> {
        let conn = self.pool.get()?;
        diesel::delete(cards::table.filter(cards::id.eq(card_id))).execute(&conn)?;
        Ok(())
    }
}
