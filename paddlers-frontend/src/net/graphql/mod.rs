pub mod query_types;
mod http_calls;
pub use query_types::*;
use http_calls::*;
use super::NetMsg;

use futures::Future;
use futures_util::future::FutureExt;
use std::sync::atomic::{AtomicI64, Ordering};
use crate::prelude::*;
use crate::game::state::current_village;

pub struct GraphQlState {
    next_attack_id: AtomicI64,
}

impl GraphQlState {

    pub (super) const fn new() -> GraphQlState {
        GraphQlState {
            next_attack_id: AtomicI64::new(0),
        }
    }

    pub (super) fn attacks_query(&'static self) -> PadlResult<impl Future<Output = PadlResult<NetMsg>>> {
        let fp = http_read_incoming_attacks(Some(self.next_attack_id.load(Ordering::Relaxed)), current_village().num())?;
        Ok(
            fp.map(
                move |response| {
                    let r = response?;
                    if let Some(data) = &r.data {
                        let max_id = data.village.attacks.iter()
                            .map(|atk| atk.id.parse().unwrap())
                            .fold(0, i64::max);
                        let next = self.next_attack_id.load(Ordering::Relaxed).max(max_id + 1);
                        self.next_attack_id.store(next, Ordering::Relaxed);
                    }
                    Ok(NetMsg::Attacks(r))
                }
            )
        )
    }
    pub (super) fn resource_query(&self) -> PadlResult<impl Future<Output = PadlResult<NetMsg>>> {
        let fp = http_read_resources(current_village().num())?;
        Ok(
            fp.map(
                |response|
                Ok(NetMsg::Resources(response?))
            )
        )
    }
    pub (super) fn buildings_query(&self) -> PadlResult<impl Future<Output = PadlResult<NetMsg>>> {
        let fp = http_read_buildings(current_village().num())?;
        Ok(
            fp.map(
                |response| Ok(NetMsg::Buildings(response?)),
            )
        )
    }
    pub (super) fn workers_query(&self) -> PadlResult<impl Future<Output = PadlResult<NetMsg>>> {
        let fp = http_read_workers(current_village().num())?; 
        Ok(
            fp.map(
                |response| {
                    if let Some(data) = response?.data {
                        let workers = data.village.workers;
                        Ok(NetMsg::Workers(workers))
                    }
                    else {
                        gql_empty_error("workers")
                    }
                }
            )
        )
    }
    pub (super) fn worker_tasks_query(&self, unit_id: i64) -> PadlResult<impl Future<Output = PadlResult<NetMsg>>> {
        let fp = http_read_worker_tasks(unit_id)?;
        Ok(
            fp.map(
                |response| {
                    if let Some(data) = response?.data {
                        Ok(NetMsg::UpdateWorkerTasks(data.worker))
                    }
                    else {
                        gql_empty_error("worker_tasks")
                    }
                }
            )
        )
    }
    pub (super) fn map_query(&self, min: i32, max: i32) -> PadlResult<impl Future<Output = PadlResult<NetMsg>>> {
        let fp = http_read_map(min as i64, max as i64)?;
        Ok(
            fp.map(
                move |response| Ok(NetMsg::Map(response?, min, max)),
            )
        )
    }
}

fn gql_empty_error<R>(data_set: &'static str) -> PadlResult<R> {
    PadlErrorCode::EmptyGraphQLData(data_set).dev()
}