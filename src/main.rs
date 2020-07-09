use actix_multipart::Multipart;
//use actix_web::{http::header, middleware, web, App, Error, HttpResponse, HttpServer};
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use actix_files::Files;
// use actix_cors::Cors;
use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt};

use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use rand::Rng;

use std::path::Path;
use std::env;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct FileObj {
    fid: String,
    fileName: String,
    fileUrl: String,
    size: usize,
}

fn get_random_fid() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const RANDOM_LEN: usize = 12;
    let mut rng = rand::thread_rng();
    let random_filename: String = (0..RANDOM_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    random_filename.to_owned()
}

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream

    let pub_url = &env::var("PUB_URL").expect("`PUB_URL` must be set to run this server");
    let save_dir = &env::var("SAVE_DIR").expect("`SAVE_DIR` must be set to run this server");

    let mut results: Vec<FileObj> = vec![];

    while let Ok(Some(mut field)) = payload.try_next().await {

        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;

        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;

        // file extension
        let path =  Path::new(filename);
        let filext = &path.extension() // Some(std::ffi::OsStr)
            .unwrap()     // std::ffi::OsStr
            .to_str()     // Some(&str)
            .unwrap();    // &str

        // random filename
        let fid = get_random_fid();

        // random filename with extension
        let filename = fid.to_owned() + "." + &filext.to_owned();

        // file url
        let fileurl = pub_url.to_owned() + "/public/" + &filename;

        let filepath_os = Path::new(&save_dir).join(sanitize_filename::sanitize(&filename));
        let filepath = filepath_os.to_str().unwrap();
        let mut f = async_std::fs::File::create(filepath).await?;

        // Field in turn is stream of *Bytes* object
        let mut filesize: usize = 0;
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            filesize += data.len();
            f.write_all(&data).await?;
        }

        let obj: FileObj = FileObj {
            fid: fid.to_owned(),
            fileName: filename.to_owned(),
            fileUrl: fileurl.to_owned(),
            size: filesize
        };

        results.push(obj);

        println!("Saved: {}", &filename);
    }

    Ok(HttpResponse::Ok().json(results))
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/submit" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    // env_logger::init();

    dotenv().ok();

    let meep_ip = &env::var("MEEP_IP").expect("`MEEP_IP` must be set to run this server");
    let meep_port = &env::var("MEEP_PORT").expect("`MEEP_PORT` must be set to run this server");
    let save_dir = &env::var("SAVE_DIR").expect("`SAVE_DIR` must be set to run this server");

    async_std::fs::create_dir_all(&save_dir).await?;

    let bind_addr = meep_ip.to_owned() + ":" + &meep_port;

    let save_dir2 = save_dir.clone();

    HttpServer::new(move || {
        App::new()
            //.wrap(
            //    Cors::new()
            //         .allowed_origin("*")
            //         .allowed_methods(vec!["GET", "POST"])
            //         .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            //         .allowed_header(header::CONTENT_TYPE)
            //         .max_age(3600)
            //         .finish(),
            //)
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/submit")
                    .route(web::get().to(index))
                    .route(web::post().to(|item| save_file(item))),
            )
            .service(Files::new("/public", &save_dir2).show_files_listing())
    })
    .bind(&bind_addr)?
    .run()
    .await
}

