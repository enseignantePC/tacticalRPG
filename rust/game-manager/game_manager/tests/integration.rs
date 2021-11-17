use game_manager;

fn new_manager() {
    let mut builder = todo!();
    let id_dirt = builder.with_terrain_crossable("Dirt");
    let id_water = builder.with_terrain_attack_crossable("Water");
    let id_wall = builder.with_terrain_crossable("Wall");
    builder.with_map(todo!());
    builder.build()
}

#[test]
fn build_manager() {
    new_manager()
}

#[test]
fn simple_moving_around() {
    let gm = new_manager();
    todo!()
}

#[test]
fn impl_counter_attacks() {
    let gm = new_manager();
    todo!()
}
