# kl-rs
That's an implementation of a interpreter for my own programming language,
called kl-rs(kevin's language), the postfix is of course because i will be using
rust as its host language!

## Language
This language is hightly expression-based, but there are 2 types of statements
in it: `return` statements and `let` statements. Everything else is an
expression!

That has some interesting implications, we can bind any expresion to a name
using the `let` statement, so we can do stuff like this for example:

```bash
let result = if (<expression>) { <block of code> } else { <block of code> }
```

This works because the if statement itself is an expression, so the block of
code that is evaluated generates an expresion value, which is in turn binded to
the result variable!

### Keywords
This is a really simple language with just a few keywords: `let`, `return`,
`fn`, `else`, `if`, `false`, `true`.

#### Defining functions
To define a function, as it is an expresion, we can just use a bind:

```bash
let foo = fn(params...) { <body> }
```

but we can also create clojures quite easily:

```bash
let foo = fn(params...) { <body> }
foo(fn(...) {})
```

In the example above, we're defining the `foo` function and running it with an
unnamed function, a clojure. 

### Strings
As in most languages, strings here are also represented using `""`, and they are
expressions as well, that means we can bind them to variables, return them from
functions or even let them as the last expression in a block of code, in which
case they will be evaluated!

#### Binding to a name
```bash
let foo = "hello world";
```

#### Using as the result of an expression
```bash
let foo = if(<condition>) { "foo" } else { "bar" };
```

In this case, if the condition is true, the result binded to the variable _foo_
will be _"foo"_, otherwise it will be _"bar"_

#### Returning from a function
```bash
let foo = fn(...) { return "foo" };
foo(); # "foo"
```

### Booleans
In this language, there is support for `true` and `false` booleans, they can be
used as you would expect!

##### Truthy values
In this language, just like C, numbers can be used in if-else expressions. `0`
Meaning `false` and any other number meaning `true`


## How to run it?
In order to run this project on you machine, first make sure you got [rust](https://www.rust-lang.org/)
correctly installed.

Then, you can run it directly using cargo with the following command:

```bash
cargo run
```

Afterwards, a REPL will appear and you can start writing kl-rs code!

There are a couple flags that you can use to inspect this program, you can see
all of them using the command:

```bash
cargo run -- -h
```


## How to build it?
Building this project is extremely straightforward, you just run: 

```bash
cargo build --release
```

After that, an executable will be generated in the folder
`target/release/kl-rs`, then you can just use it like normal


## TODOS
- [x] Add support for math expressions
- [x] Add support for return statements
- [x] Add support for if-else expressions
- [x] Add support for function expressions
- [x] Add support for comments
- [x] Add support for boolean expressions
- [x] Add support for let statements
- [ ] Add standard library
- [ ] Add loops
- [ ] Add support for arrays
- [ ] Add build in functions
- [ ] Add support for hashes
- [ ] Refactor tests
- [ ] Optimize project
