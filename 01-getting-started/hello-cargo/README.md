# Hello cargo lesson

1. Create the project

    ```
    cargo new hello-cargo --vcs=git
    ```

    Option `--vcs=git` used to create `.gitignore` when you already have an existing Git repository.

2. Write "hello world" program

    See `src/main.rs`

3. Build the project

    ```
    cargo build
    ```

4. Run the program

    ```
    ./target/debug/hello-cargo
    ```

5. \* We can build and run a project in one step using `cargo run`.

6. \* We can build a project without producing a binary to check for errors using `cargo check`.

7. Build the project for release

    ```
    cargo build --release
    ```
    
    It will compile the code with optimizations.

8. Run the release program

    ```
    ./target/release/hello-cargo
    ```

