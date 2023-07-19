// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

// use core::num::dec2flt::number::Number;
use rand::seq::SliceRandom;
use rand::Rng;
use std::{path::PathBuf, sync::Arc};
use serde::{Deserialize, Serialize};

static mut ALREADY_ASKED: Vec<String> = Vec::new();

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Question {
    text: String,
    id: u64,
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
        let fil= data
            .iter()
            .map(|x| x.text.clone())
            .filter(|x| !ALREADY_ASKED.contains(x))
            .collect::<Vec<_>>();

        let q = fil.choose(&mut rand::thread_rng()).cloned();
        if let Some(qu) = q.clone() {
            ALREADY_ASKED.push(qu);
        }
        q.unwrap()
    }
}

fn main() {
    // println!("{:?}", get_all_question());

  
    // println!("{:?}", question(get_all_question()).unwrap());

    println!("{}", question());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![question])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
