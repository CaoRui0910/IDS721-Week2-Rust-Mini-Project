# Rust CLI: determine right triangle
## Description
This project is to judge whether a triangle is a right triangle according to the length of the three sides of the triangle.

## Usage
- User can type in `cargo run -- subcommand -- commandLineArg1 commandLineArg2 commandLineArg3` in command line. The three command line arguments are the lengths of the three sides of the triangle, and they are arranged in ascending order.
- Here is an example:
    ```
    cargo run -- right-triangle -- 3 4 5
    ```
- Output examples: 
  - The output of `cargo run -- right-triangle -- 3 4 5` is:
  <img width="897" alt="Screen Shot 2023-02-08 at 04 17 31" src="https://user-images.githubusercontent.com/93239143/217487653-25f1eb26-5c91-42ec-a1bb-e2f7b4c16322.png">
  - The output of `cargo run -- right-triangle -- 3 5 5` is:
  <img width="893" alt="Screen Shot 2023-02-08 at 04 17 16" src="https://user-images.githubusercontent.com/93239143/217487703-2027c394-7879-47d0-a82f-06f5da01a643.png">

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
