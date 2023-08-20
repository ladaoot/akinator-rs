// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::write;
use std::io::Error;
use std::path::PathBuf;
use std::sync::Mutex;

static mut ALREADY_ASKED: Vec<u64> = Vec::new();
static mut ACTUAL_PERSON: Vec<Person> = Vec::new();
static mut FLAG: bool = false;
static mut YES_QYESTION: Vec<u64> = Vec::new();
static mut QUESTION_MAP: Lazy<Mutex<HashMap<String, Vec<Question>>>> = Lazy::new(|| {
    let m: HashMap<String, Vec<Question>> = HashMap::new();
    Mutex::new(m)
});
static mut TITLE: Lazy<Vec<String>> = Lazy::new(|| vec!["general".to_string()]);
static mut YES_NEXT_TITLE: Lazy<Vec<String>> = Lazy::new(|| vec!["general".to_string()]);
static mut NO_NEXT_TITLE: Lazy<Vec<String>> = Lazy::new(|| vec!["general".to_string()]);

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Question {
    text: String,
    id: u64,
    yes: Option<String>,
    no: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
struct Person {
    name: String,
    questions: Vec<u64>,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq_ignore_ascii_case(other.name.as_str()) || self.questions == other.questions
    }
}

fn get_all_question() {
    let questions_path: PathBuf = ["../data", "question.json"].iter().collect();

    let questions_json =
        std::fs::read_to_string(&questions_path).expect("Failed to read questions file");

    let values: Vec<serde_json::Value> = serde_json::from_str(&questions_json).unwrap();

    let mut map: HashMap<String, Vec<Question>> = HashMap::new();

    for i in values {
        let t = i.as_object().unwrap().clone();
        let title = t.get("title").unwrap().as_str().unwrap().to_string();
        let mut v = Vec::new();
        let que = t.get("questions").unwrap().as_array().unwrap().clone();
        for q in que {
            let ob = q.as_object().unwrap().clone();
            let yes = match ob.get("yes") {
                Some(x) => Some(x.as_str().unwrap().to_string()),
                None => None,
            };
            let no = match ob.get("no") {
                Some(x) => Some(x.as_str().unwrap().to_string()),
                None => None,
            };
            v.push(Question {
                text: ob.get("q").unwrap().as_str().unwrap().to_string(),
                id: ob.get("id").unwrap().as_u64().unwrap(),
                yes,
                no,
            })
        }
        map.insert(title, v);
    }

    unsafe {
        let c = QUESTION_MAP.get_mut().unwrap();
        for (k, v) in map {
            c.insert(k, v);
        }
    }
}

#[tauri::command]
fn question() -> String {
    unsafe {
        let question = QUESTION_MAP.get_mut().unwrap();
        if question.len() == 0 {
            get_all_question()
        }

        let mut vec = question.get_mut(TITLE.get(0).unwrap()).unwrap();
        let q = vec.remove(0);
        ALREADY_ASKED.push(q.id);

        match q.yes {
            Some(x) => {
                YES_NEXT_TITLE.pop();
                YES_NEXT_TITLE.push(x)
            }
            None => {
                YES_NEXT_TITLE.pop();
                YES_NEXT_TITLE.push(TITLE.get(0).unwrap().clone())
            }
        }
        match q.no {
            Some(x) => {
                NO_NEXT_TITLE.pop();
                NO_NEXT_TITLE.push(x)
            }
            None => {
                NO_NEXT_TITLE.pop();
                NO_NEXT_TITLE.push(TITLE.get(0).unwrap().clone())
            }
        }

        q.text
    }
}

fn get_all_persons() {
    let person_path: PathBuf = ["../data", "persons.json"].iter().collect();
    let person_json = std::fs::read_to_string(&person_path).expect("Failed to read persons file");
    let values: Vec<serde_json::Value> = serde_json::from_str(&person_json).unwrap();
    let persons = values
        .iter()
        .map(|x| -> Person {
            let p = x.as_object().unwrap();
            Person {
                name: p.get("name").unwrap().as_str().unwrap().to_string(),
                questions: p
                    .get("questions")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|el| el.as_u64().unwrap())
                    .collect(),
            }
        })
        .collect::<Vec<_>>();
    unsafe {
        for per in persons {
            ACTUAL_PERSON.push(per);
        }
    }
}

#[tauri::command]
fn check(answer: u8) -> Vec<String> {
    unsafe {
        if !FLAG && ACTUAL_PERSON.len() == 0 {
            get_all_persons();
            FLAG = true;
        }

        let d = ACTUAL_PERSON
            .iter()
            .filter(|x| {
                let q = ALREADY_ASKED.last().unwrap();
                let ret = x.questions.contains(q);
                if answer == 1 {
                    ret
                } else {
                    !ret
                }
            })
            .map(|x| x.clone())
            .collect::<Vec<_>>();
        ACTUAL_PERSON.clear();
        for el in &d {
            ACTUAL_PERSON.push(el.clone());
        }
        if answer == 1 {
            YES_QYESTION.push(*(ALREADY_ASKED.last().unwrap()));
            TITLE.pop();
            TITLE.push(YES_NEXT_TITLE.get(0).unwrap().clone());
        } else {
            TITLE.pop();
            TITLE.push(NO_NEXT_TITLE.get(0).unwrap().clone());
        }

        let mut answ: Vec<String> = d.iter().map(|x| x.name.clone()).collect();
        if YES_QYESTION.len() < 15 && *ALREADY_ASKED.last().unwrap()!=404{
            answ.push("none".to_string());
            answ.push("none".to_string());

        }
        println!("{:?}", answ);
        println!("asked {}, yes {}", ALREADY_ASKED.len(), YES_QYESTION.len());
        answ
    }
}

#[tauri::command]
fn restart() {
    unsafe {
        FLAG = false;
        ALREADY_ASKED = Vec::new();
        ACTUAL_PERSON = Vec::new();
        YES_QYESTION = Vec::new();
        QUESTION_MAP = Lazy::new(|| {
            let m: HashMap<String, Vec<Question>> = HashMap::new();
            Mutex::new(m)
        });
        TITLE = Lazy::new(|| vec!["general".to_string()]);
        YES_NEXT_TITLE = Lazy::new(|| vec!["general".to_string()]);
        NO_NEXT_TITLE = Lazy::new(|| vec!["general".to_string()]);
    }
}

#[tauri::command]
fn isStart() -> bool {
    unsafe { FLAG || !ALREADY_ASKED.is_empty() || !ACTUAL_PERSON.is_empty() }
}

#[tauri::command]
fn save(name: String) -> Result<String, String> {
    unsafe {
        let per = Person {
            name: name,
            questions: YES_QYESTION.clone(),
        };

        get_all_persons();
        if ACTUAL_PERSON.contains(&per) {
            return Result::Ok("alredy exists".to_string());
        }

        let person_path: PathBuf = ["../data", "persons.json"].iter().collect();

        let mut person_json =
            std::fs::read_to_string(&person_path).expect("Failed to read persons file");
        person_json.pop();
        person_json.pop();
        person_json.push_str(
            format!(
                ",{{\"name\": \"{}\", \"questions\":{:?}}}\n",
                per.name, per.questions
            )
            .as_str(),
        );
        person_json.push(']');
        write(person_path, person_json).expect("Can't write to file");
        return Result::Ok("ok".to_string());
    }
}

#[tauri::command]
fn cleanYes() {
    unsafe {
        YES_QYESTION = Vec::new();
    }
}

#[tauri::command]
fn isYesEmpty() -> bool {
    unsafe { YES_QYESTION.is_empty() }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            question, check, restart, isStart, save, cleanYes, isYesEmpty
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
