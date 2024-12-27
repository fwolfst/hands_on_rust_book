use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn moving_randomly(world: &mut SubWorld, #[resource] map: &Map) {
    let mut movers = <(&mut Point, &MovingRandomly)>::query();
    movers
        .iter_mut(world)
        .for_each(|(pos, _)| {
            let mut rng = RandomNumberGenerator::new();
            let dest = match rng.range(0,4) {
                0 => Point::new(-1,0),
                1 => Point::new(1,0),
                2 => Point::new(0,-1),
                _ => Point::new(0,1),
            } + *pos;
            if map.can_enter(dest) {
                *pos = dest;
            }
        });
}
