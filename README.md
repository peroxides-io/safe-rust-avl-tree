Implements the same algorithms as the `main` branch, but nodes are `struct`s, and their children are stored as `Option`s instead of sum type variants. Hopefully this elucidates some of the tradeoffs that are made when using this strategy.
To run the project, all that's needed is:

`cargo run`
