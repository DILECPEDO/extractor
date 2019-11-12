use easy_http_request::DefaultHttpRequest;
use serde::Serialize;
use serde_json;
use std::error::Error;
use std::fs;

fn extract_json(file: &str) -> serde_json::value::Value {
    let file = fs::File::open(file).expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");
    json
}
fn random_number(min: i32, max: i32) -> i16 {
    let url = format!(
        "https://www.random.org/integers/?num=1&min={}&max={}&col=1&base=10&format=plain&rnd=new",
        min, max
    );
    let response = DefaultHttpRequest::get_from_url_str(url)
        .unwrap()
        .send()
        .unwrap();
    let response_body = String::from_utf8(response.body).unwrap();
    response_body.replace("\n", "").parse().unwrap()
}

fn studente_interrogato(numero_studente: i16, materia: &String) -> bool {

    let data = extract_json("data.json");

    let x = data["materie"][materia].as_array().unwrap();

    let x = x
.iter()
.any(|c| c.as_u64().unwrap() as i16 == numero_studente);

    x
}

// fn add_student_to_subject(numero_studente: i16, materia: &String){
//     let data = extract_json("config.json");

// }

fn estrazione(config: &String, data: &String) {
    let data = extract_json(data);
    let config = extract_json(config);
    // let mut estratti = vec![];
    for i in config["interrogazioni"].as_array().unwrap() {
        let mut estratti : std::vec::Vec<i16> = Vec::new();
        let materia = i["materia"].as_str().unwrap().to_string();
        let size = i["size"].as_u64().unwrap() as i16;
        for a in 0..size {
            let mut estratti_da_verificare : std::vec::Vec<i16> = Vec::new();
            loop {

                let interrogato: i16 = random_number(1, 25);
                if !studente_interrogato(interrogato, &materia) && !estratti.contains(&interrogato) {
                    estratti.push(interrogato);
                    break;
                }
            }
        }
        println!("{} -> {:?}", materia, estratti);
    }
}

fn main() {
    estrazione(&"config.json".to_string(), &"data.json".to_string());
    // for _ in 0..100 {
    //     println!("{}", random_number(1, 25));
    // }

}
    
    
