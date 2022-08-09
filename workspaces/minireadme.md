this is an example of a workspace, just in case a project is growing big.
This is from chapter 14 but the part on using binary crates in skipped.

commands:

`cargo build` in the workspace folder builds everything whereas in a specific member it just build that member. cargo lock will be in the workspace folder instead in each single member.

`cargo run` can be used in the runner folder or `cargo run -p runner` in the workspace. 

`cargo test -p <path to a library>` can be used if there some tests to be perfomed in a specific library.
`cargo test` in the workspace folder tests every single library whereas in a specific member it does the usual.