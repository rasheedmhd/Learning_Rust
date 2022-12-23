// THE RUST PROGRAMMING LANGUAGE BOOK
// CHAPTER 10 - GENERIC TYPES, TRAITS AND LIFETIMES

// === 10.2 - TRAITS - DEFINING SHARED BEHAVIOUR


// TRAITS -> any behaviour that a type has that it can share with other types
// TYPES BEHAVIOUR -> methods that we can call on that type

// we define traits using the trait keyword
// when define a method inside a trait's body we terminate the definition with a ';'
// instead of implementing the method within '{}'
// the compiler forces any type with our trait to implement the method
// a trait can have several methods delimited after the method signature by ';'

//pub trait Summary {
//    fn summarize(&self) -> String;
//}

use std::fmt::Display;

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
    pub date: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//impl Summary for Tweet {
//    fn summarize(&self) -> String {
//        format!("{}, by {}", self.content, self.username)
//    }
//}
//
//impl Summary for NewArticle {
//    fn summarize(&self) -> String {
//        format!("{}, by {} ({})", self.headline, self.author, self.location)
//    }
//}

/// DEFAULT IMPLEMENTATIONS
// traits with default defined behaviour/methods

// a default method implementation can call other methods in the same trait
// that may have default implementation or not

// this allows traits to have several useful methods that let types implement
// onlye a few of the methods
pub trait Summary {
    // require to be implemented on the sharing type since it has no default implementation
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("{} Read more...", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Summary for NewArticle {
    // here we define the method of summarize_author since its implementation
    // is empty as seen in line 57
    fn summarize_author(&self) -> String {
        format!("by {}", self.author)
    }
    // here we have commented out, an overwrite implementation of the summarize method
    // this works because the summarize method has default implementation in line 59
    //    fn summarize(&self) -> String {
    //        String::from("..( read more...)")
    //    }
}

// Just like we created a new empty trait, when we create methods for traits and want our
// types to use the default traits, we create an empty trait impl signature like below
// the line below tells Rust that we want NewArticle to share the summarize method from the
// Summary trait

// impl Summary for NewArticle {}

// TRAITS AS PARAMETERS
// notify is a function that takes in a parameter(item) -> a type that implements Summary
// by virtue of the type implementing the Summary trait, notify is able to Call
// any of the methods defined by Summary on its arguements

// think of notify as a generic function with constraints, been that the type T
// must implement Summary

// prepended 'G' so that we could have hte 2 different function signature print the same thing
// TRAIT BOUND SYNTAX -
pub fn notifyG<T: Summary>(item: &T) {
    println!("Breaking news!, {}", item.summarize());
}
// this syntax is used when we want to enforce that, the parameters are of the same type
// and they must implement Summary
// pub fn notifyG<T: Summary> (item: &T, item2: &T) {}



pub fn notify(item: &impl Summary) {
    println!("Breaking news!, {}", item.summarize());
}
// this is used when the parameters can be of different types
// and they must implement Summary
//pub fn notify(item: &impl Summary, item2: &impl Summary) {}

// I think we can use generic functions to achieve this too, let's see
pub fn nofify<T: Summary, U: Summary>(item: &T, item2: &U) {
    println!("Breaking news!, {}", item.summarize());
    // yayyayyyyy! check line 157, it works, I am learning HA!
}

// function name in 117, is nofify, that is a typo
// i first named it as nofityG2(intended for notifyG2)
// so when I was calling it in line 157, as notifyG2, Rust actually refused to compile the code
// of course it shouldn't because there is no function called notifyG2 but the reason why I
// wrote this is that rust went ahead to tell me that there is a similar function named
// nofifyG2, and I quickly fixed, the bug, I am impressed!

// SPECIFYING MULTIPLE TRAIT BOUNDS WITH THE + SYNTAX
pub fn notify_plus(item: &(impl Summary + Display )) {
    println!("Breaking news!, {}", item.summarize());
}

// with GENERICS
pub fn notifyG_plus<T: Summary + Display>(item: &T) {
    println!("Breaking news!, {}", item.summarize());
}

//impl Display for Tweet {}
//impl Display for NewArticle {}

// CLEARER TRAIT BOUNDS WITH where CLAUSES
// instead of writing this, Rust gives as the where clause to clean this up
fn some_function<T: Display + Clone + Copy, U: Clone + Debug + Display>(t: &T, u: &U) -> String {}
fn some_function<T, U>(t: &T, u: &U) -> String
where
T: Display + Clone + Copy,
U: Clone + Debug + Display,
{
    // function body here
}

// RETURNING TYPES THAT IMPLEMENT TRAITS
// We can use the "impl Trait" syntax to return a type that must have\
// a particular trait implemented

// however, this syntax only work if you are returning a single type

pub fn summarizables() -> impl Summary {
    Tweet {
        username: String::from("Joe L Arms"),
        content: String::from(
                "Controlling Sound with OSC Messages"
        ),
        reply: true,
        retweet: true,
    }
}

// THIS WON'T WORK. WE are not returning a single type
fn summarizables(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            // ... sample NewArticle data
        }
    } else {
        Tweet {
            // ... sample tweet data
        }
    }
}

// USING TRAIT BOUNDS TO CONDITIONALLY IMPLEMENT METHODS
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

// BLANKET IMPLEMENTATIONS
// Rust allows us to implement traits for any type that implements a particular trait
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {}
    // -- code snippets --
}
// the above is a similar implementation of how the std allows us to call .to_string()
// on any method that shares/implements the Display trait



fn main() {
    println!("Hello, world!");
    let article_un = NewArticle {
        headline: String::from("Calling Elixir from Erlang"),
        location: String::from("Sweden KTH"),
        author: String::from("Joe Leslie Armstrong"),
        content: String::from(
                "Calling Erlang from Elixir is pretty well documented - \
                not surprisingly since there is far more Erlang code than \
                Elixir code and Elixir programmers want to reuse Erlang libraries."
        ),
        date: String::from("23-12-2022 at 12:46AM GMT"),
    };
    //    pub username: String,
    //    pub content: String,
    //    pub reply: bool,
    //    pub retweet: bool,
    let tweet_un = Tweet {
        username: String::from("Joe L Arms"),
        content: String::from(
                "Controlling Sound with OSC Messages"
        ),
        reply: true,
        retweet: true,
    };

    println!("{}, New article by, {}",
    article_un.date,
    //article_un.author,
    article_un.summarize()
    );

    //notify(article_deux);
    notify(&tweet_un);
    notifyG(&tweet_un);
    nofify(&tweet_un, &article_un);

    //notifyG_plus(&tweet_un);
    //notify_plus(&tweet_un);
}