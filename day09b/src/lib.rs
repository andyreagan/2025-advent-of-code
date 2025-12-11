use std::cmp::max;
use std::cmp::min;

fn area(p1: &Vec<i32>, p2: &Vec<i32>) -> i64 {
    let dx = ((p1[0] - p2[0]).abs() + 1) as i64;
    let dy = ((p1[1] - p2[1]).abs() + 1) as i64;
    dx*dy
}

fn straight_line_distance(p1: &Vec<i32>, p2: &Vec<i32>) -> i32 {
    (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs()
}

fn n_intersections(p: &Vec<i32>, up_walls: &Vec<&Vec<&Vec<i32>>>) -> usize {
    // direction 0 right 1 down 2 left 3 up
    // the matching x[0][1] <= p[1] will hit the corner (end of the wall)
    // while x[0][1] < p[1] will not
    // since we dont care about corners, we'll forgo it
    let perpendicular_walls_hit: Vec<&&Vec<&Vec<i32>>> = up_walls.iter()
        .filter(|vertices| vertices[0][0] > p[0])
        .filter(|vertices|
            min(vertices[0][1], vertices[1][1]) < p[1] &&
            p[1] <= max(vertices[0][1], vertices[1][1])
        )
        .collect();
    // println!("point {:?} going right hits walls {:?} perpendicularly", p, perpendicular_walls_hit);
    perpendicular_walls_hit.len()
}

fn inside_rectangle(p: &Vec<i32>, p1: &Vec<i32>, p2: &Vec<i32>) -> bool {
    // strictly inside, not just on the edge
    p[0] > min(p1[0], p2[0]) &&
    p[0] < max(p1[0], p2[0]) &&
    p[1] > min(p1[1], p2[1]) &&
    p[1] < max(p1[1], p2[1])
}

fn inside_shape(p: &Vec<i32>, up_walls: &Vec<&Vec<&Vec<i32>>>, across_walls: &Vec<&Vec<&Vec<i32>>>) -> bool {
    // first let's check if the point fall directly on a wall
    // that's easier to check than the ray casting?
    let on_walls_vertical: Vec<&&Vec<&Vec<i32>>> = up_walls.iter()
        .filter(|vertices| vertices[0][0] == p[0])
        .filter(|vertices|
            min(vertices[0][1], vertices[1][1]) <= p[1] &&
            p[1] <= max(vertices[0][1], vertices[1][1])
        )
        .collect();
    if on_walls_vertical.len() > 0 {
        return true
    }
    let on_walls_horizontal: Vec<&&Vec<&Vec<i32>>> = across_walls.iter()
        .filter(|vertices| vertices[0][1] == p[1])
        .filter(|vertices|
            min(vertices[0][0], vertices[1][0]) <= p[0] &&
            p[0] <= max(vertices[0][0], vertices[1][0])
        )
        .collect();
    if on_walls_horizontal.len() > 0 {
        return true
    }
    n_intersections(p, up_walls) % 2 == 1
}

fn vertices_to_walls(points: &Vec<Vec<i32>>) -> Vec<Vec<&Vec<i32>>> {
    // let's make a vector of the walls (including connecting start and beginning)
    let mut walls: Vec<Vec<&Vec<i32>>> = Vec::new(); // could maybe build this directly...
    walls.push([&points[points.len()-1], &points[0]].to_vec());
    for i in 0..points.len()-1 {
        walls.push([&points[i], &points[i+1]].to_vec());
    }
    walls
}

// the lifetimes annotation here is from the compiler
fn split_walls<'a>(walls: &'a Vec<Vec<&'a Vec<i32>>>) -> (Vec<&'a Vec<&'a Vec<i32>>>, Vec<&'a Vec<&'a Vec<i32>>>) {
    // vertical walls are "up" walls, horizontal are "across" walls
    // vertical means the x points are equal
    let mut up_walls = walls.iter()
        .filter(|vertices| vertices[0][0] == vertices[1][0])
        .collect::<Vec<&Vec<&Vec<i32>>>>();
    up_walls.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
    // horizontal means the y points are equal
    let mut across_walls = walls.iter()
        .filter(|vertices| vertices[0][1] == vertices[1][1])
        .collect::<Vec<&Vec<&Vec<i32>>>>();
    across_walls.sort_by(|a, b| a[1].partial_cmp(&b[1]).unwrap());
    (up_walls, across_walls)
}

pub fn process(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let points: Vec<Vec<i32>> = lines.iter()
        .map(|x| {
            x.split(",")
                .map(|n| n.parse::<i32>().expect("should parse to int"))
                .collect()
        })
        .collect();

    // for part 2 - let's check if any points are adjacent
    let mut min_distance = straight_line_distance(&points[0], &points[points.len()-1]);
    for i in 0..points.len()-1 {
        min_distance = min(min_distance, straight_line_distance(&points[i], &points[i+1]));
    }
    // we'll rely on this fact for our algorithm

    // let's collapse the points so that min_x and min_y are at 0,0, and every jump is by 2 (second x position will be 2, etc)
    let mut adjusted_points: Vec<Vec<i32>> = Vec::new();
    // we'll need to sort every existing x point, and y point
    // then use those sorted lists to determine the adjusted position
    let mut sorted_x: Vec<i32> = points.iter().map(|p| p[0]).collect();
    sorted_x.sort();
    sorted_x.dedup();
    let mut sorted_y: Vec<i32> = points.iter().map(|p| p[1]).collect();
    sorted_y.sort();
    sorted_y.dedup();
    for p in points.iter() {
        let adjusted_x = (sorted_x.iter().position(|x| *x == p[0]).unwrap() * 2) as i32;
        let adjusted_y = (sorted_y.iter().position(|y| *y == p[1]).unwrap() * 2) as i32;
        adjusted_points.push(vec![adjusted_x, adjusted_y]);
    }

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
    let walls = vertices_to_walls(&adjusted_points);
    // we can separate the walls by orientation:
    let (up_walls, across_walls) = split_walls(&walls);
    let mut biggest_area_inside: i64 = 0;
    for i in 0..points.len() {
        for j in i+1..points.len() {
            // we'll do a series of checks in order of increasing complexity
            // first, are there any other points inside the rectangle?
            // can use the raw coordinates for this, same as area calc
            let mut other_points_inside: bool = false;
            for k in 0..points.len() {
                if k != i && k != j {
                    if inside_rectangle(&points[k], &points[i], &points[j]) {
                        other_points_inside = true;
                        break;
                    }
                }
            }
            let area_value = area(&points[i], &points[j]);
            if !other_points_inside && area_value > biggest_area_inside {
                // now we check the corners
                // and this really boils down to checking only the two "opposite" corners
                // since the red tiles we have are certainly on an edge already
                // we need to do this in adjusted coordinates
                // since our walls are in adjusted coordinates
                let other_corner_1 = vec![adjusted_points[i][0], adjusted_points[j][1]];
                let other_corner_2 = vec![adjusted_points[j][0], adjusted_points[i][1]];
                if inside_shape(&other_corner_1, &up_walls, &across_walls) &&
                    inside_shape(&other_corner_2, &up_walls, &across_walls) {
                    // need to check the whole walls
                    let x_min = min(adjusted_points[i][0], adjusted_points[j][0]);
                    let y_min = min(adjusted_points[i][1], adjusted_points[j][1]);
                    let x_max = max(adjusted_points[i][0], adjusted_points[j][0]);
                    let y_max = max(adjusted_points[i][1], adjusted_points[j][1]);
                    let mut walls_inside = true;
                    // first check the horizontal walls
                    for x in x_min..x_max {
                        let bottom_wall_pt = vec![x, y_min];
                        if !inside_shape(&bottom_wall_pt, &up_walls, &across_walls) {
                            walls_inside = false;
                            break
                        }
                        let top_wall_pt = vec![x, y_max];
                        if !inside_shape(&top_wall_pt, &up_walls, &across_walls) {
                            walls_inside = false;
                            break
                        }
                    }
                    // check the vertical walls if we need to
                    if walls_inside {
                        for y in y_min..y_max {
                            let left_wall_pt = vec![x_min, y];
                            if !inside_shape(&left_wall_pt, &up_walls, &across_walls) {
                                walls_inside = false;
                                break
                            }
                            let right_wall_pt = vec![x_max, y];
                            if !inside_shape(&right_wall_pt, &up_walls, &across_walls) {
                                walls_inside = false;
                                break
                            }
                        }
                    }
                    if walls_inside {
                        biggest_area_inside = max(biggest_area_inside, area_value);
                    }
                }
            }
        }
    }
    biggest_area_inside
}

pub fn main() {
    let input = include_str!("../input.txt");
    println!("{}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day09b() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let biggest_area_inside = process(input);
        assert_eq!(biggest_area_inside, 24);
    }
}
