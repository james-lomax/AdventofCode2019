use itertools::Itertools;

fn meets_criteria(val: i32) -> bool {
    let digits = val.to_string();
    
    let mut sorted : Vec<char> = digits.clone().chars().collect();
    sorted.sort();
    let sorted : String = sorted.drain(..).collect();

    if digits != sorted {
        return false;
    }

    for (k, v) in digits.chars().group_by(|c| c.clone()).into_iter() {
        if v.count() == 2 {
            return true;
        }
    }
    return false;
}

fn main() {
    let c = (134792..=675810).filter(|v| meets_criteria(*v)).count();
    println!("c={}", c);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(false, meets_criteria(223450));
        assert_eq!(false, meets_criteria(123789));
        assert_eq!(false, meets_criteria(111111));
        assert_eq!(true, meets_criteria(123445));
        assert_eq!(true, meets_criteria(566789));
        assert_eq!(false, meets_criteria(599999));
        assert_eq!(false, meets_criteria(123444));
        assert_eq!(false, meets_criteria(675678));
        assert_eq!(true, meets_criteria(111122));
    }
}