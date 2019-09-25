use crate::models::*;
use chrono::Duration;

impl AbilityType {
    /// Returns affected attribute and strength of ability
    pub fn apply(&self) -> (HoboAttributeType, i32) {
        match self {
            AbilityType::Work => panic!("Cannot apply work to hobo"),
            AbilityType::Welcome =>  {
                (HoboAttributeType::Health, 1)
            }
        }
    }

    /// How long it takes a worker to perform the ability
    pub fn busy_duration(&self) -> Duration {
        let ms = 
        match self {
            AbilityType::Welcome => 1000,
            AbilityType::Work => 0,
        };
        Duration::milliseconds(ms)
    }
}
