#[cfg(test)]
mod test {
    use super::rocket;
    use super::*;
    use rocket::http::Status;

    #[rocket::async_test]
    /// make sure pagination in the API works
    /// different pages will return different results
    async fn test_search_pagination() {
        use rocket::local::asynchronous::Client;

        let client = Client::tracked(rocket()).await.unwrap();
        let page_1_response = client
            .get(uri!("/meetup", search("tech", 1, 10)))
            .dispatch()
            .await;

        let page_2_response = client
            .get(uri!("/meetup", search("tech", 2, 10)))
            .dispatch()
            .await;

        assert_eq!(page_1_response.status(), Status::Ok);
        assert_eq!(page_2_response.status(), Status::Ok);

        // make sure both pages are different
        assert_ne!(
            &page_1_response.into_string().await.unwrap(),
            &page_2_response.into_string().await.unwrap()
        );
    }
}
