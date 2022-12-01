type FoodList = Vec<Vec<u32>>;

fn read_food(input: &String) -> FoodList {
    let vectored: Vec<&str> = input.trim().lines().collect();
    let grouped: Vec<&[&str]> = vectored.split(|s| *s == "").collect();
    grouped.iter()
        .map(|v| v.iter()
            .map(|s| s.parse::<u32>().expect("Input must be numeric")).collect()).collect()
}

fn find_max(list: FoodList) -> (usize, u32){
    let maxes: Vec<(usize, u32)> = list.iter().map(|l| l.iter().sum()).enumerate().collect();

    *maxes.iter().max_by_key(|(_, v)| v).expect("A maximum should exist")
}

pub fn find_most_food(input: &String) -> (usize, u32) {
    let food = read_food(input);
    find_max(food)
}

pub fn find_top_three_holders(input: &String) -> (u32, Vec<u32>) {
    let food = read_food(input);
    find_top_three(food)
}

fn find_top_three(list: FoodList) -> (u32, Vec<u32>){
    let mut maxes: Vec<u32> = list.iter().map(|l| l.iter().sum()).collect();

    maxes.sort();
    let n: Vec<u32> = maxes.iter().rev().take(3).map(|v|*v).collect();

    let sum: u32 = n.iter().sum();

    (sum, n)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_part_one() {
        let sample = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"
        .to_string();

        let (number, value) = find_most_food(&sample);

        assert_eq!(number, 3);
        assert_eq!(value, 24000);
    }

    #[test]
    fn sample_part_two() {
        let sample = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"
        .to_string();

        let (total, top_three) = find_top_three_holders(&sample);

        assert_eq!(total, 45000);
        assert_eq!(top_three.get(0), Some(&24000));
        assert_eq!(top_three.get(1), Some(&11000));
        assert_eq!(top_three.get(2), Some(&10000));

    }
}