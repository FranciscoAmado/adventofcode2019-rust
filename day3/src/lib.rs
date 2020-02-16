// --- Day 3: Crossed Wires ---
// The gravity assist was successful, and you're well on your way to the Venus refuelling station. During the rush back on Earth, the fuel management system wasn't completely installed, so that's next on the priority list.

// Opening the front panel reveals a jumble of wires. Specifically, two wires are connected to a central port and extend outward on a grid. You trace the path each wire takes as it leaves the central port, one wire per line of text (your puzzle input).

// The wires twist and turn, but the two wires occasionally cross paths. To fix the circuit, you need to find the intersection point closest to the central port. Because the wires are on a grid, use the Manhattan distance for this measurement. While the wires do technically cross right at the central port where they both start, this point does not count, nor does a wire count as crossing with itself.

// For example, if the first wire's path is R8,U5,L5,D3, then starting from the central port (o), it goes right 8, up 5, left 5, and finally down 3:

// ...........
// ...........
// ...........
// ....+----+.
// ....|....|.
// ....|....|.
// ....|....|.
// .........|.
// .o-------+.
// ...........
// Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down 4, and left 4:

// ...........
// .+-----+...
// .|.....|...
// .|..+--X-+.
// .|..|..|.|.
// .|.-X--+.|.
// .|..|....|.
// .|.......|.
// .o-------+.
// ...........
// These wires cross at two locations (marked X), but the lower-left one is closer to the central port: its distance is 3 + 3 = 6.

// Here are a few more examples:

// R75,D30,R83,U83,L12,D49,R71,U7,L72
// U62,R66,U55,R34,D71,R55,D58,R83 = distance 159
// R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
// U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135
// What is the Manhattan distance from the central port to the closest intersection?

#[derive(PartialEq, Debug, Clone)]
pub struct Coordinate {
    x: i16,
    y: i16,
}

impl Coordinate {
    fn distance(&self) -> i16 {
        return self.x + self.y;
    }
}

// Find the Coordinate intersection between two slices of Moves.
// Moves can be represented as a String with the format "<direction><count>"
// where <direction> can be "U" (up), "D" (down), "R" (right), "L" (left)
pub fn intersection(rhs: &[&str], lhs: &[&str]) -> Coordinate {
    // TODO:
    // - Parse each slice into a vector of Coordinates
    // - Find intersections points between the slices
    // - Reduce to the closest coordinate
    let rcoord = parse(&rhs);
    let lcoord = parse(&lhs);
    let frcoord: Vec<Coordinate> = rcoord.iter().flat_map(|arr| arr.iter()).cloned().collect();
    let flcoord: Vec<Coordinate> = lcoord.iter().flat_map(|arr| arr.iter()).cloned().collect();

    let intersect: Vec<Coordinate> = frcoord
        .into_iter()
        .filter(|el| flcoord.contains(el) == true)
        .collect();

    if let Some(first) = intersect.first() {
        return intersect[1..].into_iter().fold(first.clone(), |pt, el| {
            if pt.distance() > el.distance() {
                return el.clone();
            }

            return pt;
        });
    }

    return Coordinate { x: 0, y: 0 };
}

// Parse a Slice of Moves into a Coordinates Matrix
fn parse(moves: &[&str]) -> Vec<Vec<Coordinate>> {
    let mut cursor = Coordinate { x: 0, y: 0 };

    let coordinates: Vec<Vec<Coordinate>> = moves
        .into_iter()
        .map(|item| {
            let (dir, val) = item.split_at(1);
            let value: i16 = val.parse().unwrap();

            let line: Vec<Coordinate> = (1..=value)
                .map(|i| {
                    return match dir {
                        "U" => Coordinate {
                            x: cursor.x,
                            y: cursor.y + i,
                        },
                        "D" => Coordinate {
                            x: cursor.x,
                            y: cursor.y - i,
                        },
                        "R" => Coordinate {
                            x: cursor.x + i,
                            y: cursor.y,
                        },
                        "L" => Coordinate {
                            x: cursor.x - i,
                            y: cursor.y,
                        },
                        _ => Coordinate {
                            x: cursor.x,
                            y: cursor.y,
                        },
                    };
                })
                .collect();

            if let Some(last) = line.last() {
                cursor = last.clone();
            }

            return line;
        })
        .collect();

    return coordinates;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_parses_moves_into_matrix() {
        let moves: [&str; 4] = ["R8", "U5", "L5", "D3"];
        let result = parse(&moves);
        let expected: Vec<Vec<Coordinate>> = vec![
            vec![
                Coordinate { x: 1, y: 0 },
                Coordinate { x: 2, y: 0 },
                Coordinate { x: 3, y: 0 },
                Coordinate { x: 4, y: 0 },
                Coordinate { x: 5, y: 0 },
                Coordinate { x: 6, y: 0 },
                Coordinate { x: 7, y: 0 },
                Coordinate { x: 8, y: 0 },
            ],
            vec![
                Coordinate { x: 8, y: 1 },
                Coordinate { x: 8, y: 2 },
                Coordinate { x: 8, y: 3 },
                Coordinate { x: 8, y: 4 },
                Coordinate { x: 8, y: 5 },
            ],
            vec![
                Coordinate { x: 7, y: 5 },
                Coordinate { x: 6, y: 5 },
                Coordinate { x: 5, y: 5 },
                Coordinate { x: 4, y: 5 },
                Coordinate { x: 3, y: 5 },
            ],
            vec![
                Coordinate { x: 3, y: 4 },
                Coordinate { x: 3, y: 3 },
                Coordinate { x: 3, y: 2 },
            ],
        ];
        assert_eq!(result, expected);
    }
    #[test]
    fn it_calculates_intersection_distance_simple() {
        let wire1: [&str; 4] = ["R8", "U5", "L5", "D3"];
        let wire2: [&str; 4] = ["U7", "R6", "D4", "L4"];
        let coordinate = intersection(&wire1, &wire2);
        let result: i16 = coordinate.distance();
        let expected: i16 = 6; // distance 6
        assert_eq!(result, expected);
    }
    #[test]
    fn it_calculates_intersection_distance_complex1() {
        // FIXME: This is failling.. Why??
        let wire1: [&str; 9] = ["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
        let wire2: [&str; 8] = ["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];
        let coordinate = intersection(&wire1, &wire2);
        let result: i16 = coordinate.distance();
        let expected: i16 = 159; // distance 159
        assert_eq!(result, expected);
    }

    #[test]
    fn it_calculates_intersection_distance_complex2() {
        let wire1: [&str; 11] = [
            "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
        ];
        let wire2: [&str; 10] = [
            "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
        ];
        let coordinate = intersection(&wire1, &wire2);
        let result: i16 = coordinate.distance();
        let expected: i16 = 135; // distance 135
        assert_eq!(result, expected);
    }
}
