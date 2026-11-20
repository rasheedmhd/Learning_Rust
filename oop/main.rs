use oop::{Post, SafePost};

fn main() {
    // unapproved posts should not be able to get posted
    let mut post = Post::new();

    post.add_content("Where Lifetimes come from and where they go!");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(
        "Where Lifetimes come from and where they go!",
        post.content()
    );

    // ENCODING TYPE SAFETY
    let mut safepost = SafePost::new();

    safepost.add_text("Writing Pharaoh Web Framework in Rust");

    let safepost = safepost.request_review();

    let safepost = safepost.approve();
}
