// use warp::Filter;
#[path = "../entities/post.rs"]
mod post;
use post::Post;

#[derive(Clone)]
pub struct PostUsecase {
    pub title: String,
}

impl PostUsecase {
    pub fn new(title: String) -> Self {
        PostUsecase { title }
    }
    // A function to handle GET requests at /posts/{id}
    pub async fn get_post(&self, id: u64) -> Result<impl warp::Reply, warp::Rejection> {
        // For simplicity, let's say we are returning a static post
        let post: Post = Post {
            id: id,
            title: self.title.clone(),
            body: String::from("This is a post about Warp."),
        };
        Ok(warp::reply::json(&post))
    }
}
