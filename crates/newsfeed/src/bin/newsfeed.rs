use newsfeed::{
    models::{comment::Comment, post::Post, user::User},
    service::Service,
};
fn main() {
    let mut service = Service::new();
    let user1 = User::new("Alice".to_string(), "email1@email.com".to_string());
    let user2 = User::new("Bob".to_string(), "email1@email.com".to_string());
    let userid1 = user1.get_id().to_string();
    let userid2 = user2.get_id().to_string();

    service.add_user(user1);
    service.add_user(user2);

    let post1 = Post::new(userid1.clone(), "Hello, world!".to_string());
    let post2 = Post::new(userid2.clone(), "Goodbye, world!".to_string());
    let post_id1 = post1.get_id().to_string();
    // let post_id2 = post2.get_id().to_string();

    service
        .login_user(&userid1)
        .expect("Failed to log in user1");

    service.add_post(post1).expect("Failed to add post1");

    service
        .follow_user(&userid2, &userid1)
        .expect("Failed to follow user2");

    service
        .login_user(&userid2)
        .expect("Failed to log in user2");
    service.add_post(post2).expect("Failed to add post2");

    let comment1 = Comment::new(
        post_id1.clone(),
        userid1.clone(),
        "This is a comment".to_string(),
    );
    let comment2 = Comment::new(
        post_id1.clone(),
        userid2.clone(),
        "This is another comment".to_string(),
    );

    service
        .add_comment(&post_id1, comment1)
        .expect("Failed to add comment1");
    service
        .add_comment(&post_id1, comment2)
        .expect("Failed to add comment2");

    let user1_feed = service
        .get_feed(&userid1)
        .expect("Failed to get user1 feed");

    for post in user1_feed {
        println!(
            "Post ID: {}, Content: {}",
            post.get_id(),
            post.get_content()
        );
        for comment in post.get_comments() {
            println!(
                "  Comment ID: {}, Content: {}",
                comment.get_id(),
                comment.get_content()
            );
        }

        println!("\n");
    }

    // service
    //     .logout_user(&userid2)
    //     .expect("Failed to log out user2");

    let user2_feed = service
        .get_feed(&userid2)
        .expect("Failed to get user2 feed");

    for post in user2_feed {
        println!(
            "Post ID: {}, Content: {}",
            post.get_id(),
            post.get_content()
        );
        for comment in post.get_comments() {
            println!(
                "  Comment ID: {}, Content: {}",
                comment.get_id(),
                comment.get_content()
            );
        }

        println!("\n");
    }
}
