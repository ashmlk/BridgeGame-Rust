//Tester for testing correct output of program 
#[cfg(test)]
use crate::with_permutation;
use crate::format_game;
#[test]
fn t1() {
    assert_eq!(
        "          NORTH
          S 4 8 9 J Q
          H 9
          D 4 6 Q
          C 3 5 6 J
WEST                  EAST
S 3 6                 S 10 A
H 6 10 J K            H 2 3 4 5 8 A
D 2 5 J K             D 8 10 A
C 4 7 8               C 10 K
          SOUTH
          S 2 5 7 K
          H 7 Q
          D 3 7 9
          C 2 9 Q A\nSOUTH WEST NORTH EAST\nDeclarer:",
     format_game(with_permutation(vec![
        6, 41, 19, 14, 40, 15, 27, 36, 43, 23,
         28, 0, 30, 1, 8, 18, 13, 3, 29,
         20, 35, 9, 21, 39, 24, 17, 47, 10,
          22, 33, 25, 7, 34, 45, 32, 12, 37,
           49, 11, 50, 5, 46, 51, 44, 2, 48, 26,
            31, 16, 4, 38, 42])))
}
#[test]
fn t2() {
    assert_eq!(
        "            NORTH
            S 7 9
            H 4 6 10
            D 5 10 J
            C 3 6 7 Q K
WEST                    EAST
S 2 5 J K A             S 3 6 10
H 3 J                   H 5 8 9 Q K A
D 6 8 K                 D 2 Q
C 2 8 J                 C 4 5
            SOUTH
            S 4 8 Q
            H 2 7
            D 3 4 7 9 A
            C 9 10 A\nSOUTH WEST NORTH EAST\nDeclarer:",
            format_game(with_permutation(vec![
            27, 16, 43, 25, 35, 5, 3, 31, 0, 28, 40,
             8, 48, 4, 47, 26, 51, 44, 13, 14, 42,
             11, 33, 49, 9, 21, 38, 41, 19, 22,37,
             15, 6, 46, 23, 12, 50, 34, 32,18,
             39, 10, 36, 20, 17, 1, 29, 7, 24, 30, 2, 45])))
}
#[test]
fn t3() {
assert_eq!(
    "          North
          S Q 8 4
          H K 9 5
          D A 10 6 2
          C J 7 3
West                East
S J 7 3             S K 9 5
H Q 8 4             H A 10 6 2
D K 9 5             D J 7 3
C A 10 6 2          C Q 8 4
          South
          S A 10 6 2
          H J 7 3
          D Q 8 4
          C K 9 5\nSOUTH WEST NORTH EAST\nDeclarer:",
          format_game(with_permutation(from_north_east_south_west(
           "SQ S8 S4 HK H9 H5 DA D10 D6 D2 CJ C7 C3",
           "SK S9 S5 HA H10 H6 H2 DJ D7 D3 CQ C8 C4",
           "SA S10 S6 S2 HJ H7 H3 DQ D8 D4 CK C9 C5",
           "SJ S7 S3 HQ H8 H4 DK D9 D5 CA C10 C6 C2"))))
}
#[test]
fn check_to_indices() {
    assert_eq!(vec![2, 3, 49, 50, 51],to_indices("SA SK SQ C4 C3"))
}
fn to_indices(cards: &str) -> Vec<usize> {
    let deck : Vec<&str> = "C2 C3 C4 C5 C6 C7 C8 C9 C10 CJ CQ CK CA D2 D3 D4 D5 D6 D7 D8 D9 D10 DJ DQ DK DA H2 H3 H4 H5
H6 H7 H8 H9 H10 HJ HQ HK HA S2 S3 S4 S5 S6 S7 S8 S9 S10 SJ SQ SK SA".split(" ").collect();
    let cards = cards.split(" ");
    let mut indices : Vec<usize> = cards.map(|card| deck.iter().position(|&r| r == card).unwrap()+1).collect();
    indices.sort();
    indices
}
fn from_north_east_south_west(north:&str,east:&str,south:&str,west:&str) -> Vec<usize> {
    let mut result : Vec<usize> = Vec::new();
    let north = to_indices(north);
    let east = to_indices(east);
    let south = to_indices(south);
    let west = to_indices(west);
    for i in 0..west.len() {
        result.push(west[i]);
        result.push(north[i]);
        result.push(east[i]);
        result.push(south[i])
    }
    result
}
