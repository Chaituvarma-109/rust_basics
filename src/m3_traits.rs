trait Attacker {
    fn choose_style(&self) -> String;
}

#[derive(Debug)]
#[allow(dead_code)]
enum Character {
    Human,
    Elf,
    Orc,
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Human => "Sword".to_string(),
            Character::Elf => "Bow".to_string(),
            Character::Orc => "Spear".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_traits() {
        let my_character: Character = Character::Human;
        let chosen_fightining_style: String = my_character.choose_style();
        dbg!(chosen_fightining_style);
    }
}
