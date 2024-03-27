/*
Rank list optimizer

1. Pair resident with a track, (doesn't matter how, so we just choose the range)
2. Step though permutations of pairs of residents.
3. If a switch causes a decrease in the ojbective function, perform the swap, then
   perform step 2 again.
4. If step 2 completes without a swap return the order.
Ojective function = sum of the ranks.
*/
use std::{error::Error, io, mem::swap};
use csv::StringRecord;
//

#[derive(Debug, PartialEq)]
pub struct Resident {
    id: i32,
    rank_list: Vec<i32>
}

impl Resident {
    fn from_record(r: &StringRecord) -> Self {
        let mut rank_list: Vec<i32> = Vec::new();
        let id = r.get(0).unwrap().parse::<i32>().unwrap();
        for i in 1..r.len() {
            rank_list.push(r[i].to_string().parse::<i32>().unwrap());
        }
        Self {
            id: id,
            rank_list: rank_list
        }
    }
}

fn element_to_track(element: usize) -> i32 {
    i32::try_from(element).unwrap() + 1
}
fn get_track_position_on_rank_list(track: i32, rank_list: &Vec<i32>) -> i32 {
    rank_list.iter().position(|&x| x == track).unwrap() as i32
}
impl Resident {
    fn score(&self, element: usize) -> i32 {
        let track = element_to_track(element);
        let position = get_track_position_on_rank_list(track, &self.rank_list);
        return position*position;
    }
}
pub fn read_csv() -> Result<Vec<Resident>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut residents: Vec<Resident> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        residents.push(Resident::from_record(&record));
    }
    println!("{:?}", residents);
    Ok(residents)
}
fn generate_unique_pairs(n: usize) -> Vec<(usize, usize)> {
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for i in 0..n {
        for j in i+1..n {
            pairs.push((i, j));
        }
    }
    pairs
}
fn swap_residents_if_better(residents: &mut Vec<Resident>, pair: (usize, usize)) -> bool {
    let r1_old = residents[pair.0].score(pair.0);
    let r2_old = residents[pair.1].score(pair.1);
    let old_score = r1_old + r2_old;
    let r1_new = residents[pair.1].score(pair.0);
    let r2_new = residents[pair.0].score(pair.1);
    let new_score = r1_new + r2_new;
    if new_score < old_score {
        residents.swap(pair.0, pair.1);
        return true;
    }
    return false;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_to_resident() {
        let r = StringRecord::from(vec!["1", "1", "2"]);
        let r1 = Resident::from_record(&r);
        assert_eq!(r1, Resident {
            id: 1,
            rank_list: vec![1, 2]
        })
    }

    #[test]
    fn test_ojective() {
        // Note the best match will have a value of 0
        // The second best match will have a value of 1, then 4, 9, 16, 25, 36, 49, 64, 81, 100
        let r = StringRecord::from(vec!["1", "2", "1", "3"]);
        let r1 = Resident::from_record(&r);
        assert_eq!(r1.score(0), 1);
        assert_eq!(r1.score(1), 0);
        assert_eq!(r1.score(2), 4);
    }

    #[test]
    fn test_swap() {
        // Create a list of residents
        // Swap the first two residents
        let r = StringRecord::from(vec!["1", "2", "1", "3"]);
        let r1 = Resident::from_record(&r);
        let r = StringRecord::from(vec!["2", "1", "1", "3"]);
        let r2 = Resident::from_record(&r);
        let mut rs = vec![r1, r2];
        rs.swap(0, 1);
        assert_eq!(rs[0].id, 2);
        assert_eq!(rs[1].id, 1);
    }

    #[test]
    fn test_swap_residents_if_better() {
        // Create a list of residents
        // r1 would prefer to be in the second position
        // r2 would prefer to be in the first position
        // The swap should be made
        let r = StringRecord::from(vec!["1", "2", "1", "3"]);
        let r1 = Resident::from_record(&r);
        let r = StringRecord::from(vec!["2", "1", "2", "3"]);
        let r2 = Resident::from_record(&r);
        let mut residents = vec![r1, r2];
        let pair: (usize, usize) = (0, 1);
        let result = swap_residents_if_better(&mut residents, pair);
        assert_eq!(residents[0].id, 2);
        assert_eq!(residents[1].id, 1);
        assert_eq!(result, true);
    }

    #[test]
    fn test_swap_residents_if_better1() {
        // Create a list of residents
        // r1 would prefer to be in the first position
        // r2 would prefer to be in the second position
        // The swap should not be made
        let r = StringRecord::from(vec!["1", "1", "2", "3"]);
        let r1 = Resident::from_record(&r);
        let r = StringRecord::from(vec!["2", "2", "1", "3"]);
        let r2 = Resident::from_record(&r);
        let mut residents = vec![r1, r2];
        let pair: (usize, usize) = (0, 1);
        let result = swap_residents_if_better(&mut residents, pair);
        assert_eq!(residents[0].id, 1); 
        assert_eq!(residents[1].id, 2);
    }


    // #[test]
    // fn test_lmr() {
    //     assert_eq!(left_middle_right("2B"), Ok("Left"));
    //     assert_eq!(left_middle_right("58I"), Err("No Seat"));
    //     assert_eq!(left_middle_right("60D"), Ok("Middle"));
    //     assert_eq!(left_middle_right("17K"), Ok("Right"));
    // }

    // #[test]
    // fn test_basic() {
    //     assert_eq!(plane_seat("2B"), "Front-Left");
    //     assert_eq!(plane_seat("20B"), "Front-Left");
    //     assert_eq!(plane_seat("58I"), "No Seat!!");
    //     assert_eq!(plane_seat("60D"), "Back-Middle");
    //     assert_eq!(plane_seat("17K"), "Front-Right");
    // }
}