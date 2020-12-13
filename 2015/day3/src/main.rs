use std::collections::HashSet;

fn main() {
    {
        let mut seen = HashSet::new();
        seen.insert((0, 0));

        let s = include_str!("../in.txt");

        let mut x = 0;
        let mut y = 0;

        for c in s.chars() {
            let m = match c {
                '>' => (1, 0),
                '<' => (-1, 0),
                '^' => (0, 1),
                'v' => (0, -1),
                _ => (0, 0),
            };

            x += m.0;
            y += m.1;

            seen.insert((x, y));
        }

        println!("Part 1: {}", seen.len());
    }

    {
        let mut santa = HashSet::new();
        let mut robo = HashSet::new();
        santa.insert((0, 0));
        robo.insert((0, 0));

        let s = include_str!("../in.txt");

        let mut santa_x = 0;
        let mut santa_y = 0;
        let mut robo_x = 0;
        let mut robo_y = 0;

        for (i, c) in s.chars().enumerate() {
            let m = match c {
                '>' => (1, 0),
                '<' => (-1, 0),
                '^' => (0, 1),
                'v' => (0, -1),
                _ => (0, 0),
            };

            if i % 2 == 0 {
                santa_x += m.0;
                santa_y += m.1;
                santa.insert((santa_x, santa_y));
            } else {
                robo_x += m.0;
                robo_y += m.1;
                santa.insert((robo_x, robo_y));
            }
        }

        println!(
            "Part 2: {} + {} = {}",
            santa.len(),
            robo.len(),
            santa.len()
        );
    }
}
