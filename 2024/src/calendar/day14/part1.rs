use crate::common::types::{Robot, Position};

pub fn result(robots: &mut Vec<Robot>, map_size: (usize, usize)) -> isize {
    // simulate robots moving over x seconds while wrapping around the edges
    // count robots in each quadrant while ignoring those directly in the middle (H or V)
    // score per each quadrant is the sum of the robots in that quadrant
    // final score is the multiplication of the scores of all quadrants
    const SECONDS: usize = 100;
    let mut quadrant_scores = [0,0,0,0];

    for _ in 0..SECONDS {
        for robot in robots.iter_mut() {
            robot.update_position(map_size);
        }
    }

    for robot in robots.iter() {
        let Position(x,y) = robot.get_position();
        let (map_x, map_y) = map_size;

        match (x,y) {
            (x,y) if x < map_x / 2 && y < map_y / 2 => quadrant_scores[0] += 1,
            (x,y) if x > map_x / 2 && y < map_y / 2 => quadrant_scores[1] += 1,
            (x,y) if x < map_x / 2 && y > map_y / 2 => quadrant_scores[2] += 1,
            (x,y) if x > map_x / 2 && y > map_y / 2 => quadrant_scores[3] += 1,
            _ => ()
        }
        println!("{:?}, Quadrant scores: {:?}", robot.get_position(), quadrant_scores);
    }

    quadrant_scores[0] * quadrant_scores[1] * quadrant_scores[2] * quadrant_scores[3]
    
}

