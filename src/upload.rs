extern crate reqwest;

pub fn upload_file(key: &str, file: &str){

    let client = reqwest::blocking::Client::new();
    let form = reqwest::blocking::multipart::Form::new()
    .text("key", format!("{}", &key))
    .file("upload", file);

    match form {
        Ok(formx) => {
            
            let res = client.post("https://cameron.media/api/upload")
                .query(&[("key", &key)])
                .multipart(formx)
                .send();
            
            match res {
                Ok(resx) => {
                    println!("Responded with {:?}", resx);
                    println!("Body: {:?}", resx.text());
                },
                Err(err) => {
                    println!("Failed to upload {}", err);
                }
            }
        },
        Err(_) => {
            println!("Failed to create form");
        }
    }
}

