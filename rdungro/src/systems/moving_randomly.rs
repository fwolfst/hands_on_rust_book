use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
pub fn moving_randomly(world: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    movers
        .iter(world)
        .for_each(|(entity, pos, _)| {
            let mut rng = RandomNumberGenerator::new();
            let dest = match rng.range(0,4) {
                0 => Point::new(-1,0),
                1 => Point::new(1,0),
                2 => Point::new(0,-1),
                _ => Point::new(0,1),
            } + *pos;
            commands.push( ((), WantsToMove { entity: *entity, destination: dest } ) );
        });
}
