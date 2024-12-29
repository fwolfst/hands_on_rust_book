use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player,
            Health {
                max: 20,
                current: 20,
            },
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@'),
            }
        )
        );
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Gobbelin".to_string(), to_cp437('g'))
}

fn eyething() -> (i32, String, FontCharType) {
    (2, "Eyething".to_string(), to_cp437('E'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

fn orc2() -> (i32, String, FontCharType) {
    (2, "Orc2".to_string(), to_cp437('O'))
}
//                1 => to_cp437('O'),

pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    pos: Point
    ) {
    let (hp, name, glyph) = match rng.roll_dice(1,10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push(
        (Enemy,
         MovingRandomly,
         pos,
         Health { max: hp, current: hp},
         Name(name),
         Render { 
            color: ColorPair::new(WHITE, BLACK),
            glyph
            }
        )
    );
}
