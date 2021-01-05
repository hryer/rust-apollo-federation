#[macro_use]
extern crate thiserror;

use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, ID,ErrorExtensions, FieldError};
use actix_web::{guard, web, App, HttpResponse, HttpServer, Responder};
use async_graphql_actix_web::{Request, Response };

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Could not find resource")]
    NotFound,

    #[error("ServerError")]
    ServerError(String),

    #[error("No Extensions")]
    ErrorWithoutExtensions,
}

impl ErrorExtensions for MyError {
  // lets define our base extensions
  fn extend(&self) -> FieldError {
      self.extend_with(|err, e| match err {
          MyError::NotFound => e.set("code", "NOT_FOUND"),
          MyError::ServerError(reason) => e.set("reason", reason.to_string()),
          MyError::ErrorWithoutExtensions => {}
      })
  }
}

#[derive(SimpleObject)]
struct Leads {
  id: ID,
  username: String,
}

struct Query;

#[Object(extends)]
impl Query {
  async fn me(&self) -> Leads {
    Leads {
      id: "1234".into(),
      username: "Me".to_string(),
    }
  }

  #[graphql(entity)]
  async fn find_user_by_id(&self, id: ID) -> Leads{
    let username = if id == "1234" {
      "Me".to_string()
    } else {
      format!("Leads{:?}", id)
    };
    Leads{ id, username }
  }
}

async fn index(
  schema: web::Data<Schema<Query, EmptyMutation, EmptySubscription>>,
  req: Request,
) -> Response {
  schema.execute(req.into_inner()).await.into()
}

async fn health() -> impl Responder {
  HttpResponse::Ok()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  // let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

  HttpServer::new(move || {
    App::new()
        .data(Schema::new(Query, EmptyMutation, EmptySubscription))
        .service(web::resource("/").guard(guard::Post()).to(index))
        .service(web::resource("/").guard(guard::Get()).to(health))
    })
    .bind("0.0.0.0:4001")?
    .run()
    .await

}
