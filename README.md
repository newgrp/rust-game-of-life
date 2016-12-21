# Rusty Game of Life

This is an implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life) in Rust using [Piston](http://www.piston.rs/).

This is our final project for the Parallel & Distributed Computing class at St. Olaf College for Fall 2016. 

We wanted to learn Rust while exploring ways to parallelize the game of life. 

# Running the Code

Just download or clone the repository, and then run:

```
cargo run
```

Which will launch the simulation!

# Credits 

Conrad Parker - Core algorithm design and implementation. 
Omar Shehata - Piston/graphics integration, UI, optimization and benchmarking. 

# TODO
* Add some pictures/gifs of interface.
* Add instructions of how to pause/edit/scroll.
* Document how to use commandline flags to load initial seed.
* Add more seed examples.
* Implement Rust traits for the sequential/parallel versions of the algorithm.
* Implement the [Hashlife](https://en.wikipedia.org/wiki/Hashlife) algorithm.
* Compare and benchmark different algorithms (and be able to switch between them in real time???)