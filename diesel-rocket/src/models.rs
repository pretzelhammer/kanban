// src/models.rs

use crate::schema::*;

// for authentication

#[derive(diesel::Queryable)]
pub struct Token {
    pub id: String,
    pub expired_at: chrono::DateTime<chrono::Utc>,
}

// for GET requests

#[derive(Debug, serde::Serialize, diesel::Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    id: i64,
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Serialize, diesel::Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    id: i64,
    board_id: i64,
    description: String,
    status: Status,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, diesel_derive_enum::DbEnum)]
#[serde(rename_all = "camelCase")]
#[DieselType = "Status_enum"]
pub enum Status {
    Todo,
    Doing,
    Done,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct BoardSummary {
    todo: i64,
    doing: i64,
    done: i64,
}

#[derive(diesel::QueryableByName)]
pub struct StatusCount {
    #[sql_type = "diesel::sql_types::BigInt"]
    pub count: i64,
    #[sql_type = "Status_enum"]
    pub status: Status,
}

impl From<Vec<StatusCount>> for BoardSummary {
    fn from(counts: Vec<StatusCount>) -> BoardSummary {
        let mut summary = BoardSummary::default();
        for StatusCount { count, status } in counts {
            match status {
                Status::Todo => summary.todo += count,
                Status::Doing => summary.doing += count,
                Status::Done => summary.done += count,
            }
        }
        summary
    }
}

// for POST requests

#[derive(serde::Deserialize, diesel::Insertable)]
#[table_name = "boards"]
pub struct CreateBoard {
    pub name: String,
}

#[derive(serde::Deserialize, diesel::Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "cards"]
pub struct CreateCard {
    pub board_id: i64,
    pub description: String,
}

// for PATCH requests

#[derive(serde::Deserialize, diesel::AsChangeset)]
#[table_name = "cards"]
pub struct UpdateCard {
    pub description: String,
    pub status: Status,
}
