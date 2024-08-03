use std::io::{Error, ErrorKind};

#[allow(dead_code)]
async fn api_call(url: &str) -> Result<serde_json::Value, std::io::Error> {
    let response = reqwest::get(url)
        .await
        .map_err(|_| Error::new(ErrorKind::Other, "Could not retrieve response"))?;

    let response = response
        .json::<serde_json::Value>()
        .await
        .map_err(|_| Error::new(ErrorKind::Other, "Could not decode to json"))?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_calls_async_fn_7_1() {
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
