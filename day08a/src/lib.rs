fn distance(p1: &Vec<i32>, p2: &Vec<i32>) -> f64 {
    let dx = (p1[0] - p2[0]).abs() as f64;
    let dy = (p1[1] - p2[1]).abs() as f64;
    let dz = (p1[2] - p2[2]).abs() as f64;
    (dx*dx + dy*dy + dz*dz).sqrt()
}

pub fn process(input: &str, connections_to_make: usize) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let points: Vec<Vec<i32>> = lines.iter()
        .map(|x| {
            x.split(",")
                .map(|n| n.parse::<i32>().expect("should parse to int"))
                .collect()
        })
        .collect();
    let mut all_distances: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let dist = distance(&points[i], &points[j]);
            all_distances.push((i, j, dist));
        }
    }
    all_distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let mut components: Vec<Vec<usize>> = Vec::new();
    for i in 0..points.len() {
        components.push(vec![i]);
    }
    for i in 0..connections_to_make {
        let connection: (usize, usize, f64) = all_distances[i];
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
    }
    let mut component_sizes: Vec<usize> = components.iter().map(|x| x.len()).collect();
    component_sizes.sort();
    component_sizes.reverse();
    component_sizes[0] * component_sizes[1] * component_sizes[2]
}

pub fn main() {
    let input = include_str!("../input.txt");
    println!("{}", process(input, 1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day08a() {
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
        let total = process(input, 10);
        assert_eq!(total, 40);
    }
}
