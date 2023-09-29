// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[allow(warnings,unused)]
mod conway;
use ureq;
use std::sync::{Mutex, Arc};
use serde_json::{to_string,from_str, json};
use conway::*;


#[tauri::command]
fn test_normal(){
  println!("tauri funk");
}

#[tauri::command]
fn error_test() ->  Result<(),ConwayError> {
  Err(ConwayError::NotFound)
}

#[tauri::command]
fn test_param(asd:String) -> Result<serde_json::Value,ConwayError> {
  println!("tauri funk {}", asd);
  return Ok(json!(["s","f"]));
}


fn main() {
  let mut game_entities = Arc::new(Mutex::new(GameEntities::new()));
  let efe = [1,2];
  //if efe[3]{}
  tauri::Builder::default()
    .manage(game_entities)
    .invoke_handler(tauri::generate_handler![test_normal,test_param,error_test,add_agent,update_game,get_game_data,game_data_restart])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
 
 }



