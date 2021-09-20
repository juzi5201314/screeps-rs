use std::convert::TryFrom;

macro_rules! convert_js_to_c_enum {
    ($(#[$attr:meta])* $name:ident, {
        $($field:ident : $val:expr),*
    }) => {
        $(#[$attr])*
        #[allow(non_camel_case_types)]
        pub enum $name {
            $($field = $val),*
        }
    };
}

macro_rules! convert_js_to_enum {
    ($(#[$attr:meta])* $name:ident : $ty:ty, {
        $($field:ident : $val:expr),*
    }) => {
        $(#[$attr])*
        #[allow(non_camel_case_types)]
        pub enum $name {
            $($field),*
        }

        impl From<$name> for $ty {
            fn from(val: $name) -> Self {
                match val {
                    $($name::$field => $val),*
                }
            }
        }

        impl TryFrom<$ty> for $name {
            type Error = ();

            fn try_from(val: $ty) -> Result<Self, Self::Error> {
                match val {
                    $(#[allow(unreachable_patterns)] $val => Ok($name::$field)),*,
                    _ => Err(())
                }
            }
        }
    };
}

macro_rules! convert_js_to_const {
    ($ty:ty, {
        $($field:ident : $val:expr),*
    }) => {
        $(#[allow(unused)] const $field: $ty = $val;)*
    };
}

macro_rules! const_and_enum {
    ($(#[$attr:meta])* $name:ident : $ty:ty, {
        $($field:ident : $val:expr),*
    }) => {
        convert_js_to_const!($ty, { $($field : $val),* });
        convert_js_to_enum!($(#[$attr])* $name : $ty, { $($field : $val),* });
    };
}

const_and_enum!(
    #[derive(Eq, PartialEq, Debug)]
    ResCode: i8, {
        OK: 0,
        ERR_NOT_OWNER: -1,
        ERR_NO_PATH: -2,
        ERR_NAME_EXISTS: -3,
        ERR_BUSY: -4,
        ERR_NOT_FOUND: -5,
        ERR_NOT_ENOUGH_ENERGY: -6,
        ERR_NOT_ENOUGH_RESOURCES: -6,
        ERR_INVALID_TARGET: -7,
        ERR_FULL: -8,
        ERR_NOT_IN_RANGE: -9,
        ERR_INVALID_ARGS: -10,
        ERR_TIRED: -11,
        ERR_NO_BODYPART: -12,
        ERR_NOT_ENOUGH_EXTENSIONS: -6,
        ERR_RCL_NOT_ENOUGH: -14,
        ERR_GCL_NOT_ENOUGH: -15
    }
);

convert_js_to_c_enum!(
    #[derive(Eq, PartialEq, Debug)]
    #[repr(u8)]
    Find, {
        FIND_EXIT_TOP: 1,
        FIND_EXIT_RIGHT: 3,
        FIND_EXIT_BOTTOM: 5,
        FIND_EXIT_LEFT: 7,
        FIND_EXIT: 10,
        FIND_CREEPS: 101,
        FIND_MY_CREEPS: 102,
        FIND_HOSTILE_CREEPS: 103,
        FIND_SOURCES_ACTIVE: 104,
        FIND_SOURCES: 105,
        FIND_DROPPED_RESOURCES: 106,
        FIND_STRUCTURES: 107,
        FIND_MY_STRUCTURES: 108,
        FIND_HOSTILE_STRUCTURES: 109,
        FIND_FLAGS: 110,
        FIND_CONSTRUCTION_SITES: 111,
        FIND_MY_SPAWNS: 112,
        FIND_HOSTILE_SPAWNS: 113,
        FIND_MY_CONSTRUCTION_SITES: 114,
        FIND_HOSTILE_CONSTRUCTION_SITES: 115,
        FIND_MINERALS: 116,
        FIND_NUKES: 117,
        FIND_TOMBSTONES: 118,
        FIND_POWER_CREEPS: 119,
        FIND_MY_POWER_CREEPS: 120,
        FIND_HOSTILE_POWER_CREEPS: 121,
        FIND_DEPOSITS: 122,
        FIND_RUINS: 123
    }
);

convert_js_to_c_enum!(
    #[derive(Eq, PartialEq, Debug)]
    #[repr(u8)]
    Direction, {
        TOP: 1,
        TOP_RIGHT: 2,
        RIGHT: 3,
        BOTTOM_RIGHT: 4,
        BOTTOM: 5,
        BOTTOM_LEFT: 6,
        LEFT: 7,
        TOP_LEFT: 8
    }
);

convert_js_to_c_enum!(
    #[derive(Eq, PartialEq, Debug)]
    #[repr(u8)]
    Colors, {
        COLOR_RED: 1,
        COLOR_PURPLE: 2,
        COLOR_BLUE: 3,
        COLOR_CYAN: 4,
        COLOR_GREEN: 5,
        COLOR_YELLOW: 6,
        COLOR_ORANGE: 7,
        COLOR_BROWN: 8,
        COLOR_GREY: 9,
        COLOR_WHITE: 10
    }
);

convert_js_to_enum!(
    #[derive(Eq, PartialEq, Debug)]
    Look: &'static str, {
        LOOK_CREEPS: "creep",
        LOOK_ENERGY: "energy",
        LOOK_RESOURCES: "resource",
        LOOK_SOURCES: "source",
        LOOK_MINERALS: "mineral",
        LOOK_DEPOSITS: "deposit",
        LOOK_STRUCTURES: "structure",
        LOOK_FLAGS: "flag",
        LOOK_CONSTRUCTION_SITES: "constructionSite",
        LOOK_NUKES: "nuke",
        LOOK_TERRAIN: "terrain",
        LOOK_TOMBSTONES: "tombstone",
        LOOK_POWER_CREEPS: "powerCreep",
        LOOK_RUINS: "ruin"
    }
);

#[allow(unused)]
const OBSTACLE_OBJECT_TYPES: [&'static str; 20] = [
    "spawn",
    "creep",
    "powerCreep",
    "source",
    "mineral",
    "deposit",
    "controller",
    "constructedWall",
    "extension",
    "link",
    "storage",
    "tower",
    "observer",
    "powerSpawn",
    "powerBank",
    "lab",
    "terminal",
    "nuker",
    "factory",
    "invaderCore",
];

convert_js_to_enum!(
    #[derive(Eq, PartialEq, Debug)]
    BodyPart: &'static str, {
        MOVE: "move",
        WORK: "work",
        CARRY: "carry",
        ATTACK: "attack",
        RANGED_ATTACK: "ranged_attack",
        TOUGH: "tough",
        HEAL: "heal",
        CLAIM: "claim"
    }
);

convert_js_to_const!(u16, {
    MOVE: 50,
    WORK: 100,
    ATTACK: 80,
    CARRY: 50,
    HEAL: 250,
    RANGED_ATTACK: 150,
    TOUGH: 10,
    CLAIM: 600
});

#[test]
fn test_eq() {
    assert_eq!(ERR_NOT_ENOUGH_RESOURCES, -6);
    assert_eq!(ResCode::try_from(ERR_FULL).unwrap(), ResCode::ERR_FULL);
    assert_eq!(Find::FIND_NUKES as u8, 117);
    assert_eq!(Into::<&'static str>::into(Look::LOOK_CREEPS), "creep");
}
