#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub pulls: Vec<Pull>
}

#[derive(Debug)]
pub struct Pull {
    pub total_red: u32,
    pub total_green: u32,
    pub total_blue: u32,
}

impl Pull {
    pub fn from_str(input: &String) -> Pull {
        let mut pull = Pull {
            total_red: 0,
            total_green: 0,
            total_blue: 0
        };

        let colors = input
            .split(", ")
            .collect::<Vec<&str>>();

        for color in colors {
            let num = color.split_whitespace().nth(0).unwrap().parse::<u32>().unwrap();
            let color = color.split_whitespace().nth(1).unwrap();

            match color {
                "red" => { pull.total_red = num },
                "green" => { pull.total_green = num },
                "blue" => { pull.total_blue = num }
                _ => panic!("unexpected color")
            };
        }

        pull
    }
}