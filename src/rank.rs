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
use csv::StringRecord;
//

#[derive(Debug, PartialEq)]
struct Resident {
    id: u8,
    rank_list: Vec<u8>
}

impl Resident {
    fn from_record(r: StringRecord) -> Self {
        let mut rank_list: Vec<u8> = Vec::new();
        let id = r.get(0).unwrap().parse::<u8>().unwrap();
        for i in 1..r.len() {
            rank_list.push(r[i].to_string().parse::<u8>().unwrap());
        }
        return Self {
            id: id,
            rank_list: rank_list
        }
    }
}

pub fn read_csv() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_to_resident() {
        let r = StringRecord::from(vec!["1", "10", "4"]);
        let r1 = Resident::from_record(r);
        assert_eq!(r1, Resident {
            id: 1,
            rank_list: vec![10, 4]
        })
    }

    // #[test]
    // fn test_ojective() {
    //     let r1 = Resident {
    //         id: 1,
    //         rank_list: Vec::from([1, 2]),
    //         track: 0
    //     };
    // }

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