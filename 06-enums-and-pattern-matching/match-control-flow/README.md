# The match Control Flow Construct

In this lesson I learned:
  - `match` allows to compare a value against a series of patterns and then execute code based on which pattern matches; 
  - *patterns* can be made up of literal values, variable names, wildcards, and many other things;
  - when the match expression executes, it compares the resulting value against the pattern of each arm, *in order*;
  - match arms can bind to the parts of the values that match the pattern. This is a way to extract values out of enum variants;
  - `match` can be used with `Option` with arms `None` and `Some(value)`;
  - matches in Rust are *exhaustive*: we must exhaust every last possibility in order for the code to be valid;
  - with *catch-all pattern* we can also take special actions for a few particular values, but for all other values take one default action.
