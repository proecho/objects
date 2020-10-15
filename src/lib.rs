use chrono::{DateTime, TimeZone, NaiveDateTime, Utc, Date};
use std::boxed::Box;
use std::fs::{OpenOptions, File};
use std::time::SystemTime;
use std::io::prelude::*;



#[derive(Debug,PartialEq,Clone)]
pub enum Terminator{
	Terminate,
	No,
}

#[derive(Debug,PartialEq,Clone)]
pub enum entrys{
	Todo(Todo ),
	Events(Events),
	appointments(Appointments),
}

#[derive(Debug,PartialEq,Clone)]
pub struct Todo{
	Title: Option<String>,
	DateTime: Box<Option<DateTime<Utc>>>,
	List: Option<String>,
}

#[derive(Debug,PartialEq,Clone)]
pub struct Events{
	Title: Option<String>,
	DateTime: Box<Option<DateTime<Utc>>>,
	Description: Option<String>,
	Attendees:Option<String>,
}
	
#[derive(Debug,PartialEq,Clone)]
pub struct Appointments{
	Title: Option<String>,
	DateTime: Box<Option<DateTime<Utc>>>,
	With_who: Option<String>,
	Description: Option<String>,
}



pub trait entry_type {
	fn save(&self) -> Result<Terminator,String> where Self: std::fmt::Debug{
		let file_name = match self.get_date() {
			Some(a) => format!("{}",a),
			None => format!("None"),
		};
			
		let file = OpenOptions::new()
		           .append(true)
		           .create(true)
		           .open(file_name);
		           
		if self.get_date() == Some(DateTime::<Utc>::from(SystemTime::now()).date()) {
	    }
		           
		let success = file.unwrap().write_all((format!("{:?}", self)).as_bytes()); //fix this later
		
//		if 
		
		match success {
		   Ok(a) => return Ok(Terminator::No),
		   Err(a) => return Err("Did not save".to_string()),
		}
	}
	
	fn get_date(&self) -> Option<Date<Utc>> ;
	
	fn new(Title: Option<String>, DateTime: Box<Option<DateTime<Utc>>>, Description: Option<String>, Attendees:Option<String>)	-> Self;
	
}




impl entry_type for Todo {
	fn get_date(&self) -> Option<Date<Utc>> {
		match *(self.DateTime).clone() {
			None => return None,
			Some(a) => return Some(a.date())
		}
		
	}
    fn new(Title: Option<String>, DateTime: Box<Option<DateTime<Utc>>>, List: Option<String>, Other: Option<String>) -> Todo{
		Todo{
			Title,
			DateTime,
			List,
		}
	}
	
}

impl entry_type for Events {
	fn get_date(&self) -> Option<Date<Utc>> {
		match*(self.DateTime).clone() {
			None => return None,
			Some(a) => return Some(a.date())
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
	   
}

impl  entry_type for Appointments{
    fn get_date(&self) -> Option<Date<Utc>> {
	    match *(self.DateTime).clone(){
			None => return None,
			Some(a) => return Some(a.date())
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
}

//impl<'a> Todo <'a> {}

//impl<'a> Events<'a> {}

//impl<'a> Appointments<'a>{}








#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn todo_date_time() {
		let title = "timmy".to_string();
		let list ="buy tomato".to_string();
		let datetime = Box::new(None);
		
		let tester = Todo::new(Some(&title[..]), Some(&list[..]), datetime); 
		
		assert_eq!(tester.get_date(), None);
	
    }
    
    #[test]
    fn events_date_time() {
		let title = "timmy".to_string();
		let datetime = Box::new(None);
		let description = "celebrate tomato".to_string();
		let attendees = "Rosa, Lucy, Molly".to_string();
		
		
		let tester = Events::new(Some(&title[..]), datetime, Some(&description[..]), Some(&attendees[..])); 
		
		assert_eq!(tester.get_date(), None);
	
    }
    
    #[test]
    fn appointments_date_time() {
		let title = "timmy".to_string();
		let datetime = Box::new(None);
		let with_who = "timmy".to_string();
		let description ="buy tomato".to_string();
		
		
		let tester = Appointments::new(Some(&title[..]), datetime, Some(&with_who[..]), Some(&description[..]));
		
		assert_eq!(tester.get_date(), None);
	
    }
    
    #[test]
    fn test_save(){
		let title = "title".to_string();
		let list = "buy tomato".to_string();
		let datetime = Box::new(None);
		
		let tester = Todo::new(
		                 Some(&title[..]),
		                 Some(&list[..]),
		                 datetime,
		             );
		    
	   assert_eq!(tester.save(), Ok(Terminator::No));
	   
	}
		
		
		      
}

