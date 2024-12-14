use std::time::Duration;

#[derive(Debug, PartialEq)]
struct Robot {
    x: usize,
    y: usize,
    vx: isize,
    vy: isize,
}

#[derive(Debug)]
struct Submap {
    x_left: usize,
    y_top: usize,
    x_right: usize,
    y_bot: usize,
}

fn parse_input(s: &str) -> Vec<Robot> {
    s.lines()
        .map(|line| {
            let (p, v) = line.split_once(" ").unwrap();
            let (px, py) = p.trim_start_matches("p=").split_once(',').unwrap();
            let (vx, vy) = v.trim_start_matches("v=").split_once(',').unwrap();
            Robot {
                x: px.parse().unwrap(),
                y: py.parse().unwrap(),
                vx: vx.parse().unwrap(),
                vy: vy.parse().unwrap(),
            }
        })
        .collect()
}

fn move_robot(r: Robot, grid_width: usize, grid_height: usize) -> Robot {
    debug_assert!(r.vx < grid_width as isize);
    debug_assert!(r.vy < grid_height as isize);
    let mut x =
        r.x.checked_add_signed(r.vx)
            .unwrap_or((grid_width + r.x).checked_add_signed(r.vx).unwrap());
    let mut y =
        r.y.checked_add_signed(r.vy)
            .unwrap_or((grid_height + r.y).checked_add_signed(r.vy).unwrap());
    if x >= grid_width {
        x -= grid_width
    }
    if y >= grid_height {
        y -= grid_height
    }
    Robot {
        x,
        y,
        vx: r.vx,
        vy: r.vy,
    }
}

fn move_robots(robots: Vec<Robot>, grid_width: usize, grid_height: usize) -> Vec<Robot> {
    robots
        .into_iter()
        .map(|robot| move_robot(robot, grid_width, grid_height))
        .collect()
}

fn get_quadrants(grid_width: usize, grid_height: usize) -> [Submap; 4] {
    let horiz_mid_left = (grid_width - 2) / 2;
    let horiz_mid_right = (grid_width + 2) / 2;
    let vert_mid_top = (grid_height - 2) / 2;
    let vert_mid_bot = (grid_height + 2) / 2;
    [
        Submap {
            x_left: 0,
            y_top: 0,
            x_right: horiz_mid_left,
            y_bot: vert_mid_top,
        },
        Submap {
            x_left: horiz_mid_right,
            y_top: 0,
            x_right: grid_width - 1,
            y_bot: vert_mid_top,
        },
        Submap {
            x_left: 0,
            y_top: vert_mid_bot,
            x_right: horiz_mid_left,
            y_bot: grid_height - 1,
        },
        Submap {
            x_left: horiz_mid_right,
            y_top: vert_mid_bot,
            x_right: grid_width - 1,
            y_bot: grid_height - 1,
        },
    ]
}

fn robot_in_submap(r: &Robot, q: &Submap) -> bool {
    r.x >= q.x_left && r.x <= q.x_right && r.y >= q.y_top && r.y <= q.y_bot
}

fn calculate_safety_factor_after_seconds(
    s: &str,
    grid_width: usize,
    grid_height: usize,
    seconds: usize,
) -> usize {
    let mut robots = parse_input(s);
    for i in 0..seconds {
        robots = move_robots(robots, grid_width, grid_height)
    }
    calculate_safety_factor(&robots, grid_width, grid_height)
}

fn calculate_safety_factor(robots: &[Robot], grid_width: usize, grid_height: usize) -> usize {
    let quadrants = get_quadrants(grid_width, grid_height);
    quadrants
        .iter()
        .map(|q| robots.iter().filter(|r| robot_in_submap(r, q)).count())
        .reduce(|acc, e| acc * e)
        .unwrap()
}

// This assumes when the christmas tree is drawn, a majority of robots are
// located in the centre of the map.
fn calculate_christmas_tree_factor(
    robots: &[Robot],
    grid_width: usize,
    grid_height: usize,
) -> usize {
    let centre = Submap {
        x_left: grid_width / 4,
        y_top: grid_height / 4,
        x_right: grid_width * 3 / 4,
        y_bot: grid_height * 3 / 4,
    };
    robots
        .iter()
        .filter(|r| robot_in_submap(r, &centre))
        .count()
}

fn display_robots(robots: &[Robot], grid_width: usize, grid_height: usize) {
    let mut grid = vec![vec![false; grid_width]; grid_height];
    for r in robots {
        *grid.get_mut(r.y).and_then(|row| row.get_mut(r.x)).unwrap() = true;
    }
    for row in grid {
        for c in row {
            if c {
                print!("▉");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

pub(crate) fn part_1(input: String) {
    println!(
        "Safety factor is {}",
        calculate_safety_factor_after_seconds(&input, 101, 103, 100)
    )
}

pub(crate) fn part_2(input: String) {
    let mut robots = parse_input(&input);
    let w = 101;
    let h = 103;
    let mut max = (0, 0);
    for i in 0..1000000 {
        let factor = calculate_christmas_tree_factor(&robots, w, h);
        if factor > max.0 {
            max = (factor, i);
            display_robots(&robots, w, h);
            println!("High christmas tree factor detected {:?}", max);
            println!("Press enter to display next");
            std::io::stdin().read_line(&mut String::new());
        }
        robots = move_robots(robots, w, h);
    }
}

#[cfg(test)]
mod tests {
    use super::{move_robot, move_robots, parse_input, Robot};
    use crate::day_14::calculate_safety_factor_after_seconds;

    const TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part_1() {
        assert_eq!(
            calculate_safety_factor_after_seconds(TEST_INPUT, 11, 7, 100),
            12
        )
    }
    #[test]
    fn test_single_move() {
        let robot = Robot {
            x: 2,
            y: 4,
            vx: 2,
            vy: -3,
        };
        let robot = move_robot(robot, 11, 7);
        assert_eq!(
            robot,
            Robot {
                x: 4,
                y: 1,
                vx: 2,
                vy: -3
            }
        )
    }
    #[test]
    fn test_2_moves() {
        let mut robot = Robot {
            x: 2,
            y: 4,
            vx: 2,
            vy: -3,
        };
        robot = move_robot(robot, 11, 7);
        robot = move_robot(robot, 11, 7);
        assert_eq!(
            robot,
            Robot {
                x: 6,
                y: 5,
                vx: 2,
                vy: -3
            }
        )
    }
    #[test]
    fn test_3_moves() {
        let mut robot = Robot {
            x: 2,
            y: 4,
            vx: 2,
            vy: -3,
        };
        robot = move_robot(robot, 11, 7);
        robot = move_robot(robot, 11, 7);
        robot = move_robot(robot, 11, 7);
        assert_eq!(
            robot,
            Robot {
                x: 8,
                y: 2,
                vx: 2,
                vy: -3
            }
        )
    }
}
