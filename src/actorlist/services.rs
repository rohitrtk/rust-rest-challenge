use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use crate::{AppState, ActorListEntry};
use super::models::{AddActorEntry, UpdateActorEntry, ActorId};

#[get("/api/getActors")]
async fn get_actors(data: web::Data<AppState>) -> impl Responder {
  HttpResponse::Ok().json(data.actorlist_entries.lock().unwrap().to_vec())
}

#[get("/api/getActor")]
async fn get_actor(data: web::Data<AppState>, req_body: web::Json<ActorId>) -> impl Responder {
  let actorlist_entries = data.actorlist_entries.lock().unwrap();

  let id = req_body.id;

  for i in 0..actorlist_entries.len() {
    if actorlist_entries[i].id == id {
      return HttpResponse::Ok().json(actorlist_entries[i].clone());
    }
  }

  HttpResponse::NotFound().json("")
}

#[post("/api/addActor")]
async fn add_actor(data: web::Data<AppState>, entry: web::Json<AddActorEntry>) -> impl Responder {
  // Get list of actors
  let mut actorlist_entries = data.actorlist_entries.lock().unwrap();

  // Get the current maximum id to determine what the next actor id should be
  let mut max_id: i32 = 0;
  for i in 0..actorlist_entries.len() {
    if actorlist_entries[i].id > max_id {
      max_id = actorlist_entries[i].id;
    }
  }

  actorlist_entries.push(ActorListEntry {
    id: max_id + 1,
    first_name: entry.first_name.clone(),
    last_name: entry.last_name.clone(),
    age: entry.age,
    dob: entry.dob.clone(),
    movies: entry.movies.clone(),
  });

  HttpResponse::Ok().json(actorlist_entries.to_vec())
}

#[put("/api/updateActor")]
async fn update_actor(data: web::Data<AppState>, req_body: web::Json<UpdateActorEntry>) -> impl Responder {
  let id = req_body.id;
  let mut actorlist_entries = data.actorlist_entries.lock().unwrap();

  for i in 0..actorlist_entries.len() {

    let actorlist = &mut actorlist_entries[i];

    if actorlist.id == id {
      if req_body.first_name.is_some() {
        actorlist.first_name = req_body.first_name.clone().unwrap();
        println!("{}", req_body.first_name.clone().unwrap());
      }
      
      if req_body.last_name.is_some() {
        actorlist.last_name = req_body.last_name.clone().unwrap();
      }
    
      if req_body.age.is_some() {
        actorlist.age = req_body.age.unwrap();
      }
    
      if req_body.dob.is_some() {
        actorlist.dob = req_body.dob.clone().unwrap();
      }
    
      if req_body.movies.is_some() {
        actorlist.movies = req_body.movies.clone().unwrap();
      }
    }

    return HttpResponse::Ok().json(actorlist);
  }

  HttpResponse::NotFound().json("")
} 

#[delete("/api/deleteActor")]
async fn delete_actor(data: web::Data<AppState>, req_body: web::Json<ActorId>) -> impl Responder {
  let mut actorlist_entries = data.actorlist_entries.lock().unwrap();

  let id = req_body.id;
  *actorlist_entries = actorlist_entries.to_vec().into_iter().filter(|x| x.id != id).collect();

  HttpResponse::Ok().json(actorlist_entries.to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .service(get_actors)
    .service(get_actor)
    .service(add_actor)
    .service(update_actor)
    .service(delete_actor);
}