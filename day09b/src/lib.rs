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
    let perpendicular_walls_hit: Vec<&&Vec<&Vec<i32>>> = up_walls.iter()
        .filter(|vertices| vertices[0][0] > p[0])
        .filter(|vertices|
            min(vertices[0][1], vertices[1][1]) < p[1] &&
            p[1] <= max(vertices[0][1], vertices[1][1])
        )
        .collect();
    perpendicular_walls_hit.len()
}

fn inside_rectangle(p: &Vec<i32>, p1: &Vec<i32>, p2: &Vec<i32>) -> bool {
    p[0] > min(p1[0], p2[0]) &&
    p[0] < max(p1[0], p2[0]) &&
    p[1] > min(p1[1], p2[1]) &&
    p[1] < max(p1[1], p2[1])
}

fn inside_shape(p: &Vec<i32>, up_walls: &Vec<&Vec<&Vec<i32>>>, across_walls: &Vec<&Vec<&Vec<i32>>>) -> bool {
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
    let mut walls: Vec<Vec<&Vec<i32>>> = Vec::new();
    walls.push([&points[points.len()-1], &points[0]].to_vec());
    for i in 0..points.len()-1 {
        walls.push([&points[i], &points[i+1]].to_vec());
    }
    walls
}

fn split_walls<'a>(walls: &'a Vec<Vec<&'a Vec<i32>>>) -> (Vec<&'a Vec<&'a Vec<i32>>>, Vec<&'a Vec<&'a Vec<i32>>>) {
    let mut up_walls = walls.iter()
        .filter(|vertices| vertices[0][0] == vertices[1][0])
        .collect::<Vec<&Vec<&Vec<i32>>>>();
    up_walls.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
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

    let mut min_distance = straight_line_distance(&points[0], &points[points.len()-1]);
    for i in 0..points.len()-1 {
        min_distance = min(min_distance, straight_line_distance(&points[i], &points[i+1]));
    }

    let mut adjusted_points: Vec<Vec<i32>> = Vec::new();
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

    let walls = vertices_to_walls(&adjusted_points);
    let (up_walls, across_walls) = split_walls(&walls);
    let mut biggest_area_inside: i64 = 0;
    for i in 0..points.len() {
        for j in i+1..points.len() {
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
                let other_corner_1 = vec![adjusted_points[i][0], adjusted_points[j][1]];
                let other_corner_2 = vec![adjusted_points[j][0], adjusted_points[i][1]];
                if inside_shape(&other_corner_1, &up_walls, &across_walls) &&
                    inside_shape(&other_corner_2, &up_walls, &across_walls) {
                    let x_min = min(adjusted_points[i][0], adjusted_points[j][0]);
                    let y_min = min(adjusted_points[i][1], adjusted_points[j][1]);
                    let x_max = max(adjusted_points[i][0], adjusted_points[j][0]);
                    let y_max = max(adjusted_points[i][1], adjusted_points[j][1]);
                    let mut walls_inside = true;
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
