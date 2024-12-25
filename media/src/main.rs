mod content;
use content::catalog::Catalog;
use content::media::Media;

fn main() {
    let mut catalog = Catalog::new();
    let audiobook = Media::Audiobook {
        title: String::from("An audiobook"),
    };
    let book = Media::Book {
        title: String::from("A book"),
        author: String::from("the authore"),
    };
    let movie = Media::Movie {
        title: String::from("A movie"),
        director: String::from("The director"),
    };
    let podcast = Media::Podcast(32);
    let placeholder = Media::Placeholder;
    catalog.add(audiobook);
    catalog.add(book);
    catalog.add(movie);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("{:#?}", catalog.describe());
    println!("Number of items: {}", catalog.count());

    // let item = catalog.get_by_index(1);
    // let value = item.unwrap();
    // let value = item.expect("There should be a value here");
    // let value = item.unwrap_or(&Media::Placeholder);
}
