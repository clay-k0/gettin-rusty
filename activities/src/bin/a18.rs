// Topic: Result
//

//* Implement Debug print functionality using `derive`
#[derive(Debug)]
//* Create an structure named `Adult` that represents a person aged 21 or older:
struct Adult {
  //* The structure must contain the person's name and age
  name: String,
  age: u8
}

impl Adult {
  //* Implement a `new` function for the `Adult` structure that returns a Result:
  fn new(name: String, age: u8) -> Result<Self, String> {
    if age >= 21 {
      //* The Ok variant should contain the initialized structure, but only
      //  if the person is aged 21 or older
      Ok(Self { name: String::from(name), age })
    } else {
      //* The Err variant should contain a String (or &str) that explains why
      //  the structure could not be created
      Err(String::from("Age must be 21"))
    }
  }
}

fn main() {
  //* Instantiate two `Adult` structures:
  //  * One should be aged under 21
  //  * One should be 21 or over
  let adult = Adult::new(String::from("Daniel"), 78);
  let child = Adult::new(String::from("Sereia"), 5);

  //* Use `match` to print out a message for each `Adult`:
  match adult {
    //* For the Ok variant, print any message you want
    Ok(adult) => println!("{} is {} years old.", adult.name, adult.age),
    //* For the Err variant, print out the error message
    Err(e) => println!("Error: {:?}", e)
  }

  match child {
    Ok(child) => println!("{} is {} years old.", child.name, child.age),
    Err(e) => println!("Error: {:?}", e)
  }

}
