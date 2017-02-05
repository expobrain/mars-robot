mod setup;
mod engine;
mod terrain;
mod robot;

use engine::Engine;
use setup::Setup;


const INPUT_FILE: &'static str = "input.txt";


fn main() {
    let setup = Setup::read_from_file(INPUT_FILE);

    let mut engine = Engine::with_size(&setup.size);

    for instruction in setup.instructions {
        engine.run_instructions(&instruction);
    }
}
