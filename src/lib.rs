use clap::ValueEnum;
use reqwest::header::{self, CONTENT_DISPOSITION};
use serde::Deserialize;
use std::fmt::Display;
use std::{error::Error, fs, path::PathBuf};

// TODO(evert): Remove reference from clap so library can be used on own?
#[derive(ValueEnum, Clone, Deserialize, Debug)]
pub enum ExportType {
    #[serde(rename = "html")]
    HTML,
    #[serde(rename = "pdf")]
    PDF,
    #[serde(rename = "markdown")]
    Markdown,
}

impl Display for ExportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ExportType::HTML => String::from("html"),
            ExportType::PDF => String::from("pdf"),
            ExportType::Markdown => String::from("markdown"),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug)]
pub struct BookstackClient {
    bookstack_url: String,
    client: reqwest::blocking::Client,
}

#[derive(Deserialize, Debug)]
pub struct ShelveListItem {
    id: u32,
    slug: String,
}

#[derive(Deserialize, Debug)]
pub struct ShelveListResponse {
    data: Vec<ShelveListItem>,
}

#[derive(Deserialize, Debug)]
pub struct ShelveBookItem {
    id: u32,
    slug: String,
}

#[derive(Deserialize, Debug)]
pub struct Shelve {
    books: Vec<ShelveBookItem>,
}

#[derive(Deserialize, Debug)]
pub struct Page {
    id: u32,
}

#[derive(Deserialize, Debug)]
pub struct BookContent {
    id: u32,
    slug: String,
    r#type: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Book {
    contents: Vec<BookContent>,
}

#[derive(Deserialize, Debug)]
pub struct Chapter {
    pages: Vec<Page>,
}

// TODO(evert): This feels like a hack
pub fn get_filename(content_disposition: &str) -> String {
    match content_disposition.split_once("filename=") {
        Some((_, after)) => after.replace(['"', ';', '/', '\\'], "").to_string(),
        None => {
            panic!(
                "Could not get file name from content-disposition: {}",
                content_disposition
            );
        }
    }
}

impl BookstackClient {
    pub fn new(bookstack_url: &str, token_id: &str, token_secret: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        let mut auth_token = "Token ".to_string();
        auth_token.push_str(token_id);
        auth_token.push(':');
        auth_token.push_str(token_secret);

        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&auth_token)
                .expect("Failed to set Authorization header from API ID and Secret"),
        );

        let reqwest_client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create Reqwest client");

        BookstackClient {
            bookstack_url: bookstack_url.to_string(),
            client: reqwest_client,
        }
    }

    pub fn get_shelves(&self) -> Result<Vec<ShelveListItem>, Box<dyn Error>> {
        println!("get_shelves");

        let res = self
            .client
            .get(self.bookstack_url.to_owned() + "/api/shelves")
            .send()?;
        let data: ShelveListResponse = res.json()?;

        Ok(data.data)
    }

    pub fn get_shelve(&self, id: u32) -> Result<Shelve, Box<dyn Error>> {
        println!("get_shelve");
        dbg!(id);
        let res = self
            .client
            .get(self.bookstack_url.to_owned() + "/api/shelves/" + &id.to_string())
            .send()?;

        Ok(res.json::<Shelve>()?)
    }

    pub fn get_book(&self, id: u32) -> Result<Book, Box<dyn Error>> {
        println!("get_book");
        dbg!(id);
        let res = self
            .client
            .get(self.bookstack_url.to_owned() + "/api/books/" + &id.to_string())
            .send()?;

        Ok(res.json::<Book>()?)
    }

    pub fn get_chapter(&self, id: u32) -> Result<Chapter, Box<dyn Error>> {
        println!("get_chapter");
        dbg!(id);

        let res = self
            .client
            .get(self.bookstack_url.to_owned() + "/api/chapters/" + &id.to_string())
            .send()?;

        Ok(res.json::<Chapter>()?)
    }

    pub fn clone_page(
        &self,
        export_type: &ExportType,
        parent_path: &PathBuf,
        page_id: u32,
    ) -> Result<(), Box<dyn Error>> {
        println!("clone_page");
        dbg!(parent_path, page_id);

        let mut res = self
            .client
            .get(
                self.bookstack_url.to_owned()
                    + "/api/pages/"
                    + &page_id.to_string()
                    + "/export/"
                    + &export_type.to_string(),
            )
            .send()?;

        dbg!(&res);

        let filename: String;

        let content_disposition_result = res.headers().get(CONTENT_DISPOSITION);
        if let Some(content_disposition) = content_disposition_result {
            dbg!(&content_disposition);
            filename = get_filename(content_disposition.to_str()?);
            dbg!(&filename);
        } else {
            panic!(
                "Failed to get filename from content-disposition for exporting page ID {}, as {}",
                page_id, export_type
            );
        }

        println!("Exporting to {}", filename);
        let mut file = fs::File::create(parent_path.join(filename))?;
        std::io::copy(&mut res, &mut file)?;

        Ok(())
    }

    pub fn clone_chapter(
        &self,
        export_type: &ExportType,
        parent_path: &PathBuf,
        chapter_id: u32,
    ) -> Result<(), Box<dyn Error>> {
        println!("clone_chapter");
        dbg!(parent_path, chapter_id);

        let chapter = self.get_chapter(chapter_id)?;

        for page in chapter.pages {
            self.clone_page(export_type, parent_path, page.id)?;
        }

        Ok(())
    }

    pub fn clone_book(
        &self,
        export_type: &ExportType,
        parent_path: &PathBuf,
        book_id: u32,
    ) -> Result<(), Box<dyn Error>> {
        println!("clone_book");
        dbg!(parent_path, book_id);
        let book = self.get_book(book_id)?;

        for book_content in book.contents {
            if let Some(content_type) = book_content.r#type {
                if content_type == "chapter" {
                    let child_path = parent_path.join(book_content.slug);
                    fs::create_dir_all(&child_path)?;

                    self.clone_chapter(export_type, &child_path, book_content.id)?;
                } else if content_type == "page" {
                    self.clone_page(export_type, parent_path, book_content.id)?;
                }
            } else {
                self.clone_page(export_type, parent_path, book_content.id)?;
            }
        }

        Ok(())
    }

    pub fn clone_shelve(
        &self,
        export_type: &ExportType,
        parent_path: &PathBuf,
        shelve_id: u32,
    ) -> Result<(), Box<dyn Error>> {
        println!("clone_shelve");
        dbg!(parent_path, shelve_id);

        let shelve = self.get_shelve(shelve_id)?;

        for book_stub in shelve.books {
            let child_path = parent_path.join(&book_stub.slug);
            fs::create_dir_all(&child_path)?;

            self.clone_book(export_type, &child_path, book_stub.id)?;
        }

        Ok(())
    }

    pub fn clone_bookstack(
        &self,
        export_type: &ExportType,
        output_dir: &str,
    ) -> Result<(), Box<dyn Error>> {
        println!("clone_bookstack");
        dbg!(export_type, output_dir);

        let output_path = PathBuf::from(output_dir);
        fs::create_dir_all(&output_path)?;

        let shelves = self.get_shelves()?;

        for shelve_stub in shelves {
            let child_path = output_path.join(shelve_stub.slug);

            fs::create_dir_all(&child_path)?;
            self.clone_shelve(export_type, &child_path, shelve_stub.id)?;
        }

        Ok(())
    }
}
