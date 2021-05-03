use std::pin::Pin;

use actix_web::{FromRequest, HttpRequest, HttpResponse};
use actix_web::web::{Data, Json, Path};
use actix_web::http::StatusCode;
use actix_web::error::InternalError;
use actix_web::dev::{HttpServiceFactory, Payload};
use futures::{Future, FutureExt, future};

use crate::StdErr;
use crate::db::Db;
use crate::models::*;

impl FromRequest for Token {
    type Error = InternalError<&'static str>;
    type Config = ();

    // we return a Future that is either
    // - immediately ready (on a bad request with a missing or malformed Authorization header)
    // - ready later (pending on a db query that validates the request's Bearer token)
    type Future = future::Either<
        future::Ready<Result<Self, Self::Error>>,
        Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + 'static>>,
    >;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // get request headers
        let headers = req.headers();

        // check that Authorization header exists
        let maybe_auth = headers.get("Authorization");
        if maybe_auth.is_none() {
            return future::err(InternalError::new(
                "missing Authorization header",
                StatusCode::BAD_REQUEST,
            ))
            .left_future();
        }

        // and is well-formed
        let auth_config = maybe_auth.unwrap().to_str();
        if auth_config.is_err() {
            return future::err(InternalError::new(
                "malformed Authorization header",
                StatusCode::BAD_REQUEST,
            ))
            .left_future();
        }

        // and specifies some authorization method / is non-empty
        let mut auth_config_parts = auth_config.unwrap().split_ascii_whitespace();
        let maybe_auth_type = auth_config_parts.next();
        if maybe_auth_type.is_none() {
            return future::err(InternalError::new(
                "missing Authorization type",
                StatusCode::BAD_REQUEST,
            ))
            .left_future();
        }

        // and that authorization method is a bearer token
        let auth_type = maybe_auth_type.unwrap();
        if auth_type != "Bearer" {
            return future::err(InternalError::new(
                "unsupported Authorization type",
                StatusCode::BAD_REQUEST,
            ))
            .left_future();
        }

        // and a bearer token is present
        let maybe_token_id = auth_config_parts.next();
        if maybe_token_id.is_none() {
            return future::err(InternalError::new(
                "missing Bearer token",
                StatusCode::BAD_REQUEST,
            ))
            .left_future();
        }

        // we can fetch managed application data using HTTPRequest.app_data::<T>()
        let db = req.app_data::<Data<Db>>();
        if db.is_none() {
            return future::err(InternalError::new(
                "internal error",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
            .left_future();
        }

        // clone these so that we can return an impl Future + 'static
        let db = db.unwrap().clone();
        let token_id = maybe_token_id.unwrap().to_owned();

        async move {
            db.validate_token(token_id)
                .await
                .map_err(|_| InternalError::new("invalid Bearer token", StatusCode::UNAUTHORIZED))
        }
        .boxed_local()
        .right_future()
    }
}

// some convenience functions

fn to_internal_error(e: StdErr) -> InternalError<StdErr> {
    InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
}

fn to_ok(_: ()) -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}

// board routes

#[actix_web::get("/boards")]
async fn boards(
    db: Data<Db>,
    _t: Token
) -> Result<Json<Vec<Board>>, InternalError<StdErr>> {
    db.boards()
        .await
        .map(Json)
        .map_err(to_internal_error)
}

#[actix_web::post("/boards")]
async fn create_board(
    db: Data<Db>,
    create_board: Json<CreateBoard>,
    _t: Token,
) -> Result<Json<Board>, InternalError<StdErr>> {
    db.create_board(create_board.0)
        .await
        .map(Json)
        .map_err(to_internal_error)
}

#[actix_web::get("/boards/{board_id}/summary")]
async fn board_summary(
    db: Data<Db>,
    Path(board_id): Path<i64>,
    _t: Token,
) -> Result<Json<BoardSummary>, InternalError<StdErr>> {
    db.board_summary(board_id)
        .await
        .map(Json)
        .map_err(to_internal_error)
}

#[actix_web::delete("/boards/{board_id}")]
async fn destroy_board(
    db: Data<Db>,
    Path(board_id): Path<i64>,
    _t: Token,
) -> Result<HttpResponse, InternalError<StdErr>> {
    db.destroy_board(board_id)
        .await
        .map(to_ok)
        .map_err(to_internal_error)
}

// card routes

#[actix_web::get("/boards/{board_id}/cards")]
async fn cards(
    db: Data<Db>,
    Path(board_id): Path<i64>,
    _t: Token,
) -> Result<Json<Vec<Card>>, InternalError<StdErr>> {
    db.cards(board_id)
        .await
        .map(Json)
        .map_err(to_internal_error)
}

#[actix_web::post("/cards")]
async fn create_card(
    db: Data<Db>,
    create_card: Json<CreateCard>,
    _t: Token,
) -> Result<Json<Card>, InternalError<StdErr>> {
    db.create_card(create_card.0)
        .await
        .map(Json)
        .map_err(to_internal_error)
}

#[actix_web::patch("/cards/{card_id}")]
async fn update_card(
    db: Data<Db>,
    Path(card_id): Path<i64>,
    update_card: Json<UpdateCard>,
    _t: Token,
) -> Result<Json<Card>, InternalError<StdErr>> {
    db.update_card(card_id, update_card.0)
        .await
        .map(Json)
        .map_err(to_internal_error)
}

#[actix_web::delete("/cards/{card_id}")]
async fn destroy_card(
    db: Data<Db>,
    Path(card_id): Path<i64>,
    _t: Token,
) -> Result<HttpResponse, InternalError<StdErr>> {
    db.destroy_card(card_id)
        .await
        .map(to_ok)
        .map_err(to_internal_error)
}

pub fn api() -> impl HttpServiceFactory + 'static {
    actix_web::web::scope("/api")
        .service(boards)
        .service(board_summary)
        .service(create_board)
        .service(destroy_board)
        .service(cards)
        .service(create_card)
        .service(update_card)
        .service(destroy_card)
}
