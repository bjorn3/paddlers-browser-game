#![feature(result_map_or_else)]
extern crate env_logger;

mod db;
mod resource_system;
mod api;
mod game_master;
mod buildings;
mod worker_actions;
mod town_view;

use db::*;
use game_master::{
    GameMaster,
    town_worker::TownWorker,
    economy_worker::EconomyWorker,
};

use actix::prelude::*;
use actix_web::{
    http::header, 
    web, App, HttpServer
};
use actix_cors::Cors;
use paddlers_shared_lib::{
    api::{
        shop::{BuildingPurchase, BuildingDeletion},
        tasks::TaskList,
    },
    config::{
        Config,
    },
    sql_db::{
        initiliaze_db_if_env_set,
        sql::GameDB,
    }
};

type StringErr = Result<(),String>;

struct ActorAddresses {
    _game_master: Addr<GameMaster>,
    town_worker: Addr<TownWorker>,
    _econ_worker: Addr<EconomyWorker>,
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let dbpool: Pool = DB::new_pool();
    let conn: DB = (&dbpool.clone()).into();
    initiliaze_db_if_env_set(conn.dbconn()).expect("DB initialization failed.");
    println!("DB successfully migrated");

    let config = Config::from_env()
        .unwrap_or(Config::default());
        // XXX
    // let origin = "http://".to_owned() + &config.frontend_base_url;

    let sys = actix::System::new("background-worker-example");
    let gm_actor = GameMaster::new(dbpool.clone()).start();
    let town_worker_actor = TownWorker::new(dbpool.clone()).start();
    let econ_worker = EconomyWorker::new(dbpool.clone()).start();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    // .allowed_origin(&origin)
                    .send_wildcard()
                    .allowed_methods(vec!["POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600*24),
            )
            .wrap(actix_web::middleware::Logger::default())
            .data(
                ActorAddresses {
                    _game_master: gm_actor.clone(),
                    town_worker: town_worker_actor.clone(),
                    _econ_worker: econ_worker.clone(),
                })
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
    .bind(&config.game_master_base_url).expect("binding")
    .start();

    sys.run().unwrap();
    println!("Web-Actix returned");
}