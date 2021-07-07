use cyberdrop_dl::get_album_images;
use cyberdrop_dl::get_album_size;
use cyberdrop_dl::get_album_title;
use cyberdrop_dl::image_name_from_url;

#[tokio::test]
async fn test_image_name_from_url() {
    let url = String::from("https://fs-01.cyberdrop.cc/123.jpg");
    let name = image_name_from_url(&url).await.unwrap();
    assert_eq!(name, "/123.jpg")
}

#[tokio::test]
async fn test_get_album_title() {
    let body = include_str!("album.html");
    let result = get_album_title(&body).await.unwrap();
    assert_eq!(result, "Test")
}

#[tokio::test]
async fn test_get_album_size() {
    let body = include_str!("album.html");
    let result = get_album_size(&body).await.unwrap();
    assert_eq!(result, "447.75 KB")
}

#[tokio::test]
async fn test_get_album_images() {
    let body = include_str!("album.html");
    let vec = vec![
        "https://fs-03.cyberdrop.to/1.jpeg".to_string(),
        "https://fs-03.cyberdrop.to/2.jpeg".to_string(),
    ];
    let result = get_album_images(&body).await.unwrap();
    assert_eq!(result, vec)
}
