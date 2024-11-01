
#[tokio::main]
pub async fn execute() -> anyhow::Result<()> {
    // Request a new server from the pool
    let mut server = mockito::Server::new();

    // Use one of these addresses to configure your client
    let _host = server.host_with_port();
    let url = server.url();

    // Create a mock
    let mock = server.mock("GET", "/hello")
      .with_status(201)
      .with_header("content-type", "text/plain")
      .with_header("x-api-key", "1234")
      .with_body("world")
      .create();

    let client = reqwest::Client::new();
    let response = client.get(&format!("{}/hello", url)).send().await?;
    assert_eq!(response.status(), 201);
    assert_eq!("world", response.text().await?);

    mock.assert();
    Ok(())
}