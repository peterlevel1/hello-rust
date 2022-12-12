#[derive(Debug)]
enum MenuChoice {
  MainMenu,
  Start,
  Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
  match input {
    "mainmenu" => Ok(MenuChoice::MainMenu),
    "start" => Ok(MenuChoice::Start),
    "quit" => Ok(MenuChoice::Quit),
    _ => Err("menu choice not found".to_owned()),
  }
}

fn main() {
  let choice = get_choice("start");
  println!("choice = {:?}", &choice);

  let choice2 = get_choice("start1");
  println!("choice2 = {:?}", &choice2);

  match choice2 {
    Ok(sth) => print!("match choice2: Ok {:?}", &sth),
    Err(e) => print!("match chice2 Err: {:?}", &e),
  }
}