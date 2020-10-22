use chrono::{DateTime, TimeZone, NaiveDateTime, Utc, Date};
use std::boxed::Box;
use std::fs::{OpenOptions, File};
use std::time::SystemTime;
use std::io::prelude::*;



#[derive(Debug,PartialEq,Clone)] //enum which passes up instruction to terminate program
pub enum Terminator{
	Terminate,
	No,
}

#[derive(Debug,PartialEq,Clone)] // enum which can hold all entry types
pub enum entrys{
	Todo(Todo ),
	Events(Events),
	appointments(Appointments),
}

#[derive(Debug,PartialEq,Clone)]    //struct for storing and processing todo lists 
pub struct Todo{
	Title: Option<String>,
	DateTime: Box<Option<DateTime<Utc>>>,
	List: Option<String>,
}

#[derive(Debug,PartialEq,Clone)]    //struct for storing and processing events and parties
pub struct Events{
	Title: Option<String>,
	DateTime: Box<Option<DateTime<Utc>>>,
	Description: Option<String>,
	Attendees:Option<String>,
}
	
#[derive(Debug,PartialEq,Clone)]    //struct for storing and processing events and parties
pub struct Appointments{
	Title: Option<String>,
	DateTime: Box<Option<DateTime<Utc>>>,
	With_who: Option<String>,
	Description: Option<String>,
}


// trait for all kinds of stored reminders
pub trait entry_type {
	//save is the core part of the program which stores all reminders in files by day for later recovery on that day 
	fn save(&self) -> Result<Terminator,String> where Self: std::fmt::Debug{
		let file_name = match self.get_date() {
			Some(a) => format!("{}.txt",a),
			None => format!("None.txt"),
		};
		// if date field value is none then it will put in the none folder whic is always loadded good for daily reminders	
		let file = OpenOptions::new()
		           .append(true)
		           .create(true)
		           .open(file_name);
		           
		if self.get_date() == Some(DateTime::<Utc>::from(SystemTime::now()).date()) {
	    }
		           
		let success = file.unwrap().write_all((self.save_display()).as_bytes()); //fix this later
		
//		if 
		
		match success {
		   Ok(a) => return Ok(Terminator::No),
		   Err(a) => return Err("Did not save".to_string()),
		}
	}
	
	//gets date from objects datetime field
	fn get_date(&self) -> Option<Date<Utc>> ;
	
	//creates new instance of object
	
	fn new(Title: Option<String>, DateTime: Box<Option<DateTime<Utc>>>, Description: Option<String>, Attendees:Option<String>)	-> Self;
	
	fn save_display(&self) -> String;
	
	fn get_date_time(&self) -> Option<DateTime<Utc>>;
}



//these just implement methods that need to be implemented on a case by case basis
impl entry_type for Todo {
	fn get_date(&self) -> Option<Date<Utc>> {
		match *(self.DateTime).clone() {
			None => return None,
			Some(a) => return Some(a.date())
		}
		
	}
	fn get_date_time(&self) -> Option<DateTime<Utc>> {
		match *(self.DateTime).clone() {
			None => return None,
			Some(a) => return Some(a)
		}
		
	}
    fn new(Title: Option<String>, DateTime: Box<Option<DateTime<Utc>>>, List: Option<String>, Other: Option<String>) -> Todo{
		Todo{
			Title,
			DateTime,
			List,
		}
	}
	fn save_display(&self) -> String{
		format!(" /n Todo Title {:?} /n DateTime {:?} /n List {:?} /n #", 
		    match &self.Title{
				Some(a) => (*a.clone()).to_string(),
				None => "None".to_string(), 
			},
			match *(self.DateTime){
				Some(a) => a.to_string(),
				None => "None".to_string(),
			},
			match &self.List{
				Some(a) => (*a.clone()).to_string(),
				None => "None".to_string(),
			})
	}
	
}
//these just implement methods that need to be implemented on a case by case basis
impl entry_type for Events {
	fn get_date(&self) -> Option<Date<Utc>> {
		match*(self.DateTime).clone() {
			None => return None,
			Some(a) => return Some(a.date())
		}
	}
	fn get_date_time(&self) -> Option<DateTime<Utc>> {
		match *(self.DateTime).clone() {
			None => return None,
			Some(a) => return Some(a)
		}
		
	}
	fn new(Title: Option<String>, DateTime: Box<Option<DateTime<Utc>>>, Description: Option<String>, Attendees:Option<String>) -> Events{
		Events{
			Title,
			DateTime,
			Description,
			Attendees,
		}
	}
	fn save_display(&self) -> String{
		format!(" /n Events Title {:?} /n DateTime {:?} /n Description {:?} /n Attendees {:?} /n #", 
		    match &self.Title{
				Some(a) => (*a.clone()).to_string(),
				None => "None".to_string(), 
			},
			match *(self.DateTime){
				Some(a) => a.to_string(),
				None => "None".to_string(), 
			},
			match &self.Description{
				Some(a) => (*a.clone()).to_string(),
				None => "None".to_string(), 
			},
			match &self.Attendees{
				Some(a) => (*a.clone()).to_string(),
				None => "None".to_string(), 
			})
	}
	   
}
//these just implement methods that need to be implemented on a case by case basis
impl  entry_type for Appointments{
    fn get_date(&self) -> Option<Date<Utc>> {
	    match *(self.DateTime).clone(){
			None => return None,
			Some(a) => return Some(a.date())
		}	    
    }
	fn get_date_time(&self) -> Option<DateTime<Utc>> {
		match *(self.DateTime).clone() {
			None => return None,
			Some(a) => return Some(a)
		}
		
	}
	fn new(Title: Option<String>, DateTime: Box<Option<DateTime<Utc>>>, With_who: Option<String>, Description: Option<String>) -> Appointments {
		Appointments{
			Title, 
	        DateTime,
	        With_who,
	        Description,
        }
     }
     fn save_display(&self) -> String{
		format!(" /n Appointments Title {:?} /n DateTime {:?} /n With_who{:?} /n Description{:?} /n #", 
		    match &self.Title{
				Some(a) => (*a.clone()).to_string(),
				None => "None".to_string(), 
			}, 
			match *(self.DateTime){
				Some(a) => a.to_string(),
				None => "None".to_string(), 
			}, 
			match &self.With_who{
				Some(a) => (*a.clone()).to_string(),
				None => "None".to_string(), 
			},
			match &self.Description{
				Some(a) => (*a.clone()).to_string(),
				None => "None".to_string(), 
			})
	 }
}

//impl<'a> Todo <'a> {}

//impl<'a> Events<'a> {}

//impl<'a> Appointments<'a>{}







// all my tests - probably need more
#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    //
    #[test]
    fn todo_date_time() {
		let title = "timmy".to_string();
		let list ="buy tomato".to_string();
		let datetime = Box::new(None);
		let other = "sdnjabjlkvkjbaj".to_string();
		
		let tester = Todo::new(Some(title), datetime, Some(list), Some(other)); 
		
		assert_eq!(tester.get_date(), None);
	
    }
    
    #[test]
    fn events_date_time() {
		let title = "timmy".to_string();
		let datetime = Box::new(None);
		let description = "celebrate tomato".to_string();
		let attendees = "Rosa, Lucy, Molly".to_string();
		
		
		let tester = Events::new(Some(title), datetime, Some(description), Some(attendees)); 
		
		assert_eq!(tester.get_date(), None);
	
    }
    
    #[test]
    fn appointments_date_time() {
		let title = "timmy".to_string();
		let datetime = Box::new(None);
		let with_who = "timmy".to_string();
		let description ="buy tomato".to_string();
		
		
		let tester = Appointments::new(Some(title), datetime, Some(with_who), Some(description));
		println!("{:?}", tester);
		
		assert_eq!(tester.get_date(), None);
	
    }
    
    #[test]
    fn test_save(){
		let title = "title".to_string();
		let list = "buy tomato".to_string();
		let datetime = Box::new(None);
		let other = "sdnjabjlkvkjbaj".to_string();
		
		let tester = Todo::new(
		                 Some(title),
		                 datetime,
		                 Some(list),
		                 Some(other),
		                
		             );
		    
	   assert_eq!(tester.save(), Ok(Terminator::No));
	   
	}
	#[test]
	fn testing_save_display(){
	    let title = "timmy".to_string();
		let datetime = Box::new(None);
		let description = "celebrate tomato".to_string();
		let attendees = "Rosa, Lucy, Molly".to_string();
	}
		
		
		
		
		      
}

