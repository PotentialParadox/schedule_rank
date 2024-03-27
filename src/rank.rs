/*
Rank list optimizer

1. Pair resident with a track, (doesn't matter how, so we just choose the range)
2. Step though permutations of pairs of residents.
3. If a switch causes a decrease in the ojbective function, perform the swap, then
   perform step 2 again.
4. If step 2 completes without a swap return the order.
Ojective function = sum of the ranks.
*/
use std::{error::Error, io};
use csv::{Position, StringRecord};
//

#[derive(Debug, PartialEq)]
pub struct Resident {
    id: i32,
    rank_list: Vec<i32>,
    track: Option<i32>
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
            rank_list: rank_list,
            track: None
        }
    }
}

fn element_to_track(element: usize) -> i32 {
    i32::try_from(element).unwrap() + 1
}
fn get_track_position_on_rank_list(track: i32, rank_list: &Vec<i32>) -> i32 {
    let result = rank_list.iter().position(|&x| x == track);
    match result {
        Some(x) => return i32::try_from(x).unwrap(),
        None => {
            panic!("Track {:?} not found in rank list {:?}", track, rank_list)
        }
    }
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
pub fn optimize(residents: &mut Vec<Resident>) {
    let n = residents.len();
    let mut pairs = generate_unique_pairs(n);
    loop {
        let mut swapped = false;
        for pair in pairs.iter() {
            if swap_residents_if_better(residents, *pair) {
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
pub fn assign_tracks(residents: &mut Vec<Resident>) {
    for i in 0..residents.len() {
        residents[i].track = Some(element_to_track(i));
    }
}
fn sort_residents_by_id(residents: &mut Vec<Resident>) {
    residents.sort_by(|a, b| a.id.cmp(&b.id));
}

pub fn print_residents(residents: &mut Vec<Resident>) {
    sort_residents_by_id(residents);
    println!("id, track, track_position");
    for r in residents {
        let track_position = get_track_position_on_rank_list(r.track.unwrap(), &r.rank_list);
        println!("{:?},{:?},{:?}", r.id, r.track.unwrap(), track_position);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_residents_by_id() {
        // Create a list of residents
        // Sort the list by id
        // The list should be sorted by id
        let r = StringRecord::from(vec!["2", "1", "2", "3"]);
        let r1 = Resident::from_record(&r);
        let r = StringRecord::from(vec!["1", "2", "1", "3"]);
        let r2 = Resident::from_record(&r);
        let mut residents = vec![r1, r2];
        sort_residents_by_id(&mut residents);
        assert_eq!(residents[0].id, 1);
        assert_eq!(residents[1].id, 2);
    }

    #[test]
    fn test_assign_tracks() {
        // Create a list of residents
        // Assign tracks to the residents
        // The tracks should be assigned in order
        let r = StringRecord::from(vec!["1", "1", "2"]);
        let r1 = Resident::from_record(&r);
        let r = StringRecord::from(vec!["2", "2", "1"]);
        let r2 = Resident::from_record(&r);
        let mut residents = vec![r1, r2];
        assign_tracks(&mut residents);
        assert_eq!(residents[0].track, Some(1));
        assert_eq!(residents[1].track, Some(2));
    }

    #[test]
    fn test_optimize1() {
        // Create a list of residents
        // Optimize the list
        // The list should be sorted by rank
        // We have a tie for first in this one
        // Resident 1 really doesn't want the second track, so
        // the list should be [2, 1, 3]
        let r = StringRecord::from(vec!["1", "1", "3", "2"]);
        let r1 = Resident::from_record(&r);
        let r = StringRecord::from(vec!["2", "1", "2", "3"]);
        let r2 = Resident::from_record(&r);
        let r = StringRecord::from(vec!["3", "3", "1", "2"]);
        let r3 = Resident::from_record(&r);
        let mut residents = vec![r1, r2, r3];
        optimize(&mut residents);
        assert!(residents[0].id == 1); // The resident's first choice
        assert!(residents[1].id == 2); // The resident's second choice
        assert!(residents[2].id == 3); // The resident's first choice
    }

    #[test]
    fn test_optimize() {
        // Create a list of residents
        // Optimize the list
        // The list should be sorted by rank
        let r = StringRecord::from(vec!["1", "2", "1", "3"]);
        let r1 = Resident::from_record(&r);
        let r = StringRecord::from(vec!["2", "1", "2", "3"]);
        let r2 = Resident::from_record(&r);
        let r = StringRecord::from(vec!["3", "3", "1", "2"]);
        let r3 = Resident::from_record(&r);
        let mut residents = vec![r1, r2, r3];
        optimize(&mut residents);
        assert!(residents[0].id == 2);
        assert!(residents[1].id == 1);
        assert!(residents[2].id == 3);
    }

    #[test]
    fn test_get_track_position_on_rank_list() {
        let rank_list = vec![1, 2, 3];
        assert_eq!(get_track_position_on_rank_list(1, &rank_list), 0);
        assert_eq!(get_track_position_on_rank_list(2, &rank_list), 1);
        assert_eq!(get_track_position_on_rank_list(3, &rank_list), 2);
    }

    #[test]
    fn test_element_to_track() {
        assert_eq!(element_to_track(0), 1);
        assert_eq!(element_to_track(1), 2);
        assert_eq!(element_to_track(2), 3);
    }

    #[test]
    fn test_generate_unique_pairs() {
        let pairs = generate_unique_pairs(3);
        assert_eq!(pairs, vec![(0, 1), (0, 2), (1, 2)]);
    }

    #[test]
    fn test_record_to_resident() {
        let r = StringRecord::from(vec!["1", "1", "2"]);
        let r1 = Resident::from_record(&r);
        assert_eq!(r1, Resident {
            id: 1,
            rank_list: vec![1, 2],
            track: None
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