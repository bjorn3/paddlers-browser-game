#![feature(result_map_or_else)]

mod db;
mod resource_system;
mod api;
mod game_master;
mod buildings;
mod worker_actions;
mod town_view;

use db::*;
use game_master::GameMaster;
use game_master::town_worker::TownWorker;

use actix::prelude::*;
use actix_web::{
    http::header, 
    web, App, HttpServer
};
use actix_cors::Cors;
use paddlers_shared_lib::api::{
    shop::{BuildingPurchase, BuildingDeletion},
    tasks::TaskList,
};

type StringErr = Result<(),String>;

struct ActorAddresses {
    game_master: Addr<GameMaster>,
    town_worker: Addr<TownWorker>,
}

fn main() {

    let dbpool = DB::new_pool();

    let sys = actix::System::new("background-worker-example");
    let gm_actor = GameMaster::new(dbpool.clone()).start();
    let town_worker_actor = TownWorker::new(dbpool.clone()).start();
    // let town_worker_actor = SyncArbiter::start(1, move || TownWorker::new(dbpool.clone()));

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://127.0.0.1:8000")
                    .allowed_methods(vec!["POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600*24),
            )
            .data(ActorAddresses { game_master: gm_actor.clone(), town_worker: town_worker_actor.clone() })
            .data(dbpool.clone())
            .route("/", web::get().to(api::index))
            .service(
                web::resource("/shop/building")
                .data(web::Json::<BuildingPurchase>)
                .route(web::post().to(api::purchase_building))
            )
            .service(
                web::resource("/shop/building/delete")
                .data(web::Json::<BuildingDeletion>)
                .route(web::post().to(api::delete_building))
            )
            .service(
                web::resource("/worker/overwriteTasks")
                .data(web::Json::<TaskList>)
                .route(web::post().to(api::overwrite_tasks))
            )
    })
    .disable_signals()
    .bind("127.0.0.1:8088")
    .unwrap()
    .start();

    sys.run().unwrap();
    println!("Web-Actix returned");
}