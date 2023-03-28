# Proposal (written with ChatGPT)
`rust-monster` is a collection of utilities for developers of TTRPG-related applications. This package provides a range of tools for managing game mechanics, character creation, combat, inventory management, and more. 

## Features and Sub-Packages

- `rust-monster/character`: A character and stats management package that provides tools for creating and managing character profiles, including race and class selection, ability score generation, and character progression.
- `rust-monster/combat`: A combat and spells tracker that provides tools for managing turn-based combat scenarios and tracking spellcasting.
- `rust-monster/creatures`: A monster manual that let's you add, edit and retrieve monster and enemy NPCs.
- `rust-monster/dicer`: A dice-related package that provides tools for rolling dice, calculating modifiers, and generating random numbers.
- `rust-monster/encounter`: An encounter and enemy package that provides tools for generating and managing encounters with enemies and creatures.
- `rust-monster/inventory`: An item and inventory manager that provides tools for creating, storing, and retrieving items and inventories.
- `rust-monster/mission`: A quest and objective management package that provides tools for creating and tracking quests, objectives, and rewards.
- `rust-monster/music`: A music composer and sound effects package that provides tools for playing background music and sound effects during gameplay.
- `rust-monster/randoms`: A random generator package that provides tools for generating random names, locations, items, and more.
- `rust-monster/ruleset-manager`: A package for managing custom rulesets and homebrew content, including support for 3rd party TTRPG systems.
- `rust-monster/session`: An RPG session keeper that provides tools for managing notes, NPCs, and encounters across multiple sessions.
- `rust-monster/spells`: An spell manager that provides tools for creating, editing, and retrieving spells data.
- `rust-monster/story`: A story and plot generator package that provides tools for generating storylines and plot twists.
- `rust-monster/terrain`: A map and terrain generator package that provides tools for generating maps and terrain features.
- `rust-monster/utilities`: Unit converters and more.
- `rust-monster/world-builder`: A package for creating and managing detailed campaign settings, including geography, culture, and political systems.

## Installation

To install `rust-monster`, simply add the following line to your `Cargo.toml` file:

```toml
[dependencies]
rust-monster = "0.1.0"
```

## Usage

To use rust-monster in your project, simply import the desired sub-package and call the appropriate functions. For example:


```rust
use rust_monster::combat::roll_dice;

fn main() {
    let result = roll_dice(20);
    println!("You rolled a {}", result);
}
```

## Contributing

rust-monster is licensed under the GPL, and contributions are welcome! To contribute, simply fork the repository and submit a pull request. Please ensure that all contributions are well-documented.

`rust-monster` was created by [Your Name Here]. Special thanks to [contributor names here] for their contributions to the project.