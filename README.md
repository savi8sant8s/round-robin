## Tournament Round-robin
Create a vector of rounds and their respective matches in a tournament, 
using the Round-robin algorithm.

<strong>Accepted</strong>:
- generic data types;
- number of even players;
- round-trip.
 
#### Examples with numbers:
```
let rounds1 = round_robin((1..= 4).collect(), false);
print!("{:?}", rounds1);
```
Output: [[(1,4), (2,3)], [(1,3), (4,2)], [(1,2), (3,4)]] | 

#### Examples with strings:
```
let rounds2 = round_robin(vec!["a", "b", "c", "d"], false);
print!("{:?}", rounds2);
```
Output: [[("a", "d"), ("b", "c")], [("a", "c"), ("d", "b")], [("a", "b"), ("c", "d")]]
