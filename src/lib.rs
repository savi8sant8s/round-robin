///## Tournament Round-robin
///Create a vector of rounds and their respective matches in a tournament, 
///using the Round-robin algorithm.
///
///#### Examples with numbers:
///```
///let rounds1 = round_robin((1..= 4).collect());
///print!("{:?}", rounds1);
///```
///Output: [[(1,4), (2,3)], [(1,3), (4,2)], [(1,2), (3,4)]]
///
///#### Examples with strings:
///```
///let rounds2 = round_robin(vec!["a", "b", "c", "d"]);
///print!("{:?}", rounds2);
///```
///Output: [[("a", "d"), ("b", "c")], [("a", "c"), ("d", "b")], [("a", "b"), ("c", "d")]]
pub fn round_robin<T: Clone + Eq>(players: Vec<T>) -> Vec<Vec<(T, T)>> {
    generate_rounds(players, vec![])
}

fn generate_rounds<T: Clone + Eq>(
    mut players: Vec<T>,
    mut rounds: Vec<Vec<(T, T)>>,
) -> Vec<Vec<(T, T)>> {
    if players.len() % 2 != 0 {
        return vec![];
    }

    let mut round = Vec::new();
    for n in 0..(players.len() / 2) {
        round.push((players[n].clone(), players[players.len() - n - 1].clone()));
    }
    rounds.push(round);

    if rounds.len() == players.len() - 1 {
        return rounds;
    }

    players[1..].rotate_right(1);
    generate_rounds(players, rounds)
}

#[cfg(test)]
pub mod tests{
    use super::*;


    fn generate_current_rounds(last: isize)-> Vec<Vec<(isize, isize)>> {
        round_robin((1..=last).collect())
    }

    #[test]
    fn count_of_rounds_equal_to_number_of_players_minus_1() {
        let rounds = generate_current_rounds(20);
        assert_eq!(rounds.len(), 19);
    }

    #[test]
    fn does_not_accept_number_of_even_players(){
        let rounds = generate_current_rounds(19);
        assert_eq!(rounds.len(), 0);
    }
    
    #[test]
    fn first_round_has_first_versus_last(){
        let rounds = generate_current_rounds(20);
        assert_eq!(rounds[0][0], (1,20));
    }

    #[test]
    fn last_round_has_first_versus_second(){
        let rounds = generate_current_rounds(20);
        assert_eq!(rounds[18][0], (1,2));
    }
}