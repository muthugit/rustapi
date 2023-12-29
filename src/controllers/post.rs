use warp::Filter;
#[path = "../usecases/post.rs"] mod post;
use post::PostUsecase;


// A function to build our routes
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post_usecase = PostUsecase::new(String::from("Hello, Warp!"));
    get_post(post_usecase)
}

fn get_post(post_usecase: PostUsecase) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post_usecase = warp::any().map(move || post_usecase.clone());
    warp::path!("posts" / u64)
        .and(warp::get())
        .and(post_usecase)
        .and_then(|id: u64, post_usecase: PostUsecase| async move {
            post_usecase.get_post(id).await
        })
}
