# Control flow

In this lesson I learned:

 - the condition in **if** statement must be a bool. If the condition isn’t a bool, we’ll get an error;
 - **if** is an expression;
 - because **if** is an expression, we can use it on the right side of a let statement to assign the outcome to a variable. In this case, the values that have the potential to be results from each arm of the if must be the same type;
 - Rust has three kinds of loops: **loop**, **while**, and **for**;
 - the **loop** keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop;
 - the **loop** can return value (see example in **src/main.rs**);
 - breaking multiple loops:
    - by default **break** and **continue** applied to the innermost loop at that point;
    - you can optionally specify a *loop label* on a loop that can be used with break or continue to apply these keywords to the labeled loop instead of the innermost loop;
    - *loop labels* must begin with a single quote `'`;
 - the keyword **while** runs loop, while the condition is true;
 - **for** loop allows to execute some code for each item in a collection.
