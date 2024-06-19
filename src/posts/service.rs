use crate::posts::repository::*;
use crate::models::Post;
use crate::db::DbConn;

pub fn get_all_posts(conn: &DbConn) -> Vec<Post> {
    fetch_all_posts(conn)
}

pub fn get_post_by_id(conn: &DbConn, post_id: i32) -> Post {
    fetch_post_by_id(conn, post_id)
}


pub fn create_post(conn: &DbConn, post: Post) -> Post {
    insert_post(conn, post)
}

pub fn update_post(conn: &DbConn, post_id: i32, post: Post) -> Post {
    modify_post(conn, post_id, post)
}

pub fn delete_post(conn: &DbConn, post_id: i32) -> usize {
    remove_post(conn, post_id)
}


