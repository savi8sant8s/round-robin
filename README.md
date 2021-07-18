## Tournament Round-robin
Create a vector of rounds and their respective matches in a tournament, 
using the Round-robin algorithm.

<strong>Accepted</strong>:
- String data type players;
- number of even players;
- round-trip.
 
#### Example:
```
fn main(){
    let rounds = round_robin(vec![
        "Liverpool".to_string(),
        "Chelsea".to_string(),
        "M. City".to_string(),
        "M. United".to_string()
    ], false);

    println!("{:#?}", rounds);
}
```
#### Output:
```
[
    Round {
        id: 1,
        games: [
            Game {
                home: "Liverpool",
                away: "M. United",
            },
            Game {
                home: "Chelsea",
                away: "M. City",
            },
        ],
    },
    Round {
        id: 2,
        games: [
            Game {
                home: "Liverpool",
                away: "M. City",
            },
            Game {
                home: "M. United",
                away: "Chelsea",
            },
        ],
    },
    Round {
        id: 3,
        games: [
            Game {
                home: "Liverpool",
                away: "Chelsea",
            },
            Game {
                home: "M. City",
                away: "M. United",
            },
        ],
    },
]
```
