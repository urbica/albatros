use iron::status;
use iron::prelude::*;
use iron_router::Router;
use bodyparser;
use serde_json;
use diesel;
use diesel::prelude::*;

use super::super::db;
use super::{Project, NewProject};
use super::super::schema::projects;
use super::super::schema::projects::dsl::*;

pub fn index(req: &mut Request) -> IronResult<Response> {
    let connection = match db::get_connection(req) {
        Ok(conn) => conn,
        Err(error) => {
            eprintln!("Couldn't get a connection to postgres: {}", error);
            return Ok(Response::with((status::InternalServerError)));
        }
    };

    let ref conn = *connection;
    let results = match projects.load::<Project>(conn) {
        Ok(results) => results,
        Err(error) => {
            eprintln!("Couldn't get projects: {}", error);
            return Ok(Response::with((status::InternalServerError)));
        }
    };

    let result = serde_json::to_string(&results).unwrap_or("[]".to_owned());
    Ok(Response::with((status::Ok, result)))
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    let new_project = match req.get::<bodyparser::Struct<NewProject>>() {
        Ok(Some(new_project)) => new_project,
        Ok(None) => {
            eprintln!("Couldn't parse project");
            return Ok(Response::with((status::BadRequest)));
        },
        Err(error) => {
            eprintln!("Couldn't parse project: {}", error);
            return Ok(Response::with((status::BadRequest)));
        }
    };

    let connection = match db::get_connection(req) {
        Ok(conn) => conn,
        Err(error) => {
            eprintln!("Couldn't get a connection to postgres: {}", error);
            return Ok(Response::with((status::InternalServerError)));
        }
    };

    let ref conn = *connection;
    let results = match diesel::insert_into(projects::table)
        .values(&new_project)
        .get_result::<Project>(conn) {
            Ok(results) => results,
            Err(error) => {
                eprintln!("Couldn't create project: {}", error);
                return Ok(Response::with((status::InternalServerError)));
            }
        };

    let result = serde_json::to_string(&results).unwrap_or("{}".to_owned());
    Ok(Response::with((status::Ok, result)))
}

pub fn get(req: &mut Request) -> IronResult<Response> {
    let connection = match db::get_connection(req) {
        Ok(conn) => conn,
        Err(error) => {
            eprintln!("Couldn't get a connection to postgres: {}", error);
            return Ok(Response::with((status::InternalServerError)));
        }
    };

    let ref conn = *connection;

    let project_id = match req
        .extensions.get::<Router>().unwrap()
        .find("id").unwrap()
        .to_string().parse::<i32>() {
            Ok(project_id) => project_id,
            Err(error) => {
                eprintln!("Couldn't parse project id: {}", error);
                return Ok(Response::with((status::NotFound)));
            }
        };

    let results = match projects.filter(id.eq(project_id)).load::<Project>(conn) {
        Ok(results) => results,
        Err(error) => {
            eprintln!("Couldn't get project: {}", error);
            return Ok(Response::with((status::InternalServerError)));
        }
    };

    let result = serde_json::to_string(&results).unwrap_or("{}".to_owned());
    Ok(Response::with((status::Ok, result)))
}