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
    let perpendicular_walls_hit: Vec<&&Vec<&Vec<i32>>> = match direction {
        0 => up_walls.iter().filter(|vertices| vertices[0][0] > p[0]).filter(|vertices| (vertices[0][1] < p[1] && vertices[1][1] > p[1]) || (vertices[1][1] < p[1] && vertices[0][1] > p[1])).collect(),
        1 => across_walls.iter().filter(|vertices| vertices[0][1] > p[1]).filter(|vertices| (vertices[0][0] < p[0] && vertices[1][0] > p[0]) || (vertices[1][0] < p[0] && vertices[0][0] > p[0])).collect(),
        2 => up_walls.iter().filter(|vertices| vertices[0][0] < p[0]).filter(|vertices| (vertices[0][1] < p[1] && vertices[1][1] > p[1]) || (vertices[1][1] < p[1] && vertices[0][1] > p[1])).collect(),
        3 => across_walls.iter().filter(|vertices| vertices[0][1] < p[1]).filter(|vertices| (vertices[0][0] < p[0] && vertices[1][0] > p[0]) || (vertices[1][0] < p[0] && vertices[0][0] > p[0])).collect(),
        _ => panic!("not a valid direction!"),
    };
    let parallel_walls_hit: Vec<&&Vec<&Vec<i32>>> = match direction {
        0 => across_walls.iter().filter(|vertices| vertices[0][1] == p[1]).filter(|vertices| vertices[0][0] > p[0] && vertices[1][0] > p[0]).collect(),
        1 => up_walls.iter().filter(|vertices| vertices[0][0] == p[0]).filter(|vertices| vertices[0][1] > p[1] && vertices[1][1] > p[1]).collect(),
        2 => across_walls.iter().filter(|vertices| vertices[0][1] == p[1]).filter(|vertices| vertices[0][0] < p[0] && vertices[1][0] < p[0]).collect(),
        3 => up_walls.iter().filter(|vertices| vertices[0][0] == p[0]).filter(|vertices| vertices[0][1] < p[1] && vertices[1][1] < p[1]).collect(),
        _ => panic!("not a valid direction!"),
    };
    // println!("point {:?} going direction {} hits walls {:?} perpendicularly, and walls {:?} parallely", p, direction, perpendicular_walls_hit, parallel_walls_hit);
    perpendicular_walls_hit.len() + parallel_walls_hit.len()
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
    // if n_intersections(p, &0, up_walls, across_walls) % 2 == 1 {
    //     println!("point {:?} hit a single wall going right, must be inside", p);
    //     return true;
    // }
    // if n_intersections(p, &1, up_walls, across_walls) % 2 == 1 {
    //     println!("point {:?} hit a single wall going down, must be inside", p);
    //     return true;        
    // }
    // if n_intersections(p, &2, up_walls, across_walls) % 2 == 1 {
    //     println!("point {:?} hit a single wall going left, must be inside", p);
    //     return true;        
    // }
    // if n_intersections(p, &3, up_walls, across_walls) % 2 == 1 {
    //     println!("point {:?} hit a single wall going up, must be inside", p);
    //     return true;
    // }
    // println!("point {:?} is not on a wall and didn't ray cast onto a single wall", p);
    // false
    // instead of any single hit, we check for any double hit
    if n_intersections(p, &0, up_walls, across_walls) % 2 == 0 {
        // println!("point {:?} hit even # of walls right, must be outside", p);
        return false;
    }
    if n_intersections(p, &1, up_walls, across_walls) % 2 == 0 {
        // println!("point {:?} hit even # of walls down, must be outside", p);
        return false;        
    }
    if n_intersections(p, &2, up_walls, across_walls) % 2 == 0 {
        // println!("point {:?} hit even # of walls left, must be outside", p);
        return false;        
    }
    if n_intersections(p, &3, up_walls, across_walls) % 2 == 0 {
        // println!("point {:?} hit even # of walls up, must be outside", p);
        return false;
    }
    // !("point {:?} is not on a wall and didn't have any even # transitions", p);
    true    
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

fn process_day09(input: &str, show_progress: bool) -> (i64, i64) {
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

    // look at the total size of the shape we've formed
    let min_x = points.iter().map(|p| p[0]).min().unwrap() as i64;
    let max_x = points.iter().map(|p| p[0]).max().unwrap() as i64;
    let min_y = points.iter().map(|p| p[1]).min().unwrap() as i64;
    let max_y = points.iter().map(|p| p[1]).max().unwrap() as i64;
    println!("bounding box is x={}..{} y={}..{}, total area {}", min_x , max_x , min_y, max_y, (max_x - min_x) * (max_y - min_y));

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
        adjusted_points.push(vec![adjusted_x , adjusted_y]);
    }

    // check the new points bounding box and area
    let adj_min_x = adjusted_points.iter().map(|p| p[0]).min().unwrap() as i64;
    let adj_max_x = adjusted_points.iter().map(|p| p[0]).max().unwrap() as i64;
    let adj_min_y = adjusted_points.iter().map(|p| p[1]).min().unwrap() as i64;
    let adj_max_y = adjusted_points.iter().map(|p| p[1]).max().unwrap() as i64;
    println!("adjusted bounding box is x={}..{} y={}..{}, total area {}", adj_min_x , adj_max_x , adj_min_y, adj_max_y, (adj_max_x - adj_min_x) * (adj_max_y - adj_min_y));

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
    println!("we have {} walls", walls.len());

    // pretty_print_walls(&walls);

    // we can separate the walls by orientation:
    let (up_walls, across_walls) = split_walls(&walls);
    println!("we have {} up walls and {} across walls", up_walls.len(), across_walls.len());
    let mut biggest_area_inside: i64 = 0;
    let bar = ProgressBar::new((points.len()*(points.len()+1)/2).try_into().unwrap());
    for i in 0..points.len() {
        for j in i+1..points.len() {
            if show_progress {
                bar.inc(1);
            }
            // println!("trying rectangle [{:?},{:?}], using compressed coordinates [{:?},{:?}]", points[i], points[j], adjusted_points[i], adjusted_points[j]);
            
            // there are shapes where we have no extra points inside the rectangle
            // and the middle is in the bigger shape, but one of the corners is outside
            // let's resort to checking that every point is inside the shape
            // just the corners isn't sufficient
            // let mut all_inside: bool = true;
            // for k in min(points[i][0], points[j][0])..=max(points[i][0], points[j][0]) {
            //     for l in min(points[i][1], points[j][1])..=max(points[i][1], points[j][1]) {
            //         let test_point = vec![k, l];
            //         if !inside_shape(&test_point, &up_walls, &across_walls) {
            //             all_inside = false;
            //             break;
            //         }
            //     }
            //     if !all_inside {
            //         break;
            //     }
            // }
            // if all_inside {
            //     let area_value = area(&points[i], &points[j]);
            //     // println!("valid rectangle! area is {}", area_value);
            //     biggest_area_inside = max(biggest_area_inside, area_value);
            //     if biggest_area_inside == area_value {
            //         println!("!!! new biggest area inside is {}", biggest_area_inside);
            //     }
            // }
            // that worked!! but it was too inefficient

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
            // println!("other points inside? {}", other_points_inside);
            if !other_points_inside && area_value > biggest_area_inside {
                // println!("   -> no other points inside and the area is bigger, checking corners");
                // now we check the corners
                // and this really boils down to checking only the two "opposite" corners
                // since the red tiles we have are certainly on an edge already
                // we need to do this in adjusted coordinates
                // since our walls are in adjusted coordinates
                let other_corner_1 = vec![adjusted_points[i][0], adjusted_points[j][1]];
                let other_corner_2 = vec![adjusted_points[j][0], adjusted_points[i][1]];
                if inside_shape(&other_corner_1, &up_walls, &across_walls) && 
                    inside_shape(&other_corner_2, &up_walls, &across_walls) {
                    // println!("       -> corners are inside, now checking walls");
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
                            // println!("bottom wall pt not inside");
                            walls_inside = false;
                            break
                        }
                        let top_wall_pt = vec![x, y_max];
                        if !inside_shape(&top_wall_pt, &up_walls, &across_walls) {
                            // println!("top wall pt not inside");
                            walls_inside = false;
                            break
                        }
                    }
                    // check the vertical walls if we need to
                    if walls_inside {
                        for y in y_min..y_max {
                            let left_wall_pt = vec![x_min, y];
                            if !inside_shape(&left_wall_pt, &up_walls, &across_walls) {
                                // println!("left wall pt not inside");
                                walls_inside = false;
                                break
                            }
                            let right_wall_pt = vec![x_max, y];
                            if !inside_shape(&right_wall_pt, &up_walls, &across_walls) {
                                // println!("right wall pt not inside");
                                walls_inside = false;
                                break
                            }
                        }                
                    }
                    if walls_inside {
                        // println!("           -> walls inside, considering for biggest area");
                        // println!("valid rectangle! area is {}", area_value);
                        biggest_area_inside = max(biggest_area_inside, area_value);
                        // if biggest_area_inside == area_value {
                        //     println!("!!!!!!!!!! new biggest area inside is {}", biggest_area_inside);
                        // }                    
                    }
                }
            }
        }
    }
    if show_progress {
        bar.finish();
    }
    (biggest_area, biggest_area_inside)
}

pub fn run(day: i8) {
    println!("---------  day {}  ----------", day);
    let file = File::open(format!("inputs/day{:02}.txt", day));
    let mut input = String::new();
    let _ = file.expect(&format!("file inputs/day{:02}.txt does not exist", day)).read_to_string(&mut input);
    let (part_1, part_2) = process_day09(&input, true);
    println!("biggest rectangle is {part_1}");
    println!("biggest inside rectangle is {part_2}");
    println!("------- end of day {} -------\n", day);
}

#[cfg(test)]
fn pretty_print_grid(grid: &Vec<Vec<bool>>) {
    for y in grid.iter() {
        println!("{}", y.iter().map(|x| {
            if *x {
                return 'X';
            } else {
                return '.';
            }
        }).collect::<String>());
    }
}

// fn walls_to_grid(walls: &Vec<Vec<&Vec<i32>>>, max_x: i32, max_y: i32) -> Vec<Vec<bool>> {
//     let mut grid: Vec<Vec<bool>> = vec![vec![false; (max_x+1) as usize]; (max_y+1) as usize];
//     for wall in walls.iter() {
//         if wall[0][0] == wall[1][0] {
//             // vertical wall
//             let x = wall[0][0];
//             let y_start = min(wall[0][1], wall[1][1]);
//             let y_end = max(wall[0][1], wall[1][1]);
//             for y in y_start..=y_end {
//                 grid[y as usize][x as usize] = true;
//             }
//         } else if wall[0][1] == wall[1][1] {
//             // horizontal wall
//             let y = wall[0][1];
//             let x_start = min(wall[0][0], wall[1][0]);
//             let x_end = max(wall[0][0], wall[1][0]);
//             for x in x_start..=x_end {
//                 grid[y as usize][x as usize] = true;
//             }
//         } else {
//             println!("non straight wall found: {:?}", wall);
//         }
//     }
//     grid
// }

// fn pretty_print_walls(walls: &Vec<Vec<&Vec<i32>>>) {
//     // build a grid of these walls for pretty printing/debugging
//     let grid: Vec<Vec<bool>> = walls_to_grid(
//         walls, 
//         walls.iter().map(|x| x[0][0]).max().unwrap_or(0), 
//         walls.iter().map(|x| x[0][1]).max().unwrap_or(0)
//     );
//     pretty_print_grid(&grid);
// }

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
        let (part_1, part_2) = process_day09(input, false);
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
        let above_right = n_intersections(&vec![9, 0].to_vec(), &(0 as i8), &up_walls, &across_walls);
        assert_eq!(above_right, 0);
        let below_right = n_intersections(&vec![9, 5].to_vec(), &(0 as i8), &up_walls, &across_walls);
        assert_eq!(below_right, 0);
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

    #[test]
    fn test_intersection_count_spot() {
        let up_wall1 = vec![vec![10, 0], vec![10, 5]];
        let up_wall2 = vec![vec![15, 0], vec![15, 5]];
        let across_wall1 = vec![vec![2, 3], vec![7, 3]];
        let across_wall2 = vec![vec![9, 5], vec![2, 5]];
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
        let cross_down = n_intersections(&vec![3, 0].to_vec(), &(1 as i8), &up_walls, &across_walls);
        assert_eq!(cross_down, 2);
    }    

    #[test]
    fn test_inside_shape() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let lines: Vec<&str> = input.lines().collect();
        let points: Vec<Vec<i32>> = lines.iter()
            .map(|x| {
                x.split(",")
                    .map(|n| n.parse::<i32>().expect("should parse to int"))
                    .collect()
            })
            .collect();
        let walls = vertices_to_walls(&points);
        let (up_walls, across_walls) = split_walls(&walls);
        // convert this full map into T/F for where we're inside

        // let expected = walls_to_grid(&walls, 12, 8);
        let expected = vec![
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, ],
            vec![false, false, false, false, false, false, false, true , true , true , true , true , false, false, ],
            vec![false, false, false, false, false, false, false, true , true , true , true , true , false, false, ],
            vec![false, false, true , true , true , true , true , true , true , true , true , true , false, false, ],
            vec![false, false, true , true , true , true , true , true , true , true , true , true , false, false, ],
            vec![false, false, true , true , true , true , true , true , true , true , true , true , false, false, ],
            vec![false, false, false, false, false, false, false, false, false, true , true , true , false, false, ],
            vec![false, false, false, false, false, false, false, false, false, true , true , true , false, false, ],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, ],
        ];
        pretty_print_grid(&expected);
        let mut result: Vec<Vec<bool>> = Vec::new();
        for test_y in 0..expected.len() {
            let mut tmp_vec: Vec<bool> = Vec::new();
            for test_x in 0..expected[0].len() {
                let test_point = vec![test_x as i32, test_y as i32];
                tmp_vec.push(inside_shape(&test_point, &up_walls, &across_walls));
                // assert_eq!(inside_shape(test_point, &up_walls, &across_walls), full_results[test_y][test_x]);
            }
            result.push(tmp_vec);
        }
        pretty_print_grid(&result);
        for test_x in 0..expected[0].len() {
            for test_y in 0..expected.len() {
                assert_eq!(result[test_y][test_x], expected[test_y][test_x]);
            }
        }
    }

//     #[test]
//     fn test_inside_shape_2() {
//         let input = "1,1
// 5,1
// 5,5
// 3,5
// 3,3
// 1,3";
//         let lines: Vec<&str> = input.lines().collect();
//         let points: Vec<Vec<i32>> = lines.iter()
//             .map(|x| {
//                 x.split(",")
//                     .map(|n| n.parse::<i32>().expect("should parse to int"))
//                     .collect()
//             })
//             .collect();
//         let walls = vertices_to_walls(&points);
//         let (up_walls, across_walls) = split_walls(&walls);
//         // convert this full map into T/F for where we're inside

//         pretty_print_walls(&walls);
//         assert_eq!(true , false);
//     }

}
