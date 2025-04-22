## How to Run This Code

This project implements an in-memory key-value database with transaction support using the Rust programming language.

### Setup

1.  **Install Rust:** If you don't have Rust installed, follow the official instructions [here](https://www.rust-lang.org/tools/install). This will install both the Rust compiler (`rustc`) and the Cargo build system/package manager.
2.  **Clone Repository:** Clone this repository to your local machine using Git:  

    ```bash
    git clone https://github.com/kevnster/ferris-db
    cd ferris-db
    ```

### Running

1.  **Navigate:** Open your terminal or command prompt and navigate into the root directory of the cloned repository (the one containing the `Cargo.toml` file).
2.  **Execute:** Run the project using Cargo. This command will compile the code and run the `main` function located in `src/main.rs`, which demonstrates the database operations based on Figure 2 from the assignment description.

    ```bash
    cargo run
    ```
    You will see output messages indicating the sequence of operations being performed and their results or errors, according to the implemented logic.

## Suggestions for Future Assignment 

I found this assignment was a great way to grasp transaction basics, but thinking about how this connects to larger systems, I have a few ideas. For instance, I think it would be really valuable to require wrapping this logic in a simple web API; in practice, you rarely call database functions directly but instead interact through service layers. Building on that, handling concurrent requests would be a key next step, since real applications almost always serve multiple users simultaneously, unlike this single-access model. Adding basic file persistence would also make it more realistic, showing how data usually needs to stick around after the program stops, unlike our purely in-memory approach here.
