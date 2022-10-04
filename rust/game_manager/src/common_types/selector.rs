use super::{EntityId, TeamId};

#[derive(Debug, Clone)]
pub struct Selector {
    pub mode: SelectorMode,
    // pub filter: SelectorFilter,
}

#[derive(Debug, Clone)]
pub enum SelectorMode {
    Djikstra { move_force: f32 },
}

#[derive(Debug, Clone)]
pub struct SelectorResult;

// #[derive(Debug, Clone)]
// pub struct SelectorFilter {
//     filters: Vec<SFilter>,
// }
// impl SelectorFilter {
//     pub fn filter() {}
// }

// #[derive(Debug, Clone)]
// pub enum SFilter {
//     RefuseAll,
//     AcceptAllEntity,
//     AcceptEntity(EntityId),
//     RefuseEntity(EntityId),
//     AcceptEmptyPos,
//     AcceptEntityOfTeam(TeamId),
// }

// impl SFilter {
//     pub fn filter(
//         &self,
//         entity: EntityId,
//         entity_team: TeamId,
//     ) -> bool {
//         match self {
//             SFilter::RefuseAll => todo!(),
//             SFilter::AcceptEmptyPos => todo!(),
//             SFilter::AcceptAllEntity => todo!(),
//             SFilter::AcceptEntity(id) => todo!(),
//             SFilter::RefuseEntity(id) => todo!(),
//             SFilter::AcceptEntityOfTeam(tid) => todo!(),
//         }
//     }
// }
