use std::fs::File;
use std::io::prelude::*;
use std::cmp::max;
use std::cmp::min;
use more_asserts::assert_ge;
use indicatif::ProgressBar;

fn area(p1: &Vec<i32>, p2: &Vec<i32>) -> i64 {
    let dx = ((p1[0] - p2[0]).abs() + 1) as i64;
    let dy = ((p1[1] - p2[1]).abs() + 1) as i64;
    dx*dy
}

fn straight_line_distance(p1: &Vec<i32>, p2: &Vec<i32>) -> i32 {
    if ((p1[0] - p2[0]) != 0) && ((p1[1] - p2[1]) != 0) {
        println!("hey these points aren't on a straight line: {:?} to {:?}", p1, p2);
    }
    (p1[0] - p2[0]).abs() +  (p1[1] - p2[1]).abs()
}

fn n_intersections(p: &Vec<i32>, direction: &i8, up_walls: &Vec<&Vec<&Vec<i32>>>, across_walls: &Vec<&Vec<&Vec<i32>>>) -> usize {
    // direction 0 right 1 down 2 left 3 up
    // the matching x[0][1] <= p[1] will hit the corner (end of the wall)
    // while x[0][1] < p[1] will not
    // since we dont care about corners, we'll forgo it
    match direction {
        0 => up_walls.iter().filter(|x| x[0][0] > p[0]).filter(|x| (x[0][1] < p[1] && x[1][1] > p[1]) || (x[1][1] < p[1] && x[0][1] > p[1])).collect::<Vec<&&Vec<&Vec<i32>>>>().len(),
        1 => across_walls.iter().filter(|x| x[0][1] > p[1]).filter(|x| (x[0][0] < p[0] && x[1][0] > p[1]) || (x[1][0] < p[1] && x[0][0] > p[1])).collect::<Vec<&&Vec<&Vec<i32>>>>().len(),
        2 => up_walls.iter().filter(|x| x[0][0] < p[0]).filter(|x| (x[0][1] < p[1] && x[1][1] > p[1]) || (x[1][1] < p[1] && x[0][1] > p[1])).collect::<Vec<&&Vec<&Vec<i32>>>>().len(),
        3 => across_walls.iter().filter(|x| x[0][1] < p[1]).filter(|x| (x[0][0] < p[0] && x[1][0] > p[1]) || (x[1][0] < p[1] && x[0][0] > p[1])).collect::<Vec<&&Vec<&Vec<i32>>>>().len(),
        _ => panic!("not a valid direction!"),
    }
}

// fn inside_rectangle(p: &Vec<i32>, p1: &Vec<i32>, p2: &Vec<i32>) -> bool {
//     p[0] > min(p1[0], p2[0]) &&
//     p[0] < max(p1[0], p2[0]) &&
//     p[1] > min(p1[1], p2[1]) &&
//     p[1] < max(p1[1], p2[1])
// }


fn inside_shape(p: &Vec<i32>, up_walls: &Vec<&Vec<&Vec<i32>>>, across_walls: &Vec<&Vec<&Vec<i32>>>) -> bool {
    // first let's check if the point fall directly on a wall
    for wall in up_walls.iter() {
        if wall[0][0] == p[0] && ((wall[0][1] <= p[1] && wall[1][1] >= p[1]) || (wall[1][1] <= p[1] && wall[0][1] >= p[1])) {
            // println!("point {:?} is on wall {:?}", p, wall);
            return true;
        }
        if wall[0][1] == p[1] && ((wall[0][0] <= p[0] && wall[1][0] >= p[0]) || (wall[1][0] <= p[0] && wall[0][0] >= p[0])) {
            // println!("point {:?} is on wall {:?}", p, wall);
            return true;
        }
    }
    for wall in across_walls.iter() {
        if wall[0][0] == p[0] && ((wall[0][1] <= p[1] && wall[1][1] >= p[1]) || (wall[1][1] <= p[1] && wall[0][1] >= p[1])) {
            // println!("point {:?} is on wall {:?}", p, wall);
            return true;
        }
        if wall[0][1] == p[1] && ((wall[0][0] <= p[0] && wall[1][0] >= p[0]) || (wall[1][0] <= p[0] && wall[0][0] >= p[0])) {
            // println!("point {:?} is on wall {:?}", p, wall);
            return true;
        }
    }
    if n_intersections(p, &0, up_walls, across_walls) % 2 == 1 {
        // println!("point {:?} hit a single wall going right, must be inside", p);
        return true;
    }
    if n_intersections(p, &1, up_walls, across_walls) % 2 == 1 {
        // println!("point {:?} hit a single wall going down, must be inside", p);
        return true;        
    }
    if n_intersections(p, &2, up_walls, across_walls) % 2 == 1 {
        // println!("point {:?} hit a single wall going left, must be inside", p);
        return true;        
    }
    if n_intersections(p, &3, up_walls, across_walls) % 2 == 1 {
        // println!("point {:?} hit a single wall going up, must be inside", p);
        return true;
    }
    false
}

fn process_day09(input: &str) -> (i64, i64) {
    let lines: Vec<&str> = input.lines().collect();
    let points: Vec<Vec<i32>> = lines.iter()
        .map(|x| {
            x.split(",")
                .map(|n| n.parse::<i32>().expect("should parse to int"))
                .collect()
        })
        .collect();
    println!("we have {} points to process", points.len());
    let mut biggest_area: i64 = 0;
    for i in 0..points.len() {
        for j in i..points.len() {
            biggest_area = max(biggest_area, area(&points[i], &points[j]));
        }
    }
    // for part 2 - let's check if any points are adjacent
    let mut min_distance = straight_line_distance(&points[0], &points[points.len()-1]);
    for i in 0..points.len()-1 {
        min_distance = min(min_distance, straight_line_distance(&points[i], &points[i+1]));
    }
    println!("min distance is {min_distance}");
    assert_ge!(min_distance, 2);
    // we'll rely on this fact for our algorithm
    // for every possible rectangle: it's not all red/green if any point exists
    // inside of our rectangle
    // that's necessary but not sufficient: we can't draw on the _outside_ of our
    // area either
    // we have to know whether we're winding it clockwise or counterclockwise
    // determining this seems hard - you don't know until it's all the way connected
    // if you loop back around (can basically change the direction)
    // nevermind: we'll just drop a point in the rectangle,
    // then check how many walls it crosses on it's way out
    // I think I remember this algorithm... it came from somewhere in my head
    // so now, we can take a point, and a direction,
    // and determine if it cross any other line?
    // let's make a vector of the walls (including connecting start and beginning)
    let mut walls: Vec<Vec<&Vec<i32>>> = Vec::new(); // could maybe build this directly...
    walls.push([&points[points.len()-1], &points[0]].to_vec());
    for i in 0..points.len()-1 {
        walls.push([&points[i], &points[i+1]].to_vec());
    }
    println!("we have {} walls", walls.len());
    // we can separate the walls by orientation:
    let mut up_walls = walls.iter()
        .filter(|x| x[0][0] == x[1][0])
        .collect::<Vec<&Vec<&Vec<i32>>>>();
    up_walls.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
    let mut across_walls = walls.iter()
        .filter(|x| x[0][1] == x[1][1])
        .collect::<Vec<&Vec<&Vec<i32>>>>();
    across_walls.sort_by(|a, b| a[1].partial_cmp(&b[1]).unwrap());
    println!("we have {} up walls and {} across walls", up_walls.len(), across_walls.len());
    let mut biggest_area_inside: i64 = 0;
    let bar = ProgressBar::new((points.len()*(points.len()+1)/2).try_into().unwrap());
    for i in 0..points.len() {
        for j in i+1..points.len() {
            bar.inc(1);
            // println!("trying rectangle [{:?},{:?}]", points[i], points[j]);
            
            // let straight_line = points[i][0] == points[j][0] || points[i][1] == points[j][1];
            // let mut right_intersections: usize = 1;
            // if !straight_line {
            //     let inside_point = vec![
            //         (points[i][0] + points[j][0]) / 2,
            //         (points[i][1] + points[j][1]) / 2,
            //     ];                
            //     right_intersections = n_intersections(&inside_point, &(0 as i8), &up_walls, &across_walls);
            //     println!("we picked inside point {:?} which has intersections of ", inside_point);
            // } else {
            //     println!("straight line rectangle, so we consider ourselves inside");
            // }
            // if right_intersections % 2 == 1 {
            //     println!("odd # right intersections (or straight line) means we're inside");
            //     // odd # intersections means we're inside
            //     // now we need to check that there are no other points inside
            //     let mut other_points_inside: bool = false;
            //     for k in 0..points.len() {
            //         if k != i && k != j {
            //             if inside_rectangle(&points[k], &points[i], &points[j]) {
            //                 other_points_inside = true;
            //                 break;
            //             }
            //         }
            //     }
            //     println!("other points inside? {}", other_points_inside);
            //     if !other_points_inside {
            //         let area_value = area(&points[i], &points[j]);
            //         println!("valid rectangle! area is {}", area_value);
            //         biggest_area_inside = max(biggest_area_inside, area_value);
            //         if biggest_area_inside == area_value {
            //             println!("!!! new biggest area inside is {}", biggest_area_inside);
            //         }
            //     }
            // }
            
            // there are shapes where we have no extra points inside the rectangle
            // and the middle is in the bigger shape, but one of the corners is outside
            // let's resort to checking that every point is inside the shape
            // just the corners isn't sufficient
            
            let mut all_inside: bool = true;
            for k in min(points[i][0], points[j][0])..=max(points[i][0], points[j][0]) {
                for l in min(points[i][1], points[j][1])..=max(points[i][1], points[j][1]) {
                    let test_point = vec![k, l];
                    if !inside_shape(&test_point, &up_walls, &across_walls) {
                        all_inside = false;
                        break;
                    }
                }
                if !all_inside {
                    break;
                }
            }
            if all_inside {
                let area_value = area(&points[i], &points[j]);
                // println!("valid rectangle! area is {}", area_value);
                biggest_area_inside = max(biggest_area_inside, area_value);
                if biggest_area_inside == area_value {
                    println!("!!! new biggest area inside is {}", biggest_area_inside);
                }
            }
        }
    }
    bar.finish();
    (biggest_area, biggest_area_inside)
}

pub fn run(day: i8) {
    println!("---------  day {}  ----------", day);
    let file = File::open(format!("inputs/day{:02}.txt", day));
    let mut input = String::new();
    let _ = file.expect(&format!("file inputs/day{:02}.txt does not exist", day)).read_to_string(&mut input);
    let (part_1, part_2) = process_day09(&input);
    println!("biggest rectangle is {part_1}");
    println!("TBD is {part_2}");
    println!("------- end of day {} -------\n", day);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_day09() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let (part_1, part_2) = process_day09(input);
        assert_eq!(part_1, 50);
        assert_eq!(part_2, 24);
    }

    #[test]
    fn test_intersection_count() {
        let up_wall1 = vec![vec![10, 0], vec![10, 5]];
        let up_wall2 = vec![vec![15, 0], vec![15, 5]];
        let across_wall1 = vec![vec![0, 5], vec![20, 5]];
        let across_wall2 = vec![vec![0, 10], vec![20, 10]];
        let up_wall1_inner = up_wall1.iter().collect();
        let up_wall2_inner = up_wall2.iter().collect();
        let across_wall1_inner = across_wall1.iter().collect();
        let across_wall2_inner = across_wall2.iter().collect();
        let up_walls: Vec<&Vec<&Vec<i32>>> = vec![
            &up_wall1_inner,
            &up_wall2_inner,
        ];
        let across_walls: Vec<&Vec<&Vec<i32>>> = vec![
            &across_wall1_inner,
            &across_wall2_inner,
        ];
        let inside_right = n_intersections(&vec![11, 1].to_vec(), &(0 as i8), &up_walls, &across_walls);
        assert_eq!(inside_right, 1);
        let outside_right = n_intersections(&vec![16, 1].to_vec(), &(0 as i8), &up_walls, &across_walls);
        assert_eq!(outside_right, 0);
        let cross_right = n_intersections(&vec![9, 1].to_vec(), &(0 as i8), &up_walls, &across_walls);
        assert_eq!(cross_right, 2);        
        let inside_left = n_intersections(&vec![11, 1].to_vec(), &(2 as i8), &up_walls, &across_walls);
        assert_eq!(inside_left, 1);
        let cross_left = n_intersections(&vec![16, 1].to_vec(), &(2 as i8), &up_walls, &across_walls);
        assert_eq!(cross_left, 2);
        let outside_left = n_intersections(&vec![9, 1].to_vec(), &(2 as i8), &up_walls, &across_walls);
        assert_eq!(outside_left, 0);
        let inside_down = n_intersections(&vec![1, 6].to_vec(), &(1 as i8), &up_walls, &across_walls);
        assert_eq!(inside_down, 1);
        let cross_down = n_intersections(&vec![1, 4].to_vec(), &(1 as i8), &up_walls, &across_walls);
        assert_eq!(cross_down, 2);
        let outside_down = n_intersections(&vec![1, 11].to_vec(), &(1 as i8), &up_walls, &across_walls);
        assert_eq!(outside_down, 0);
        let inside_up = n_intersections(&vec![1, 6].to_vec(), &(3 as i8), &up_walls, &across_walls);
        assert_eq!(inside_up, 1);
        let outside_up = n_intersections(&vec![1, 4].to_vec(), &(3 as i8), &up_walls, &across_walls);
        assert_eq!(outside_up, 0);
        let cross_up = n_intersections(&vec![1, 11].to_vec(), &(3 as i8), &up_walls, &across_walls);
        assert_eq!(cross_up, 2);
    }
}
