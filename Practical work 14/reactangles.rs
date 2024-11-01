use std::cmp::{max, min};

struct Point { x: i32, y: i32 }
struct Rectangle { a: Point, b: Point }
struct Event { x: i32, y1: i32, y2: i32, is_open: bool }

fn area_occupied(rects: &Vec<Rectangle>) -> i32 {
	let mut events: Vec<_> = rects.iter().flat_map(|r| vec![
    	Event { x: r.a.x, y1: min(r.a.y, r.b.y), y2: max(r.a.y, r.b.y), is_open: true },
    	Event { x: r.b.x, y1: min(r.a.y, r.b.y), y2: max(r.a.y, r.b.y), is_open: false }
	]).collect();

	events.sort_by_key(|e| e.x);
	let mut active = Vec::new();
	let mut total_area = 0;
	let mut prev_x = 0;

	for event in events {
    	let y_coverage = active.iter().fold((0, -1), |(sum, last_y), &(y1, y2)| {
        	let new_y = max(last_y, y1);
        	if y2 > new_y { (sum + y2 - new_y, y2) } else { (sum, new_y) }
    	}).0;

    	if event.x != prev_x {
        	total_area += y_coverage * (event.x - prev_x);
        	prev_x = event.x;
    	}

    	if event.is_open {
        	active.push((event.y1, event.y2));
        	active.sort();
    	} else {
        	active.retain(|&(y1, y2)| y1 != event.y1 || y2 != event.y2);
    	}
	}

	total_area
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

fn main() {
	let occupied = area_occupied(&test_data());
	assert_eq!(occupied, 60);
	println!("Test passed! Area: {}", occupied);
}