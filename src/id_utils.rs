// ID type is u16, so I use lower order 16 bits of u32
pub mod Const {
    pub const ID_MASK: u32 = 0xFFFF;
    pub const ROOT_MASK: u16 = 0x0000_0001;
    pub const ROOT_SHIFT: u16 = 0;
    pub const LOCALNET_LOCATION_MASK: u16 = 0b0000_0000_0000_0110;
    pub const LOCALNET_LOCATION_SHIFT: u16 = 1;
    pub const LOCALNET_ID_MASK: u16 = 0b1111_1111_1111_1000;
    pub const LOCALNET_ID_SHIFT: u16 = 3;
    pub const MAC_ADDRESS_MASK: u16 = 0b1111_1111_1111_1111;
    pub const MAC_ADDRESS_SHIFT: u16 = 0;

    pub const ROOT: u16 = 0x00000001;
    pub const LOCALNET_UPLEFT: u16 = 0x00000000;
    pub const LOCALNET_UPRIGHT: u16 = 0x00000002;
    pub const LOCALNET_DOWNLEFT: u16 = 0x00000004;
    pub const LOCALNET_DOWNRIGHT: u16 = 0x00000006;
}
pub mod TypeAlias {
    pub type Id = u16;
    pub type Coordinate = (i16, i16);
    pub type CoordinateComponent = i16;
}

pub mod Util {
    use super::Const::*;
    use super::TypeAlias::*;
    use crate::network::localnet::LocalNetworkLocation;
    pub fn get_raw_localnet_id(id: Id) -> u16 {
        id & LOCALNET_ID_MASK
    }
    pub fn get_raw_mac_address(id: Id) -> u16 {
        id & MAC_ADDRESS_MASK
    }
    pub fn get_raw_localnet_location(id: Id) -> u16 {
        id & LOCALNET_LOCATION_MASK
    }
    pub fn get_raw_root(id: Id) -> u16 {
        id & ROOT_MASK
    }

    // convenient value of efuse
    pub fn get_localnet_id(id: Id) -> u16 {
        (id & LOCALNET_ID_MASK) >> LOCALNET_ID_SHIFT
    }
    pub fn get_mac_address(id: Id) -> u16 {
        (id & MAC_ADDRESS_MASK) >> MAC_ADDRESS_SHIFT
    }
    pub fn get_localnet_location(id: Id) -> LocalNetworkLocation {
        // (id & LOCALNET_LOCATION_MASK) >> LOCALNET_LOCATION_SHIFT
        match get_raw_localnet_location(id) {
            LOCALNET_UPLEFT => LocalNetworkLocation::UpLeft,
            LOCALNET_UPRIGHT => LocalNetworkLocation::UpRight,
            LOCALNET_DOWNLEFT => LocalNetworkLocation::DownLeft,
            LOCALNET_DOWNRIGHT => LocalNetworkLocation::DownRight,
            _ => panic!(
                "Invalid localnet: localnet is less than 5, but {:?},
                raw_localnet_location: {},
                mac_address: {}",
                get_localnet_location(id),
                get_raw_localnet_location(id),
                get_mac_address(id)
            ),
        }
    }
    pub fn is_root(id: Id) -> bool {
        (id & ROOT_MASK) >> ROOT_SHIFT == ROOT
    }
}
