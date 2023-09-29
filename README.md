# About

A simple confluence API wrapper. Contributions are welcome!

## Example

```rust
let session = Session::new(
    "joe@example.com".to_string(),
    "your token".to_string(),
    "https://example.atlassian.net/wiki".to_string(),
);

let spaces = session.get_spaces().await.expect("Failed to get spaces");
for space in spaces {
    let pages = session
        .get_pages_for_space(&space.key, None)
        .await
        .expect("Failed to get pages");
    
    info!(
        "Space({:?}): {:?} with {} pages",
        space.key,
        space.name,
        pages.len()
    );

    for page in pages {
        let html = &page.body.unwrap().view.unwrap().value;

        // replace relative links with absolute links
        let html = html.replace("(/wiki/", "(https://example.atlassian.net/wiki/");
  }
}
```
