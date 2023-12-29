#[path = "controllers/post.rs"] mod post;

#[tokio::main]
async fn main() {
    let routes = post::routes();

    println!("Server started at http://localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
