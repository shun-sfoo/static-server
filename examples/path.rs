use tokio::fs;

#[tokio::main]
async fn main() {
    let mut dir = fs::read_dir(".").await.unwrap();

    while let Some(child) = dir.next_entry().await.unwrap() {
        let path = child.path().to_string_lossy().to_string();
        println!("{}", path);
    }
}
