### Possible solutions

## A (naïve)

The easiest way is to create to create two 2D arrays `[[bool; 1024]; 1024]` to store pixel colours and visited cells.

`1024 * 1024 * 2` bits.


## B (coordinates-based)

An alternative is to store coordinates of black pixels (less than 3700 in our case) as well as visited ones. `bit_struct::u10` can be used to represent a coordinate and `sorted_vec::SortedSet` can be used as a container.

About `(3700 + visited) * 20` bits.

(There're more visited pixels than black ones, but we don't have to remember them all at once - one can store only visited pixels of particular area and run the algo once for each area)


## C (path-based)

Pixel colours can be derived from the ant's path; the time complexity is `O(current_path_length)`, which is quite much (final length in our case is about 35000), but it's RAM usage we're optimizing in this task.

About `35000` bits; see `main.rs` for its implementation.


### P.S.

Once the path for 64×64 field is calculated, the ant's movement becomes quite predictable and requiring almost no more RAM; for this field size **A** is the best solution. 