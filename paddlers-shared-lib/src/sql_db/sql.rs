use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;

pub trait GameDB {
    fn dbconn(&self) -> &PgConnection;

    fn unit(&self, unit_id: i64) -> Option<Unit> {
        let results = units::table
            .filter(units::id.eq(unit_id))
            .get_result::<Unit>(self.dbconn())
            .optional()
            .expect("Error loading data");
        results
    }
    fn units(&self, village_id: i64) -> Vec<Unit> {
        let results = units::table
            .filter(units::home.eq(village_id))
            .limit(500)
            .load::<Unit>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn attacks(&self, min_id: Option<i64>) -> Vec<Attack> {
        let results = attacks::table
            .filter(attacks::id.ge(min_id.unwrap_or(0)))
            .limit(500)
            .load::<Attack>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn attack_units(&self, atk: &Attack) -> Vec<Unit> {
        let results = attacks_to_units::table
        .inner_join(units::table)
        .filter(attacks_to_units::attack_id.eq(atk.id))
        .select(units::all_columns) 
        .limit(500)
        .load::<Unit>(self.dbconn())
        .expect("Error loading data");
        results
    }
    fn buildings(&self) -> Vec<Building> {
        let results = buildings::table
            .limit(500)
            .load::<Building>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn find_building_by_coordinates(&self, x: i32, y: i32) -> Option<Building> {
        let result = buildings::table
            .filter(buildings::x.eq(x).and(buildings::y.eq(y)))
            .first::<Building>(self.dbconn())
            .optional()
            .expect("Error loading data");
        result
    }
    fn resource(&self, r: ResourceType) -> i64 {
        resources::table
        .find(r)
        .first(self.dbconn())
        .map(|res: Resource| res.amount)
        .unwrap_or(0)
    }
    fn unit_tasks(&self, unit_id: i64) -> Vec<Task> {
        let results = tasks::table
        .filter(tasks::unit_id.eq(unit_id))
        .limit(500)
        .load::<Task>(self.dbconn())
        .expect("Error loading data");
        results
    }
    fn past_unit_tasks(&self, unit_id: i64) -> Vec<Task> {
        let results = tasks::table
            .filter(tasks::unit_id.eq(unit_id))
            .order(tasks::start_time.asc())
            .limit(500)
            .load::<Task>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn earliest_future_task(&self, unit_id: i64) -> Option<Task> {
        tasks::table
            .filter(tasks::unit_id.eq(unit_id))
            .filter(tasks::start_time.ge(diesel::dsl::now))
            .order(tasks::start_time.asc())
            .first(self.dbconn())
            .optional()
            .expect("Error loading data")
    }
    fn task(&self, task_id: i64) -> Option<Task> {
        tasks::table
            .find(task_id)
            .first(self.dbconn())
            .optional()
            .expect("Error loading task")
    }
}