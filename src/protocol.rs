use robotics_lib::event::events::Event;
use robotics_lib::utils::LibError;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::tile::{Content, Tile};
use serde::{Deserialize, Serialize};
use crate::protocol::LibEvent::{AddedToBackpack, RemovedFromBackpack};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum LibEvent {
    /// Robot has been initialized and its lifecycle has started
    Ready,
    /// Robot has ended its lifecycle
    Terminated,
    /// [Event] fired when time of the day changes, contains the new [EnvironmentalConditions]
    TimeChanged(EnvironmentalConditions),

    /// [Event] fired when the day changes, contains the new [EnvironmentalConditions]
    DayChanged(EnvironmentalConditions),

    /// [Event] fired when energy gets recharged, contains the recharge amount
    EnergyRecharged(usize),

    /// [Event] fired when energy is consumed, contains the consumed amount
    EnergyConsumed(usize),

    /// [Event] fired when the robot moves to new coordinates
    ///
    /// This [Event] contains the [Tile] to which the robot moved and the coordinates
    Moved(Tile, (usize, usize)),

    /// [Event] fired when a tile content gets updated.
    ///
    /// This [Event] contains the [Tile] of the updated content and the coordinates
    TileContentUpdated(Tile, (usize, usize)),

    /// [Event] fired when a [Content] is added to the backpack, also contains the amount of content added
    AddedToBackpack(Content, usize),

    /// [Event] fired when a [Content] is removed from the backpack, also contains the amount of content removed
    RemovedFromBackpack(Content, usize),

    ///Sends the tiles that have been discovered
    DiscoveredTiles(Vec<(Tile,(usize,usize))>),

    ToolUsed
}

impl From<Event> for LibEvent {
    fn from(value: Event) -> Self {
        match value{
            Event::Ready => {Self::Ready}
            Event::Terminated => {Self::Terminated}
            Event::TimeChanged(e) => {Self::TimeChanged(e)}
            Event::DayChanged(e) => {Self::TimeChanged(e)}
            Event::EnergyRecharged(e) => {Self::EnergyRecharged(e)}
            Event::EnergyConsumed(e) => {Self::EnergyConsumed(e)}
            Event::Moved(a, b) => {Self::Moved(a,b)}
            Event::TileContentUpdated(a, b) => {Self::TileContentUpdated(a,b)}
            Event::AddedToBackpack(a, b) => {AddedToBackpack(a,b)}
            Event::RemovedFromBackpack(a, b) => {RemovedFromBackpack(a,b)}
        }
    }
}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum EventError{
    NotEnoughEnergy,
    OutOfBounds,
    NoContent,
    NotEnoughSpace(usize),
    CannotDestroy,
    CannotWalk,
    WrongContentUsed,
    NotEnoughContentProvided,
    OperationNotAllowed,
    NotCraftable,
    NoMoreDiscovery,
    EmptyForecast,
    WrongHour,
    NotEnoughContentInBackPack,
    WorldIsNotASquare,
    TeleportIsTrueOnGeneration,
    ContentValueIsHigherThanMax,
    ContentNotAllowedOnTile,
    MustDestroyContentFirst,
}
impl From<LibError> for EventError{
    fn from(value: LibError) -> Self {
        match value{
            LibError::NotEnoughEnergy => {EventError::NotEnoughEnergy}
            LibError::OutOfBounds => {EventError::OutOfBounds}
            LibError::NoContent => {EventError::NoContent}
            LibError::NotEnoughSpace(x) => {EventError::NotEnoughSpace(x)}
            LibError::CannotDestroy => {EventError::CannotDestroy}
            LibError::CannotWalk => {EventError::CannotWalk}
            LibError::WrongContentUsed => {EventError::WrongContentUsed}
            LibError::NotEnoughContentProvided => {EventError::NotEnoughContentProvided}
            LibError::OperationNotAllowed => {EventError::OperationNotAllowed}
            LibError::NotCraftable => {EventError::NotCraftable}
            LibError::NoMoreDiscovery => {EventError::NoMoreDiscovery}
            LibError::EmptyForecast => {EventError::EmptyForecast}
            LibError::WrongHour => {EventError::WrongHour}
            LibError::NotEnoughContentInBackPack => {EventError::NotEnoughContentInBackPack}
            LibError::WorldIsNotASquare => {EventError::WorldIsNotASquare}
            LibError::TeleportIsTrueOnGeneration => {EventError::TeleportIsTrueOnGeneration}
            LibError::ContentValueIsHigherThanMax => {EventError::ContentValueIsHigherThanMax}
            LibError::ContentNotAllowedOnTile => {EventError::ContentNotAllowedOnTile}
            LibError::MustDestroyContentFirst => {EventError::MustDestroyContentFirst}
        }
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum Message{
    LibEvent(LibEvent),
    LibError(EventError)
}
