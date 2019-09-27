#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvailableAbility {
    #[prost(int32, optional, tag = "1")]
    pub ability_id: ::std::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub requires_point: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageData {
    /// Number of bits per pixel; 8 bits for a byte etc.
    #[prost(int32, optional, tag = "1")]
    pub bits_per_pixel: ::std::option::Option<i32>,
    /// Dimension in pixels.
    #[prost(message, optional, tag = "2")]
    pub size: ::std::option::Option<Size2Di>,
    /// Binary data; the size of this buffer in bytes is width * height * bits_per_pixel / 8.
    #[prost(bytes, optional, tag = "3")]
    pub data: ::std::option::Option<std::vec::Vec<u8>>,
}
/// Point on the screen/minimap (e.g., 0..64).
/// Note: bottom left of the screen is 0, 0.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointI {
    #[prost(int32, optional, tag = "1")]
    pub x: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub y: ::std::option::Option<i32>,
}
/// Screen space rectangular area.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RectangleI {
    #[prost(message, optional, tag = "1")]
    pub p0: ::std::option::Option<PointI>,
    #[prost(message, optional, tag = "2")]
    pub p1: ::std::option::Option<PointI>,
}
/// Point on the game board, 0..255.
/// Note: bottom left of the screen is 0, 0.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point2D {
    #[prost(float, optional, tag = "1")]
    pub x: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub y: ::std::option::Option<f32>,
}
/// Point on the game board, 0..255.
/// Note: bottom left of the screen is 0, 0.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    #[prost(float, optional, tag = "1")]
    pub x: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub y: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub z: ::std::option::Option<f32>,
}
/// Screen dimensions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Size2Di {
    #[prost(int32, optional, tag = "1")]
    pub x: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub y: ::std::option::Option<i32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Race {
    NoRace = 0,
    Terran = 1,
    Zerg = 2,
    Protoss = 3,
    Random = 4,
}
/// May not relevant: queueable (everything is queueable).
/// May not be important: AbilSetId - marine stim, marauder stim.
/// Stuff omitted: transient.
/// Stuff that may be important: cost, range, Alignment, targetfilters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbilityData {
    /// Stable ID.
    #[prost(uint32, optional, tag = "1")]
    pub ability_id: ::std::option::Option<u32>,
    /// Catalog name of the ability.
    #[prost(string, optional, tag = "2")]
    pub link_name: ::std::option::Option<std::string::String>,
    /// Catalog index of the ability.
    #[prost(uint32, optional, tag = "3")]
    pub link_index: ::std::option::Option<u32>,
    /// Name used for the command card. May not always be set.
    #[prost(string, optional, tag = "4")]
    pub button_name: ::std::option::Option<std::string::String>,
    /// A human friendly name when the button name or link name isn't descriptive.
    #[prost(string, optional, tag = "5")]
    pub friendly_name: ::std::option::Option<std::string::String>,
    /// Hotkey. May not always be set.
    #[prost(string, optional, tag = "6")]
    pub hotkey: ::std::option::Option<std::string::String>,
    /// This ability id may be represented by the given more generic id.
    #[prost(uint32, optional, tag = "7")]
    pub remaps_to_ability_id: ::std::option::Option<u32>,
    /// If true, the ability may be used by this set of mods/map.
    #[prost(bool, optional, tag = "8")]
    pub available: ::std::option::Option<bool>,
    /// Determines if a point is optional or required.
    #[prost(enumeration = "ability_data::Target", optional, tag = "9")]
    pub target: ::std::option::Option<i32>,
    /// Can be cast in the minimap.
    #[prost(bool, optional, tag = "10")]
    pub allow_minimap: ::std::option::Option<bool>,
    /// Autocast can be set.
    #[prost(bool, optional, tag = "11")]
    pub allow_autocast: ::std::option::Option<bool>,
    /// Requires placement to construct a building.
    #[prost(bool, optional, tag = "12")]
    pub is_building: ::std::option::Option<bool>,
    /// Estimation of the footprint size. Need a better footprint.
    #[prost(float, optional, tag = "13")]
    pub footprint_radius: ::std::option::Option<f32>,
    /// Placement next to an existing structure, e.g., an add-on like a Tech Lab.
    #[prost(bool, optional, tag = "14")]
    pub is_instant_placement: ::std::option::Option<bool>,
    /// Range unit can cast ability without needing to approach target.
    #[prost(float, optional, tag = "15")]
    pub cast_range: ::std::option::Option<f32>,
}
pub mod ability_data {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Target {
        /// Does not require a target.
        None = 1,
        /// Requires a target position.
        Point = 2,
        /// Requires a unit to target. Given by position using feature layers.
        Unit = 3,
        /// Requires either a target point or target unit.
        PointOrUnit = 4,
        /// Requires either a target point or no target. (eg. building add-ons)
        PointOrNone = 5,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DamageBonus {
    #[prost(enumeration = "Attribute", optional, tag = "1")]
    pub attribute: ::std::option::Option<i32>,
    #[prost(float, optional, tag = "2")]
    pub bonus: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Weapon {
    #[prost(enumeration = "weapon::TargetType", optional, tag = "1")]
    pub r#type: ::std::option::Option<i32>,
    #[prost(float, optional, tag = "2")]
    pub damage: ::std::option::Option<f32>,
    #[prost(message, repeated, tag = "3")]
    pub damage_bonus: ::std::vec::Vec<DamageBonus>,
    /// Number of hits per attack. (eg. Colossus has 2 beams)
    #[prost(uint32, optional, tag = "4")]
    pub attacks: ::std::option::Option<u32>,
    #[prost(float, optional, tag = "5")]
    pub range: ::std::option::Option<f32>,
    /// Time between attacks.
    #[prost(float, optional, tag = "6")]
    pub speed: ::std::option::Option<f32>,
}
pub mod weapon {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TargetType {
        Ground = 1,
        Air = 2,
        Any = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnitTypeData {
    /// Stable ID.
    #[prost(uint32, optional, tag = "1")]
    pub unit_id: ::std::option::Option<u32>,
    /// Catalog name of the unit.
    #[prost(string, optional, tag = "2")]
    pub name: ::std::option::Option<std::string::String>,
    /// If true, the ability may be used by this set of mods/map.
    #[prost(bool, optional, tag = "3")]
    pub available: ::std::option::Option<bool>,
    /// Number of cargo slots it occupies in transports.
    #[prost(uint32, optional, tag = "4")]
    pub cargo_size: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "12")]
    pub mineral_cost: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "13")]
    pub vespene_cost: ::std::option::Option<u32>,
    #[prost(float, optional, tag = "14")]
    pub food_required: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "18")]
    pub food_provided: ::std::option::Option<f32>,
    /// The ability that builds this unit.
    #[prost(uint32, optional, tag = "15")]
    pub ability_id: ::std::option::Option<u32>,
    #[prost(enumeration = "Race", optional, tag = "16")]
    pub race: ::std::option::Option<i32>,
    #[prost(float, optional, tag = "17")]
    pub build_time: ::std::option::Option<f32>,
    #[prost(bool, optional, tag = "19")]
    pub has_vespene: ::std::option::Option<bool>,
    #[prost(bool, optional, tag = "20")]
    pub has_minerals: ::std::option::Option<bool>,
    /// Range unit reveals vision.
    #[prost(float, optional, tag = "25")]
    pub sight_range: ::std::option::Option<f32>,
    /// Other units that satisfy the same tech requirement.
    #[prost(uint32, repeated, packed = "false", tag = "21")]
    pub tech_alias: ::std::vec::Vec<u32>,
    /// The morphed variant of this unit.
    #[prost(uint32, optional, tag = "22")]
    pub unit_alias: ::std::option::Option<u32>,
    /// Structure required to build this unit. (Or any with the same tech_alias)
    #[prost(uint32, optional, tag = "23")]
    pub tech_requirement: ::std::option::Option<u32>,
    /// Whether tech_requirement is an add-on.
    #[prost(bool, optional, tag = "24")]
    pub require_attached: ::std::option::Option<bool>,
    /// Values include changes from upgrades
    #[prost(enumeration = "Attribute", repeated, packed = "false", tag = "8")]
    pub attributes: ::std::vec::Vec<i32>,
    #[prost(float, optional, tag = "9")]
    pub movement_speed: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "10")]
    pub armor: ::std::option::Option<f32>,
    #[prost(message, repeated, tag = "11")]
    pub weapons: ::std::vec::Vec<Weapon>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeData {
    /// Stable ID.
    #[prost(uint32, optional, tag = "1")]
    pub upgrade_id: ::std::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub name: ::std::option::Option<std::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub mineral_cost: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub vespene_cost: ::std::option::Option<u32>,
    #[prost(float, optional, tag = "5")]
    pub research_time: ::std::option::Option<f32>,
    #[prost(uint32, optional, tag = "6")]
    pub ability_id: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuffData {
    /// Stable ID.
    #[prost(uint32, optional, tag = "1")]
    pub buff_id: ::std::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub name: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EffectData {
    /// Stable ID.
    #[prost(uint32, optional, tag = "1")]
    pub effect_id: ::std::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub name: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "3")]
    pub friendly_name: ::std::option::Option<std::string::String>,
    #[prost(float, optional, tag = "4")]
    pub radius: ::std::option::Option<f32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Attribute {
    Light = 1,
    Armored = 2,
    Biological = 3,
    Mechanical = 4,
    Robotic = 5,
    Psionic = 6,
    Massive = 7,
    Structure = 8,
    Hover = 9,
    Heroic = 10,
    Summoned = 11,
}
/// Issue various useful commands to the game engine.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugCommand {
    #[prost(oneof = "debug_command::Command", tags = "1, 2, 3, 4, 5, 6, 7, 8")]
    pub command: ::std::option::Option<debug_command::Command>,
}
pub mod debug_command {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Command {
        #[prost(message, tag = "1")]
        Draw(super::DebugDraw),
        #[prost(enumeration = "super::DebugGameState", tag = "2")]
        GameState(i32),
        #[prost(message, tag = "3")]
        CreateUnit(super::DebugCreateUnit),
        #[prost(message, tag = "4")]
        KillUnit(super::DebugKillUnit),
        #[prost(message, tag = "5")]
        TestProcess(super::DebugTestProcess),
        /// Useful only for single-player "curriculum" maps.
        #[prost(message, tag = "6")]
        Score(super::DebugSetScore),
        #[prost(message, tag = "7")]
        EndGame(super::DebugEndGame),
        #[prost(message, tag = "8")]
        UnitValue(super::DebugSetUnitValue),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugDraw {
    #[prost(message, repeated, tag = "1")]
    pub text: ::std::vec::Vec<DebugText>,
    #[prost(message, repeated, tag = "2")]
    pub lines: ::std::vec::Vec<DebugLine>,
    #[prost(message, repeated, tag = "3")]
    pub boxes: ::std::vec::Vec<DebugBox>,
    #[prost(message, repeated, tag = "4")]
    pub spheres: ::std::vec::Vec<DebugSphere>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Line {
    #[prost(message, optional, tag = "1")]
    pub p0: ::std::option::Option<Point>,
    #[prost(message, optional, tag = "2")]
    pub p1: ::std::option::Option<Point>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Color {
    #[prost(uint32, optional, tag = "1")]
    pub r: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub g: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub b: ::std::option::Option<u32>,
}
/// Display debug text on screen.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugText {
    #[prost(message, optional, tag = "1")]
    pub color: ::std::option::Option<Color>,
    /// Text to display.
    #[prost(string, optional, tag = "2")]
    pub text: ::std::option::Option<std::string::String>,
    /// Virtualized position in 2D (the screen is 0..1, 0..1 for any resolution).
    #[prost(message, optional, tag = "3")]
    pub virtual_pos: ::std::option::Option<Point>,
    /// Position in the world.
    #[prost(message, optional, tag = "4")]
    pub world_pos: ::std::option::Option<Point>,
    /// Pixel height of the text. Defaults to 8px.
    #[prost(uint32, optional, tag = "5")]
    pub size: ::std::option::Option<u32>,
}
/// Display debug lines on screen.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugLine {
    #[prost(message, optional, tag = "1")]
    pub color: ::std::option::Option<Color>,
    /// World space line.
    #[prost(message, optional, tag = "2")]
    pub line: ::std::option::Option<Line>,
}
/// Display debug boxes on screen.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugBox {
    #[prost(message, optional, tag = "1")]
    pub color: ::std::option::Option<Color>,
    #[prost(message, optional, tag = "2")]
    pub min: ::std::option::Option<Point>,
    #[prost(message, optional, tag = "3")]
    pub max: ::std::option::Option<Point>,
}
/// Display debug spheres on screen.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugSphere {
    #[prost(message, optional, tag = "1")]
    pub color: ::std::option::Option<Color>,
    #[prost(message, optional, tag = "2")]
    pub p: ::std::option::Option<Point>,
    #[prost(float, optional, tag = "3")]
    pub r: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugCreateUnit {
    #[prost(uint32, optional, tag = "1")]
    pub unit_type: ::std::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub owner: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub pos: ::std::option::Option<Point2D>,
    #[prost(uint32, optional, tag = "4")]
    pub quantity: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugKillUnit {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub tag: ::std::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugTestProcess {
    #[prost(enumeration = "debug_test_process::Test", optional, tag = "1")]
    pub test: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub delay_ms: ::std::option::Option<i32>,
}
pub mod debug_test_process {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Test {
        Hang = 1,
        Crash = 2,
        Exit = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugSetScore {
    #[prost(float, optional, tag = "1")]
    pub score: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugEndGame {
    #[prost(enumeration = "debug_end_game::EndResult", optional, tag = "1")]
    pub end_result: ::std::option::Option<i32>,
}
pub mod debug_end_game {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EndResult {
        /// Default if nothing is set. The current player admits defeat.
        Surrender = 1,
        DeclareVictory = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugSetUnitValue {
    #[prost(enumeration = "debug_set_unit_value::UnitValue", optional, tag = "1")]
    pub unit_value: ::std::option::Option<i32>,
    #[prost(float, optional, tag = "2")]
    pub value: ::std::option::Option<f32>,
    #[prost(uint64, optional, tag = "3")]
    pub unit_tag: ::std::option::Option<u64>,
}
pub mod debug_set_unit_value {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UnitValue {
        Energy = 1,
        Life = 2,
        Shields = 3,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DebugGameState {
    ShowMap = 1,
    ControlEnemy = 2,
    Food = 3,
    Free = 4,
    AllResources = 5,
    God = 6,
    Minerals = 7,
    Gas = 8,
    Cooldown = 9,
    TechTree = 10,
    Upgrade = 11,
    FastBuild = 12,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActionResult {
    Success = 1,
    NotSupported = 2,
    Error = 3,
    CantQueueThatOrder = 4,
    Retry = 5,
    Cooldown = 6,
    QueueIsFull = 7,
    RallyQueueIsFull = 8,
    NotEnoughMinerals = 9,
    NotEnoughVespene = 10,
    NotEnoughTerrazine = 11,
    NotEnoughCustom = 12,
    NotEnoughFood = 13,
    FoodUsageImpossible = 14,
    NotEnoughLife = 15,
    NotEnoughShields = 16,
    NotEnoughEnergy = 17,
    LifeSuppressed = 18,
    ShieldsSuppressed = 19,
    EnergySuppressed = 20,
    NotEnoughCharges = 21,
    CantAddMoreCharges = 22,
    TooMuchMinerals = 23,
    TooMuchVespene = 24,
    TooMuchTerrazine = 25,
    TooMuchCustom = 26,
    TooMuchFood = 27,
    TooMuchLife = 28,
    TooMuchShields = 29,
    TooMuchEnergy = 30,
    MustTargetUnitWithLife = 31,
    MustTargetUnitWithShields = 32,
    MustTargetUnitWithEnergy = 33,
    CantTrade = 34,
    CantSpend = 35,
    CantTargetThatUnit = 36,
    CouldntAllocateUnit = 37,
    UnitCantMove = 38,
    TransportIsHoldingPosition = 39,
    BuildTechRequirementsNotMet = 40,
    CantFindPlacementLocation = 41,
    CantBuildOnThat = 42,
    CantBuildTooCloseToDropOff = 43,
    CantBuildLocationInvalid = 44,
    CantSeeBuildLocation = 45,
    CantBuildTooCloseToCreepSource = 46,
    CantBuildTooCloseToResources = 47,
    CantBuildTooFarFromWater = 48,
    CantBuildTooFarFromCreepSource = 49,
    CantBuildTooFarFromBuildPowerSource = 50,
    CantBuildOnDenseTerrain = 51,
    CantTrainTooFarFromTrainPowerSource = 52,
    CantLandLocationInvalid = 53,
    CantSeeLandLocation = 54,
    CantLandTooCloseToCreepSource = 55,
    CantLandTooCloseToResources = 56,
    CantLandTooFarFromWater = 57,
    CantLandTooFarFromCreepSource = 58,
    CantLandTooFarFromBuildPowerSource = 59,
    CantLandTooFarFromTrainPowerSource = 60,
    CantLandOnDenseTerrain = 61,
    AddOnTooFarFromBuilding = 62,
    MustBuildRefineryFirst = 63,
    BuildingIsUnderConstruction = 64,
    CantFindDropOff = 65,
    CantLoadOtherPlayersUnits = 66,
    NotEnoughRoomToLoadUnit = 67,
    CantUnloadUnitsThere = 68,
    CantWarpInUnitsThere = 69,
    CantLoadImmobileUnits = 70,
    CantRechargeImmobileUnits = 71,
    CantRechargeUnderConstructionUnits = 72,
    CantLoadThatUnit = 73,
    NoCargoToUnload = 74,
    LoadAllNoTargetsFound = 75,
    NotWhileOccupied = 76,
    CantAttackWithoutAmmo = 77,
    CantHoldAnyMoreAmmo = 78,
    TechRequirementsNotMet = 79,
    MustLockdownUnitFirst = 80,
    MustTargetUnit = 81,
    MustTargetInventory = 82,
    MustTargetVisibleUnit = 83,
    MustTargetVisibleLocation = 84,
    MustTargetWalkableLocation = 85,
    MustTargetPawnableUnit = 86,
    YouCantControlThatUnit = 87,
    YouCantIssueCommandsToThatUnit = 88,
    MustTargetResources = 89,
    RequiresHealTarget = 90,
    RequiresRepairTarget = 91,
    NoItemsToDrop = 92,
    CantHoldAnyMoreItems = 93,
    CantHoldThat = 94,
    TargetHasNoInventory = 95,
    CantDropThisItem = 96,
    CantMoveThisItem = 97,
    CantPawnThisUnit = 98,
    MustTargetCaster = 99,
    CantTargetCaster = 100,
    MustTargetOuter = 101,
    CantTargetOuter = 102,
    MustTargetYourOwnUnits = 103,
    CantTargetYourOwnUnits = 104,
    MustTargetFriendlyUnits = 105,
    CantTargetFriendlyUnits = 106,
    MustTargetNeutralUnits = 107,
    CantTargetNeutralUnits = 108,
    MustTargetEnemyUnits = 109,
    CantTargetEnemyUnits = 110,
    MustTargetAirUnits = 111,
    CantTargetAirUnits = 112,
    MustTargetGroundUnits = 113,
    CantTargetGroundUnits = 114,
    MustTargetStructures = 115,
    CantTargetStructures = 116,
    MustTargetLightUnits = 117,
    CantTargetLightUnits = 118,
    MustTargetArmoredUnits = 119,
    CantTargetArmoredUnits = 120,
    MustTargetBiologicalUnits = 121,
    CantTargetBiologicalUnits = 122,
    MustTargetHeroicUnits = 123,
    CantTargetHeroicUnits = 124,
    MustTargetRoboticUnits = 125,
    CantTargetRoboticUnits = 126,
    MustTargetMechanicalUnits = 127,
    CantTargetMechanicalUnits = 128,
    MustTargetPsionicUnits = 129,
    CantTargetPsionicUnits = 130,
    MustTargetMassiveUnits = 131,
    CantTargetMassiveUnits = 132,
    MustTargetMissile = 133,
    CantTargetMissile = 134,
    MustTargetWorkerUnits = 135,
    CantTargetWorkerUnits = 136,
    MustTargetEnergyCapableUnits = 137,
    CantTargetEnergyCapableUnits = 138,
    MustTargetShieldCapableUnits = 139,
    CantTargetShieldCapableUnits = 140,
    MustTargetFlyers = 141,
    CantTargetFlyers = 142,
    MustTargetBuriedUnits = 143,
    CantTargetBuriedUnits = 144,
    MustTargetCloakedUnits = 145,
    CantTargetCloakedUnits = 146,
    MustTargetUnitsInAStasisField = 147,
    CantTargetUnitsInAStasisField = 148,
    MustTargetUnderConstructionUnits = 149,
    CantTargetUnderConstructionUnits = 150,
    MustTargetDeadUnits = 151,
    CantTargetDeadUnits = 152,
    MustTargetRevivableUnits = 153,
    CantTargetRevivableUnits = 154,
    MustTargetHiddenUnits = 155,
    CantTargetHiddenUnits = 156,
    CantRechargeOtherPlayersUnits = 157,
    MustTargetHallucinations = 158,
    CantTargetHallucinations = 159,
    MustTargetInvulnerableUnits = 160,
    CantTargetInvulnerableUnits = 161,
    MustTargetDetectedUnits = 162,
    CantTargetDetectedUnits = 163,
    CantTargetUnitWithEnergy = 164,
    CantTargetUnitWithShields = 165,
    MustTargetUncommandableUnits = 166,
    CantTargetUncommandableUnits = 167,
    MustTargetPreventDefeatUnits = 168,
    CantTargetPreventDefeatUnits = 169,
    MustTargetPreventRevealUnits = 170,
    CantTargetPreventRevealUnits = 171,
    MustTargetPassiveUnits = 172,
    CantTargetPassiveUnits = 173,
    MustTargetStunnedUnits = 174,
    CantTargetStunnedUnits = 175,
    MustTargetSummonedUnits = 176,
    CantTargetSummonedUnits = 177,
    MustTargetUser1 = 178,
    CantTargetUser1 = 179,
    MustTargetUnstoppableUnits = 180,
    CantTargetUnstoppableUnits = 181,
    MustTargetResistantUnits = 182,
    CantTargetResistantUnits = 183,
    MustTargetDazedUnits = 184,
    CantTargetDazedUnits = 185,
    CantLockdown = 186,
    CantMindControl = 187,
    MustTargetDestructibles = 188,
    CantTargetDestructibles = 189,
    MustTargetItems = 190,
    CantTargetItems = 191,
    NoCalldownAvailable = 192,
    WaypointListFull = 193,
    MustTargetRace = 194,
    CantTargetRace = 195,
    MustTargetSimilarUnits = 196,
    CantTargetSimilarUnits = 197,
    CantFindEnoughTargets = 198,
    AlreadySpawningLarva = 199,
    CantTargetExhaustedResources = 200,
    CantUseMinimap = 201,
    CantUseInfoPanel = 202,
    OrderQueueIsFull = 203,
    CantHarvestThatResource = 204,
    HarvestersNotRequired = 205,
    AlreadyTargeted = 206,
    CantAttackWeaponsDisabled = 207,
    CouldntReachTarget = 208,
    TargetIsOutOfRange = 209,
    TargetIsTooClose = 210,
    TargetIsOutOfArc = 211,
    CantFindTeleportLocation = 212,
    InvalidItemClass = 213,
    CantFindCancelOrder = 214,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestQuery {
    #[prost(message, repeated, tag = "1")]
    pub pathing: ::std::vec::Vec<RequestQueryPathing>,
    #[prost(message, repeated, tag = "2")]
    pub abilities: ::std::vec::Vec<RequestQueryAvailableAbilities>,
    #[prost(message, repeated, tag = "3")]
    pub placements: ::std::vec::Vec<RequestQueryBuildingPlacement>,
    /// Ignores requirements like food, minerals and so on.
    #[prost(bool, optional, tag = "4")]
    pub ignore_resource_requirements: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseQuery {
    #[prost(message, repeated, tag = "1")]
    pub pathing: ::std::vec::Vec<ResponseQueryPathing>,
    #[prost(message, repeated, tag = "2")]
    pub abilities: ::std::vec::Vec<ResponseQueryAvailableAbilities>,
    #[prost(message, repeated, tag = "3")]
    pub placements: ::std::vec::Vec<ResponseQueryBuildingPlacement>,
}
///--------------------------------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestQueryPathing {
    #[prost(message, optional, tag = "3")]
    pub end_pos: ::std::option::Option<Point2D>,
    #[prost(oneof = "request_query_pathing::Start", tags = "1, 2")]
    pub start: ::std::option::Option<request_query_pathing::Start>,
}
pub mod request_query_pathing {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Start {
        #[prost(message, tag = "1")]
        StartPos(super::Point2D),
        #[prost(uint64, tag = "2")]
        UnitTag(u64),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseQueryPathing {
    /// 0 if no path exists
    #[prost(float, optional, tag = "1")]
    pub distance: ::std::option::Option<f32>,
}
///--------------------------------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestQueryAvailableAbilities {
    #[prost(uint64, optional, tag = "1")]
    pub unit_tag: ::std::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseQueryAvailableAbilities {
    #[prost(message, repeated, tag = "1")]
    pub abilities: ::std::vec::Vec<AvailableAbility>,
    #[prost(uint64, optional, tag = "2")]
    pub unit_tag: ::std::option::Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub unit_type_id: ::std::option::Option<u32>,
}
///--------------------------------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestQueryBuildingPlacement {
    #[prost(int32, optional, tag = "1")]
    pub ability_id: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub target_pos: ::std::option::Option<Point2D>,
    /// Not required
    #[prost(uint64, optional, tag = "3")]
    pub placing_unit_tag: ::std::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseQueryBuildingPlacement {
    #[prost(enumeration = "ActionResult", optional, tag = "1")]
    pub result: ::std::option::Option<i32>,
}
//
// Start
//

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartRaw {
    /// Width and height of the map.
    #[prost(message, optional, tag = "1")]
    pub map_size: ::std::option::Option<Size2Di>,
    /// 1 bit bitmap of the pathing grid.
    #[prost(message, optional, tag = "2")]
    pub pathing_grid: ::std::option::Option<ImageData>,
    /// 1 byte bitmap of the terrain height.
    #[prost(message, optional, tag = "3")]
    pub terrain_height: ::std::option::Option<ImageData>,
    /// 1 bit bitmap of the building placement grid.
    #[prost(message, optional, tag = "4")]
    pub placement_grid: ::std::option::Option<ImageData>,
    /// The playable cells.
    #[prost(message, optional, tag = "5")]
    pub playable_area: ::std::option::Option<RectangleI>,
    /// Possible start locations for players.
    #[prost(message, repeated, tag = "6")]
    pub start_locations: ::std::vec::Vec<Point2D>,
}
//
// Observation
//

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObservationRaw {
    #[prost(message, optional, tag = "1")]
    pub player: ::std::option::Option<PlayerRaw>,
    #[prost(message, repeated, tag = "2")]
    pub units: ::std::vec::Vec<Unit>,
    /// Fog of war, creep and so on. Board stuff that changes per frame.
    #[prost(message, optional, tag = "3")]
    pub map_state: ::std::option::Option<MapState>,
    #[prost(message, optional, tag = "4")]
    pub event: ::std::option::Option<Event>,
    #[prost(message, repeated, tag = "5")]
    pub effects: ::std::vec::Vec<Effect>,
    #[prost(message, repeated, tag = "6")]
    pub radar: ::std::vec::Vec<RadarRing>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RadarRing {
    #[prost(message, optional, tag = "1")]
    pub pos: ::std::option::Option<Point>,
    #[prost(float, optional, tag = "2")]
    pub radius: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PowerSource {
    #[prost(message, optional, tag = "1")]
    pub pos: ::std::option::Option<Point>,
    #[prost(float, optional, tag = "2")]
    pub radius: ::std::option::Option<f32>,
    #[prost(uint64, optional, tag = "3")]
    pub tag: ::std::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerRaw {
    #[prost(message, repeated, tag = "1")]
    pub power_sources: ::std::vec::Vec<PowerSource>,
    #[prost(message, optional, tag = "2")]
    pub camera: ::std::option::Option<Point>,
    /// TODO: Add to UI observation?
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub upgrade_ids: ::std::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnitOrder {
    #[prost(uint32, optional, tag = "1")]
    pub ability_id: ::std::option::Option<u32>,
    /// Progress of train abilities. Range: [0.0, 1.0]
    #[prost(float, optional, tag = "4")]
    pub progress: ::std::option::Option<f32>,
    #[prost(oneof = "unit_order::Target", tags = "2, 3")]
    pub target: ::std::option::Option<unit_order::Target>,
}
pub mod unit_order {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        #[prost(message, tag = "2")]
        TargetWorldSpacePos(super::Point),
        #[prost(uint64, tag = "3")]
        TargetUnitTag(u64),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PassengerUnit {
    #[prost(uint64, optional, tag = "1")]
    pub tag: ::std::option::Option<u64>,
    #[prost(float, optional, tag = "2")]
    pub health: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub health_max: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub shield: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "7")]
    pub shield_max: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "5")]
    pub energy: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "8")]
    pub energy_max: ::std::option::Option<f32>,
    #[prost(uint32, optional, tag = "6")]
    pub unit_type: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RallyTarget {
    /// Will always be filled.
    #[prost(message, optional, tag = "1")]
    pub point: ::std::option::Option<Point>,
    /// Only if it's targeting a unit.
    #[prost(uint64, optional, tag = "2")]
    pub tag: ::std::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Unit {
    /// Fields are populated based on type/alliance
    #[prost(enumeration = "DisplayType", optional, tag = "1")]
    pub display_type: ::std::option::Option<i32>,
    #[prost(enumeration = "Alliance", optional, tag = "2")]
    pub alliance: ::std::option::Option<i32>,
    /// Unique identifier for a unit
    #[prost(uint64, optional, tag = "3")]
    pub tag: ::std::option::Option<u64>,
    #[prost(uint32, optional, tag = "4")]
    pub unit_type: ::std::option::Option<u32>,
    #[prost(int32, optional, tag = "5")]
    pub owner: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "6")]
    pub pos: ::std::option::Option<Point>,
    #[prost(float, optional, tag = "7")]
    pub facing: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "8")]
    pub radius: ::std::option::Option<f32>,
    /// Range: [0.0, 1.0]
    #[prost(float, optional, tag = "9")]
    pub build_progress: ::std::option::Option<f32>,
    #[prost(enumeration = "CloakState", optional, tag = "10")]
    pub cloak: ::std::option::Option<i32>,
    #[prost(uint32, repeated, packed = "false", tag = "27")]
    pub buff_ids: ::std::vec::Vec<u32>,
    #[prost(float, optional, tag = "31")]
    pub detect_range: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "32")]
    pub radar_range: ::std::option::Option<f32>,
    #[prost(bool, optional, tag = "11")]
    pub is_selected: ::std::option::Option<bool>,
    /// Visible and within the camera frustrum.
    #[prost(bool, optional, tag = "12")]
    pub is_on_screen: ::std::option::Option<bool>,
    /// Detected by sensor tower
    #[prost(bool, optional, tag = "13")]
    pub is_blip: ::std::option::Option<bool>,
    #[prost(bool, optional, tag = "35")]
    pub is_powered: ::std::option::Option<bool>,
    /// Building is training/researching (ie animated).
    #[prost(bool, optional, tag = "39")]
    pub is_active: ::std::option::Option<bool>,
    #[prost(int32, optional, tag = "40")]
    pub attack_upgrade_level: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "41")]
    pub armor_upgrade_level: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "42")]
    pub shield_upgrade_level: ::std::option::Option<i32>,
    /// Not populated for snapshots
    #[prost(float, optional, tag = "14")]
    pub health: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "15")]
    pub health_max: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "16")]
    pub shield: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "36")]
    pub shield_max: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "17")]
    pub energy: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "37")]
    pub energy_max: ::std::option::Option<f32>,
    #[prost(int32, optional, tag = "18")]
    pub mineral_contents: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "19")]
    pub vespene_contents: ::std::option::Option<i32>,
    #[prost(bool, optional, tag = "20")]
    pub is_flying: ::std::option::Option<bool>,
    #[prost(bool, optional, tag = "21")]
    pub is_burrowed: ::std::option::Option<bool>,
    /// Unit is your own or detected as a hallucination.
    #[prost(bool, optional, tag = "38")]
    pub is_hallucination: ::std::option::Option<bool>,
    /// Not populated for enemies
    #[prost(message, repeated, tag = "22")]
    pub orders: ::std::vec::Vec<UnitOrder>,
    #[prost(uint64, optional, tag = "23")]
    pub add_on_tag: ::std::option::Option<u64>,
    #[prost(message, repeated, tag = "24")]
    pub passengers: ::std::vec::Vec<PassengerUnit>,
    #[prost(int32, optional, tag = "25")]
    pub cargo_space_taken: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "26")]
    pub cargo_space_max: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "28")]
    pub assigned_harvesters: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "29")]
    pub ideal_harvesters: ::std::option::Option<i32>,
    #[prost(float, optional, tag = "30")]
    pub weapon_cooldown: ::std::option::Option<f32>,
    #[prost(uint64, optional, tag = "34")]
    pub engaged_target_tag: ::std::option::Option<u64>,
    /// How long a buff or unit is still around (eg mule, broodling, chronoboost).
    #[prost(int32, optional, tag = "43")]
    pub buff_duration_remain: ::std::option::Option<i32>,
    /// How long the buff or unit is still around (eg mule, broodling, chronoboost).
    #[prost(int32, optional, tag = "44")]
    pub buff_duration_max: ::std::option::Option<i32>,
    #[prost(message, repeated, tag = "45")]
    pub rally_targets: ::std::vec::Vec<RallyTarget>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapState {
    /// 1 byte visibility layer.
    #[prost(message, optional, tag = "1")]
    pub visibility: ::std::option::Option<ImageData>,
    /// 1 bit creep layer.
    #[prost(message, optional, tag = "2")]
    pub creep: ::std::option::Option<ImageData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub dead_units: ::std::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Effect {
    #[prost(uint32, optional, tag = "1")]
    pub effect_id: ::std::option::Option<u32>,
    /// Effect may impact multiple locations. (eg. Lurker attack)
    #[prost(message, repeated, tag = "2")]
    pub pos: ::std::vec::Vec<Point2D>,
    #[prost(enumeration = "Alliance", optional, tag = "3")]
    pub alliance: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub owner: ::std::option::Option<i32>,
    #[prost(float, optional, tag = "5")]
    pub radius: ::std::option::Option<f32>,
}
//
// Action
//

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionRaw {
    #[prost(oneof = "action_raw::Action", tags = "1, 2, 3")]
    pub action: ::std::option::Option<action_raw::Action>,
}
pub mod action_raw {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        #[prost(message, tag = "1")]
        UnitCommand(super::ActionRawUnitCommand),
        #[prost(message, tag = "2")]
        CameraMove(super::ActionRawCameraMove),
        #[prost(message, tag = "3")]
        ToggleAutocast(super::ActionRawToggleAutocast),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionRawUnitCommand {
    #[prost(int32, optional, tag = "1")]
    pub ability_id: ::std::option::Option<i32>,
    #[prost(uint64, repeated, packed = "false", tag = "4")]
    pub unit_tags: ::std::vec::Vec<u64>,
    #[prost(bool, optional, tag = "5")]
    pub queue_command: ::std::option::Option<bool>,
    #[prost(oneof = "action_raw_unit_command::Target", tags = "2, 3")]
    pub target: ::std::option::Option<action_raw_unit_command::Target>,
}
pub mod action_raw_unit_command {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        #[prost(message, tag = "2")]
        TargetWorldSpacePos(super::Point2D),
        #[prost(uint64, tag = "3")]
        TargetUnitTag(u64),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionRawCameraMove {
    #[prost(message, optional, tag = "1")]
    pub center_world_space: ::std::option::Option<Point>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionRawToggleAutocast {
    #[prost(int32, optional, tag = "1")]
    pub ability_id: ::std::option::Option<i32>,
    #[prost(uint64, repeated, packed = "false", tag = "2")]
    pub unit_tags: ::std::vec::Vec<u64>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DisplayType {
    /// Fully visible
    Visible = 1,
    /// Dimmed version of unit left behind after entering fog of war
    Snapshot = 2,
    /// Fully hidden
    Hidden = 3,
    /// Building that hasn't started construction.
    Placeholder = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Alliance {
    Me = 1,
    Ally = 2,
    Neutral = 3,
    Enemy = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CloakState {
    /// Under the fog, so unknown whether it's cloaked or not.
    CloakedUnknown = 0,
    Cloaked = 1,
    CloakedDetected = 2,
    NotCloaked = 3,
    CloakedAllied = 4,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Score {
    #[prost(enumeration = "score::ScoreType", optional, tag = "6")]
    pub score_type: ::std::option::Option<i32>,
    /// Note: check score_type to know whether this is a melee score or curriculum score
    #[prost(int32, optional, tag = "7")]
    pub score: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "8")]
    pub score_details: ::std::option::Option<ScoreDetails>,
}
pub mod score {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ScoreType {
        /// map generated score (from curriculum maps with special scoring)
        Curriculum = 1,
        /// summation of in-progress and current units/buildings value + minerals + vespene
        Melee = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CategoryScoreDetails {
    /// Used when no other category is configured in game data
    #[prost(float, optional, tag = "1")]
    pub none: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub army: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub economy: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub technology: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "5")]
    pub upgrade: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VitalScoreDetails {
    #[prost(float, optional, tag = "1")]
    pub life: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub shields: ::std::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub energy: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScoreDetails {
    /// Sum of time any available structure able to produce a unit is not. The time stacks, as in, three idle barracks will increase idle_production_time three times quicker than just one.
    #[prost(float, optional, tag = "1")]
    pub idle_production_time: ::std::option::Option<f32>,
    /// Sum of time any worker is not mining. Note a worker building is not idle and three idle workers will increase this value three times quicker than just one.
    #[prost(float, optional, tag = "2")]
    pub idle_worker_time: ::std::option::Option<f32>,
    /// Sum of minerals and vespene spent on completed units.
    #[prost(float, optional, tag = "3")]
    pub total_value_units: ::std::option::Option<f32>,
    /// Sum of minerals and vespene spent on completed structures.
    #[prost(float, optional, tag = "4")]
    pub total_value_structures: ::std::option::Option<f32>,
    /// Sum of minerals and vespene of units, belonging to the opponent, that the player has destroyed.
    #[prost(float, optional, tag = "5")]
    pub killed_value_units: ::std::option::Option<f32>,
    /// Sum of minerals and vespene of structures, belonging to the opponent, that the player has destroyed.
    #[prost(float, optional, tag = "6")]
    pub killed_value_structures: ::std::option::Option<f32>,
    /// Sum of minerals collected by the player.
    #[prost(float, optional, tag = "7")]
    pub collected_minerals: ::std::option::Option<f32>,
    /// Sum of vespene collected by the player.
    #[prost(float, optional, tag = "8")]
    pub collected_vespene: ::std::option::Option<f32>,
    /// Estimated income of minerals over the next minute based on the players current income. The unit is minerals per minute.
    #[prost(float, optional, tag = "9")]
    pub collection_rate_minerals: ::std::option::Option<f32>,
    /// Estimated income of vespene over the next minute based on the players current income. The unit is vespene per minute.
    #[prost(float, optional, tag = "10")]
    pub collection_rate_vespene: ::std::option::Option<f32>,
    /// Sum of spent minerals at the moment it is spent. For example, this number is incremented by 50 the moment an scv is queued in a command center.  It is decremented by 50 if that unit is canceled.
    #[prost(float, optional, tag = "11")]
    pub spent_minerals: ::std::option::Option<f32>,
    /// Sum of spent vespene at the moment it is spent. For example, this number is incremented by 50 when a reaper is queued but decremented by 50 if it is canceled.
    #[prost(float, optional, tag = "12")]
    pub spent_vespene: ::std::option::Option<f32>,
    // The following entries contains floating point values for the following catgories:
    //   none - There is no category defined in game data.
    //   army - This category includes all military units but not workers.
    //   economy - This category contains town halls, supply structures, vespene buildings and workers.
    //   technology - This category is any structure that produces units or upgrades, Barracks and Engineering Bays both fall in this category, for example.
    //   upgrade - This category is upgrades such as warp gate or weapons upgrades.
    /// Sum of food, or supply, utilized in the categories above.
    #[prost(message, optional, tag = "13")]
    pub food_used: ::std::option::Option<CategoryScoreDetails>,
    /// Sum of enemies catagories destroyed in minerals.
    #[prost(message, optional, tag = "14")]
    pub killed_minerals: ::std::option::Option<CategoryScoreDetails>,
    /// Sum of enemies catagories destroyed in vespene.
    #[prost(message, optional, tag = "15")]
    pub killed_vespene: ::std::option::Option<CategoryScoreDetails>,
    ///  Sum of lost minerals for the player in each category.
    #[prost(message, optional, tag = "16")]
    pub lost_minerals: ::std::option::Option<CategoryScoreDetails>,
    /// Sum of lost vespene for the player in each category.
    #[prost(message, optional, tag = "17")]
    pub lost_vespene: ::std::option::Option<CategoryScoreDetails>,
    /// Sum of the lost minerals via destroying the players own units/buildings.
    #[prost(message, optional, tag = "18")]
    pub friendly_fire_minerals: ::std::option::Option<CategoryScoreDetails>,
    /// Sum of the lost vespene via destroying the players own units/buildings.
    #[prost(message, optional, tag = "19")]
    pub friendly_fire_vespene: ::std::option::Option<CategoryScoreDetails>,
    /// Sum of used minerals for the player in each category for each existing unit or upgrade. Therefore if a unit died worth 50 mierals this number will be decremented by 50.
    #[prost(message, optional, tag = "20")]
    pub used_minerals: ::std::option::Option<CategoryScoreDetails>,
    /// Sum of used vespene for the player in each category. Therefore if a unit died worth 50 vespene this number will be decremented by 50.
    #[prost(message, optional, tag = "21")]
    pub used_vespene: ::std::option::Option<CategoryScoreDetails>,
    /// Sum of used minerals throughout the entire game for each category. Unliked used_minerals, this value is never decremented.
    #[prost(message, optional, tag = "22")]
    pub total_used_minerals: ::std::option::Option<CategoryScoreDetails>,
    /// Sum of used vespene throughout the entire game for each category. Unliked used_vespene, this value is never decremented.
    #[prost(message, optional, tag = "23")]
    pub total_used_vespene: ::std::option::Option<CategoryScoreDetails>,
    /// Sum of damage dealt to the player's opponent for each category.
    #[prost(message, optional, tag = "24")]
    pub total_damage_dealt: ::std::option::Option<VitalScoreDetails>,
    /// Sum of damage taken by the player for each category.
    #[prost(message, optional, tag = "25")]
    pub total_damage_taken: ::std::option::Option<VitalScoreDetails>,
    /// Sum of health healed by the player. Note that technology can be healed (by queens) or repaired (by scvs).
    #[prost(message, optional, tag = "26")]
    pub total_healed: ::std::option::Option<VitalScoreDetails>,
    /// Recent raw APM.
    #[prost(float, optional, tag = "27")]
    pub current_apm: ::std::option::Option<f32>,
    /// Recent effective APM.
    #[prost(float, optional, tag = "28")]
    pub current_effective_apm: ::std::option::Option<f32>,
}
//
// Observation - Feature Layer
//

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObservationFeatureLayer {
    #[prost(message, optional, tag = "1")]
    pub renders: ::std::option::Option<FeatureLayers>,
    #[prost(message, optional, tag = "2")]
    pub minimap_renders: ::std::option::Option<FeatureLayersMinimap>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureLayers {
    /// uint8. Terrain height. World space units of [-200, 200] encoded into [0, 255].
    #[prost(message, optional, tag = "1")]
    pub height_map: ::std::option::Option<ImageData>,
    /// uint8. 0=Hidden, 1=Fogged, 2=Visible, 3=FullHidden
    #[prost(message, optional, tag = "2")]
    pub visibility_map: ::std::option::Option<ImageData>,
    /// 1-bit. Zerg creep.
    #[prost(message, optional, tag = "3")]
    pub creep: ::std::option::Option<ImageData>,
    /// 1-bit. Protoss power.
    #[prost(message, optional, tag = "4")]
    pub power: ::std::option::Option<ImageData>,
    /// uint8. Participants: [1, 15] Neutral: 16
    #[prost(message, optional, tag = "5")]
    pub player_id: ::std::option::Option<ImageData>,
    /// int32. Unique identifier for type of unit.
    #[prost(message, optional, tag = "6")]
    pub unit_type: ::std::option::Option<ImageData>,
    /// 1-bit. Selected units.
    #[prost(message, optional, tag = "7")]
    pub selected: ::std::option::Option<ImageData>,
    /// int32.
    #[prost(message, optional, tag = "8")]
    pub unit_hit_points: ::std::option::Option<ImageData>,
    /// uint8. Ratio of current health to max health. [0%, 100%] encoded into [0, 255].
    #[prost(message, optional, tag = "17")]
    pub unit_hit_points_ratio: ::std::option::Option<ImageData>,
    /// int32.
    #[prost(message, optional, tag = "9")]
    pub unit_energy: ::std::option::Option<ImageData>,
    /// uint8. Ratio of current energy to max energy. [0%, 100%] encoded into [0, 255].
    #[prost(message, optional, tag = "18")]
    pub unit_energy_ratio: ::std::option::Option<ImageData>,
    /// int32.
    #[prost(message, optional, tag = "10")]
    pub unit_shields: ::std::option::Option<ImageData>,
    /// uint8. Ratio of current shields to max shields. [0%, 100%] encoded into [0, 255].
    #[prost(message, optional, tag = "19")]
    pub unit_shields_ratio: ::std::option::Option<ImageData>,
    /// uint8. See "Alliance" enum in raw.proto. Range: [1, 4]
    #[prost(message, optional, tag = "11")]
    pub player_relative: ::std::option::Option<ImageData>,
    /// uint8. Density of units overlapping a pixel, anti-aliased. [0.0, 16.0f] encoded into [0, 255].
    #[prost(message, optional, tag = "14")]
    pub unit_density_aa: ::std::option::Option<ImageData>,
    /// uint8. Count of units overlapping a pixel.
    #[prost(message, optional, tag = "15")]
    pub unit_density: ::std::option::Option<ImageData>,
    /// uint8. Visuals of persistent abilities. (eg. Psistorm)
    #[prost(message, optional, tag = "20")]
    pub effects: ::std::option::Option<ImageData>,
    /// 1-bit. Whether the unit here is a hallucination.
    #[prost(message, optional, tag = "21")]
    pub hallucinations: ::std::option::Option<ImageData>,
    /// 1-bit. Whether the unit here is cloaked. Hidden units will show up too, but with less details in other layers.
    #[prost(message, optional, tag = "22")]
    pub cloaked: ::std::option::Option<ImageData>,
    /// 1-bit. Whether the unit here is a blip.
    #[prost(message, optional, tag = "23")]
    pub blip: ::std::option::Option<ImageData>,
    /// int32. One of the buffs applied to this unit. Extras are ignored.
    #[prost(message, optional, tag = "24")]
    pub buffs: ::std::option::Option<ImageData>,
    /// uint8. Ratio of buff remaining. [0%, 100%] encoded into [0, 255].
    #[prost(message, optional, tag = "26")]
    pub buff_duration: ::std::option::Option<ImageData>,
    /// 1-bit. Whether the unit here is active.
    #[prost(message, optional, tag = "25")]
    pub active: ::std::option::Option<ImageData>,
    /// uint8. How far along the building is building something. [0%, 100%] encoded into [0, 255].
    #[prost(message, optional, tag = "27")]
    pub build_progress: ::std::option::Option<ImageData>,
    /// 1-bit. Whether a building can be built here.
    #[prost(message, optional, tag = "28")]
    pub buildable: ::std::option::Option<ImageData>,
    /// 1-bit. Whether a unit can walk here.
    #[prost(message, optional, tag = "29")]
    pub pathable: ::std::option::Option<ImageData>,
    /// 1-bit. Whether the unit here is a placeholder building to be constructed.
    #[prost(message, optional, tag = "30")]
    pub placeholder: ::std::option::Option<ImageData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureLayersMinimap {
    /// uint8. Terrain height. World space units of [-200, 200] encoded into [0, 255].
    #[prost(message, optional, tag = "1")]
    pub height_map: ::std::option::Option<ImageData>,
    /// uint8. 0=Hidden, 1=Fogged, 2=Visible, 3=FullHidden
    #[prost(message, optional, tag = "2")]
    pub visibility_map: ::std::option::Option<ImageData>,
    /// 1-bit. Zerg creep.
    #[prost(message, optional, tag = "3")]
    pub creep: ::std::option::Option<ImageData>,
    /// 1-bit. Area covered by the camera.
    #[prost(message, optional, tag = "4")]
    pub camera: ::std::option::Option<ImageData>,
    /// uint8. Participants: [1, 15] Neutral: 16
    #[prost(message, optional, tag = "5")]
    pub player_id: ::std::option::Option<ImageData>,
    /// uint8. See "Alliance" enum in raw.proto. Range: [1, 4]
    #[prost(message, optional, tag = "6")]
    pub player_relative: ::std::option::Option<ImageData>,
    /// 1-bit. Selected units.
    #[prost(message, optional, tag = "7")]
    pub selected: ::std::option::Option<ImageData>,
    /// 1-bit. Shows 'UnitAttacked' alert location.
    #[prost(message, optional, tag = "9")]
    pub alerts: ::std::option::Option<ImageData>,
    /// 1-bit. Whether a building can be built here.
    #[prost(message, optional, tag = "10")]
    pub buildable: ::std::option::Option<ImageData>,
    /// 1-bit. Whether a unit can walk here.
    #[prost(message, optional, tag = "11")]
    pub pathable: ::std::option::Option<ImageData>,
    /// Cheat layers, enable with SpatialCameraSetup.allow_cheating_layers.
    ///
    /// int32. Unique identifier for type of unit.
    #[prost(message, optional, tag = "8")]
    pub unit_type: ::std::option::Option<ImageData>,
}
//
// Observation - Rendered
//

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObservationRender {
    #[prost(message, optional, tag = "1")]
    pub map: ::std::option::Option<ImageData>,
    #[prost(message, optional, tag = "2")]
    pub minimap: ::std::option::Option<ImageData>,
}
//
// Action
//

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionSpatial {
    #[prost(oneof = "action_spatial::Action", tags = "1, 2, 3, 4")]
    pub action: ::std::option::Option<action_spatial::Action>,
}
pub mod action_spatial {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        #[prost(message, tag = "1")]
        UnitCommand(super::ActionSpatialUnitCommand),
        #[prost(message, tag = "2")]
        CameraMove(super::ActionSpatialCameraMove),
        #[prost(message, tag = "3")]
        UnitSelectionPoint(super::ActionSpatialUnitSelectionPoint),
        #[prost(message, tag = "4")]
        UnitSelectionRect(super::ActionSpatialUnitSelectionRect),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionSpatialUnitCommand {
    #[prost(int32, optional, tag = "1")]
    pub ability_id: ::std::option::Option<i32>,
    /// Equivalent to shift+command.
    #[prost(bool, optional, tag = "4")]
    pub queue_command: ::std::option::Option<bool>,
    #[prost(oneof = "action_spatial_unit_command::Target", tags = "2, 3")]
    pub target: ::std::option::Option<action_spatial_unit_command::Target>,
}
pub mod action_spatial_unit_command {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        #[prost(message, tag = "2")]
        TargetScreenCoord(super::PointI),
        #[prost(message, tag = "3")]
        TargetMinimapCoord(super::PointI),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionSpatialCameraMove {
    /// Simulates a click on the minimap to move the camera.
    #[prost(message, optional, tag = "1")]
    pub center_minimap: ::std::option::Option<PointI>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionSpatialUnitSelectionPoint {
    #[prost(message, optional, tag = "1")]
    pub selection_screen_coord: ::std::option::Option<PointI>,
    #[prost(
        enumeration = "action_spatial_unit_selection_point::Type",
        optional,
        tag = "2"
    )]
    pub r#type: ::std::option::Option<i32>,
}
pub mod action_spatial_unit_selection_point {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Equivalent to normal click. Changes selection to unit.
        Select = 1,
        /// Equivalent to shift+click. Toggle selection of unit.
        Toggle = 2,
        /// Equivalent to control+click. Selects all units of a given type.
        AllType = 3,
        /// Equivalent to shift+control+click. Selects all units of a given type.
        AddAllType = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionSpatialUnitSelectionRect {
    /// Eventually this should not be an array, but a single field (multiple would be cheating).
    #[prost(message, repeated, tag = "1")]
    pub selection_screen_coord: ::std::vec::Vec<RectangleI>,
    /// Equivalent to shift+drag. Adds units to selection.
    #[prost(bool, optional, tag = "2")]
    pub selection_add: ::std::option::Option<bool>,
}
//
// Observation
//

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObservationUi {
    #[prost(message, repeated, tag = "1")]
    pub groups: ::std::vec::Vec<ControlGroup>,
    #[prost(oneof = "observation_ui::Panel", tags = "2, 3, 4, 5")]
    pub panel: ::std::option::Option<observation_ui::Panel>,
}
pub mod observation_ui {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Panel {
        #[prost(message, tag = "2")]
        Single(super::SinglePanel),
        #[prost(message, tag = "3")]
        Multi(super::MultiPanel),
        #[prost(message, tag = "4")]
        Cargo(super::CargoPanel),
        #[prost(message, tag = "5")]
        Production(super::ProductionPanel),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlGroup {
    #[prost(uint32, optional, tag = "1")]
    pub control_group_index: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub leader_unit_type: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub count: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnitInfo {
    #[prost(uint32, optional, tag = "1")]
    pub unit_type: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub player_relative: ::std::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub health: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub shields: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub energy: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub transport_slots_taken: ::std::option::Option<i32>,
    /// Range: [0.0, 1.0]
    #[prost(float, optional, tag = "7")]
    pub build_progress: ::std::option::Option<f32>,
    #[prost(message, optional, boxed, tag = "8")]
    pub add_on: ::std::option::Option<::std::boxed::Box<UnitInfo>>,
    #[prost(int32, optional, tag = "9")]
    pub max_health: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub max_shields: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub max_energy: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SinglePanel {
    #[prost(message, optional, tag = "1")]
    pub unit: ::std::option::Option<UnitInfo>,
    #[prost(int32, optional, tag = "2")]
    pub attack_upgrade_level: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub armor_upgrade_level: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub shield_upgrade_level: ::std::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "5")]
    pub buffs: ::std::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiPanel {
    #[prost(message, repeated, tag = "1")]
    pub units: ::std::vec::Vec<UnitInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CargoPanel {
    #[prost(message, optional, tag = "1")]
    pub unit: ::std::option::Option<UnitInfo>,
    #[prost(message, repeated, tag = "2")]
    pub passengers: ::std::vec::Vec<UnitInfo>,
    /// TODO: Change to cargo size
    #[prost(int32, optional, tag = "3")]
    pub slots_available: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildItem {
    #[prost(uint32, optional, tag = "1")]
    pub ability_id: ::std::option::Option<u32>,
    /// Range: [0.0, 1.0]
    #[prost(float, optional, tag = "2")]
    pub build_progress: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductionPanel {
    #[prost(message, optional, tag = "1")]
    pub unit: ::std::option::Option<UnitInfo>,
    /// build_queue ONLY gives information about units that are being produced.
    /// Use production_queue instead to see both units being trained as well as research in the queue.
    #[prost(message, repeated, tag = "2")]
    pub build_queue: ::std::vec::Vec<UnitInfo>,
    #[prost(message, repeated, tag = "3")]
    pub production_queue: ::std::vec::Vec<BuildItem>,
}
//
// Action
//

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionUi {
    #[prost(oneof = "action_ui::Action", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9")]
    pub action: ::std::option::Option<action_ui::Action>,
}
pub mod action_ui {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        #[prost(message, tag = "1")]
        ControlGroup(super::ActionControlGroup),
        #[prost(message, tag = "2")]
        SelectArmy(super::ActionSelectArmy),
        #[prost(message, tag = "3")]
        SelectWarpGates(super::ActionSelectWarpGates),
        #[prost(message, tag = "4")]
        SelectLarva(super::ActionSelectLarva),
        #[prost(message, tag = "5")]
        SelectIdleWorker(super::ActionSelectIdleWorker),
        #[prost(message, tag = "6")]
        MultiPanel(super::ActionMultiPanel),
        #[prost(message, tag = "7")]
        CargoPanel(super::ActionCargoPanelUnload),
        #[prost(message, tag = "8")]
        ProductionPanel(super::ActionProductionPanelRemoveFromQueue),
        #[prost(message, tag = "9")]
        ToggleAutocast(super::ActionToggleAutocast),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionControlGroup {
    #[prost(
        enumeration = "action_control_group::ControlGroupAction",
        optional,
        tag = "1"
    )]
    pub action: ::std::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub control_group_index: ::std::option::Option<u32>,
}
pub mod action_control_group {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ControlGroupAction {
        /// Equivalent to number hotkey. Replaces current selection with control group.
        Recall = 1,
        /// Equivalent to Control + number hotkey. Sets control group to current selection.
        Set = 2,
        /// Equivalent to Shift + number hotkey. Adds current selection into control group.
        Append = 3,
        /// Equivalent to Control + Alt + number hotkey. Sets control group to current selection. Units are removed from other control groups.
        SetAndSteal = 4,
        /// Equivalent to Shift + Alt + number hotkey. Adds current selection into control group. Units are removed from other control groups.
        AppendAndSteal = 5,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionSelectArmy {
    #[prost(bool, optional, tag = "1")]
    pub selection_add: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionSelectWarpGates {
    #[prost(bool, optional, tag = "1")]
    pub selection_add: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionSelectLarva {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionSelectIdleWorker {
    #[prost(enumeration = "action_select_idle_worker::Type", optional, tag = "1")]
    pub r#type: ::std::option::Option<i32>,
}
pub mod action_select_idle_worker {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Equivalent to click with no modifiers. Replaces selection with single idle worker.
        Set = 1,
        /// Equivalent to shift+click. Adds single idle worker to current selection.
        Add = 2,
        /// Equivalent to control+click. Selects all idle workers.
        All = 3,
        /// Equivalent to shift+control+click. Adds all idle workers to current selection.
        AddAll = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionMultiPanel {
    #[prost(enumeration = "action_multi_panel::Type", optional, tag = "1")]
    pub r#type: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub unit_index: ::std::option::Option<i32>,
}
pub mod action_multi_panel {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Click on icon
        SingleSelect = 1,
        /// Shift Click on icon
        DeselectUnit = 2,
        /// Control Click on icon.
        SelectAllOfType = 3,
        /// Control+Shift Click on icon.
        DeselectAllOfType = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionCargoPanelUnload {
    #[prost(int32, optional, tag = "1")]
    pub unit_index: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionProductionPanelRemoveFromQueue {
    #[prost(int32, optional, tag = "1")]
    pub unit_index: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionToggleAutocast {
    #[prost(int32, optional, tag = "1")]
    pub ability_id: ::std::option::Option<i32>,
}
//
// Notes:
//  Single player flow:
//    1) Call Request.create_game with a valid single player map (a multiplayer map will end right away).
//    2) Call Request.join_game, wait for the response.
//    3) Request.end will terminate the game. Observations can still be made.
//  Multi-player flow:
//    1) Launch two game instances with separate ports.
//    2) Designate a host, and Request.create_game with a multiplayer map.
//    3) Call Request.join on BOTH clients. Join will block until both clients connect.
//    4) Wait for a response from both clients. They can now play/step.
//    5) Steps should be syncronized. One client may time out if they are not. Multiple step sizes are ok.
//    4) Call Request.leave at any point or when the game ends. Observations will not be valid after this.
//
// States:
//
//------------------|---------------------------------------------------|-----------------------|
//  Request         | Valid in State                                    | Transition to State   |
//------------------|---------------------------------------------------|-----------------------|
// create_game      | launched                                          | init_game             |
//                  | ended (singleplayer only)                         | init_game             |
// join_game*       | init_game (singleplayer or multiplayer host only) | in_game               |
//                  | launched (multiplayer client only)                | in_game               |
// restart_game     | ended                                             | in_game               |
// start_replay     | launched                                          | in_replay             |
//                  | ended (singleplayer only)                         |                       |
// leave_game       | in_game (required when finishing multiplayer)     | launched              |
// quick_save       | in_game                                           |                       |
// quick_load       | in_game                                           |                       |
//                  | ended                                             |                       |
// quit             | any                                               | quit (not sent)       |
// game_info        | in_game                                           |                       |
//                  | in_replay                                         |                       |
//                  | ended                                             |                       |
// observation      | in_game                                           |                       |
//                  | in_replay                                         |                       |
//                  | ended                                             |                       |
// step*            | in_game (not available in realtime mode)          | in_game               |
//                  | in_replay                                         | ended                 |
// action           | in_game (not available to observers)              |                       |
// obs_action       | in_game (only for observers)                      |                       |
//                  | in_replay                                         |                       |
// data             | in_game                                           |                       |
//                  | in_replay                                         |                       |
//                  | ended                                             |                       |
// query            | in_game                                           |                       |
//                  | in_replay                                         |                       |
//                  | ended                                             |                       |
// save_replay      | in_game                                           |                       |
//                  | ended (only after a game)                         |                       |
// map_command      | in_game                                           |                       |
// replay_info      | any                                               |                       |
// available_maps   | any                                               |                       |
// save_map         | any                                               |                       |
// ping             | any                                               |                       |
// debug            | in_game                                           | various               |
//------------------|---------------------------------------------------|-----------------------|
//
// * In multiplayer, these require synchronization between clients.
//
// Notes:
//      - if a request fails, the game remains in the current state.
//

//
// Request/Response
//

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(uint32, optional, tag = "97")]
    pub id: ::std::option::Option<u32>,
    #[prost(
        oneof = "request::Request",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 21, 12, 13, 14, 15, 22, 16, 17, 18, 19, 20"
    )]
    pub request: ::std::option::Option<request::Request>,
}
pub mod request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Game Setup
        ///
        /// Send to host to initialize game.
        #[prost(message, tag = "1")]
        CreateGame(super::RequestCreateGame),
        /// Send to host and all clients for game to begin.
        #[prost(message, tag = "2")]
        JoinGame(super::RequestJoinGame),
        /// Single player only. Reinitializes the game with the same player setup.
        #[prost(message, tag = "3")]
        RestartGame(super::RequestRestartGame),
        /// Start playing a replay.
        #[prost(message, tag = "4")]
        StartReplay(super::RequestStartReplay),
        /// Multiplayer only. Disconnects from a multiplayer game, equivalent to surrender.
        #[prost(message, tag = "5")]
        LeaveGame(super::RequestLeaveGame),
        /// Saves game to an in-memory bookmark.
        #[prost(message, tag = "6")]
        QuickSave(super::RequestQuickSave),
        /// Loads from an in-memory bookmark.
        #[prost(message, tag = "7")]
        QuickLoad(super::RequestQuickLoad),
        /// Terminates the application.
        #[prost(message, tag = "8")]
        Quit(super::RequestQuit),
        /// During Game
        ///
        /// Static data about the current game and map.
        #[prost(message, tag = "9")]
        GameInfo(super::RequestGameInfo),
        /// Snapshot of the current game state.
        #[prost(message, tag = "10")]
        Observation(super::RequestObservation),
        /// Executes an action for a participant.
        #[prost(message, tag = "11")]
        Action(super::RequestAction),
        /// Executes an action for an observer.
        #[prost(message, tag = "21")]
        ObsAction(super::RequestObserverAction),
        /// Advances the game simulation.
        #[prost(message, tag = "12")]
        Step(super::RequestStep),
        /// Data about different gameplay elements. May be different for different games.
        #[prost(message, tag = "13")]
        Data(super::RequestData),
        /// Additional methods for inspecting game state.
        #[prost(message, tag = "14")]
        Query(super::RequestQuery),
        /// Generates a replay.
        #[prost(message, tag = "15")]
        SaveReplay(super::RequestSaveReplay),
        /// Execute a particular trigger through a string interface
        #[prost(message, tag = "22")]
        MapCommand(super::RequestMapCommand),
        /// Other.
        ///
        /// Returns metadata about a replay file. Does not load the replay.
        #[prost(message, tag = "16")]
        ReplayInfo(super::RequestReplayInfo),
        /// Returns directory of maps that can be played on.
        #[prost(message, tag = "17")]
        AvailableMaps(super::RequestAvailableMaps),
        /// Saves binary map data to the local temp directory.
        #[prost(message, tag = "18")]
        SaveMap(super::RequestSaveMap),
        /// Debugging
        ///
        /// Network ping for testing connection.
        #[prost(message, tag = "19")]
        Ping(super::RequestPing),
        /// Display debug information and execute debug actions.
        #[prost(message, tag = "20")]
        Debug(super::RequestDebug),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(uint32, optional, tag = "97")]
    pub id: ::std::option::Option<u32>,
    /// If command is missing, this will contain the error. Otherwise this will contain any warnings.
    #[prost(string, repeated, tag = "98")]
    pub error: ::std::vec::Vec<std::string::String>,
    /// Should be sent back with all responses.
    #[prost(enumeration = "Status", optional, tag = "99")]
    pub status: ::std::option::Option<i32>,
    #[prost(
        oneof = "response::Response",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 21, 12, 13, 14, 15, 16, 17, 18, 22, 19, 20"
    )]
    pub response: ::std::option::Option<response::Response>,
}
pub mod response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        CreateGame(super::ResponseCreateGame),
        #[prost(message, tag = "2")]
        JoinGame(super::ResponseJoinGame),
        #[prost(message, tag = "3")]
        RestartGame(super::ResponseRestartGame),
        #[prost(message, tag = "4")]
        StartReplay(super::ResponseStartReplay),
        #[prost(message, tag = "5")]
        LeaveGame(super::ResponseLeaveGame),
        #[prost(message, tag = "6")]
        QuickSave(super::ResponseQuickSave),
        #[prost(message, tag = "7")]
        QuickLoad(super::ResponseQuickLoad),
        #[prost(message, tag = "8")]
        Quit(super::ResponseQuit),
        #[prost(message, tag = "9")]
        GameInfo(super::ResponseGameInfo),
        #[prost(message, tag = "10")]
        Observation(super::ResponseObservation),
        #[prost(message, tag = "11")]
        Action(super::ResponseAction),
        #[prost(message, tag = "21")]
        ObsAction(super::ResponseObserverAction),
        #[prost(message, tag = "12")]
        Step(super::ResponseStep),
        #[prost(message, tag = "13")]
        Data(super::ResponseData),
        #[prost(message, tag = "14")]
        Query(super::ResponseQuery),
        #[prost(message, tag = "15")]
        SaveReplay(super::ResponseSaveReplay),
        #[prost(message, tag = "16")]
        ReplayInfo(super::ResponseReplayInfo),
        #[prost(message, tag = "17")]
        AvailableMaps(super::ResponseAvailableMaps),
        #[prost(message, tag = "18")]
        SaveMap(super::ResponseSaveMap),
        #[prost(message, tag = "22")]
        MapCommand(super::ResponseMapCommand),
        /// Debugging
        #[prost(message, tag = "19")]
        Ping(super::ResponsePing),
        #[prost(message, tag = "20")]
        Debug(super::ResponseDebug),
    }
}
///-----------------------------------------------------------------------------
/// If successful, puts the game into the status: init_game.
/// The next expected request should be RequestJoinGame. Can also quit (exit).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestCreateGame {
    #[prost(message, repeated, tag = "3")]
    pub player_setup: ::std::vec::Vec<PlayerSetup>,
    #[prost(bool, optional, tag = "4")]
    pub disable_fog: ::std::option::Option<bool>,
    /// Sets the pseudo-random seed for the game.
    #[prost(uint32, optional, tag = "5")]
    pub random_seed: ::std::option::Option<u32>,
    /// If set, the game plays in real time.
    #[prost(bool, optional, tag = "6")]
    pub realtime: ::std::option::Option<bool>,
    #[prost(oneof = "request_create_game::Map", tags = "1, 2")]
    pub map: ::std::option::Option<request_create_game::Map>,
}
pub mod request_create_game {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Map {
        /// Local .SC2Map file
        #[prost(message, tag = "1")]
        LocalMap(super::LocalMap),
        /// Map published to BattleNet
        #[prost(string, tag = "2")]
        BattlenetMapName(std::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalMap {
    /// A map can be specified either by a file path or the data of the .SC2Map file.
    /// If you provide both, it will play the game using map_data and store map_path
    /// into the replay. (260 character max)
    #[prost(string, optional, tag = "1")]
    pub map_path: ::std::option::Option<std::string::String>,
    #[prost(bytes, optional, tag = "7")]
    pub map_data: ::std::option::Option<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseCreateGame {
    #[prost(enumeration = "response_create_game::Error", optional, tag = "1")]
    pub error: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub error_details: ::std::option::Option<std::string::String>,
}
pub mod response_create_game {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Error {
        MissingMap = 1,
        InvalidMapPath = 2,
        InvalidMapData = 3,
        InvalidMapName = 4,
        InvalidMapHandle = 5,
        MissingPlayerSetup = 6,
        InvalidPlayerSetup = 7,
        /// Multiplayer is not supported in the current build.
        MultiplayerUnsupported = 8,
    }
}
///-----------------------------------------------------------------------------
/// If successful, puts the game into the status: in_game. Will be able to
/// request actions, observations and step the game.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestJoinGame {
    /// This is limited to what is specified in RequestCreateGame, but you can request less information if you want.
    #[prost(message, optional, tag = "3")]
    pub options: ::std::option::Option<InterfaceOptions>,
    /// Do not set in the single-player case. This is the port a server will use.
    #[prost(message, optional, tag = "4")]
    pub server_ports: ::std::option::Option<PortSet>,
    /// Do not set in the single-player case. These are the ports clients will use to initialize communication.
    #[prost(message, repeated, tag = "5")]
    pub client_ports: ::std::vec::Vec<PortSet>,
    /// Currently only a singe client is supported.
    ///
    /// deprecated
    #[prost(int32, optional, tag = "6")]
    pub shared_port: ::std::option::Option<i32>,
    /// Use this to set the player's name to something other than autogenerated name.
    #[prost(string, optional, tag = "7")]
    pub player_name: ::std::option::Option<std::string::String>,
    /// Both game creator and joiner should provide the ip address of the game creator in order to play remotely. Defaults to localhost.
    #[prost(string, optional, tag = "8")]
    pub host_ip: ::std::option::Option<std::string::String>,
    #[prost(oneof = "request_join_game::Participation", tags = "1, 2")]
    pub participation: ::std::option::Option<request_join_game::Participation>,
}
pub mod request_join_game {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Participation {
        /// Join as participant
        #[prost(enumeration = "super::Race", tag = "1")]
        Race(i32),
        /// Join as observer
        #[prost(uint32, tag = "2")]
        ObservedPlayerId(u32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortSet {
    /// Game right now needs two internal ports to establish a multiplay game on the local host.
    #[prost(int32, optional, tag = "1")]
    pub game_port: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub base_port: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseJoinGame {
    #[prost(uint32, optional, tag = "1")]
    pub player_id: ::std::option::Option<u32>,
    #[prost(enumeration = "response_join_game::Error", optional, tag = "2")]
    pub error: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub error_details: ::std::option::Option<std::string::String>,
}
pub mod response_join_game {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Error {
        MissingParticipation = 1,
        InvalidObservedPlayerId = 2,
        MissingOptions = 3,
        MissingPorts = 4,
        GameFull = 5,
        LaunchError = 6,
        /// Multiplayer specific.
        ///
        /// Multiplayer is not supported in the current build for the requested features.
        FeatureUnsupported = 7,
        NoSpaceForUser = 8,
        MapDoesNotExist = 9,
        CannotOpenMap = 10,
        ChecksumError = 11,
        NetworkError = 12,
        OtherError = 13,
    }
}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestRestartGame {}
/// The defaultRestartGameLoops is specified to be (1<<18) by default
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseRestartGame {
    #[prost(enumeration = "response_restart_game::Error", optional, tag = "1")]
    pub error: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub error_details: ::std::option::Option<std::string::String>,
    /// This will occur once the simulation_loop is greater then defaultRestartGameLoops
    #[prost(bool, optional, tag = "3")]
    pub need_hard_reset: ::std::option::Option<bool>,
}
pub mod response_restart_game {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Error {
        LaunchError = 1,
    }
}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestStartReplay {
    /// Overrides the map path stored in the replay.
    #[prost(bytes, optional, tag = "6")]
    pub map_data: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(int32, optional, tag = "2")]
    pub observed_player_id: ::std::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub options: ::std::option::Option<InterfaceOptions>,
    #[prost(bool, optional, tag = "4")]
    pub disable_fog: ::std::option::Option<bool>,
    #[prost(bool, optional, tag = "7")]
    pub realtime: ::std::option::Option<bool>,
    /// Allow RequestSaveReplay from a replay. Useful for truncating a replay, or restoring tracker.events.
    #[prost(bool, optional, tag = "8")]
    pub record_replay: ::std::option::Option<bool>,
    #[prost(oneof = "request_start_replay::Replay", tags = "1, 5")]
    pub replay: ::std::option::Option<request_start_replay::Replay>,
}
pub mod request_start_replay {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Replay {
        #[prost(string, tag = "1")]
        ReplayPath(std::string::String),
        #[prost(bytes, tag = "5")]
        ReplayData(std::vec::Vec<u8>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseStartReplay {
    #[prost(enumeration = "response_start_replay::Error", optional, tag = "1")]
    pub error: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub error_details: ::std::option::Option<std::string::String>,
}
pub mod response_start_replay {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Error {
        MissingReplay = 1,
        InvalidReplayPath = 2,
        InvalidReplayData = 3,
        InvalidMapData = 4,
        InvalidObservedPlayerId = 5,
        MissingOptions = 6,
        LaunchError = 7,
    }
}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMapCommand {
    #[prost(string, optional, tag = "1")]
    pub trigger_cmd: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMapCommand {
    #[prost(enumeration = "response_map_command::Error", optional, tag = "1")]
    pub error: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub error_details: ::std::option::Option<std::string::String>,
}
pub mod response_map_command {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Error {
        NoTriggerError = 1,
    }
}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestLeaveGame {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseLeaveGame {}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestQuickSave {}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseQuickSave {}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestQuickLoad {}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseQuickLoad {}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestQuit {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseQuit {}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestGameInfo {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseGameInfo {
    #[prost(string, optional, tag = "1")]
    pub map_name: ::std::option::Option<std::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub mod_names: ::std::vec::Vec<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub local_map_path: ::std::option::Option<std::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub player_info: ::std::vec::Vec<PlayerInfo>,
    /// Populated if Raw interface is enabled.
    #[prost(message, optional, tag = "4")]
    pub start_raw: ::std::option::Option<StartRaw>,
    #[prost(message, optional, tag = "5")]
    pub options: ::std::option::Option<InterfaceOptions>,
}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestObservation {
    #[prost(bool, optional, tag = "1")]
    pub disable_fog: ::std::option::Option<bool>,
    /// In realtime the request will only return once the simulation game loop has reached this value. When not realtime this value is ignored.
    #[prost(uint32, optional, tag = "2")]
    pub game_loop: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseObservation {
    /// Actions this player did since the last Observation.
    #[prost(message, repeated, tag = "1")]
    pub actions: ::std::vec::Vec<Action>,
    /// Equivalent of UI "red text" errors.
    #[prost(message, repeated, tag = "2")]
    pub action_errors: ::std::vec::Vec<ActionError>,
    #[prost(message, optional, tag = "3")]
    pub observation: ::std::option::Option<Observation>,
    /// Only populated if the game ended during this step.
    #[prost(message, repeated, tag = "4")]
    pub player_result: ::std::vec::Vec<PlayerResult>,
    #[prost(message, repeated, tag = "5")]
    pub chat: ::std::vec::Vec<ChatReceived>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatReceived {
    #[prost(uint32, optional, tag = "1")]
    pub player_id: ::std::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub message: ::std::option::Option<std::string::String>,
}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestAction {
    #[prost(message, repeated, tag = "1")]
    pub actions: ::std::vec::Vec<Action>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseAction {
    #[prost(enumeration = "ActionResult", repeated, packed = "false", tag = "1")]
    pub result: ::std::vec::Vec<i32>,
}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestObserverAction {
    #[prost(message, repeated, tag = "1")]
    pub actions: ::std::vec::Vec<ObserverAction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseObserverAction {}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestStep {
    /// Number of game loops to simulate for the next frame.
    #[prost(uint32, optional, tag = "1")]
    pub count: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseStep {
    ///  Max simulation_loop is (1<<19) before "end of time" will occur
    ///  The "end of time" is classified as the maximum number of game loops or absolute game time
    ///  representable as a positive fixed point number.
    ///  When we reach the "end of time", permanently pause the game and end the game for all.
    #[prost(uint32, optional, tag = "1")]
    pub simulation_loop: ::std::option::Option<u32>,
}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestData {
    #[prost(bool, optional, tag = "1")]
    pub ability_id: ::std::option::Option<bool>,
    #[prost(bool, optional, tag = "2")]
    pub unit_type_id: ::std::option::Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub upgrade_id: ::std::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub buff_id: ::std::option::Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub effect_id: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseData {
    #[prost(message, repeated, tag = "1")]
    pub abilities: ::std::vec::Vec<AbilityData>,
    #[prost(message, repeated, tag = "2")]
    pub units: ::std::vec::Vec<UnitTypeData>,
    #[prost(message, repeated, tag = "3")]
    pub upgrades: ::std::vec::Vec<UpgradeData>,
    #[prost(message, repeated, tag = "4")]
    pub buffs: ::std::vec::Vec<BuffData>,
    #[prost(message, repeated, tag = "5")]
    pub effects: ::std::vec::Vec<EffectData>,
}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestSaveReplay {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseSaveReplay {
    #[prost(bytes, optional, tag = "1")]
    pub data: ::std::option::Option<std::vec::Vec<u8>>,
}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestReplayInfo {
    /// Ensure the data and binary are downloaded if this is an old version replay.
    #[prost(bool, optional, tag = "3")]
    pub download_data: ::std::option::Option<bool>,
    #[prost(oneof = "request_replay_info::Replay", tags = "1, 2")]
    pub replay: ::std::option::Option<request_replay_info::Replay>,
}
pub mod request_replay_info {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Replay {
        /// Limitation: might fail if the replay file is currently loaded.
        #[prost(string, tag = "1")]
        ReplayPath(std::string::String),
        #[prost(bytes, tag = "2")]
        ReplayData(std::vec::Vec<u8>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerInfoExtra {
    #[prost(message, optional, tag = "1")]
    pub player_info: ::std::option::Option<PlayerInfo>,
    #[prost(message, optional, tag = "2")]
    pub player_result: ::std::option::Option<PlayerResult>,
    #[prost(int32, optional, tag = "3")]
    pub player_mmr: ::std::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub player_apm: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseReplayInfo {
    #[prost(string, optional, tag = "1")]
    pub map_name: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub local_map_path: ::std::option::Option<std::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub player_info: ::std::vec::Vec<PlayerInfoExtra>,
    #[prost(uint32, optional, tag = "4")]
    pub game_duration_loops: ::std::option::Option<u32>,
    #[prost(float, optional, tag = "5")]
    pub game_duration_seconds: ::std::option::Option<f32>,
    #[prost(string, optional, tag = "6")]
    pub game_version: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "11")]
    pub data_version: ::std::option::Option<std::string::String>,
    #[prost(uint32, optional, tag = "7")]
    pub data_build: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub base_build: ::std::option::Option<u32>,
    #[prost(enumeration = "response_replay_info::Error", optional, tag = "9")]
    pub error: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "10")]
    pub error_details: ::std::option::Option<std::string::String>,
}
pub mod response_replay_info {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Error {
        MissingReplay = 1,
        InvalidReplayPath = 2,
        InvalidReplayData = 3,
        ParsingError = 4,
        DownloadError = 5,
    }
}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestAvailableMaps {}
/// This will only contain locally cached BattleNet maps.
/// To download all ladder maps, log in and queue into a ladder match.
/// To download any other map, play a custom game on that map.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseAvailableMaps {
    /// All the maps in the "Maps/" directory.
    #[prost(string, repeated, tag = "1")]
    pub local_map_paths: ::std::vec::Vec<std::string::String>,
    /// All the maps in the BattleNet cache.
    #[prost(string, repeated, tag = "2")]
    pub battlenet_map_names: ::std::vec::Vec<std::string::String>,
}
///-----------------------------------------------------------------------------
/// Copies map data into the path specified.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestSaveMap {
    /// Path the game process will write to, relative to the temp directory. (260 character max)
    #[prost(string, optional, tag = "1")]
    pub map_path: ::std::option::Option<std::string::String>,
    /// Binary map data of a .SC2Map.
    #[prost(bytes, optional, tag = "2")]
    pub map_data: ::std::option::Option<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseSaveMap {
    #[prost(enumeration = "response_save_map::Error", optional, tag = "1")]
    pub error: ::std::option::Option<i32>,
}
pub mod response_save_map {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Error {
        InvalidMapData = 1,
    }
}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPing {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponsePing {
    #[prost(string, optional, tag = "1")]
    pub game_version: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag = "2")]
    pub data_version: ::std::option::Option<std::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub data_build: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub base_build: ::std::option::Option<u32>,
}
///-----------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDebug {
    #[prost(message, repeated, tag = "1")]
    pub debug: ::std::vec::Vec<DebugCommand>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseDebug {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerSetup {
    #[prost(enumeration = "PlayerType", optional, tag = "1")]
    pub r#type: ::std::option::Option<i32>,
    /// Only used for a computer player.
    #[prost(enumeration = "Race", optional, tag = "2")]
    pub race: ::std::option::Option<i32>,
    #[prost(enumeration = "Difficulty", optional, tag = "3")]
    pub difficulty: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub player_name: ::std::option::Option<std::string::String>,
    #[prost(enumeration = "AiBuild", optional, tag = "5")]
    pub ai_build: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpatialCameraSetup {
    #[prost(message, optional, tag = "2")]
    pub resolution: ::std::option::Option<Size2Di>,
    #[prost(message, optional, tag = "3")]
    pub minimap_resolution: ::std::option::Option<Size2Di>,
    /// Below are only relevant for feature layers.
    ///
    /// Set the screen camera width in world units.
    #[prost(float, optional, tag = "1")]
    pub width: ::std::option::Option<f32>,
    /// Crop minimap to the playable area.
    #[prost(bool, optional, tag = "4")]
    pub crop_to_playable_area: ::std::option::Option<bool>,
    /// Return unit_type on the minimap, and potentially other cheating layers.
    #[prost(bool, optional, tag = "5")]
    pub allow_cheating_layers: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfaceOptions {
    /// Interface options
    #[prost(bool, optional, tag = "1")]
    pub raw: ::std::option::Option<bool>,
    #[prost(bool, optional, tag = "2")]
    pub score: ::std::option::Option<bool>,
    /// Omit to disable.
    #[prost(message, optional, tag = "3")]
    pub feature_layer: ::std::option::Option<SpatialCameraSetup>,
    /// Omit to disable.
    #[prost(message, optional, tag = "4")]
    pub render: ::std::option::Option<SpatialCameraSetup>,
    /// By default cloaked units are completely hidden. This shows some details.
    #[prost(bool, optional, tag = "5")]
    pub show_cloaked: ::std::option::Option<bool>,
    /// By default burrowed units are completely hidden. This shows some details for those that produce a shadow.
    #[prost(bool, optional, tag = "9")]
    pub show_burrowed_shadows: ::std::option::Option<bool>,
    /// Return placeholder units (buildings to be constructed), both for raw and feature layers.
    #[prost(bool, optional, tag = "8")]
    pub show_placeholders: ::std::option::Option<bool>,
    /// By default raw actions select, act and revert the selection. This is useful
    /// if you're playing simultaneously with the agent so it doesn't steal your
    /// selection. This inflates APM (due to deselect) and makes the actions hard
    /// to follow in a replay. Setting this to true will cause raw actions to do
    /// select, act, but not revert the selection.
    #[prost(bool, optional, tag = "6")]
    pub raw_affects_selection: ::std::option::Option<bool>,
    /// Changes the coordinates in raw.proto to be relative to the playable area.
    /// The map_size and playable_area will be the diagonal of the real playable area.
    #[prost(bool, optional, tag = "7")]
    pub raw_crop_to_playable_area: ::std::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerInfo {
    /// Identifier that will be used to reference this player.
    /// SC2 will always assign playerIds starting from 1 in standard Melee maps. This may not be true in custom maps.
    #[prost(uint32, optional, tag = "1")]
    pub player_id: ::std::option::Option<u32>,
    #[prost(enumeration = "PlayerType", optional, tag = "2")]
    pub r#type: ::std::option::Option<i32>,
    #[prost(enumeration = "Race", optional, tag = "3")]
    pub race_requested: ::std::option::Option<i32>,
    /// Only populated for your player or when watching replay
    #[prost(enumeration = "Race", optional, tag = "4")]
    pub race_actual: ::std::option::Option<i32>,
    #[prost(enumeration = "Difficulty", optional, tag = "5")]
    pub difficulty: ::std::option::Option<i32>,
    #[prost(enumeration = "AiBuild", optional, tag = "7")]
    pub ai_build: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "6")]
    pub player_name: ::std::option::Option<std::string::String>,
}
//
// During Game
//

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerCommon {
    #[prost(uint32, optional, tag = "1")]
    pub player_id: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub minerals: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub vespene: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub food_cap: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub food_used: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub food_army: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub food_workers: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub idle_worker_count: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub army_count: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub warp_gate_count: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub larva_count: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Observation {
    #[prost(uint32, optional, tag = "9")]
    pub game_loop: ::std::option::Option<u32>,
    #[prost(message, optional, tag = "1")]
    pub player_common: ::std::option::Option<PlayerCommon>,
    #[prost(enumeration = "Alert", repeated, packed = "false", tag = "10")]
    pub alerts: ::std::vec::Vec<i32>,
    /// Abilities available in the selection. Enabled if in this list, disabled otherwise.
    #[prost(message, repeated, tag = "3")]
    pub abilities: ::std::vec::Vec<AvailableAbility>,
    #[prost(message, optional, tag = "4")]
    pub score: ::std::option::Option<Score>,
    /// Populated if Raw interface is enabled.
    #[prost(message, optional, tag = "5")]
    pub raw_data: ::std::option::Option<ObservationRaw>,
    /// Populated if Feature Layer interface is enabled.
    #[prost(message, optional, tag = "6")]
    pub feature_layer_data: ::std::option::Option<ObservationFeatureLayer>,
    /// Populated if Render interface is enabled.
    #[prost(message, optional, tag = "7")]
    pub render_data: ::std::option::Option<ObservationRender>,
    /// Populated if Feature Layer or Render interface is enabled.
    #[prost(message, optional, tag = "8")]
    pub ui_data: ::std::option::Option<ObservationUi>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    /// Populated if Raw interface is enabled.
    #[prost(message, optional, tag = "1")]
    pub action_raw: ::std::option::Option<ActionRaw>,
    /// Populated if Feature Layer interface is enabled.
    #[prost(message, optional, tag = "2")]
    pub action_feature_layer: ::std::option::Option<ActionSpatial>,
    /// Not implemented. Populated if Render interface is enabled.
    #[prost(message, optional, tag = "3")]
    pub action_render: ::std::option::Option<ActionSpatial>,
    /// Populated if Feature Layer or Render interface is enabled.
    #[prost(message, optional, tag = "4")]
    pub action_ui: ::std::option::Option<ActionUi>,
    /// Chat messages as a player typing into the chat channel.
    #[prost(message, optional, tag = "6")]
    pub action_chat: ::std::option::Option<ActionChat>,
    /// Populated for actions in ResponseObservation. The game loop on which the action was executed.
    #[prost(uint32, optional, tag = "7")]
    pub game_loop: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionChat {
    #[prost(enumeration = "action_chat::Channel", optional, tag = "1")]
    pub channel: ::std::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub message: ::std::option::Option<std::string::String>,
}
pub mod action_chat {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Channel {
        Broadcast = 1,
        Team = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionError {
    /// Only populated when using raw interface.
    #[prost(uint64, optional, tag = "1")]
    pub unit_tag: ::std::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub ability_id: ::std::option::Option<u64>,
    #[prost(enumeration = "ActionResult", optional, tag = "3")]
    pub result: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObserverAction {
    #[prost(oneof = "observer_action::Action", tags = "1, 2, 3, 4")]
    pub action: ::std::option::Option<observer_action::Action>,
}
pub mod observer_action {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        /// Not implemented
        #[prost(message, tag = "1")]
        PlayerPerspective(super::ActionObserverPlayerPerspective),
        #[prost(message, tag = "2")]
        CameraMove(super::ActionObserverCameraMove),
        #[prost(message, tag = "3")]
        CameraFollowPlayer(super::ActionObserverCameraFollowPlayer),
        /// Not implemented
        #[prost(message, tag = "4")]
        CameraFollowUnits(super::ActionObserverCameraFollowUnits),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionObserverPlayerPerspective {
    /// 0 to observe "Everyone"
    #[prost(uint32, optional, tag = "1")]
    pub player_id: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionObserverCameraMove {
    #[prost(message, optional, tag = "1")]
    pub world_pos: ::std::option::Option<Point2D>,
    /// Distance between camera and terrain. Larger value zooms out camera.
    /// Defaults to standard camera distance if set to 0.
    #[prost(float, optional, tag = "2")]
    pub distance: ::std::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionObserverCameraFollowPlayer {
    /// Not implemented. Value must be [1, 15]
    #[prost(uint32, optional, tag = "1")]
    pub player_id: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionObserverCameraFollowUnits {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub unit_tags: ::std::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerResult {
    #[prost(uint32, optional, tag = "1")]
    pub player_id: ::std::option::Option<u32>,
    #[prost(enumeration = "Result", optional, tag = "2")]
    pub result: ::std::option::Option<i32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    /// Game has been launch and is not yet doing anything.
    Launched = 1,
    /// Create game has been called, and the host is awaiting players.
    InitGame = 2,
    /// In a single or multiplayer game.
    InGame = 3,
    /// In a replay.
    InReplay = 4,
    /// Game has ended, can still request game info, but ready for a new game.
    Ended = 5,
    /// Application is shutting down.
    Quit = 6,
    /// Should not happen, but indicates an error if it occurs.
    Unknown = 99,
}
//
// Game Setup
//

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Difficulty {
    VeryEasy = 1,
    Easy = 2,
    Medium = 3,
    MediumHard = 4,
    Hard = 5,
    Harder = 6,
    VeryHard = 7,
    CheatVision = 8,
    CheatMoney = 9,
    CheatInsane = 10,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayerType {
    Participant = 1,
    Computer = 2,
    Observer = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AiBuild {
    RandomBuild = 1,
    Rush = 2,
    Timing = 3,
    Power = 4,
    Macro = 5,
    Air = 6,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Alert {
    Error = 3,
    AddOnComplete = 4,
    BuildingComplete = 5,
    BuildingUnderAttack = 6,
    LarvaHatched = 7,
    MergeComplete = 8,
    MineralsExhausted = 9,
    MorphComplete = 10,
    MothershipComplete = 11,
    MuleExpired = 12,
    NuclearLaunchDetected = 1,
    NukeComplete = 13,
    NydusWormDetected = 2,
    ResearchComplete = 14,
    TrainError = 15,
    TrainUnitComplete = 16,
    TrainWorkerComplete = 17,
    TransformationComplete = 18,
    UnitUnderAttack = 19,
    UpgradeComplete = 20,
    VespeneExhausted = 21,
    WarpInComplete = 22,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Result {
    Victory = 1,
    Defeat = 2,
    Tie = 3,
    Undecided = 4,
}
