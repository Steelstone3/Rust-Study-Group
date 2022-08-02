struct Ship {
    name: String,
    hull: u32,
    shields: u32,
}

struct Galaxy {
    name: String,
    star_systems: Vec<String>,
    planets: Vec<String>,
}

struct StateMachine<S> {
    state: S,
    ship: Ship,
    galaxy: Galaxy,
}

struct NewGame;

impl StateMachine<NewGame> {
    fn new() -> Self {
        print!("new game started\n");
        StateMachine {
            state: NewGame,
            ship: Ship {
                name: "Bob".to_string(),
                hull: 32,
                shields: 32,
            },
            galaxy: Galaxy {
                name: "Jerry".to_string(),
                star_systems: vec![],
                planets: vec![],
            },
        }
    }
}

struct Exploration;

impl From<StateMachine<NewGame>> for StateMachine<Exploration> {
    fn from(state: StateMachine<NewGame>) -> StateMachine<Exploration> {
        print!("exploration started\n");
        StateMachine {
            state: Exploration,
            ship: state.ship,
            galaxy: Galaxy {
                name: "Jerry".to_string(),
                star_systems: vec!["Random Star System 1".to_string()],
                planets: vec!["Random Planet 1".to_string()],
            },
        }
    }
}

struct Combat;

impl From<StateMachine<Exploration>> for StateMachine<Combat> {
    fn from(state: StateMachine<Exploration>) -> StateMachine<Combat> {
        print!("combat started\n");
        
        StateMachine {
            state: Combat,
            ship: state.ship,
            galaxy: state.galaxy,
        }
    }
}

struct GameOver;

impl From<StateMachine<Combat>> for StateMachine<GameOver> {
    fn from(state: StateMachine<Combat>) -> StateMachine<GameOver> {
        print!("game over\n");
            
        StateMachine {
            state: GameOver,
            ship: Ship {
                name: "Bob".to_string(),
                hull: 0,
                shields: 0,
            },
            galaxy: state.galaxy
        }
    }
}

fn main() {
    let new_game = StateMachine::new();

    let exploration = StateMachine::<Exploration>::from(new_game);

    let combat = StateMachine::<Combat>::from(exploration);

    let game_over = StateMachine::<GameOver>::from(combat);
}
