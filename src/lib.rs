use chrono::{DateTime, TimeZone, NaiveDateTime, Utc, Date};
use std::boxed::Box;
use std::fs::OpenOptions;

#[derive(Debug,PartialEq)]
pub enum Terminator{
	Terminate,
	No,
}

#[derive(Debug,PartialEq)]
pub enum entrys<'a>{
	Todo(Todo <'a>),
	Events(Events <'a>),
	appointments(Appointments <'a>),
}

#[derive(Debug,PartialEq)]
pub struct Todo<'a>{
	Title: Option<&'a str>,
	List: Option<&'a str>,
	DateTime: Box<Option<DateTime<Utc>>>,
}

#[derive(Debug,PartialEq)]
pub struct Events<'a>{
	Title: Option<&'a str>,
	DateTime: Box<Option<DateTime<Utc>>>,
	Description: Option<&'a str>,
	Attendees:Option<&'a str>,
}
	
#[derive(Debug,PartialEq)]
pub struct Appointments<'a>{
	Title: Option<&'a str>,
	DateTime: Box<Option<DateTime<Utc>>>,
	With_who: Option<&'a str>,
	Description: Option<&'a str>,
}
	

pub trait entry_type {
	fn save(&self) -> Result<Terminator,String> {
		let file_name = match self.get_date() {
			Some(a) => format!("{}",a),
			None => format!("None"),
		};
			
		let file = OpenOptions::new()
		           .append(true)
		           .create(true)
		           .open(file_name);
		
		Ok(Terminator::No)
	}
	
	fn get_date(&self) -> Option<Date<Utc>> ;	
	
}


impl <'a> entry_type for Todo<'a> {
	fn get_date(&self) -> Option<Date<Utc>> {
		match *(self.DateTime).clone() {
			None => return None,
			Some(a) => return Some(a.date())
		}
		
	}
}

impl <'a> entry_type for Events<'a> {
	fn get_date(&self) -> Option<Date<Utc>> {
		match*(self.DateTime).clone() {
			None => return None,
			Some(a) => return Some(a.date())
		}
	}   
}

impl <'a> entry_type for Appointments<'a> {
    fn get_date(&self) -> Option<Date<Utc>> {
	    match *(self.DateTime).clone(){
			None => return None,
			Some(a) => return Some(a.date())
		}	    
    }
}

impl<'a> Todo <'a> {
    pub fn new(Title: Option<&'a str>, List: Option<&'a str>, DateTime: Box<Option<DateTime<Utc>>>) -> Todo<'a>{
		Todo{
			Title,
			List,
			DateTime,
		}
	}
}

impl<'a> Events<'a> {
	pub fn new(Title: Option<&'a str>, DateTime: Box<Option<DateTime<Utc>>>, Description: Option<&'a str>, Attendees:Option<&'a str>) -> Events<'a> {
		Events{
			Title,
			DateTime,
			Description,
			Attendees,
		}
	}
}

impl<'a> Appointments<'a>{
	pub fn new(Title: Option<&'a str>, DateTime: Box<Option<DateTime<Utc>>>, With_who: Option<&'a str>, Description: Option<&'a str>) -> Appointments<'a> {
		Appointments{
			Title, 
	        DateTime,
	        With_who,
	        Description,
        }
     }
}







#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn Todo_Date_time() {
		let title = "timmy".to_string();
		let list ="buy tomato".to_string();
		let datetime = Box::new(None);
		
		let tester = Todo::new(Some(&title[..]), Some(&list[..]), datetime); 
		
		assert_eq!(tester.get_date(), None);
	
    }
    
        #[test]
    fn Events_Date_time() {
		let title = "timmy".to_string();
		let datetime = Box::new(None);
		let description = "celebrate tomato".to_string();
		let attendees = "Rosa, Lucy, Molly".to_string();
		
		
		let tester = Events::new(Some(&title[..]), datetime, Some(&description[..]), Some(&attendees[..])); 
		
		assert_eq!(tester.get_date(), None);
	
    }
    
        #[test]
    fn appointments_Date_time() {
		let title = "timmy".to_string();
		let datetime = Box::new(None);
		let with_who = "timmy".to_string();
		let description ="buy tomato".to_string();
		
		
		let tester = Appointments::new(Some(&title[..]), datetime, Some(&with_who[..]), Some(&description[..]));
		
		assert_eq!(tester.get_date(), None);
	
    }
}

