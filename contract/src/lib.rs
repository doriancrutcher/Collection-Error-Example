
// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc};
use near_sdk::collections::{LookupMap,UnorderedSet};

setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VotingContract {
    candidate_url: LookupMap<String, String>,
    candidate_pair: LookupMap<String, Vec<String>>,
    prompt_array: UnorderedSet<String>,
    vote_array: LookupMap<String, Vec<u32>>,
    user_participation: LookupMap<String, Vec<String>>,
    test_collection:LookupMap<String,Vec<i32>>
}

impl Default for VotingContract {
  fn default() -> Self {
    Self {
      candidate_url: LookupMap::new(b"a".to_vec()),
      candidate_pair: LookupMap::new(b"a".to_vec()),
      prompt_array: UnorderedSet::new(b"a".to_vec()),
      vote_array: LookupMap::new(b"a".to_vec()),
      user_participation: LookupMap::new(b"a".to_vec()),
      test_collection:LookupMap::new(b"a".to_vec()),
    }
  }
}

#[near_bindgen]
impl VotingContract {

    // View Methods
    // View Method are denoted by a reference to self `&self` and returning some value
    pub fn get_url(&self, name:String)->String{
        match self.candidate_url.get(&name){
            Some(url)=>url.to_string(),
            None=>"No url found".to_string()
        }
    }

    pub fn just(&self)->Vec<i32>{
        self.test_collection.get(&String::from("egg")).unwrap_or(vec![0,100])
    }

    pub fn egg(&mut self){
        self.vote_array.insert(&String::from("egg"),&vec![1,1]);
    }

    pub fn did_participate(&self, prompt:String, user:String)->bool{
       match self.user_participation.get(&prompt){
           Some(user_vector)=>user_vector.contains(&user),
           None=>false
       }
    }

    pub fn get_all_prompts(&self)->Vec<String>{ 
        self.prompt_array.to_vec()
    }

    pub fn get_votes(&self, prompt:String)->Vec<u32>{
        self.vote_array.get(&prompt).unwrap_or(vec![0,0])
    }
    
    pub fn get_candidate_pair(&self, prompt:String)->Vec<String>{
        match self.candidate_pair.get(&prompt){
            Some(candidates)=>candidates.to_vec(),
            None=>vec![]
        }
    }

    // Change Methods
    // Change Methods are denoted by a mutable reference to self `&mut self` and Do not return a value
    // These Methods Change the State of the Blockchain by adding or changing existing information

    pub fn add_url(&mut self, name:String, url:String){
        self.candidate_url.insert(&name,&url);
    }

    pub fn add_candidate_pair(&mut self, prompt:String, name_1:String, name_2:String){
       
        self.vote_array.insert(&prompt,&vec![1,1]);
        self.candidate_pair.insert(&prompt,&vec![name_1,name_2]);

    }

    pub fn new_vote(&mut self,prompt:String){
        self.vote_array.insert(&prompt,&vec![0,0]);
    }

    pub fn add_to_prompt_array(&mut self, prompt:String){
        self.prompt_array.insert(&prompt);
    }

    pub fn clear_prompt_array(&mut self){
        self.prompt_array.clear();
    }

    pub fn add_vote(&mut self,prompt:String,index:usize){
        let current_votes:Vec<u32>=match self.vote_array.get(&prompt){
            Some(mut vote_pair)=>{
                let vote_num:u32=vote_pair[index];
                std::mem::replace(&mut vote_pair[index], &vote_num+1);
                vote_pair
            }
            None=>{
                let mut vote_vec=vec![0,0];
                std::mem::replace(&mut vote_vec[index],1);
                vote_vec
            }
        };

        self.vote_array.insert(&prompt,&current_votes);
    }



    pub fn record_user(&mut self,prompt:String,user:String){
       let current_array:Vec<String>= match self.user_participation.get(&prompt){
            Some(mut prompt_user_array)=>{&prompt_user_array.push(user);
                prompt_user_array
            },
            None=>vec![user]
        };
        self.user_participation.insert(&prompt,&current_array);
    }

}

