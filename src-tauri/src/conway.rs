use std::sync::{Mutex, Arc};
use serde_json::json;
use tauri::State;
use serde::{Serialize,Deserialize};


#[derive(Debug,Serialize,Deserialize,Clone)]
pub enum ConwayError {
    Ok,
    Error(String),
    NotFound,
    AlreadyExists,
}
pub enum GameMainArray{
    Agnets,
    Buffer
}
#[derive(Debug,Serialize,Deserialize,PartialEq,Clone)]
pub enum AgentState {
    Alive,
    Dead
}


#[derive(Debug,Serialize,Deserialize)]
pub struct GameEntities{
    agents: Vec<Agent>,
    buffer: Vec<Agent>
}
impl GameEntities {
    pub fn new() -> Self {
      Self { agents: Vec::<Agent>::new(),buffer: Vec::new() }
    }

    pub fn reset_data(&mut self){
        self.agents.clear();
    }

    //if agent is already exists, change agent state 
    pub  fn new_agent(&mut self,pos_x:i32,pos_y:i32) -> Result<(),ConwayError>{
        let mut id = self.get_agent_id_from_pos(pos_x, pos_y);

        return match id {
            Ok(ok) => {
                self.change_agent_state(self.agents[ok].pos_x, self.agents[ok].pos_y);

                return Err(ConwayError::AlreadyExists)
            }
            Err(e) => {
                self.agents.push(Agent { neighbors: 0, pos_x, pos_y, state: AgentState::Alive });
                return Ok(()) 
            }
        };
    }
    pub fn new_dead_agent(&mut self,pos_x:i32,pos_y:i32) -> Result<(),ConwayError>{
        if self.agents.len() == 0 {return Err(ConwayError::NotFound)}
        for x in 0..self.agents.len() {
            
            if self.agents[x].pos_x == pos_x && self.agents[x].pos_y == pos_y{
                return Err(ConwayError::AlreadyExists)
            }
        }

        self.agents.push(Agent { neighbors: 0, pos_x, pos_y, state: AgentState::Dead });
        Ok(())
    }

    fn change_agent_state(&mut self,pos_x:i32,pos_y:i32) {
        let id = self.get_agent_id_from_pos(pos_x, pos_y).unwrap();
        if self.agents[id].state == AgentState::Dead {self.agents[id].state = AgentState::Alive;}
        else if self.agents[id].state == AgentState::Alive{self.agents[id].state = AgentState::Dead;}
    }
    //return json data of all alive agents
    pub fn return_data(&mut self) -> Result<serde_json::Value,()>{
        let mut data:Vec<(i32,i32)> = Vec::new(); 
        for x in 0..self.agents.len() {
            if self.agents[x].state == AgentState::Alive{
                data.push((self.agents[x].pos_x,self.agents[x].pos_y));
            }
        }
        return Ok(json!(serde_json::to_string(&data).unwrap()));
        
        Err(())
        //let new_vec :Vec<(i32,i32)>;
        //for x in self.agents.iter() {
        //    new_vec.push((x.pos_x,x.pos_y))
        //}
        //let efe = serde_json::from_reader();
        
    }

    pub fn make_buffer(&mut self) {
        self.buffer = self.agents.clone();
    }

    pub fn commit_changes(&mut self){
        self.agents = Vec::new();
        for x in 0..self.buffer.len(){
            if self.buffer[x].state == AgentState::Dead && self.buffer[x].neighbors == 0 {
                continue;
            }else {
                self.agents.push(self.buffer[x].clone());
            }
        }
    }

    pub fn calculate_neighbors(&mut self) -> Result<(),ConwayError> {
        let len = self.agents.len();
        for x in 0..len {
            self.agents[x].neighbors = 0;
        }

        for x in 0..len{
            if self.agents[x].state == AgentState::Alive{
                self.new_dead_agent(self.agents[x].pos_x -1, self.agents[x].pos_y +1);
                self.new_dead_agent(self.agents[x].pos_x -1, self.agents[x].pos_y );
                self.new_dead_agent(self.agents[x].pos_x -1, self.agents[x].pos_y -1);
                self.new_dead_agent(self.agents[x].pos_x , self.agents[x].pos_y );
                self.new_dead_agent(self.agents[x].pos_x , self.agents[x].pos_y );
                self.new_dead_agent(self.agents[x].pos_x +1, self.agents[x].pos_y +1);
                self.new_dead_agent(self.agents[x].pos_x +1, self.agents[x].pos_y);
                self.new_dead_agent(self.agents[x].pos_x +1, self.agents[x].pos_y -1);
                
            }
        }
        drop(len);
        let len = self.agents.len();
        //calculate neighbors
        for x in 0..len {
            //add dead neighbors
            
            for y in 0..len {
                if self.agents[y].state == AgentState::Dead{continue;}
                if x ==y{continue;}
                let x_distance = self.agents[x].pos_x - self.agents[y].pos_x; if x_distance >1 || x_distance < -1 {continue;}
                let y_distance = self.agents[x].pos_y - self.agents[y].pos_y; if y_distance >1 || y_distance < -1 {continue;}
                self.agents[x].neighbors +=1;
            }
            
        }
        //Err(ConwayError::Error("adfafasFAs".to_string()));
        return Ok(());
    }

    //bufferdan orjinale geçerken komşusuz ölü hücreler ölmeli
    fn update_game(&mut self) -> Result<(),ConwayError>{
        self.calculate_neighbors();
        self.buffer.clear();
        for x in 0..self.agents.len(){
            
            if self.agents[x].state == AgentState::Alive{
                if self.agents[x].neighbors == 2 || self.agents[x].neighbors ==3 {
                    self.buffer.push(Agent { neighbors: 0, pos_x: self.agents[x].pos_x, pos_y: self.agents[x].pos_y, state: AgentState::Alive });
                } 
            }
            else if self.agents[x].state == AgentState::Dead{
                if self.agents[x].neighbors == 3 {
                    self.buffer.push(Agent { neighbors: 0, pos_x: self.agents[x].pos_x, pos_y: self.agents[x].pos_y, state: AgentState::Alive})
                }
            }
        }

        self.agents = self.buffer.clone();
        Ok(())
    }
    fn get_agent_from_buffer(&mut self, agent:&Agent ) -> Result<&Agent,ConwayError>{
        for x in self.agents.iter(){
            if x == agent{
                return Ok(x);
            }
        }
        return Err(ConwayError::NotFound);
    }
    fn get_agent_from_pos(&self,pos_x:i32,pos_y:i32) -> Result<&Agent,ConwayError> {
        
        if self.agents.len() == 0 {
            return Err(ConwayError::NotFound);
        }
        let mut ragent:Option<&Agent>= None; 
        
        for x in &self.agents{
             if x.pos_x == pos_x && x.pos_y == pos_y{
                ragent = Some(x);
                break;
             }
        }
        return match ragent {
            Some(s) => Ok(s) ,
            None => return Err(ConwayError::NotFound)
        };
    }

    fn get_agent_id_from_pos(&self,pos_x:i32,pos_y:i32) -> Result<usize,ConwayError> {
        
        if self.agents.len() == 0 {
            return Err(ConwayError::NotFound);
        }
        let mut ragent:Option<usize> = None; 
        
        for x in 0..self.agents.len(){
             if self.agents[x].pos_x == pos_x && self.agents[x].pos_y == pos_y{
                ragent = Some(x);
                break;
             }
        }
        return match ragent {
            Some(s) => Ok(s) ,
            None => return Err(ConwayError::NotFound)
        };
    }
}

#[derive(Debug,Clone,PartialEq,Serialize,Deserialize)]
pub struct Agent{
    neighbors: u8,
    pos_x:i32,
    pos_y:i32,
    state:AgentState,
}









#[tauri::command]
pub async fn add_agent(game_entities:State<'_, Arc<Mutex<GameEntities>>>,pos_x:i32,pos_y:i32)-> Result<(),ConwayError>{
    
    let test = game_entities.lock().unwrap().new_agent(pos_x,pos_y);
    Ok(())
}


#[tauri::command]
pub async fn update_game(game_entities:State<'_,Arc<Mutex<GameEntities>>>) -> Result<(),ConwayError>{
    let mut locked_game_entities = game_entities.lock().unwrap().update_game();
    return Ok(());
    
}



#[tauri::command]
pub async fn game_data_restart(game_entities:State<'_, Arc<Mutex<GameEntities>>>) -> Result<(),ConwayError>{
    game_entities.lock().unwrap().reset_data();
        
    Ok(())
}

#[tauri::command]
pub async fn get_game_data(game_entities:State<'_, Arc<Mutex<GameEntities>>>) -> Result<serde_json::Value,ConwayError>{
    if let Ok(o) = game_entities.lock().unwrap().return_data() {
        return Ok(o);
    }else {
        return Err(ConwayError::Error("no game data available".to_string()))
    }

}
