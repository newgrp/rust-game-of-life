// This just exports all the game-of-life algorithms so they're 
// all conveniently under one namespace

pub mod sequential; // A simple sequential version 
pub mod parallel; // A parallel version that takes runs advance and cleanup across many threads
pub mod hashlife; // An implementation of the hashlife algorithm for memoizing common patterns
