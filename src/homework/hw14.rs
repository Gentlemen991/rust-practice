use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point, // верхній лівий
    b: Point, // нижній правий
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut covered: HashSet<Point> = HashSet::new();

    for rect in xs {
        let x1 = rect.a.x.min(rect.b.x);
        let x2 = rect.a.x.max(rect.b.x);
        let y1 = rect.a.y.min(rect.b.y);
        let y2 = rect.a.y.max(rect.b.y);

        for x in x1..x2 {
            for y in y1..y2 {
                covered.insert(Point { x, y });
            }
        }
    }

    covered.len() as i32
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

#[test]
fn area_occupied_test() {
    let data = test_data();
    let result = area_occupied(&data);
    assert_eq!(result, 60);
}