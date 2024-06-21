use blog_lib_crate::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    println!("Content after adding text: {}", post.content()); // Should print empty content

    post.request_review();
    println!("Content after requesting review: {}", post.content()); // Should print empty content

    post.approve();
    println!("Content after first approval: {}", post.content()); // Should print empty content

    post.approve();
    println!("Content after second approval: {}", post.content()); // Should print the actual content

    post.reject();
    println!("Content after rejection (should be Draft): {}", post.content()); // Should print empty content if not yet approved

    post.add_text(" Adding more content in draft.");
    println!("Content after adding more text in draft: {}", post.content()); // Should print empty content if in Draft
}
