mod content;

use content::{catalog::Catalog, media::Media};

fn main() {
    let mut catalog: Catalog = Catalog::new();
    let audiobook: Media = Media::Audiobook {
        title: String::from("An Audiobook"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Best Director"),
    };
    let book = Media::Book {
        title: String::from("Book Title"),
        author: String::from("Book Author"),
    };
    let podcast = Media::Podcast(127);
    let placeholder = Media::Placeholder;

    println!("{}", &book.description());

    catalog.add_media(audiobook);
    catalog.add_media(good_movie);
    catalog.add_media(book);
    catalog.add_media(podcast);
    catalog.add_media(placeholder);

    let item: Option<&Media> = catalog.get_by_index(0);

    match item {
        Some(value) => {
            println!("{:#?}", value)
        }

        None => println!("No item found!"),
    }

    println!("{:#?}", catalog);
}
