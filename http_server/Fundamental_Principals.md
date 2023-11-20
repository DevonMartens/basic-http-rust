# Rust Fundamental 

### Structs

### Strngs 

In the sample above you can see that the variable server living in the src/main.rs file is a string. The compiler would by throw an error because the `Server` struct expects a `String`  and would find a `&str` string literal.

    **Definitions**

    ```
    &str: A string slice or reference to a string literal. It is immutable view into a portion of memory that contains the string's UTF-8 encoded data. Often used for read-only access to string data.

    String: A dynamically allocated, growable, and mutable string type. It is represented as a collection of bytes stored on the heap and allows you to modify the string's content and length at runtime.
    ```

Use `.to_string()` to convert string literal to string or `String::from("")`.

### Enum 

### Option Enum 

There is no null value in rust but in the **Request** struct we dont need to have a querying string for all `Request` structs created. Therefore we use an option enum. 

The syntax for the option enum is this:

    ```rust
    pub enum Option<T> {
        None,
        Some(T)
    }
    ```

We use this in our struct like this:

    ```rust
    struct Request {
        path: String,
        query_string: Option<String> , // it is possible to not have we create an option enum 
        method: Method,
    }
    ```

### Module

`mod` keyword. In rust modules are a way to encapsulate code into seperate namespaces. 

    * Modules can be called upon in other parts of the program as well. This simplifies your codebase and will result in less duplicate code. 

    * Everything in a module is private by default we have to explicitly state otherwise with the `pub` keyword.

    * Use to group functions, types, traits together

    * Improve maintainability/visability

    * Can pull from other files

### Result Enum 

The rust `Result` enum is a commonly used type for represenring the result of an operation that may succeed or fail.  This goes back to the `Two types of errors` in rust. Recoverable vs. Unrecoverable. We don't want the program to fail due to a recoverable error so we handle it.

    ```rust
    pub Enum Result<T, E>{ // Generics
        Ok(T), // Result Value
        Err(E), // Error Value
    }
    ```