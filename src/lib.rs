#[derive(Debug)]
pub struct Round{
    pub id: usize,
    pub games: Vec<Game>
}

#[derive(Debug)]
pub struct Game{
    pub home: String,
    pub away: String
}

///
///## Tournament Round-robin
///Create a vector of rounds and their respective matches in a tournament, 
///using the Round-robin algorithm.
///
///<strong>Accepted</strong>:
///- String data type players;
///- number of even players;
///- round-trip.
/// 
///#### Example:
///```
///round_robin(vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()], false);
///```
pub fn round_robin(players: Vec<String>, round_trip: bool) -> Vec<Round> {
    let mut rounds = generate_rounds(players.clone(), false, vec![]);
    if round_trip{
        let mut two_turns = generate_rounds(players, round_trip, vec![]);
        rounds.append(&mut two_turns);
    }
    rounds
}

fn generate_rounds(mut players: Vec<String>, round_trip: bool, mut rounds: Vec<Round>) -> Vec<Round> {
    if players.len() % 2 != 0 {
        return vec![];
    }

    let mut games = Vec::<Game>::new();

    for n in 0..(players.len() / 2) {
        if round_trip{
            games.push(Game {
                home: players[players.len() - n - 1].clone(),
                away: players[n].clone()
            });
        }
        else{
            games.push(Game {
                home: players[n].clone(),
                away: players[players.len() - n - 1].clone()
            });
        }
    }
    rounds.push(Round {
        id: rounds.len() + 1,
        games: games
    });

    if rounds.len() == players.len() - 1{
        return rounds;
    }

    players[1..].rotate_right(1);
    generate_rounds(players, round_trip, rounds)
}

#[cfg(test)]
pub mod tests{
    use super::*;

    fn generate_test_rounds(round_trip: bool) -> Vec<Round>{
        round_robin(vec![
            "Liverpool".to_string(),
            "Chelsea".to_string(),
            "M. City".to_string(),
            "M. United".to_string(),
            "Everton".to_string(),
            "Leicester".to_string()
        ], round_trip)
    }

    #[test]
    fn number_rounds_with_round_trip() {
      let rounds = generate_test_rounds(true);
      assert_eq!(rounds.len(), 10);
    }

    #[test]
    fn number_rounds_without_round_trip() {
        let rounds = generate_test_rounds(false);
        assert_eq!(rounds.len(), 5);
    }

    #[test]
    fn number_games_per_round() {
      let rounds = generate_test_rounds(true);
      assert_eq!(rounds[0].games.len(), 3);
    }

    #[test]
    fn first_game_of_first_round_is_first_player_versus_last_player() {
      let rounds = generate_test_rounds(false);
      assert_eq!(rounds[0].games[0].home, "Liverpool".to_string());
      assert_eq!(rounds[0].games[0].away, "Leicester".to_string());
    }

    #[test]
    fn first_game_of_last_round_is_first_player_versus_second_player() {
        let rounds = generate_test_rounds(false);
        assert_eq!(rounds[4].games[0].home, "Liverpool".to_string());
        assert_eq!(rounds[4].games[0].away, "Chelsea".to_string());
      }
}