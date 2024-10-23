This is an extremely basic barebones language made in Rust that I did for fun.

Features: 
    1. Basic integer math 
        2a. Addition 
        2b. Subtraction
        2c. Multiplication
        2d. Division
    2. Variable declaration 
    3. Variable referencing
    4. Multi-line
    5. Command line to run a file

Examples: 

    Addition 
    ```
    1 + 1 

    Output: 2  
    ```

    Subtraction
    ```
    5 - 3 

    Output: 2
    ```

    Multiplication 
    ```
    1 * 3

    Output: 3 
    ```

    Division 
    ```
    8 / 2 

    Output: 4 
    ```

    Variable declaration
    ```
    set x = 1 + 1 

    Output: x = 2 
    ```

    Variable referencing
    ```
    set x = 1 + 1
    set y = x + 1

    Output: 3
    ```

All code has unit testing written for it with basic errors like variable re-declaration handled using `cargo test`. 