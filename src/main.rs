extern crate rand;

use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;
mod test;
/*
    The main fun_ction pasing a shuffled deck to with_permutation
    @param card_index is the indexes of each card from 1 to 52
    @param card_index shuffled deck of cards
*/
fn main() {
    let mut card_index: Vec<usize> = (0..52).collect();
    card_index.shuffle(&mut thread_rng());
    println!("{}",format_game(with_permutation(card_index)));
}
/*
 A player structure used in Game object
 */
pub struct Player{
    hand: Vec<String>,
}
/*
Constructor method that builds players
*/
impl Player {
pub fn build_player(hand: Vec<String>) -> Player{
    Player{
    hand,
}
}
// get number of players clubs
fn get_num_c(&self) -> i32{
    let mut num_c=0;
  for card in self.hand.iter() {
      if card.contains("C"){
          num_c = num_c + 1;
      }
  }
  num_c
}
// get number of players diamonds
fn get_num_d(&self) -> i32{
    let mut num_d=0;
  for card in self.hand.iter() {
      if card.contains("D"){
          num_d = num_d + 1;
      }
  }
  num_d
}
// get number of players hearts
fn get_num_h(&self) -> i32{
    let mut num_h=0;
  for card in self.hand.iter() {
      if card.contains("H"){
          num_h = num_h + 1;
      }
  }
  num_h
}
// get number of players spades
fn get_num_s(&self) -> i32{
    let mut num_s=0;
  for card in self.hand.iter() {
      if card.contains("S"){
          num_s = num_s + 1;
      }
  }
  num_s
}
// get number of players points
fn get_points(&self) -> i32{
    let mut points: i32 = 0;
for i in self.hand.iter(){
    if i.contains("J"){
        points = points + 1;
    }
    if i.contains("Q"){
        points = points + 2;
    }
    if i.contains("K"){
        points = points + 3;
    }
    if i.contains("A"){
        points = points + 4;
    }
}
if self.get_num_c() == 0{
    points = points + 3;
}
if self.get_num_d() == 0{
    points = points + 3;
}
if self.get_num_h() == 0{
    points = points + 3;
}
if self.get_num_s() == 0{
    points = points + 3;
}
if self.get_num_c() == 1{
    points = points + 2;
}
if self.get_num_d() == 1{
    points = points + 2;
}
if self.get_num_h() == 1{
    points = points + 2;
}
if self.get_num_s() == 1{
    points = points + 2;
}
if self.get_num_c() == 2{
    points = points + 1;
}
if self.get_num_d() == 2 {
    points = points + 1;
}
if self.get_num_h() == 2 {
    points = points + 1;
}
if self.get_num_s() == 2{
    points = points + 1;
}
points
}
// get largest best suit(suit with most cards)
fn get_largest_suit(&self) -> i32{
 let n_c: i32 = self.get_num_c();
 let n_d: i32 = self.get_num_d();
 let n_h: i32 = self.get_num_h();
 let n_s: i32 = self.get_num_s();
let num = vec![n_c, n_d, n_h, n_s];
let mut max = n_c;
for i in num.iter(){
    if i>&max{
        max = *i;
    }
}
max
}
fn get_nt(&self) -> bool{
    false
}
}
/*
Game is a structure that is returned by with_permutation method
*/
pub struct Game{
     north: Player,
     south: Player,
     east: Player,
     west: Player,
 }
impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        f.write_str("Game!\n")
    }
}
/*
permutation method that deals the cards
@return game object
*/
pub fn with_permutation(deck: Vec<usize>) -> Game {
    //The deck of cards
      let deck_of_cards: Vec<String>  = vec![
      "2C".to_string(),"3C".to_string(),"4C".to_string(),"5C".to_string(),"6C".to_string(),"7C".to_string(),
      "8C".to_string(),"9C".to_string(),"10C".to_string(),"JC".to_string(),"QC".to_string(),"KC".to_string(),"AC".to_string(),
       "2D".to_string(),"3D".to_string(),"4D".to_string(),"5D".to_string(),"6D".to_string(),"7D".to_string(),
       "8D".to_string(),"9D".to_string(),"10D".to_string(),"JD".to_string(),"QD".to_string(),"KD".to_string(),"AD".to_string(),
        "2H".to_string(),"3H".to_string(),"4H".to_string(),"5H".to_string(),"6H".to_string(),"7H".to_string(),
        "8H".to_string(),"9H".to_string(),"10H".to_string(),"JH".to_string(),"QH".to_string(),"KH".to_string(),"AH".to_string(),
         "2S".to_string(),"3S".to_string(),"4S".to_string(),"5S".to_string(),"6S".to_string(),"7S".to_string(),
          "8S".to_string(),"9S".to_string(),"10S".to_string(),"JS".to_string(),"QS".to_string(),"KS".to_string(),"AS".to_string()];
      //Initializing player card in_dexes
       let mut north_player: Vec<usize> = Vec::new();
       let mut south_player: Vec<usize> = Vec::new();
       let mut west_player: Vec<usize> = Vec::new();
       let mut east_player: Vec<usize> = Vec::new();
       //Initializing player cards
        let mut north: Vec<String> = Vec::new();
        let mut south: Vec<String> = Vec::new();
        let mut west: Vec<String> = Vec::new();
        let mut east: Vec<String> = Vec::new();
        //Handing cards to each player
        let mut i = 0;
        for index in 1..deck.len() {
            if i < deck.len(){
            west_player.push(deck[i]);
            i = i+1;
          }
          if i < deck.len(){
            north_player.push(deck[i]);
            i = i+1;
        }
          if i < deck.len(){
            east_player.push(deck[i]);
            i = i+1;
        }
          if i < deck.len(){
            south_player.push(deck[i]);
            i = i+1;
        }
      }
         north_player.sort();
         south_player.sort();
         east_player.sort();
         west_player.sort();
//Adding the cards to temporary player vectors
         for n in north_player.iter() {
             north.push(deck_of_cards[*n as usize].clone());
         }
         for s in south_player.iter() {
             south.push(deck_of_cards[*s as usize].clone());
         }
         for e in east_player.iter() {
             east.push(deck_of_cards[*e as usize].clone());
         }
         for w in west_player.iter() {
             west.push(deck_of_cards[*w as usize].clone());
         }
//Building players to add to Game object
let mut n_player = Player::build_player(north);
let mut s_player = Player::build_player(south);
let mut w_player = Player::build_player(west);
let mut e_player = Player::build_player(east);
//Initializing if players have opened yet
let mut n_player_open = false;
let mut w_player_open = false;
let mut e_player_open = false;
let mut s_player_open = false;
/*
Bellow is the initial implementation for opening for all player,
implementation is incomplete therefore it is commented out
//keep track of number of number of consecutive passes
let mut num_passes = 0;
game: while num_passes != 3{
    //opening for south_player
    if s_player_open == false{
        //player has more than 13 & partner hasnt opened
        if s_player.get_points() > 13 && n_player_open == false {
            //opened
            s_player_open = true;
            num_passes = 0;
            //process of openeing
            if s_player.get_largest_suit() >= 5 {
                if s_player.get_largest_suit() == s_player.get_num_c() {
                    game_process.push("1C".to_string());
                }
                else if s_player.get_largest_suit() == s_player.get_num_d() {
                    game_process.push("1D".to_string());
                }
                else if s_player.get_largest_suit() == s_player.get_num_h() {
                    game_process.push("1H".to_string());
                }
                else if s_player.get_largest_suit() == s_player.get_num_s() {
                    game_process.push("1S".to_string());
                }
            }
            else {
                if s_player.get_largest_suit() == s_player.get_num_s() {
                    game_process.push("1S".to_string());
                }
                else if s_player.get_largest_suit() == s_player.get_num_h() {
                    game_process.push("1H".to_string());
                }
                else if s_player.get_largest_suit() == s_player.get_num_d() {
                    game_process.push("1D".to_string());
                }
                else s_player.get_largest_suit() == s_player.get_num_c() {
                    game_process.push("1C".to_string());
                }
            }
        }
        //player has less than 13 points and partner hasnt opened
        else if s_player.get_points() < 13 && n_player_open == false{
            num_passes = num_passes + 1;
            board_string.push("PASS".to_string());
            }
        //player has less than 13 points but partner has answered
        else if s_player.get_points() < 13 && n_player_open == true{
            //answer
        }

    }
}
*/
//Building the game object
let game_board = Game {north: n_player, south:s_player, east: e_player, west:w_player};
game_board
}

/*
Formats the current board of the game and
 return string implementation of the Game
 @return string implementation of board
*/
pub fn format_game(game: Game) -> String {
 let n = game.north;
 let s = game.south;
 let w = game.west;
 let e = game.east;
 let mut board_string = String::from("");
 //Building the spaces and string for final result
 /*
 @param max_of_n_s is largest suit between south and north_player
 */
 let mut max_of_n_s = 0;
 if n.get_largest_suit() > s.get_largest_suit(){
     max_of_n_s = n.get_largest_suit();
 }
 else{
     max_of_n_s = s.get_largest_suit();
 }

 /*
 @param num_sfn is the "Spaces for north player"
 which is based on west player largest get_largest_suit
 */
 let mut num_sfn = ((w.get_largest_suit() + 2) * 2) - 1;
/*
@param num_sfer1 is the number of spaces required for east player
in the first row "r1".
one is deducted if the west player of the same suit has 10 since 10
has two characters
*/
 let mut num_sfer1 = ((max_of_n_s + 2) * 2) + (((w.get_largest_suit() - w.get_num_s()) * 2));
 if w.hand.iter().any(|x| x == "10S"){
     num_sfer1 = num_sfer1 - 1;
 }
 /*
 @param num_sfer3 is the number of spaces required for east player
 in the second row "r2".
 one is deducted if the west player of the same suit has 10 since 10
 has two characters
 */
 let mut num_sfer2 = ((max_of_n_s + 2) * 2) + (((w.get_largest_suit() - w.get_num_h()) * 2));
 if w.hand.iter().any(|x| x == "10H"){
     num_sfer2 = num_sfer2 - 1;
 }
 /*
 @param num_sfer3 is the number of spaces required for east player
 in the third row "r3".
 one is deducted if the west player of the same suit has 10 since 10
 has two characters
 */
 let mut num_sfer3 = ((max_of_n_s + 2) * 2) + (((w.get_largest_suit() - w.get_num_d()) * 2));
 if w.hand.iter().any(|x| x == "10D"){
      num_sfer3 = num_sfer3 - 1;
  }
  /*
  @param num_sfer4 is the number of spaces required for east player
  in the fourth row "r4".
  one is deducted if the west player of the same suit has 10 since 10
  has two characters
  */
  let mut num_sfer4 = ((max_of_n_s + 2) * 2) + (((w.get_largest_suit() - w.get_num_c()) * 2));
  if w.hand.iter().any(|x| x == "10C"){
       num_sfer4 = num_sfer4 - 1;
   }
/*
@param num_sfe is the number of spaces required for the word "EAST"
*/
let mut num_sfe = (num_sfn - 4) + (max_of_n_s * 2 + 2);
/*
@param sfe1, spaces for east player first row (Spades)
@param sfe2, spaces for east player second row (Hearts)
@param sfe3, spaces for east player third row (Diamonds)
@param sfe4, space for east player fouth row (Clubs)
*/
let mut sfe1 = String::from("");
let mut sfe2 = String::from("");
let mut sfe3 = String::from("");
let mut sfe4 = String::from("");
let mut sfe = String::from("");
let mut sfn = String::from("");
//Building the above mentioned Spaces
for i in 0..(num_sfer1-1) {
    sfe1.push(' ');
}
for i in 0..(num_sfer2-1) {
    sfe2.push(' ');
}
for i in 0..(num_sfer3-1) {
    sfe3.push(' ');
}
for i in 0..(num_sfer4-1) {
    sfe4.push(' ');
}
for i in 0..(num_sfe -1) {
    sfe.push(' ');
}
for i in 0..(num_sfn -1) {
    sfn.push(' ');
}
//Adding North player hand to String
board_string.push_str(&sfn);
board_string.push_str("NORTH\n");
board_string.push_str(&sfn);
board_string.push('S');
for each in n.hand.iter(){
    if each.contains("S"){
        board_string.push(' ');
        board_string.push_str(each.trim_matches('S'));
    }
}
//Adding hearts
board_string.push_str("\n");
board_string.push_str(&sfn);
board_string.push('H');
for each in n.hand.iter(){
    if each.contains("H"){
        board_string.push(' ');
        board_string.push_str(each.trim_matches('H'));
    }
}
//Adding diamonds
board_string.push_str("\n");
board_string.push_str(&sfn);
board_string.push('D');
for each in n.hand.iter(){
    if each.contains("D"){
        board_string.push(' ');
        board_string.push_str(each.trim_matches('D'));
    }
}
//Adding clubs
board_string.push_str("\n");
board_string.push_str(&sfn);
board_string.push('C');
for each in n.hand.iter(){
    if each.contains("C"){
        board_string.push(' ');
        board_string.push_str(each.trim_matches('C'));
    }
}
//End of adding North to String
//Adding East and West players to String
board_string.push_str("\n");
board_string.push_str("WEST");
board_string.push_str(&sfe);
board_string.push_str("EAST");
board_string.push_str("\n");
board_string.push('S');
for each in w.hand.iter(){
    if each.contains("S"){
        board_string.push(' ');
        board_string.push_str(each.trim_matches('S'));
    }
}
board_string.push_str(&sfe1);
board_string.push('S');
for each in e.hand.iter(){
    if each.contains("S"){
        board_string.push(' ');
        board_string.push_str(each.trim_matches('S'));
    }
}
board_string.push_str("\n");
board_string.push('H');
for each in w.hand.iter(){
    if each.contains("H"){
        board_string.push(' ');
        board_string.push_str(each.trim_matches('H'));
    }
}
board_string.push_str(&sfe2);
board_string.push('H');
for each in e.hand.iter(){
    if each.contains("H"){
        board_string.push(' ');
        board_string.push_str(each.trim_matches('H'));
    }
}
board_string.push_str("\n");
board_string.push('D');
for each in w.hand.iter(){
    if each.contains("D"){
        board_string.push(' ');
        board_string.push_str(each.trim_matches('D'));
    }
}
board_string.push_str(&sfe3);
board_string.push('D');
for each in e.hand.iter(){
    if each.contains("D"){
        board_string.push(' ');
        board_string.push_str(each.trim_matches('D'));
    }
}
board_string.push_str("\n");
board_string.push('C');
for each in w.hand.iter(){
    if each.contains("C"){
        board_string.push(' ');
        board_string.push_str(each.trim_matches('C'));
    }
}
board_string.push_str(&sfe4);
board_string.push('C');
for each in e.hand.iter(){
    if each.contains("C"){
        board_string.push(' ');
        board_string.push_str(each.trim_matches('C'));
    }
}
//End of adding west and east
//Add south player to string
board_string.push_str("\n");
board_string.push_str(&sfn);
board_string.push_str("SOUTH\n");
board_string.push_str(&sfn);
board_string.push('S');
for each in s.hand.iter(){
    if each.contains("S"){
        board_string.push(' ');
        board_string.push_str(each.trim_matches('S'));
    }
}
//Adding hearts
board_string.push_str("\n");
board_string.push_str(&sfn);
board_string.push('H');
for each in s.hand.iter(){
    if each.contains("H"){
        board_string.push(' ');
        board_string.push_str(each.trim_matches('H'));
    }
}
//Adding diamonds
board_string.push_str("\n");
board_string.push_str(&sfn);
board_string.push('D');
for each in s.hand.iter(){
    if each.contains("D"){
        board_string.push(' ');
        board_string.push_str(each.trim_matches('D'));
    }
}
//Adding clubs
board_string.push_str("\n");
board_string.push_str(&sfn);
board_string.push('C');
for each in s.hand.iter(){
    if each.contains("C"){
        board_string.push(' ');
        board_string.push_str(each.trim_matches('C'));
    }
}
//End of adding south player to string
board_string.push_str("\n");
board_string.push_str("SOUTH WEST NORTH EAST\nDeclarer:");
board_string
}
