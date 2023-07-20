// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

// use core::num::dec2flt::number::Number;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

static mut ALREADY_ASKED: Vec<u64> = Vec::new();
static mut ACTUAL_PERSON: Vec<Person> = Vec::new();
static mut FLAG: bool = false;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Question {
    text: String,
    id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Person {
    name: String,
    questions: Vec<u64>,
}

fn get_all_question() -> Vec<Question> {
    let questions_path: PathBuf = ["question.json"].iter().collect();

    let questions_json =
        std::fs::read_to_string(&questions_path).expect("Failed to read questions file");

    let values: Vec<serde_json::Value> = serde_json::from_str(&questions_json).unwrap();
    values
        .iter()
        .map(|x| -> Question {
            let q = x.as_object().unwrap();
            Question {
                text: q.get("text").unwrap().as_str().unwrap().to_string(),
                id: q.get("id").unwrap().as_u64().unwrap(),
            }
        })
        .collect::<Vec<_>>()
}

#[tauri::command]
fn question() -> String {
    let data = get_all_question();
    unsafe {
        let fil = data
            .iter()
            .filter(|x| !ALREADY_ASKED.contains(&x.id))
            .map(|x| x.text.clone())
            .collect::<Vec<_>>();

        let q = fil.choose(&mut rand::thread_rng()).cloned();
        if let Some(qu) = q.clone() {
            ALREADY_ASKED.push(
                data.iter()
                    .filter(|x| x.text.eq_ignore_ascii_case(qu.as_str()))
                    .map(|x| x.id)
                    .max()
                    .unwrap(),
            );
        }

        q.unwrap()
    }
}

fn get_all_persons() {
    let person_path: PathBuf = ["persons.json"].iter().collect();
    let person_json = std::fs::read_to_string(&person_path).expect("Failed to read questions file");
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
                let ret = x.questions.contains(ALREADY_ASKED.last().unwrap());
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
        d.iter().map(|x| x.name.clone()).collect()
    }
}

#[tauri::command]
fn restart() {
    unsafe {
        FLAG = false;
        ALREADY_ASKED = Vec::new();
        ACTUAL_PERSON = Vec::new();
    }
}

#[tauri::command]
fn isStart() -> bool {
    unsafe { FLAG || !ALREADY_ASKED.is_empty() || !ACTUAL_PERSON.is_empty() }
}

fn main() {
    get_all_persons();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![question, check, restart, isStart])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
