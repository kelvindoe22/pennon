use std::{collections::HashMap};
use std::env::Args;



pub struct ArgParse {
    map: HashMap<String, Option<String>>,
    stray: Vec<String>
}

impl ArgParse {
    /// This function skips the first argument and filters through the args.
    /// It finds flags and their values and other args("i call strays").
    /// 
    /// If you run cargo run -- arg0 -arg1 1 -arg2 2 -arg3 
    /// 
    /// arg0 will be a stray. arg1, arg2 and arg3 will be flags(ARE FLAGS).
    /// 
    pub fn new(args: Args) -> Self {
        let mut map = HashMap::new();
        let mut stray = Vec::new();
        let args = args.skip(1);
        let mut control = None;
        for i in args {
            if i.starts_with('-') {
               if control.is_some() {
                   map.insert(control.take().unwrap(),None);
               }
               control = Some(i.strip_prefix('-').unwrap().to_string())
            }else {
                if control.is_some(){
                    map.insert(control.take().unwrap(), Some(i));
                }else {
                    stray.push(i);
                }
            }
        }

        Self {
            map,
            stray
        }
    }

    /// This function behaves like new but it doesn't skip the first argument.(i.e the location of the binary)
    /// 
    /// If you run cargo run -- arg0 -arg1 1 -arg2 2 -arg3  
    /// The location of the binary and arg0 will be a stray. 
    /// 
    /// arg1, arg2 and arg3 will be flags(ARE FLAGS).
    /// 
    pub fn new_raw(args: Args) -> Self {
        let mut map = HashMap::new();
        let mut stray = Vec::new();
        let mut control = None;
        for i in args {
            if i.starts_with('-') {
               if control.is_some() {
                   map.insert(control.take().unwrap(),None);
               }
               control = Some(i.strip_prefix('-').unwrap().to_string())
            }else {
                if control.is_some(){
                    map.insert(control.take().unwrap(), Some(i));
                }else {
                    stray.push(i);
                }
            }
        }

        Self {
            map,
            stray
        }
    }

    /// This function lets you set the number of flags and the number of strays.  
    /// And also the number of args you want to skip.
    /// 
    pub fn new_customized(args: Args, flags: usize, strays: usize, skips: usize) -> Self {
        let mut map = HashMap::with_capacity(flags);
        let mut stray = Vec::with_capacity(strays);

        let args = args.skip(skips);

        let mut control = None;
        for i in args {
            if i.starts_with('-') {
               if control.is_some() {
                   map.insert(control.take().unwrap(),None);
               }
               control = Some(i.strip_prefix('-').unwrap().to_string())
            }else {
                if control.is_some(){
                    map.insert(control.take().unwrap(), Some(i));
                }else {
                    stray.push(i);
                }
            }
        }

        Self {
            map,
            stray
        }
    }

    /// This function checks if a flag is present.
    /// 
    pub fn is_present(&self, flag: &str) -> bool{
        self.map.contains_key(&String::from(flag))
    }

    /// This function return the value wrapped in an option of the flag if it exists. 
    /// Returns None if the flag is not present or has no value
    pub fn get_flag_item(&mut self, flag: &str) -> Option<String>{
        if !self.is_present(flag){
            return None;
        }
        self.map.remove(&String::from(flag)).unwrap()
    }

    /// Returns a Vector containing all the stray arguments
    pub fn get_strays(&mut self) -> Vec<String> {
        std::mem::take(&mut self.stray)
    }
    
}

