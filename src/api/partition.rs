use itertools::Itertools;

#[derive(Clone)]
struct Coord {
    latitude: f32,
    longitude: f32,
}

#[derive(Clone)]
struct Partition {
    items: Vec<Coord>,
}

#[derive(Clone)]
struct Partitions {
    items: Vec<Partition>,
}

fn main() {
    let temp: Vec<Coord> = (1..=9)
        .map(|i| Coord {
            latitude: (i as f32).powi(2) + 0.1,
            longitude: (i as f32).powi(2) as f32 + 0.2,
        })
        .collect();

    let n: usize = 3;

    print_coords(&temp);
    println!("{}", min_distance(&temp).unwrap());

    let temp2 = get_partitions(&temp, n);
    // print_partitions_permutations(&temp2);

    best_partition(&temp2)
}

fn print_coords(coords: &[Coord]) {
    coords
        .iter()
        .for_each(|coord| println!("({}, {})", coord.latitude, coord.longitude));
}

fn distance(coord1: &Coord, coord2: &Coord) -> f32 {
    ((coord1.latitude - coord2.latitude).powi(2) + (coord1.longitude - coord2.longitude).powi(2))
        .sqrt()
}

fn min_distance(coords: &[Coord]) -> Option<f32> {
    coords
        .iter()
        .combinations(2)
        .map(|pair| distance(&pair[0], &pair[1]))
        .fold(None, |min_dist, dist| {
            Some(match (min_dist, dist) {
                (None, dist) => dist,
                (Some(min_dist), dist) => min_dist.min(dist),
            })
        })
}

fn get_partitions(coords: &[Coord], n: usize) -> Vec<Partitions> {
    if n == 1 {
        // base case
        let single_partition = Partition {
            items: coords.to_vec(),
        };
        return vec![Partitions {
            items: vec![single_partition],
        }];
    }

    (1..=coords.len())
        .flat_map(|i| {
            let first_partition = Partition {
                items: coords[..i].to_vec(),
            };
            let remaining_coords = &coords[i..];
            get_partitions(remaining_coords, n - 1)
                .into_iter()
                .map(move |mut sub_partitions| {
                    sub_partitions.items.insert(0, first_partition.clone());
                    sub_partitions
                })
        })
        .collect()
}

fn print_partitions_permutations(partitions: &[Partitions]) {
    for (i, p) in partitions.iter().enumerate() {
        println!("Permutation {}:", i);
        print_partitions(p);
    }
}

fn print_partitions(partitions: &Partitions) {
    for (j, pp) in partitions.items.iter().enumerate() {
        println!("  Partition {}:", j);
        for (k, c) in pp.items.iter().enumerate() {
            println!("    Coord {}: ({}, {})", k, c.latitude, c.longitude);
        }
    }
}

fn best_partition(partitions_permutations: &[Partitions]) {
    for partitions in partitions_permutations.iter() {
        print_partitions(partitions);

        let mut distance_sum = 0.0;
        for partition in partitions.items.iter() {
            distance_sum += min_distance(&partition.items).unwrap_or(0.0);
        }
        println!("{}", distance_sum);
    }
}
