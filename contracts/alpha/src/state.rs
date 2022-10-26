use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, PartialEq, Debug, JsonSchema)]
pub struct Config {
	pub owner: Addr, 
	pub message: String
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct Reply {
    pub text: String
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const REPLIES: Map<&Addr, Reply> = Map::new("rep");
