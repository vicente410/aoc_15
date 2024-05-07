use std::fs;
use std::cmp::max;

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

struct Cookie {
    ingredients: Vec<Ingredient>,
    quantities: Vec<i32>,
}

impl Cookie {
    fn calc_score(&self) -> i32 {
        let mut cap = 0;
        let mut dur = 0;
        let mut fla = 0;
        let mut tex = 0;

        for i in 0..self.ingredients.len() {
            cap += self.ingredients[i].capacity * self.quantities[i];
            dur += self.ingredients[i].durability * self.quantities[i];
            fla += self.ingredients[i].flavor * self.quantities[i];
            tex += self.ingredients[i].texture * self.quantities[i];
        }
        
        if cap < 0 {cap = 0;}
        if dur < 0 {dur = 0;}
        if fla < 0 {fla = 0;}
        if tex < 0 {tex = 0;}
        cap * dur * fla * tex
    }

    fn get_calories(&self) -> i32 {
        let mut cal = 0;

        for i in 0..self.ingredients.len() {
            cal += self.ingredients[i].calories * self.quantities[i];
        }

        cal
    }

    fn best_cookie(&mut self, calories: i32) -> i32 {
        let mut best_score = 0;

        for a in 0..=100 {
            for b in a..=100 {
                for c in b..=100 {
                    self.quantities = vec![a, b - a, c - b, 100 - c];
                    if calories > 0 && self.get_calories() != calories {
                        continue;
                    }

                    best_score = max(self.calc_score(), best_score);
                }
            }
        }
        
        best_score
    }
}

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Failed to read file");
    let mut cookie = Cookie {
        ingredients: parse(input.trim()),
        quantities: vec![0, 0, 0, 0],
    };
    
    println!("Part 1: the best cookie scores {} points", cookie.best_cookie(0));
    println!("Part 2: with 500 calories it scores {} points", cookie.best_cookie(500));
}

fn parse(input: &str) -> Vec<Ingredient> {
    let mut ingredients = Vec::new();

    for line in input.split('\n') {
        let words: Vec<_> = line.split(' ').collect();
        ingredients.push(Ingredient {
            capacity: words[2][..words[2].len() - 1].parse().unwrap(), 
            durability: words[4][..words[4].len() - 1].parse().unwrap(),
            flavor: words[6][..words[6].len() - 1].parse().unwrap(),
            texture: words[8][..words[8].len() - 1].parse().unwrap(), 
            calories: words[10].parse().unwrap(),
        });
    }

    ingredients
}
