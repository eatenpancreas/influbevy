
use bevy::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::{Grid};

type UPos = (u16, u16);

#[derive(Component)]
pub struct Owner {
    pub col: Color,
    pub starting_pos: UPos,
    pub positions: Vec<UPos>
}

pub fn setup(
    mut commands: Commands,
    grid: Res<Grid>,
) {
    let mut rng = rand::thread_rng();
    let taken_positions = vec![];
    
    for _ in 0..8 {
        let pos = (
            rng.gen_range(0..grid.0.width),
            rng.gen_range(0..grid.0.height)
        );
        
        if taken_positions.contains(&pos) { continue; }
        
        commands.spawn((
            Owner { col: random_color(&mut rng), starting_pos: pos, positions: vec![pos] }
        ));
    }
}

fn random_color(rng: &mut ThreadRng) -> Color {
    Color::rgb(rng.gen_range(0.2..0.95), rng.gen_range(0.2..0.95), rng.gen_range(0.2..0.95))
}