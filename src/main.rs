use state_pattern::Post;

fn main() {
    // Approach 1 - state pattern
    let mut post1 = Post::default();

    post1.add_text("Hello world");
    println!("- {}", post1.content());

    post1.request_review();
    println!("- {}", post1.content());

    post1.approve();
    println!("- {}", post1.content());

    // Approach 2 -
    // let mut post1 = Post::new();

    // post1.add_text("Hello world");

    // let post1 = post1.request_review();

    // let post1 = post1.approve();
    // println!("- {}", post1.content());
}
