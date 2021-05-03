use rocket::http;
use rocket::State;
use rocket::request::{FromRequest, Request, Outcome};
use rocket_contrib::json::Json;

use crate::db::Db;
use crate::models::*;
use crate::StdErr;

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = &'static str;
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        // get request headers
        let headers = request.headers();

        // check that Authorization header exists
        let maybe_auth_header = headers.get_one("Authorization");
        if maybe_auth_header.is_none() {
            return Outcome::Failure((
                http::Status::Unauthorized,
                "missing Authorization header",
            ));
        }

        // and is well-formed
        let auth_header = maybe_auth_header.unwrap();
        let mut auth_header_parts = auth_header.split_ascii_whitespace();
        let maybe_auth_type = auth_header_parts.next();
        if maybe_auth_type.is_none() {
            return Outcome::Failure((
                http::Status::Unauthorized,
                "malformed Authorization header",
            ));
        }

        // and uses the Bearer token authorization method
        let auth_type = maybe_auth_type.unwrap();
        if auth_type != "Bearer" {
            return Outcome::Failure((
                http::Status::BadRequest,
                "invalid Authorization type",
            ));
        }

        // and the Bearer token is present
        let maybe_token_id = auth_header_parts.next();
        if maybe_token_id.is_none() {
            return Outcome::Failure((http::Status::Unauthorized, "missing Bearer token"));
        }
        let token_id = maybe_token_id.unwrap();

        // we can use request.guard::<T>() to get a T from a request
        // which includes managed application state like our Db
        let outcome_db = request.guard::<State<Db>>();
        let db: State<Db> = match outcome_db {
            Outcome::Success(db) => db,
            _ => return Outcome::Failure((http::Status::InternalServerError, "internal error")),
        };

        // validate token
        let token_result = db.validate_token(token_id);
        match token_result {
            Ok(token) => Outcome::Success(token),
            Err(_) => Outcome::Failure((
                http::Status::Unauthorized,
                "invalid or expired Bearer token",
            )),
        }
    }
}

// board routes

#[rocket::get("/boards")]
fn boards(db: State<Db>, _t: Token) -> Result<Json<Vec<Board>>, StdErr> {
    db.boards().map(Json)
}

#[rocket::post("/boards", data = "<create_board>")]
fn create_board(db: State<Db>, create_board: Json<CreateBoard>, _t: Token) -> Result<Json<Board>, StdErr> {
    db.create_board(create_board.0).map(Json)
}

#[rocket::get("/boards/<board_id>/summary")]
fn board_summary(db: State<Db>, board_id: i64, _t: Token) -> Result<Json<BoardSummary>, StdErr> {
    db.board_summary(board_id).map(Json)
}

#[rocket::delete("/boards/<board_id>")]
fn delete_board(db: State<Db>, board_id: i64, _t: Token) -> Result<(), StdErr> {
    db.delete_board(board_id)
}

// card routes

#[rocket::get("/boards/<board_id>/cards")]
fn cards(db: State<Db>, board_id: i64, _t: Token) -> Result<Json<Vec<Card>>, StdErr> {
    db.cards(board_id).map(Json)
}

#[rocket::post("/cards", data = "<create_card>")]
fn create_card(db: State<Db>, create_card: Json<CreateCard>, _t: Token) -> Result<Json<Card>, StdErr> {
    db.create_card(create_card.0).map(Json)
}

#[rocket::patch("/cards/<card_id>", data = "<update_card>")]
fn update_card(
    db: State<Db>,
    card_id: i64,
    update_card: Json<UpdateCard>,
    _t: Token,
) -> Result<Json<Card>, StdErr> {
    db.update_card(card_id, update_card.0).map(Json)
}

#[rocket::delete("/cards/<card_id>")]
fn delete_card(db: State<Db>, card_id: i64, _t: Token) -> Result<(), StdErr> {
    db.delete_card(card_id)
}

pub fn api() -> Vec<rocket::Route> {
    rocket::routes![
        boards,
        create_board,
        board_summary,
        delete_board,
        cards,
        create_card,
        update_card,
        delete_card,
    ]
}
