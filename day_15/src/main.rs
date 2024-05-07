use std::fs;
use std::cmp::max;

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Failed to read file");
    let ingredients = parse(input.trim());
    
    println!("Part 1: the best cookie scores {} points", find_cookie(&ingredients, 0));
    println!("Part 2: with 500 calories it scores {} points", find_cookie(&ingredients, 500));
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut ingredients = Vec::new();

    for line in input.split('\n') {
        let words: Vec<_> = line.split(' ').collect();
        let mut ing = Vec::new();

        ing.push(words[2][..words[2].len() - 1].parse().unwrap()); 
        ing.push(words[4][..words[4].len() - 1].parse().unwrap());
        ing.push(words[6][..words[6].len() - 1].parse().unwrap());
        ing.push(words[8][..words[8].len() - 1].parse().unwrap());
        ing.push(words[10].parse().unwrap());

        ingredients.push(ing);
    }

    ingredients
}

fn find_cookie(ingredients: &Vec<Vec<i32>>, target_calories: i32) -> i32 {
    let mut best_score = 0;

    for a in 0..=100 {
        for b in a..=100 {
            for c in b..=100 {
                let quantities = vec![a, b - a, c - b, 100 - c];
                let (score, calories) = calc_stats(&ingredients, quantities);

                if target_calories > 0 && calories != target_calories {
                    continue;
                }

                best_score = max(score, best_score);
            }
        }
    }
    
    best_score
}

fn calc_stats(ingredients: &Vec<Vec<i32>>, quantities: Vec<i32>) -> (i32, i32) {
        let mut scores: Vec<i32> = vec![0; ingredients[0].len()];
        let mut score = 1;
        let calories;

        for i in 0..ingredients.len() {
            for j in 0..ingredients[0].len() {
                scores[j] += ingredients[i][j] * quantities[i];
            }
        }

        calories = scores.pop().unwrap();

        for s in scores {
            score *= max(s, 0);
        }

        (score, calories)
    }

