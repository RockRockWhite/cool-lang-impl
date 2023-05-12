fn main() {
    let r = rlex_gen::rlex::Rlex {};
    r.lex(
        r#"
        -- This is a comment
class Main {
    -- Define some variables
    let x : Int <- 42;
    let y : String <- "Hello, world!";

    -- Print a message
    if x > 0 then
        println(y);
    else
        println("Error!");
    fi

    -- Loop through some numbers
    let i : Int <- 0;
    while i < 10 loop
        println("i = " + i);
        i <- i + 1;
    pool

    -- Define a function
    def fibonacci(n : Int) : Int {
        if n <= 1 then
            return n;
        else
            return fibonacci(n-1) + fibonacci(n-2);
        fi
    };

    (* Call the function *) 
    let result : Int <- fibonacci(10);
    println("Fibonacci(10) = " + result);
};
     "#,
    );
}
