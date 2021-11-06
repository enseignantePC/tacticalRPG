use super::*;

impl Map {
    pub fn test_map() -> Self {
        todo!()
    }
}

impl UnInitializedMap {
    fn test_imap() -> Self {
        let uninitialized = UnInitializedMap::new();
        let id0 = uninitialized.declare_terrain(
            "Dirt",
            TerrainType::EntityMayCross,
        );
        let id1 = uninitialized.declare_terrain(
            "Water",
            TerrainType::AttackMayCross,
        );
        let id2 = uninitialized.declare_terrain(
            "Wall",
            TerrainType::ImpossibleToCross,
        );
        uninitialized
    }
}

#[test]
fn test_new_map() {
    let m = Map::new(2, 2);
    dbg!(m);
}

#[test]
fn test_add_entity() {
    let mut m = Map::new(2, 2);
    m.register_entity_at_pos(
        Rc::new(Entity::test_entity(
            None, None,
        )),
        &Pos2D::new(0, 0),
    )
    .unwrap();
}

#[test]
#[should_panic]
fn test_cant_add_entity_if_occupied() {
    let mut m = Map::new(2, 2);
    m.register_entity_at_pos(
        Rc::new(Entity::test_entity(
            None, None,
        )),
        &Pos2D::new(0, 0),
    )
    .unwrap();
    m.register_entity_at_pos(
        Rc::new(Entity::test_entity(
            Some(1),
            None,
        )),
        &Pos2D::new(0, 0),
    )
    .unwrap();
}

#[test]
fn test_get_pos_entity() {
    let mut m = Map::new(2, 2);
    m.register_entity_at_pos(
        Rc::new(Entity::test_entity(
            None,
            Some(0),
        )),
        &Pos2D::new(0, 0),
    )
    .unwrap();
    m.register_entity_at_pos(
        Rc::new(Entity::test_entity(
            None,
            Some(1),
        )),
        &Pos2D::new(1, 0),
    )
    .unwrap();
    assert_eq!(
        Some(Pos2D::new(0, 0)),
        m.get_pos_for_entity(EntityId(0))
    );
    assert_eq!(
        Some(Pos2D::new(1, 0)),
        m.get_pos_for_entity(EntityId(1))
    );
    assert_eq!(
        None,
        m.get_pos_for_entity(EntityId(2))
    )
}

#[test]
fn test_move_entity() {
    let mut m = Map::new(2, 2);
    let x = Rc::new(Entity::test_entity(
        None, None,
    ));

    m.register_entity_at_pos(
        x.clone(),
        &Pos2D::new(0, 0),
    )
    .unwrap();
    assert_eq!(
        m.get_pos_for_entity(x.unique_id),
        Some(Pos2D::new(0, 0))
    );
    m.move_entity_from_current_position_to_next_position(
        x.clone(),
        Pos2D::new(0, 1),
    )
    .unwrap();
    assert_eq!(
        m.get_pos_for_entity(x.unique_id),
        Some(Pos2D::new(0, 1))
    );
}

#[test]
fn test_cant_move_entity_in_occupied_pos() {
    let mut m = Map::new(2, 2);
    let x = Rc::new(Entity::test_entity(
        None,
        Some(0),
    ));
    m.register_entity_at_pos(
        x.clone(),
        &Pos2D::new(0, 0),
    )
    .unwrap();
    m.register_entity_at_pos(
        Rc::new(Entity::test_entity(
            None,
            Some(1),
        )),
        &Pos2D::new(1, 0),
    )
    .unwrap();
    let e = m.move_entity_from_current_position_to_next_position(x, Pos2D::new(1, 0));
    assert!(e.is_err());
}
