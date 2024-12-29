use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltip(
    ecs: &SubWorld,
    #[resource] mousepos: &Point,
    #[resource] camera: &Camera
) {
    let mut dudes = <(Entity, &Point, &Name)>::query();
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mousepos + offset;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(RenderLayer::HUD as usize);

    dudes
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos)
        .for_each(|(e, _, name)| {
            let screen_pos = *mousepos * 4;
            let display = if let Ok(health) = ecs.entry_ref(*e)
                .unwrap()
                .get_component::<Health>() {
                    format!("{} : {} hp", &name.0, health.current)
                }
            else { name.0.clone() };
            draw_batch.print(screen_pos, &display);

        })
        ;
//    for (_, pos, name) in dudes.iter(ecs) {
//        draw_batch.print((*pos - offset)* 4, format!("{}", name.0));
//    }

    draw_batch.submit(8000);
}
