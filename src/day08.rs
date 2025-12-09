use std::fs::File;
use std::io::prelude::*;

fn distance(p1: &Vec<i32>, p2: &Vec<i32>) -> f64 {
    let dx = (p1[0] - p2[0]).abs() as f64;
    let dy = (p1[1] - p2[1]).abs() as f64;
    let dz = (p1[2] - p2[2]).abs() as f64;
    (dx*dx + dy*dy + dz*dz).sqrt()
}

fn process_day08(input: &str, connections_to_make: usize) -> (usize, i32) {
    let lines: Vec<&str> = input.lines().collect();
    let points: Vec<Vec<i32>> = lines.iter()
        .map(|x| {
            x.split(",")
                .map(|n| n.parse::<i32>().expect("should parse to int"))
                .collect()
        })
        .collect();
    // this is a more sortable format for the matrix (a sparse representation)
    let mut all_distances: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..points.len() {
        // only need to compute half the matrix
        // (and not the diagonal)
        for j in i+1..points.len() {
            let dist = distance(&points[i], &points[j]);
            all_distances.push((i, j, dist));
        }
    }
    all_distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    // now let's build up the connected components list
    let mut components: Vec<Vec<usize>> = Vec::new();
    // initialize every one as separate
    for i in 0..points.len() {
        components.push(vec![i]);
    }
    for i in 0..connections_to_make {
        let connection: (usize, usize, f64) = all_distances[i];
        let mut new_component: Vec<usize> = Vec::new();
        let mut other_components: Vec<Vec<usize>> = Vec::new();
        for component in components.iter() {
            // if either side of our connection is in the component,
            // we'll pull these into the new_component to merge and stick it on the end
            // otherwise, we'll push it onto the list
            if component.contains(&connection.0) || component.contains(&connection.1) {
                for vertex in component.iter() {
                    new_component.push(*vertex);
                }
            } else {
                other_components.push(component.clone());
            }
        }
        // now rebuild the components
        components = other_components;
        components.push(new_component);
    }
    // now let's get the size of the components, sort that list, and get the largest 3
    let mut component_sizes: Vec<usize> = components.iter().map(|x| x.len()).collect();
    component_sizes.sort();
    component_sizes.reverse();
    let total: usize = component_sizes[0] * component_sizes[1] * component_sizes[2];

    // now let's keep going until the thing is fully connected
    let mut fully_connected: bool = components.len() == 1;
    let mut connections_made = connections_to_make;
    while !fully_connected {
        let connection: (usize, usize, f64) = all_distances[connections_made];
        let mut new_component: Vec<usize> = Vec::new();
        let mut other_components: Vec<Vec<usize>> = Vec::new();
        for component in components.iter() {
            if component.contains(&connection.0) || component.contains(&connection.1) {
                for vertex in component.iter() {
                    new_component.push(*vertex);
                }
            } else {
                other_components.push(component.clone());
            }
        }
        components = other_components;
        components.push(new_component);

        connections_made += 1;
        fully_connected = components.len() == 1;
    }
    connections_made -= 1;
    let total_2 = points[all_distances[connections_made].0][0] * points[all_distances[connections_made].1][0];
    (total, total_2)
}

pub fn run(day: i8) {
    println!("---------  day {}  ----------", day);
    let file = File::open(format!("inputs/day{:02}.txt", day));
    let mut input = String::new();
    let _ = file.expect(&format!("file inputs/day{:02}.txt does not exist", day)).read_to_string(&mut input);
    let connections_to_make: usize = 1000;
    let (total, total_2) = process_day08(&input, connections_to_make);
    println!("total is {total}");
    println!("distance to the wall is {total_2}");
    println!("------- end of day {} -------\n", day);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_day08() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let (total, total_2) = process_day08(input, 10);
        assert_eq!(total, 40);
        assert_eq!(total_2, 25272);
    }
}
