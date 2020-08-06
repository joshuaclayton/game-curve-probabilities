# Game Curve Probabilities

This is alpha software.

Given an input of dice counts, this will print probabilities for each possible value.

## Install

```sh
git clone git@github.com:joshuaclayton/game-curve-probabilities.git
cd game-curve-probabilities
cargo install --path .
```

## Usage

```sh
game-curve-probabilities "2d6 + 1d20"
```

Generates output that might look something like

```
 3: 0.1389%
 4: 0.4167%
 5: 0.8333%
 6: 1.3889%
 7: 2.0833%
 8: 2.9167%
 9: 3.6111%
10: 4.1667%
11: 4.5833%
12: 4.8611%
13: 5.0000%
14: 5.0000%
15: 5.0000%
16: 5.0000%
17: 5.0000%
18: 5.0000%
19: 5.0000%
20: 5.0000%
21: 5.0000%
22: 5.0000%
23: 4.8611%
24: 4.5833%
25: 4.1667%
26: 3.6111%
27: 2.9167%
28: 2.0833%
29: 1.3889%
30: 0.8333%
31: 0.4167%
```

Supported Dice: D4, D6, D8, D10, D12, D20

## Support

Implementation of the algorithm was inspired by a response from Jasper Flick of
[Catlike Coding](https://catlikecoding.com/), who also runs
[anydice.com](https://anydice.com/).

After a cursory pass at brute-forcing, I reached out to learn if there was an
algorithm. He responded with:

> You can sum arbitrary dice very quickly by treating them as (value, probability) tuple sets. Take two sets and loop through all tuple combinations, performing an operation on the values and multiplying the probabilities, thus constructing a new set. The sets have no duplicate values, instead the probabilities are added on insertion. Repeat until done. For example:
>
> A d2 is the set {(1,1/2),(2,1/2)}.
> 2d2 is d2 + d2 = {(1,1/2),(2,1/2)} + {(1,1/2),(2,1/2)} = {(1+1,1/4),(1+2 or 2+1,1/4+1/4),(2+2,1/4)} = {(2,1/4),(3,1/2),(4,1/4)}.
> 3d2 = d2 + 2d2 =  {(1,1/2),(2,1/2)} + {(2,1/4),(3,1/2),(4,1/4)} = {(1+2,1/8),(1+3 or 2+2,1/4+1/8),(1+4 or 2+3,1/8+1/4),(2+4,1/8)} = {(3,1/8),(4,3/8),(5,3/8),(6,1/8)}.

The approach I arrived at followed this closely, using a `HashMap` to store the
roll possibilities as keys, with values being probabilities. His description
and example immediately struck me as an opportunity to fold over each die
probability, resulting in speedy calculations for most common cases.

## License

Copyright 2020 Josh Clayton. See the [LICENSE](LICENSE).
