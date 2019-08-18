# First Kata: "Multiply"

Okay as promised in [my previous post](https://dev.to/jonasbn/blog-post-46gm) additional Rust posts was possibly in the pipeline, I also mentioned [Codewars.com](https://www.codewars.com/) and the katas I use for practice.

If you are into [Codewars.com](https://www.codewars.com/) this post (and hopefully subsequent posts) will contain **spoilers** - now you have been warned!

I write these up primarily for my own education and reference, since seems to to stick better if I jot them down, if you find this useful that is bonus - so here goes.

First kata: "[multiply](https://www.codewars.com/kata/50654ddff44f800200000004)". The Kata problem was formulated as:

> The code does not execute properly. Try to figure out why.

The original code, looked as follows:

```rust
fn multiply(a:i32, b:i32) {
  a * b
}
```

Here follows my solution, the solution in itself is not particularly interesting, but I can reflect on some of the things I learned from my solution (and possibly the failures leading up to the working solution)and I can spice it up with my approach to solving katas.

```rust
fn main () {
    let c = multiply(8, 8);
    println!("{}", c);
}

fn multiply(a:i32, b:i32) -> i32 {
  a * b
}
```

The main function is just so you can run it from the command line, so the project was generated using `cargo` like so:

```bash
$ cargo new --bin multiply
     Created binary (application) `multiply` package
```

This generated the `src/main.rs` file in a directory named: `multiply` containing the `main` function.

Line 1: `fn main() {`

Read up on [the anatomy of a Rust program](https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html).

> The main function is special: it is always the first code that runs in every executable Rust program

- We have no parameters, hence the empty parentheses `()`
- The function _body_ is defined between the curly braces `{` ... `}`, well we only see the first in line one, but the other one follows,

Now we have an idea about what function looks like in **Rust**. As described in [the Rust cheatsheet](https://cheats.rs/):

```rust
fn f() {}
```

And as you have noticed, when generating an application with `cargo` you get a classical "hello world" example for free:

```rust
fn main() {
    println!("Hello, world!");
}
```

Now lets skip the contents of the `main` function and look at the next function: `multiply`

```rust
fn multiply(a:i32, b:i32) -> i32 {
  a * b
}
```

Using our _newly_ obtained knowledge, we can read that the function is named "multiply":

Line 6: `fn multiply(a:i32, b:i32) -> i32 {`

It takes parameters since the parentheses is populated and finally we observe something new: `-> i32`.

Let's start with the parameters. The function takes to parameters: `a` and `b`, if you read a little longer into the body of the code you can see that, `a` and `b` are our multiplication operands.

The two parameters are annotated with types: `i32` which mean signed integer of size 32. The important lesson here is for me to understand two things.

1. Parameter specification for our function
2. and types

As I described in my post on Learning Rust, there are good resources  documenting **Rust** so if you want to have some details on `i32` [just look it up](https://doc.rust-lang.org/nightly/std/primitive.i32.html).

And finally for the `-> i32`, it describes the return value of the function. So we both take parameters of the type `ì32` and we return `i32`.

Line 7: `a * b`

This lines uses our two operands, which match the parameters.

Here we can observe another **Rust** _thing_: the implicit return value, I would imagine that some people coming from other programming languages, find this a tad special. The implicit return value is the last value. You do not have to write an implicit return.

The same functionality exists in **Perl**, I must admit that I prefer explicit returns I am bit of an _readability freak_ and I am not a huge fan of implicitness and _magic_ in particular.

So `a * b` could be written `return a * b;`, but since I am focussed on learning **Rust**, understanding and using idiomatic **Rust** is also a part of the curriculum. I will get back to idiomatic code in another blog post, since it is something I have reflected on (_and I need to get out of my system_).

Out complete implementation of "multiply" end looking as follows:

```rust
fn multiply(a:i32, b:i32) -> i32 {
  a * b
}
```

When you work on katas on [Codewars.com](https://www.codewars.com/), most katas hold unit-tests for testing your solution. The original tests for this kata looks as follows.

```rust
#[test]
fn returns_expected() {
  assert_eq!(multiply(3, 5), 15)
}
```

You could do TDD and write a lot of tests for observing that your code works and I encourage doing this.

The test suite can be run using `cargo`

```bash
$ cargo test
   Compiling multiply v0.1.0 (/Users/jonasbn/develop/github/rust-multiply-kata)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running target/debug/deps/multiply-b4e442fac2c38b72

running 1 test
test returns_expected ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

I will not go into tests in this post, since our focus is on something else, the basic test structure is simple and you should be able to tweak it to your needs, so please read on.

Since we are using `cargo` and we get the `main` _for free_ function we can also do manual testing. It just requires some minor tweaks to the `main` function, so we can allow it to take parameters, so we provide parameters via the command line.

Lets first do it using the modified version of a "Hello World", changed to the also popular "greeting" example.

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {

        let name = &args[1];

        println!("Greetings {} from {}", name, &args[0]);
    } else {
        eprintln!("Usage: greetings «name»");
    }
}
```

Getting string parameters to your **Rust** command line application is pretty basic and [the documentation](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html) is pretty clear. If you are doing something more complex I am sure there is a crate that can help you out.

Now lets apply this to our _multiplier_:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {

        let operand1: i32 = args[1].parse().unwrap();
        let operand2: i32 = args[2].parse().unwrap();

        let result = multiply(operand1, operand2);
        println!("{}", result);
    } else {
        eprintln!("Usage: multiply «operand1» «operand2»");
    }
}
```

Do note that there are a subtle difference between then two, since we have to _cast_ the parameters from strings to integers.

so line 8 and 9 does this: `let operand1: i32 = args[1].parse().unwrap();` for each of the operands.

And there you have it, you can now test your **Rust** command line application if need be.

But lets get back to doing proper software development, since manual testing is tiresome and trivial (_and error prone_), so lets extend the unit-test suite with some more tests.

As shown earlier a test suite looks as follows:

```rust
#[test]
fn returns_expected() {
  assert_eq!(multiply(3, 5), 15)
}
```

We can easily extend this with some interesting scenarios and corner cases, because a single test should capture the essence of such a simple function:

```rust
#[test]
fn returns_expected() {
  assert_eq!(multiply(3, 5), 15);
  assert_eq!(multiply(3, 0), 0);
  assert_eq!(multiply(0, 5), 0);
  assert_eq!(multiply(-3, -5), 15);
  assert_eq!(multiply(-3, 5), -15);
  assert_eq!(multiply(3, -5), -15);
}
```

This is all marvelous and it looks as if our simple multiplier is working as expected, but I want to leave you with a cliff hanger, based on a problem I ran into in another project, which also applies here:

- Our return value is of the type `i32` right?
- The two operands are of the type `i32` right?

This mean that the product of our calculation can exceed our return value - meaning basic parameters can render our multiplier useless.

The maximum value of an `i32` can be extracted from **Rust**

```rust
fn main() {
    println!("{}", i32::max_value());
}
```

Do checkout [the playground](https://gist.github.com/dbace8d9dd26cb3403717402a12f4d52) if you want to fool around with this brief example.

The maximum value for `ì32` is `2147483647`.

If we multiply `2147483647` with `2147483647` we get: `4.611686e+18` and the problem is of course relevant for the negative equivalents also (_it there a term for this in mathematics?_).

If you want to check it out, just add the following test to the test suite, sine multiplying our maximum with 2 exceeds the maximum of our return type.

```rust
assert_eq!(multiply(2147483647, 2), 4294967294);
```

This is it for now, I will do more write ups, since they are a good way for me to get my notes and katas in order, and perhaps somebody else can benefit from my notes or get inspired to learning **Rust** or just solving katas on [Codewars.com](https://www.codewars.com/).

I known I am skipping over a lot of things, and I hope you can still make sense of what I am writing and enjoy  it, but I aim to cover these things in other posts, where they might prove more relevant or can be the focal point of the post.

Have fun,

jonasbn
