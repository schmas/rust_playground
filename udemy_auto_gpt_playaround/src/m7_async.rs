#[allow(dead_code)]
async fn api_call(url: &str) -> Result<serde_json::Value, reqwest::Error> {
    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_calls_async_fn() {
        let api_url = "https://cat-fact.herokuapp.com/facts/";
        let my_res = api_call(api_url).await;

        match my_res {
            Ok(r) => {
                dbg!(r)
            }
            Err(_) => {
                panic!("Failed to make request")
            }
        };
    }
}
