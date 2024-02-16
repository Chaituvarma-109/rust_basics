use std::io::{Error, ErrorKind};

#[allow(dead_code)]
async fn my_async_call(url: &str) -> Result<serde_json::Value, Error> {
    let response: reqwest::Response = reqwest::get(url)
        .await
        .map_err(|_| Error::new(ErrorKind::Other, "Failed to make request"))?;

    let json_response: serde_json::Value = response
        .json::<serde_json::Value>()
        .await
        .map_err(|_| Error::new(ErrorKind::Other, "Failed to parse response"))?;

    Ok(json_response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calls_async_fn() {
        let api_url: &str = "https://cat-fact.herokuapp.com/facts/";
        let my_res: Result<serde_json::Value, std::io::Error> = my_async_call(api_url).await;

        match my_res {
            Ok(res) => {
                dbg!(res);
            }
            Err(_) => {
                panic!("Failed to make request");
            }
        }
    }
}
