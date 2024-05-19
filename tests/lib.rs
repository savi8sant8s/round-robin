#[derive(Debug, PartialEq, Clone)]
pub struct Game {
    pub home: String,
    pub away: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Round {
    pub games: Vec<Game>,
}

pub fn generate_rounds<R>(mut values: Vec<R>) -> Vec<Round>
where
    R: Clone + ToString,
{
    if values.len() % 2 != 0 {
        return vec![];
    }

    let mut rounds = Vec::<Round>::new();

    for _ in 0..(values.len() - 1) {
        let mut round_games = Vec::<Game>::new();
        for i in 0..(values.len() / 2) {
            round_games.push(Game {
                home: values[i].clone().to_string(),
                away: values[values.len() - i - 1].clone().to_string(),
            });
        }
        rounds.push(Round { games: round_games });
        values[1..].rotate_right(1);
    }

    rounds
}

#[cfg(test)]
pub mod tests{
    use super::*;

    fn generate_test_rounds(round_trip: bool)-> Vec<Round>{
        let mut rounds = generate_rounds(vec![
            "Liverpool".to_string(),
            "Chelsea".to_string(),
            "M. City".to_string(),
            "M. United".to_string(),
            "Everton".to_string(),
            "Leicester".to_string()
        ]);

        if round_trip {
            let mut return_leg = generate_rounds(vec![
                "Liverpool".to_string(),
                "Chelsea".to_string(),
                "M. City".to_string(),
                "M. United".to_string(),
                "Everton".to_string(),
                "Leicester".to_string(),
            ]);
            rounds.append(&mut return_leg);
        }
        rounds
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

