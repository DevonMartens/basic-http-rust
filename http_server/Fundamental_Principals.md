# Rust Fundamental 

### Structs

### Strngs 

In the sample above you can see that the variable server living in the src/main.rs file is a string. The compiler would by throw an error because the `Server` struct expects a `String`  and would find a `&str` string literal.

    **Definitions**

    ```
    &str: A string slice or reference to a string literal. It is immutable view into a portion of memory that contains the string's UTF-8 encoded data. Often used for read-only access to string data.

    String: A dynamically allocated, growable, and mutable string type. It is represented as a collection of bytes stored on the heap and allows you to modify the string's content and length at runtime.
    ```

Use `.to_string()` to convert string literal to string or `String::from("")` and