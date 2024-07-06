//## Round-robin
///Create a vector of rounds and their respective matches, 
///using the Round-robin algorithm.
///
///<strong>Accepted</strong>:
///- String data type players;
///- number of even players;
///- round-trip.
/// 
///#### Example:
///```
///use round_robin::round_robin;
///round_robin(vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()]);
///```
///

mod round_robin;

pub use round_robin::generate_rounds;

pub fn round_robin<T>(values: Vec<T>) -> Vec<(T, T)> 
where
    T:Clone 
{
    generate_rounds(values)
}
