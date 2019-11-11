use easy_http_request::DefaultHttpRequest;
use serde_json;
use std::fs;

fn extract_json(file : &str) -> serde_json::value::Value {
    let file = fs::File::open(file)
    .expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
    .expect("file should be proper JSON");
    json
}
fn random_number(min: i32, max: i32)-> i16 {
    
    // let url = format!("https://www.random.org/integers/?num=1&min={}&max={}&col=1&base=10&format=plain&rnd=new", min ,max);
    // let response = DefaultHttpRequest::get_from_url_str(url).unwrap().send().unwrap();
    // let response_body = String::from_utf8(response.body).unwrap();
    // println!("{}", response_body);
    // // let x = response_body[2..].parse().unwrap();
    // println!("{}", response_body);
    16
}

fn studente_interrogato(numero_studente: i16, materia: &String)-> bool{
    let data = extract_json("data.json");
    let x = data["materia"][materia].as_array().unwrap();
    let x = x.iter().any(|c| c.as_u64().unwrap() as i16 ==  numero_studente);
    x
}

// fn add_student_to_subject(numero_studente: i16, materia: &String){
//     let data = extract_json("config.json");

// }


fn estrazione(config: &String, data: &String){
    let data = extract_json(data);
    let config = extract_json(config);
    println!("1");
    // let estratti = vec![];
    for i in config["interrogazioni"].as_array().unwrap() {
        let mut estratti = vec![];
        let materia = &i["materia"].as_str().unwrap().to_string();
        let size = i["size"].as_u64().unwrap() as i16;
        println!("1");

        for a in 0..size{
            println!("12");

            loop{
                println!("13");

                let interrogato = random_number(1, 25);
                if !studente_interrogato(interrogato, materia){
                    estratti.push(interrogato);
                    println!("{}", interrogato);
                

                    break
                }
            }
            println!("loop stopped");

        
        }
        
        println!("{} -> {:?}", materia,estratti );
    }
}




fn main(){

estrazione(&"config.json".to_string(), &"data.json".to_string());



}
