#![allow(unused_imports)]
#![allow(unreachable_code)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::all)]

use dijkstra_map::DijkstraMap;
use std::collections::HashMap;

pub mod common_types;
pub mod input_manager;
pub mod manager;
pub mod map;

#[cfg(test)]
pub mod test_impl;

pub mod watcher;
