use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use crate::{AppState, ActorListEntry};
use super::models::{AddActorEntry, UpdateActorEntry, ActorId};

#[get("/api/getActors")]
async fn get_actors(data: web::Data<AppState>) -> impl Responder {
  let actorlist_entries = data.actorlist_entries.lock();

  // Return success on successful unwrap, internal server error on error
  match actorlist_entries {
    Result::Ok(actorlist) => HttpResponse::Ok().json(actorlist.to_vec()),
    Result::Err(_) => HttpResponse::InternalServerError().json("")
  }
}

#[get("/api/getActor")]
async fn get_actor(data: web::Data<AppState>, req_body: web::Json<ActorId>) -> impl Responder {
  let actorlist_entries = data.actorlist_entries.lock();
  let id = req_body.id;

  // Return success if actor with the specified id is found, if they cannot be
  // found, return a 404. On error, return internal server error.
  match actorlist_entries {
    Result::Ok(actorlist) => {
      for i in 0..actorlist.len() {
        if actorlist[i].id == id {
          return HttpResponse::Ok().json(actorlist[i].clone());
        }
      }

      HttpResponse::NotFound().json("Unable to find actor")
    },
    Result::Err(_) => HttpResponse::InternalServerError().json("")
  }
}

#[post("/api/addActor")]
async fn add_actor(data: web::Data<AppState>, entry: web::Json<AddActorEntry>) -> impl Responder {
  let actorlist_entries = data.actorlist_entries.lock();
  
  // Return success if the actor has been added, internal server error on error
  match actorlist_entries {
    Result::Ok(mut actorlist) => {
      // Get the current maximum id to determine what the next actor id should be
      let mut max_id: i32 = 0;
      for i in 0..actorlist.len() {
        if actorlist[i].id > max_id { 
          max_id = actorlist[i].id;
        }
      }

      actorlist.push(ActorListEntry {
        id: max_id + 1,
        first_name: entry.first_name.clone(),
        last_name: entry.last_name.clone(),
        age: entry.age,
        dob: entry.dob.clone(),
        movies: entry.movies.clone(),
      });

      HttpResponse::Ok().json(actorlist.to_vec())
    },
    Result::Err(_) => HttpResponse::InternalServerError().json("")
  }
}

#[put("/api/updateActor")]
async fn update_actor(data: web::Data<AppState>, req_body: web::Json<UpdateActorEntry>) -> impl Responder {
  let actorlist_entries = data.actorlist_entries.lock();
  let id = req_body.id;

  // Return success on successful update, if the actor with the given id doesn't exist,
  // return a 404. Return internal server error on error.
  match actorlist_entries {
    Result::Ok(mut actorlist) => {
      
      for i in 0..actorlist.len() {
        if actorlist[i].id == id {

          if req_body.first_name.is_some() {
            actorlist[i].first_name = req_body.first_name.clone().unwrap();
          }

          if req_body.last_name.is_some() {
            actorlist[i].last_name = req_body.last_name.clone().unwrap();
          }

          if req_body.age.is_some() {
            actorlist[i].age = req_body.age.clone().unwrap();
          }

          if req_body.dob.is_some() {
            actorlist[i].dob = req_body.dob.clone().unwrap();
          }

          if req_body.movies.is_some() {
            actorlist[i].movies = req_body.movies.clone().unwrap();
          }

          return HttpResponse::Ok().json(actorlist.to_vec());
        }
      }

      HttpResponse::NotFound().json("")

    },
    Result::Err(_) => HttpResponse::InternalServerError().json("")
  }
} 

#[delete("/api/deleteActor")]
async fn delete_actor(data: web::Data<AppState>, req_body: web::Json<ActorId>) -> impl Responder {
  let actorlist_entries = data.actorlist_entries.lock();
  let id = req_body.id;

  // Return success upon successful delettion of actor with the given id, if the actor with
  // the given id doesn't exist, return 404. Return internal server error on error.
  match actorlist_entries {
    Result::Ok(mut actorlist) => {
      let old_length = actorlist.len();
      *actorlist = actorlist.to_vec().into_iter().filter(|x| x.id != id).collect();

      if old_length == (*actorlist).len() {
        return HttpResponse::NotFound().json("");
      }

      HttpResponse::Ok().json(actorlist.to_vec())
    },
    Result::Err(_) => HttpResponse::InternalServerError().json("")
  }
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_actors)
    .service(get_actor)
    .service(add_actor)
    .service(update_actor)
    .service(delete_actor);
}