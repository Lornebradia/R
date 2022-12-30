# R

An experimental implementation of R

## What can it do?

```sh
cargo run
```
```r
> x <- function(a = 1) { a + 3 }
# Function(...)
> x(2)
# Numeric([5.0])
```

This amounts to (most) of R's grammar parsing, basic primitives 
and scope management.

## Why

First and foremost, to learn - at this point, it is far too early 
to fixate on other goals. 

Although these are distant goals, if the project ever builds any
sort of momentum, I'd like for it to grow into a project that is
the foundation of a successor language for R -- providing tooling
to either port pure R code or provide a separate mode of operating
to execute R code.

## License

I welcome other contributors, but also have not thoughtfully selected
a long-term license yet. For now there's a CLA in place so that the 
license can be altered later on. I don't intend to keep it around forever. 
If you have suggestions, input selecting an appropriate license is 
appreciated.
much appreciated.