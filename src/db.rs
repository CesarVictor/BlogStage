use diesel::prelude::*;
use crate::schema::posts;
use crate::models::Post;
use crate::db::DbConn;


pub fn fetch_all_posts(conn: &DbConn) -> Vec<Post> {
    posts::dsl::posts.load::<Post>(&**conn).expect("Error loading posts")
}

pub fn fetch_post_by_id(conn: &DbConn, post_id: i32) -> Post {
    posts::dsl::posts.find(post_id).first::<Post>(&**conn).expect("Error loading post")
}

pub fn insert_post(conn: &DbConn, post: Post) -> Post {
    diesel::insert_into(posts::table)
        .values(&post)
        .get_result(&**conn)
        .expect("Error saving new post")
}

pub fn modify_post(conn: &DbConn, post_id: i32, post: Post) -> Post {
    diesel::update(posts::dsl::posts.find(post_id))
        .set(&post)
        .get_result(&**conn)
        .expect("Error updating post")
}

pub fn remove_post(conn: &DbConn, post_id: i32) -> usize {
    diesel::delete(posts::dsl::posts.find(post_id))
        .execute(&**conn)
        .is_ok()
        .expect("Error deleting post")
}