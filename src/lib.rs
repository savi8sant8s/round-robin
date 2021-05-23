///## Tournament Round-robin
///Create a vector of rounds and their respective matches in a tournament, 
///using the Round-robin algorithm.
///
///<strong>Accepted</strong>:
///- generic data types;
///- number of even players;
///- round-trip.
/// 
///#### Examples with numbers:
///```
///let rounds1 = round_robin((1..= 4).collect());
///print!("{:?}", rounds1);
///```
///Output: [[(1,4), (2,3)], [(1,3), (4,2)], [(1,2), (3,4)]] | 
///
///#### Examples with strings:
///```
///let rounds2 = round_robin(vec!["a", "b", "c", "d"]);
///print!("{:?}", rounds2);
///```
///Output: [[("a", "d"), ("b", "c")], [("a", "c"), ("d", "b")], [("a", "b"), ("c", "d")]]
pub fn round_robin<T: Clone + Eq>(players: Vec<T>, round_trip: bool) -> Vec<Vec<(T, T)>> {
    let mut rounds = generate_rounds(players.clone(), false, vec![]);
    if round_trip{
        let mut two_turns = generate_rounds(players, round_trip, vec![]);
        rounds.append(&mut two_turns);
    }
    rounds
}

fn generate_rounds<T: Clone + Eq>(
    mut players: Vec<T>,
    round_trip: bool,
    mut rounds: Vec<Vec<(T, T)>>,
) -> Vec<Vec<(T, T)>> {
    if players.len() % 2 != 0 {
        return vec![];
    }

    let mut round = Vec::new();

    for n in 0..(players.len() / 2) {
        if round_trip{
            round.push((players[players.len() - n - 1].clone(), players[n].clone()));
        }
        else{
            round.push((players[n].clone(), players[players.len() - n - 1].clone()));
        }
    }
    rounds.push(round);

    if rounds.len() == players.len() - 1{
        return rounds;
    }

    players[1..].rotate_right(1);
    generate_rounds(players, round_trip, rounds)
}

#[cfg(test)]
pub mod tests{
    use super::*;


    fn generate_current_rounds(last: isize, round_trip: bool)-> Vec<Vec<(isize, isize)>> {
        round_robin((1..=last).collect(), round_trip)
    }

    #[test]
    fn count_of_rounds_equal_to_count_of_players_minus_1_or_double() {
        let one_turn = generate_current_rounds(20, false);
        let two_turns = generate_current_rounds(20, true);
        assert_eq!(one_turn.len(), 19);
        assert_eq!(two_turns.len(), 38);
    }

    #[test]
    fn does_not_accept_count_of_odd_players(){
        let one_turn = generate_current_rounds(19, false);
        let two_turns = generate_current_rounds(37, true);
        assert_eq!(one_turn.len(), 0);
        assert_eq!(two_turns.len(), 0);
    }
    
    #[test]
    fn first_round_has_first_versus_last(){
        let one_turn = generate_current_rounds(20, false);
        let two_turns = generate_current_rounds(20, true);
        assert_eq!(one_turn[0][0], (1,20));
        assert_eq!(two_turns[19][0], (20,1));
    }

    #[test]
    fn last_round_has_first_versus_second(){
        let one_turn = generate_current_rounds(20, false);
        let two_turns = generate_current_rounds(20, true);
        assert_eq!(one_turn[18][0], (1,2));
        assert_eq!(two_turns[37][0], (2,1));
    }
}