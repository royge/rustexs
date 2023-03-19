use blog;
use blog::oop::{Post};

fn main() {
    oop();
    rust();
}

fn rust() {
    let mut post = blog::Post::new();
    post.add_text("I ate salad today.");

    let post = post.request_review();
    let post = post.approve();

    assert_eq!("I ate salad today.", post.content());
}
fn oop() {
    let mut post = Post::new();

    post.add_text("I ate salad today.");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    // 1st approval.
    post.approve();
    assert_eq!("", post.content());

    // 2nd approval.
    post.approve();
    assert_eq!("I ate salad today.", post.content());

    post.approve();
    assert_eq!("I ate salad today.", post.content());

    // Can't change content after draft.
    post.add_text("I ate apple today.");
    assert_eq!("I ate salad today.", post.content());
}
