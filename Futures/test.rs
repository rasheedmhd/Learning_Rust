use futures::future::{
    self,
    Either,
    Future,
    FutureExt,
};

use core::marker::PhantomPinned;
use std::pin::Pin;
use std::mem;


fn main() {
    /// WAITING AND POLLING FUTURES

    // locate, open and read a text doc asynchronously
    let future = async_read_file("foo.txt");
    let file_content = loop {
        // constantly ask the future holding the bits of the resulting read
        // looping through each returned result
        // and checking whether it ready for use of still pending
        match future.poll(...) {
            // if result is ready for use stop polling and return result
            Poll::Ready(value) => break value,
            // if result is not ready for use no nothing, and poll later
            Poll::Pending => {}, // do nothing
        }
    }

    /// FUTURE COMBINATORS
    // - combines and returns futures
    // generic structure of any type F with 1 field of generic type F
    struct StringLen<F> {
        inner_future: F,
    }
    // trait implementation of Future of a generic structure over any type F
    // mandating that the field return a Future of type usize
    impl<F> Future for StringLen<F> where F: Future<Output = String> {
        type Ouput = usize;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
          match self.inner_future.poll(cx) {
              Poll::Ready(s) => Poll::Ready(s.len()),
              Poll::Pending => Poll::Pending,
          }
        }
    }

    fn string_len(string: impl Future<Output = String>)
    -> impl Future<Output = usize>
    {
        StringLen {
            inner_future: string,
        }
    }

    // Usage
    fn file_len() -> impl Future<Output = usize> {
        let file_content_future = async_read_file("foo.txt");
        string_len(file_content_future)
    }

    /// WE USING FUTURE COMBINATORS ARE HARD BECAUSE OF THE TYPE SYSTEM AND
    /// ITS CLOSURE BASED INTERFACE
    fn example(min_len: usize) -> impl Future<Output = String> {
        async_read_file("foo.txt").then(move |content| {
            if content.len() < min_len {
                Either::Left(async_read_file("bar.txt").map(|s| content + &s))
            } else {
                Either::Right(future::ready(content))
            }
        })
    }

    /// ASYNC/AWAIT CODE
    // abstracts future combinator and traits implementations away, allowing
    // us to write async code as if we were writing sync code.
    async fn foo() {
        0
    }

    // rough compiler translation
    fn foo() -> impl Future<Output = i32> {
        future::ready(0)
    }

    async fn example(min_len: usize) -> String {
        let content = async_read_file("foo.txt").await;
        if content.len() < min_len {
            content + &async_read_file("bar.txt").await
        } else {
            content
        }
    }

    /// SAVING STATES
    // Compiler-generates states with structs

    struct StartState {
        min_len: usize,
    }

    struct WaitingOnFooTxtState {
        min_len: usize,
        foo_txt_future: impl Future<Output = String>,
    }

    struct WaitingOnBarTxtState {
        conten: String,
        bar_txt_future: impl Future<Output = String>,
    }

    struct EndState {}

    // AN EXAMPLE OF A FULL STATE MACHINE
    enum ExampleStateMachine {
        Start(StartState),
        WaitingOnFooTxt(WaitingOnFooTxtState),
        WaitingOnBarTxt(WaitingOnBarTxtState),
        End(EndState),
    }

    /// COMPILER FUTURE IMPLEMENTATION FOR A STATE MACHINE
    // To handle state transitions the compiler implements the future trait for "state[async] function" -> example
    impl Future for ExampleStateMachine {
        type Output = String; // return type of async function "example"

        fn poll(self: Pin<&mut Self, cx: &mut Context ) -> Poll<Self::Output> {
            loop {
                match self { // TODO: implemention for pinning

                    ExampleStateMachine::Start(state) => {
                        // from the body of the example function
                        let foo_txt_future = async_read_file("foo.txt");
                        // from the await operation
                        // creating a state
                        let state = WaitingOnFooTxtState {
                            min_len: state.min_len,
                            foo_txt_future,
                        };
                        *self = ExampleStateMachine::WaitingOnFooTxt(state)
                    }

                    ExampleStateMachine::WaitingOnFooTxt(state) => {
                        // futures are polled to get results
                        match state.foo_txt_future.poll(cx) {
                            // on polled we get a pend state
                            Poll::Pending = return Poll::Pending,
                            // or a ready state, which gets return as the result
                            Poll::Ready(content) => {
                                // execute from example function
                                // do something with the returned result
                                if content.len() < state.min_len {
                                    // should the if condition be valid, we execute the code within it,
                                    // it this case it happens to be another future
                                    let bar_txt_future = async_read_file("bar.txt");
                                    // await on the operation -> future
                                    // create state for a possible transition later
                                    let state = WaitingOnBarTxtState {
                                        // data needed by the state for execution
                                        content,
                                        bar_txt_future,
                                    };
                                    // set the next state as self to get it executed
                                    *self = ExampleStateMachine::WaitingOnBarTxt(state)
                                } else {
                                    *self = ExampleStateMachine::End(EndState);
                                    return Poll::Ready(content);
                                }
                            }
                        }
                    }

                    ExampleStateMachine::WaitingOnBarTxt(state) => {
                        // poll the future to get the returned value
                        match state.bar_txt_file.poll(cx) {
                            // if the future isn't ready yet,
                            // return a Pending
                            Poll::Pending = return Poll::Pending,
                            // if the future is ready,
                            // add the content to the received result
                            // and send it back
                            Poll::Ready(bar_txt) => {
                                *self = ExampleStateMachine::End(EndState)
                               return Poll::Ready(state.content + &bar_txt);
                            }
                        }
                    }

                    ExampleStateMachine::End(_) => {
                        panic!("poll called after Poll::Ready was returned");
                    }

                }
            }
        }
    }

    /// THE EXAMPLE FUNCTION
    // The state machine has implemented the function's body
    // We initialized the state machine and return it
    // this function does not execute the state machine
    // by design Rust Futures must be polled for the first time
    // before they execute.
    fn example(min_len) -> ExampleStateMachine {
        ExampleStateMachine::Start(StartState {
            min_len,
        })
    }

    /// PINNING
    // locking<pinning> a value to its location in memory.
    // SELF-REFERENTIAL STRUCTS
    // when variables reference each other
    async fn pin_example() -> i32 {
        let array = [1,2,3];
        let element = &array[2];
        async_write_file("foo.txt", element.to_string()).await;
        *element
    }

    struct WaitingOnWriteState {
        array: [1,2,3],
        element: 0x1001c, // addr of the last array element -> resulting from a pointer dereference
    }

    /// USE HEAP ALLOCATION TO CREATE A SELF-REFERENTIAL STRUCT
    // fn main() {
    // Creating and storing a self-referential structure on the heap
    // with mutability
    // Avoiding mutability by wrapping Box::new with a Pin<T> making the value
    // wrapped by Box to opt-out of Unpin
    // We could use Pin<Box<SelfReferential>>  or Box::pin
    //let mut heap_value = Box::new(
    let mut heap_value = Box::pin(
        SelfReferential {
            self_ptr: 0 as *const _,
        }
    );

    unsafe {
        // unsafe because we need to make sure we don't move the reference of the struct to
        // a new location leaving the self_ptr as a dangling pointer
        let mut_ref = Pin::as_mut(&mut heap_value);
        // get_unchecked_mut works on Pin<&mut T> hence we use Pin::as_mut to get Pin<&mut T>
        Pin::get_unchecked_mut(mut_ref).self_ptr = ptr;
    }
    // let ptr = &*heap_value as *const SelfReferential;
    // heap_value.self_ptr = ptr;
    println!("Heap value at: {:p}", heap_value);
    println!("internal reference: {:p}", heap_value.self_ptr);

    // }

    /// BREAK HEAP ALLOCATION
    let stack_value = mem::replace(&mut *heap_value, SelfReferential {
        self_ptr: 0 as *const _,
    });

    /// PREVENTING THE CREATION OF DANGLING POINTERS
    // By using Pin on SelfReferential we prevent the moving of the SelfReferential struct
    // from the heap to the stack breaking the self-referencing
    // Pin therefore helps and to create "correct" self-referential structs in Rust
    // allbeit responsibility is left onto the shoulders of the programmer to make sure that
    // he is doing the right thing within the unsafe {} block
    println!("stack value at: {:p}", stack_value);
    println!("internal reference: {:p}", stack_value.self_ptr);

}

// a self referential struct that has an immutable pointer
// that points to itself

// Unpin is an auto trait meaning it is impl by default for all Rust types
// Pin allows us to explicitly opt-out of Unpin to get Pinning
// To do that we need the PhantomPinned marker
struct SelfReferential {
    self_ptr: *const Self,
    // by having a single field opt-out of Unpin, the whole struct opt-outs as well
    // PhantomPinned (zero-sized marker) purpose is to not implement Unpin trait
    _pin: PhantomPinned,
}
