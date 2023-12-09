use std::env;
use std::path::Path;
use std::fs::{canonicalize, File};
use std::io::{BufReader, BufRead};

fn reader_from_path(relative_path: &str) -> BufReader<File> {
    let absolute_path = {
        match canonicalize(Path::new(relative_path)){
            Ok(file) => file,
            Err(_) => {
                eprintln!("Invalid file path: {}", relative_path);
                std::process::exit(1);
            },
        }
    };

    let file = {
        match File::open(absolute_path){
            Ok(file) => file,
            Err(_) => {
                eprintln!("File failed to open");
                std::process::exit(1);
            }
        }
    };
    BufReader::<File>::new(file)
}

#[derive(PartialEq)]
enum CardType {
    Ace,
    King,
    Queen,
    Joker,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}
// Can I declare these values staticly in enumeration declaration
impl CardType {
    fn get_card_strength(&self) -> u8 {
        match self{
            CardType::Ace => 12,
            CardType::King => 11,
            CardType::Queen => 10,
            CardType::Ten => 9,
            CardType::Nine => 8,
            CardType::Eight => 7,
            CardType::Seven => 6,
            CardType::Six => 5,
            CardType::Five => 4,
            CardType::Four => 3,
            CardType::Three => 2,
            CardType::Two => 1,
            CardType::Joker => 0,
        }
    }
}

#[derive(PartialEq)]
enum HandType {
    FiveOaK,
    FourOaK,
    FullHouse,
    ThreeOaK,
    TwoPair,
    Pair,
    HighCard,
}

impl HandType {
    fn get_hand_score(&self) -> u8 {
        match self {
            HandType::FiveOaK => 6,
            HandType::FourOaK => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOaK => 3,
            HandType::TwoPair => 2,
            HandType::Pair => 1,
            HandType::HighCard => 0,
        }
    }
}

fn get_cardtype(c: char) -> CardType {
    match c {    
        'A' => CardType::Ace,
        'K' => CardType::King,
        'Q' => CardType::Queen,
        'J' => CardType::Joker,
        'T' => CardType::Ten,
        '9' => CardType::Nine,
        '8' => CardType::Eight,
        '7' => CardType::Seven,
        '6' => CardType::Six,
        '5' => CardType::Five,
        '4' => CardType::Four,
        '3' => CardType::Three,
        '2' => CardType::Two,
        _ => {
            eprintln!("Invalid card type");
            std::process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} filepath", args[0]);
        std::process::exit(1);
    }
    
    let reader = reader_from_path(&args[1]);
    let mut hands: Vec<(Vec<CardType>, HandType , u16)> = Vec::new();

    let lines = reader.lines().filter_map(Result::ok);
    for line in lines {
        let line: Vec<&str> = line.split_whitespace().collect();
        
        let mut match_count: Vec<(CardType, u8)> = Vec::new();
        let mut hand: Vec<CardType> = Vec::new();
        let mut joker_count: Option<u8> = None;

        for c in line[0].chars() {

            let current_card = get_cardtype(c);
            if hand.len() == 0 &&  &current_card == &CardType::Joker{
                joker_count = Some(1);
            }
            for card in &hand {
                if &current_card == &CardType::Joker {
                    joker_count = match joker_count {
                        Some(x) => Some(x+1),
                        None => Some(1),
                    };
                    break;
                }

                if card == &current_card{
                    if match_count.len() == 0 {
                        match_count.push( (get_cardtype(c), 1) ); // I will never "copy" a value
                    } else if card == &match_count[0].0 {
                        match_count[0].1 += 1;
                    } else if match_count.len() == 1 {
                        match_count.push( (get_cardtype(c), 1) );
                    } else {
                        match_count[1].1 += 1;
                    }
                    break;
                }
            }
            hand.push(current_card);
        }

        if let Some(jokers) = joker_count {
            
            match match_count.len(){
                0 => {
                    if jokers == 5{ // *Weird* edge case
                        match_count.push(( CardType::Joker, 4 ));
                    } else {
                        match_count.push(( CardType::Joker, jokers ));
                    }
                },
                
                1 => { match_count[0].1 += jokers; },
                
                2 => {
                    if match_count[1].1 > match_count[0].1 {
                        match_count[1].1 += jokers; 
                    } else {
                        match_count[0].1 += jokers;    
                    }
                }
                _ => {
                    eprintln!("Invalid number of match groups");
                    std::process::exit(1);
                }
            }
        }

        let current_hand_type = match match_count.len() {
            0 => HandType::HighCard,
            2 => {
                if match_count[0].1 == 2 || match_count[1].1 == 2 {
                    HandType::FullHouse
                } else {
                    HandType::TwoPair
                }
            },
            1 => match match_count[0].1 {
                4 => HandType::FiveOaK,
                3 => HandType::FourOaK,
                2 => HandType::ThreeOaK,
                1 => HandType::Pair,
                _ => {
                    eprintln!("Invalid match count");
                    std::process::exit(1);
                }, 
            },
            _ => {
                eprintln!("Invalid match count");
                std::process::exit(1);
            },
        };

        // Binary sort assumes there is never a true equivelent case
        if let Ok(bid) = line[1].parse::<u16>(){
            let mut low = 0;
            let mut high = hands.len();
            while low < high {
                let mid = (low + high) / 2;
                let vec_hand_score = hands[mid].1.get_hand_score();
                let current_hand_score = current_hand_type.get_hand_score();
                let vec_hand = &hands[mid].0;

                if vec_hand_score < current_hand_score {
                    low = mid + 1;
                } else if vec_hand_score > current_hand_score {
                    high = mid;
                } else {
                    for (index, card) in hand.iter().enumerate() {
                        let vec_card_score = vec_hand[index].get_card_strength();
                        let current_card_score = card.get_card_strength();

                        if vec_card_score < current_card_score {
                            low = mid + 1;
                            break;
                        } else if current_card_score < vec_card_score {
                            high = mid;
                            break;
                        }
                    }
                }
            }
            hands.insert(low, (hand, current_hand_type, bid));
        }
    }
    let mut sum: u64 = 0;
    for (index, hand) in hands.iter().enumerate() {
        sum += hand.2 as u64 * (index + 1) as u64;
    }
    println!("Sum of products of bids: {}", sum);
}

