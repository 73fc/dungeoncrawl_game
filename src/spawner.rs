use crate::prelude::*;

pub fn spawn_player(esc: &mut World, pos: Point) {
    esc.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health { current: 3, max: 3 },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    let test = rng.range(0, 9) % 2;
    if test == 0 {
        ecs.push((
            Enemy,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph,
            },
            MovingRandomly {},
            Health {
                current: hp,
                max: hp,
            },
            Name(name),
        ));
    } else {
        ecs.push((
            Enemy,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph,
            },
            ChasingPlayer {},
            Health {
                current: hp,
                max: hp,
            },
            Name(name),
        ));
    }
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}
