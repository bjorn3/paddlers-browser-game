use crate::game::{
    components::*, town::new_temple_menu, units::worker_factory::create_worker_entities,
    units::workers::Worker,
};
use crate::init::loading::LoadingState;
use crate::init::quicksilver_integration::{GameState, Signal};
use crate::net::NetMsg;
use crate::prelude::*;
use std::convert::TryInto;
use std::sync::mpsc::TryRecvError;

use super::*;
use specs::prelude::*;

impl LoadingState {
    pub fn update_net(&mut self) -> PadlResult<()> {
        match self.base.net_chan.try_recv() {
            Ok(msg) => match msg {
                NetMsg::Error(e) => match e.err {
                    PadlErrorCode::UserNotInDB => {
                        self.base.rest.http_create_player()?;
                    }
                    _ => {
                        println!("Network Error: {}", e);
                    }
                },
                NetMsg::Player(player_info) => {
                    self.player_info = Some(player_info);
                }
                other => {
                    println!(
                        "Unexpected network message before complete initialization {:?}", other,
                    );
                }
            },
            Err(TryRecvError::Disconnected) => {
                return PadlErrorCode::NoNetwork.usr();
            }
            Err(TryRecvError::Empty) => {}
        }
        Ok(())
    }
}

impl GameState {
    pub fn update_net(&mut self) -> PadlResult<()> {
        match self.game.net.try_recv() {
            Ok(msg) => {
                // println!("Received Network data!");
                match msg {
                    NetMsg::Error(e) => {
                        println!("Network Error: {}", e);
                    }
                    NetMsg::Attacks(response) => {
                        if let Some(data) = response.data {
                            for atk in data.village.attacks {
                                atk.create_entities(&mut self.game.world)?;
                            }
                        } else {
                            println!("No data returned");
                        }
                    }
                    NetMsg::Buildings(response) => {
                        if let Some(data) = response.data {
                            self.game.flush_buildings()?;
                            self.game.world.maintain();
                            data.create_entities(&mut self.game);
                        } else {
                            println!("No buildings available");
                        }
                    }
                    NetMsg::Hobos(hobos) => {
                        self.game.flush_home_hobos()?;
                        self.game.insert_hobos(hobos)?;
                    }
                    NetMsg::Leaderboard(offset, list) => {
                        self.viewer.global_event(
                            &mut self.game,
                            &PadlEvent::Network(NetMsg::Leaderboard(offset, list)),
                        )?;
                    }
                    NetMsg::Map(response, min, max) => {
                        if let Some(data) = response.data {
                            let streams = data
                                .map
                                .streams
                                .iter()
                                .map(|s| {
                                    s.control_points
                                        .chunks(2)
                                        .map(|slice| (slice[0] as f32, slice[1] as f32))
                                        .collect()
                                })
                                .collect();
                            let villages = data
                                .map
                                .villages
                                .into_iter()
                                .map(VillageMetaInfo::from)
                                .collect();
                            let (map, world) = (self.game.map.as_mut(), &mut self.game.world);
                            map.map(|map| map.add_segment(world, streams, villages, min, max));
                        } else {
                            println!("No map data available");
                        }
                    }
                    NetMsg::Player(player_info) => {
                        if let Some(temple) = self.game.town().temple {
                            let mut menus = self.game.world.write_storage::<UiMenu>();
                            // This insert overwrites existing entries
                            menus
                                .insert(temple, new_temple_menu(&player_info))
                                .map_err(|_| {
                                    PadlError::dev_err(PadlErrorCode::SpecsError(
                                        "Temple menu insertion failed",
                                    ))
                                })?;
                        }
                        *self.game.world.write_resource() = DefaultShop::new(player_info.karma());
                        *self.game.world.write_resource() = player_info;
                    }
                    NetMsg::VillageInfo(response) => {
                        if let Some(data) = response.data {
                            self.game.town_mut().faith =
                                data.village.faith.try_into().map_err(|_| {
                                    PadlError::dev_err(PadlErrorCode::InvalidGraphQLData(
                                        "Faith does not fit u8",
                                    ))
                                })?;
                            self.game.resources.update(data);
                            self.viewer.event(
                                &mut self.game,
                                &PadlEvent::Signal(Signal::ResourcesUpdated),
                            )?;
                        } else {
                            println!("No resources available");
                        }
                    }
                    NetMsg::Workers(response) => {
                        self.game.flush_workers()?;
                        self.game.world.maintain();
                        let now = self.game.world.read_resource::<Now>().0;
                        let results = create_worker_entities(&response, &mut self.game.world, now);
                        let mut q = self.game.world.write_resource::<ErrorQueue>();
                        for res in results.into_iter() {
                            if let Err(e) = res {
                                q.push(e);
                            }
                        }
                    }
                    NetMsg::UpdateWorkerTasks(unit) => {
                        let entity = self
                            .game
                            .worker_entity_by_net_id(unit.id.parse().unwrap())?;
                        let workers = &mut self.game.world.write_storage::<Worker>();
                        let worker = workers.get_mut(entity).unwrap();
                        worker.tasks.clear();

                        let net = self.game.world.read_storage::<NetObj>();
                        let ent = self.game.world.entities();
                        for task in unit.tasks {
                            match task.create(&net, &ent) {
                                Ok(task) => worker.tasks.push_back(task),
                                Err(e) => {
                                    match e.err {
                                        PadlErrorCode::UnknownNetObj(_) => {
                                            // NOP: Best to ignore and move on with other tasks
                                        }
                                        _ => {
                                            return Err(e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(TryRecvError::Disconnected) => {
                return PadlErrorCode::NoNetwork.usr();
            }
            Err(TryRecvError::Empty) => {}
        }
        Ok(())
    }
}
