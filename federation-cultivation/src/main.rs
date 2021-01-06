use actix_web::{guard, web, App, HttpResponse, HttpServer, Responder};
use async_graphql::{
  Context, EmptyMutation, EmptySubscription, ErrorExtensions, FieldError, Object, Schema,
  SimpleObject, ID,
};
use async_graphql_actix_web::{Request, Response};
use std::convert::Infallible;

struct Leads {
  id: ID,
}

#[Object(extends)]
impl Leads {
  #[graphql(external)]
  async fn id(&self) -> &ID {
    &self.id
  }

  async fn ponds<'a>(&self, ctx: &'a Context<'_>) -> Vec<&'a Pond> {
    let ponds = ctx.data_unchecked::<Vec<Pond>>();
    ponds
      .iter()
      .filter(|pond| pond.leads.id == self.id)
      .collect()
  }
}

#[derive(SimpleObject)]
struct Pond {
  id: ID,
  point_name: String,
  leads: Leads
}

struct Query;

#[Object(extends)]
impl Query {
  async fn ponds(&self) -> Pond {
    Pond {
      id: "1234".into(),
      point_name: "Palas".to_string(),
      leads: Leads { id: "1234".into() },
    }
  }
  #[graphql(entity)]
  async fn find_leads_by_id(&self, id: ID) -> Leads {
    Leads { id }
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
  HttpServer::new(move || {
    App::new()
      .data(Schema::new(Query, EmptyMutation, EmptySubscription))
      .service(web::resource("/").guard(guard::Post()).to(index))
      .service(web::resource("/").guard(guard::Get()).to(health))
  })
  .bind("0.0.0.0:4002")?
  .run()
  .await
}
