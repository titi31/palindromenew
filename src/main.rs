use url::Url;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let sites=["https://www.google.com","https://www.youtube.com","https://shodan.io",
    "https://github.com/elbabili/dff"];
   
    for i in &sites {
        let url = Url::parse(i).unwrap();
        let rep = reqwest::get(url)
        .await?;
        if rep.status().is_success(){
            println!("c'est bon");
        }else{
            println!("ce n'est pas bon {}",rep.status());
            write(Url::parse(i).unwrap().to_string(),rep.status().to_string());

        }
       
    }
    Ok(())
}
fn write(site:String,erreur:String) -> std::io::Result<()>{
    let contents=format!("site = {}  erreur = {} ",site,erreur);
    fs::write("test.txt", &contents)?;
    println!("{:?} ",contents);
    Ok(())
}

