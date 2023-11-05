// THE RUST PROGRAMMING LANGUAGE BOOK
// CHAPTER 10 - GENERIC TYPES, TRAITS AND LIFETIMES

// === 10.3 - VALIDATING REFERENCES WITH LIFETIMES

// PREVENTING DANGLING REFERENCES WITH LIFETIMES
// lifetimes prevents us from having dangling references -> programs that refers to data
// that they are not intended to reference bc such data has been destroyed

// this struct holds a reference to a string &str
// Really terrible things can happen if the data that ImportantExcerpt.part is
// referencing gets destroyed, we would have a dangling reference, to prevent this,
// we are lifetime annotating the struct. The restriction this provides is that
// ImportantExcerpt.part cannot outlive whatever type it ends up referencing.

struct ImportantExcerpt<'a> {
    // LIFETIME ANNOTATION IN STRUCT DEFINITIONS
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // LIFETIME ANNOTATIONS IN METHOD DEFINITIONS
    // because of RULE TWO there is no need applying lifetime annotation on level method
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    // RULE ONE APPLIED gives &self and announcement different lifetimes
    // RULE ONE + TWO APPLIED gives the output lifetime(return type) the lifetime of self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

impl<'a> ImportantExcerpt<'a> {
    // RULE ONE APPLIED gives &self and announcement different lifetimes
    // RULE ONE + TWO APPLIED gives the output lifetime(return type) the lifetime of self
    fn announce_and_return_part<'b, 'c>(&'b self, announcement: &'c str) -> &str {
        println!("Attention please: {}", announcement);
        self.part: &'b str
    }
}

fn main() {
    //    let r;
    //    {
    //        let x = 5;
    //        r = &x;
    //    }// x goes out of scope here
    //
    //    println!("r: {}", r); // we are trying to print x, a variable that was destroyed in line 15

    // THE BORROW CHECKER
    // The rust compiler has a BORROW CHECKER that compares scopes to determine if all borrows
    // are valid. A borrow is valid when the data it is borrowed from,has an equal lifespan or
    // a longer life span.
    // if an item borrow from some data, the data needs to be available till the item goes out of scope
    // before it can be destroyed.
    // the BORROW CHECKER prevents one from having a reference to data that has been destroyed

    // GENERIC LIFETIMES IN FUNCTIONS
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    //
    //    fn longest(x: &str, y: &str) -> &str {
    //        if x.len() > y.len() {
    //            x
    //        } else {
    //            y
    //        }
    //    } // this won't compile
    // There is this our friend called the borrow checker, you see we are returning a &str,
    // but our longest fn takes 2 &str parameters, our friend does not know which of the 2
    // &str 's is going to be returned, either x or y, actually we don't know either, we don't
    // know what ends up getting returned either x or y and whether it will be valid or not.
    // So we need to help our friend, figure out the relationship between the 2 &str 's, and
    // the returned &str so it can do its analysis, this is where lifetimes come in handy

    //    fn longest<T,U>(x: &[T], y: &[U]) {
    //        if x.len() > y.len() {
    //            x
    //        } else {
    //            y
    //        }
    //    }

    // LIFETIME ANNOTATION SYNTAX
    //    Life time annotations tell the relationship between multiple references to each other.
    //    The name 'lifetimes' might throw you off, but lifetime annotations do not change
    //    the lifespan of the references, they only describe the relationship so that the borrow
    //    checker can do its analysis. Just like generic fns can take any type,
    //    fns can take any lifetime by specifying a generic lifetime parameter

    // LIFETIME SYNTAX RULES
    // 1. Starts with an apostrophe -> '
    // 2. Appears after the reference token followed by the type (&) -> &'a i32
    // 3. Can be annotated on both non mutable and mutable references -> &'a i32 && || &' mut a i32
    // 4. Are usually short like generic fn types, mostly 'a' is used
    // 5. A space is between the lifetime name and the type

    // LIFETIME ANNOTATION IN FUNCTION SIGNATURES
    // Generic lifetimes in fn signatures are defined in the same syntax as generic functions
    // going back to our fn in line 34, we are adding in lifetimes, which will decribe the
    // relationship of the references, for the return reference (&str) to be valid, both
    // &str parameters lifetime must be valid

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    } // now this works
      //
      //    let stringL = String::from("long string is long");
      //    {
      //        let stringS = String::from("xyz");
      //        let result2 = longest(stringL.as_str(), stringS.as_str());
      //        println!("The longest string is {}", result2);
      //    } // this works

    // just like the rules described in the function signature, what happens here is that
    // the lifetimes of both args should valid, so Rust takes the shortest lifespan always
    // and if that overlapse with the longer one and the code doesn't try to use the
    // longer lifespan when the shorter lifespan has been destroyed(goes out of scope), Rust
    // compiles just fine.

    //    let stringL = String::from("long string is long");
    //    let result2;
    //    {
    //        let stringS = String::from("xyz");
    //        result2 = longest(stringL.as_str(), stringS.as_str());
    //    } // stringS and result2 goes out of scope here.
    //    println!("The longest string is {}", result2); // we are trying to use result2 here,

    // this won't work because
    // 1. stringS has been destroyed, we are trying to use a longer scope rather than the
    // shorter scope, Rust doens't work like that. This goes against the lifetime
    // 2. result2 goes out of scope and gets destroyed, so result as declared above becomes
    // uninnitialized, Rust, doesn't allow uninnitialized variables.
    //.. we are trying to use a borrow, that has been destroyed,
    // Pff DANGLING REFERENCES, "good is not." - Yoda, Star Wars
    // Rust compiler gathers all this intelligent insight of our code because of a tiny piny
    // 'a, c'est genial

    // At this point to summarize all the technicalities of lifetimes, I think all we are
    // trying to do is tell the compiler, hey! dummy(he is our friend too, let's mince words),
    // not like you are dumb dumb, you are really smart but from where I see the code, I can
    // clearly see when stringS goes out of scope, so with my understanding of  scopes, borrowing
    // and ownership, I know this code won't work, (I wouldn't even write this code, but
    // if I ever write something like this, which I might, guess who is the dummy).
    // so it shouldn't run this code. (whispering) This is just between you and I, so our compiler
    // doesn't know that this code is terrible, without lifetimes? or just that it can't see it?
    // if it can't see, then lifetimes gives the compiler eyes to see the code as we see it and knows
    // that it goes against the rules of ownership and borrowing.

    // Quick Question, if the rust Rust compiler needs lifetime annotations to tell and compile our
    // code safely? how is it able to know that our code needs lifetimes and in the first place?
    // I guess we need a deep dive to understand this.

    // THINKING IN TERMS OF LIFETIMES
    // if we have a fn that takes parameters but doesn't use some parameters in a way that they
    // are related to others, we can decide to not annotate the unrelated parameters. We already
    // mentioned that lifetimes defines the relationship between the multiple parameter references
    // if there is no relationship between them, why annotate?

    fn longestT<'a>(x: &'a str, y: &str) -> &'a str {
        // y has no lifetime annotation
        // yet code works perfectly
        return x;
    }
    let resultT = longest(string1.as_str(), string2);
    println!(
        "This is the first argument returned from calling longestT: {}",
        resultT
    );

    // What happens if there is no relationship between the parameters but we still lifetime annotate all?
    fn longestAL<'a>(x: &'a str, y: &'a str) -> &'a str {
        // even though y has no relationship
        // with x we still lifetime annotate both, code works with no issues
        return x;
    }
    let resultAL = longestAL(string1.as_str(), string2);
    println!(
        "This is the first argument returned from calling longestAL: {}",
        resultAL
    );

    // WHAT HAPPENS IF THE FUNCTION TAKE multiple parameters BUT USES ONLY ONE?
    fn longestMP<'a>(x: &'a str, y: &'a str, b: &'a str, c: &'a str) -> &'a str {
        // even though y, b, c have no relationship with x
        // we still lifetime annotate all, code works with no issues
        return x;
    }
    let resultMP = longestAL(string1.as_str(), string2);
    println!(
        "This is the first argument returned from calling longestMP: {}",
        resultMP
    );

    // WHAT HAPPENS IF THE FUNCTION TAKE multiple parameters BUT USES ONLY ONE?
    fn longestON<'a>(x: &str, y: &str, b: &str, c: &'a str) -> &'a str {
        // x, b, y have no relationship with c so we lifetime annotate only c, code works with no issues
        return c;
    }
    let resultON = longestAL(string1.as_str(), string2);
    println!(
        "This is the first argument returned from calling longestON: {}",
        resultON
    );

    // LIFETIME ANNOTATION IN STRUCT DEFINITIONS
    let novel = String::from("Call me Ishmael. Some years ago....");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // i is an instance of ImportantExcerpt and because of the lifetime annotation we did,
    // i cannot outlive first_sentence, which is string slice of a String owned by novel
    // immediately novel or first_sentence gets destroyed, i.part becomes a dangling reference.
    // lifetime annotating ImportantExcerpt prevents us from having this dangling reference
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("i.part is {}", i.part);

    // LIFETIME ELISION
    // Deterministic patterns coded into the Rust compiler to allow it to infer lifetimes in programs
    // insight/patterns learnt from previous developer programs.

    // INPUT & OUTPUT LIFETIME
    // input lifetimes -> assigned to parameters of fns or methods
    // output lieftimes -> assigned to returned types of fns, methods

    // LIFETIME ELISION RULES
    // RULE ONE. If a fn/method or struct has only 1 parameter that is a reference, it is assigned a lifetime
    // if it has 2 or more each parameter is assigned a different lifetime parameters
    // RULE TWO. if there is only one input lifetime parameter, the lifetime is applied to all the
    // output lifetime parameters
    // RULE THREE. if there are multiple input lifetime parameters and one of them is &self,
    // the lifetime of self is applied to all the output lifetime parameters

    // Acting as the Compiler to annotate the following code snippets with lifetimes,
    // using the LIFETIME ELISION RULES
    fn first_word(s: &str) -> &str {}
    // it has 1 parameter that is a reference, per RULE ONE & TWO:
    // the compiler gives us this code,
    fn first_word<'a>(s: &'a str) -> &str {} // RULE ONE APPLIED
    fn first_word<'a>(s: &'a str) -> &'a str {} // RULE ONE + RULE TWO APPLIED

    fn longest(x: &str, y: &str) -> &str {}
    // 2 parameters as references, per RULE ONE
    // RULE ONE and RULE TWO do not apply
    fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
    // in situations like this, lifetime ELISION rules cannot completely infer the output lifetime
    // so the compiler throws an error and forces us to explicitly annotate the lifetime.

    // THAT SAID, LIFETIME ELISION RULES DO NOT PROVIDE COMPLETE INFERENCE

    // LIFETIME ANNOTATIONS IN METHOD DEFINITIONS

    // THE STATIC LIFETIME
    // allows the affected reference to live the entire duration of the program.
    // String literals have static lifetimes by default

    let s: &'static str = "I have a static lifetime."; // store in programs binary -> always available
                                                       // avoid pasting 'static everywhere in your code even if the compiler suggest, fix the bug
                                                       // causing the compiler to throw that error instead.

    // GENERIC TYPE PARAMETERS, TRAIT BOUNDS AND LIFETIMES
    // ... all in one function
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
