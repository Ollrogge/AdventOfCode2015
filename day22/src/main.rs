use std::collections::VecDeque;
use std::fmt;
use std::cmp;

#[derive(Clone)]
struct Entity {
    hit_points: i32,
    damage: i32,
    mana: i32,
    armor: i32,
    active_spells: Vec<Spell>
}

impl Entity {
    fn new(hit_points: i32, damage: i32, mana: i32, armor: i32) -> Entity {
        Entity {
            hit_points,
            damage,
            mana,
            armor,
            active_spells: Vec::new()
        }
    }

    pub fn new_player() -> Entity {
        Entity::new(50, 0, 500, 0)
    }
    
    pub fn new_opponent() -> Entity {
        Entity::new(55, 8, 0, 0)
    }

    fn cast_spell(&mut self, spell: Spell, opponent: &mut Entity) {
        self.mana -= spell.cost;
        self.hit_points += spell.heal;
        opponent.hit_points -= spell.damage;
        if let Some(effect) = &spell.effect {
            /*
             You cannot cast a spell that would start an effect which is already 
             active. However, effects can be started on the same turn they end.
            */
            for _s in &self.active_spells {
                if spell.name == _s.name && _s.effect.as_ref().unwrap().turns_left > 0 {
                    return;
                }
            }

            self.active_spells.push(spell);
        }
    }

    fn exec_effects(&mut self, opponent: &mut Entity) {
        if self.active_spells.len() == 0x0 {
            return;
        }

        let mut remove = Vec::new();
        let mut spells = self.active_spells.clone();
        for (i, s) in spells.iter_mut().enumerate() {
            let mut e = s.effect.as_mut().unwrap();
            (e.action)(e.turns_left, self, opponent);
            e.turns_left -= 1;

            if e.turns_left == -1 {
                remove.push(i);
            }
        }

        for idx in remove.into_iter() {
            spells.remove(idx);
        }
        
        self.active_spells = spells;
    }

    fn fight(&mut self, spell: Spell, opponent: &mut Entity) -> i32 {
        // player turn
        self.hit_points -= 1;
        if self.hit_points <= 0 {
            return -1;
        }

        self.exec_effects(opponent);
        if self.mana < spell.cost {
            return -1;
        }

        self.cast_spell(spell, opponent);
        if opponent.hit_points <= 0 {
            return spell.cost;
        }

        // boss turn
        self.exec_effects(opponent);
        if opponent.hit_points <= 0 {
            return spell.cost;
        }

        self.hit_points -= cmp::max(opponent.damage - self.armor, 1);
        if self.hit_points <= 0 {
            return -1;
        }

        return spell.cost;
    }
}

type SpellAction = fn(left: i32, player: &mut Entity, opponent: &mut Entity);

#[derive(Clone, Copy)]
struct Effect {
    turns_left: i32,
    action: SpellAction
}

impl Effect {
    pub fn new(turns_left: i32, action: SpellAction) -> Effect {
        Effect {
            turns_left,
            action
        }
    }
}

#[derive(Clone, Copy)]
struct Spell {
    name: &'static str,
    cost: i32,
    damage: i32,
    heal: i32,
    effect: Option<Effect>
}

impl fmt::Debug for Spell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Spell {
    pub fn new(name: &'static str,
               cost: i32, 
               damage: i32, 
               heal: i32, 
               effect: Option<Effect>) -> Spell {
        Spell {
            name,
            cost,
            damage,
            heal,
            effect
        }
    }

    pub fn magic_missle() -> Spell {
        Spell::new("Magic Missile", 53, 4, 0, None)
    }

    pub fn drain() -> Spell {
        Spell::new("Drain", 73, 2, 2, None)
    }

    pub fn shield() -> Spell {
        Spell::new("Shield", 113, 0, 0, 
            Some(Effect::new(6, |left: i32, ent: &mut Entity, _opp: &mut Entity| {
                if left == 6 {
                    ent.armor += 7;
                }
                else if left == 0 {
                    ent.armor -= 7;
                }
            }))
        )
    }

    pub fn poison() -> Spell {
        Spell::new("Poison", 173, 0, 0, 
            Some(Effect::new(6, |left: i32, _ent: &mut Entity, opp: &mut Entity| {
                if left > 0 {
                    opp.hit_points -= 3;
                }
            }))
        )
    }

    pub fn recharge() -> Spell {
        Spell::new("Recharge", 229, 0, 0,
            Some(Effect::new(5, |left: i32, ent: &mut Entity, _opp: &mut Entity| {
                if left > 0 {
                    ent.mana += 101;
                }
            }))
        )
    }
}

#[derive(Clone)]
struct FightState {
    player: Entity,
    opponent: Entity,
    spell: Spell,
    cost: i32
}

impl FightState {
    pub fn new(player: Entity, opponent: Entity, spell: Spell) -> FightState {
        FightState {
            player,
            opponent,
            spell,
            cost: 0
        }
    }
}

fn main() {
    let mut min_mana : i32 = std::i32::MAX;
    let mut fights = VecDeque::new(); 

    fights.push_back(FightState::new(Entity::new_player(), 
                                     Entity::new_opponent(),
                                     Spell::magic_missle()));

    fights.push_back(FightState::new(Entity::new_player(), 
                                     Entity::new_opponent(),
                                     Spell::drain()));

    fights.push_back(FightState::new(Entity::new_player(), 
                                     Entity::new_opponent(),
                                     Spell::shield()));

    fights.push_back(FightState::new(Entity::new_player(), 
                                     Entity::new_opponent(),
                                     Spell::poison()));

    fights.push_back(FightState::new(Entity::new_player(), 
                                     Entity::new_opponent(),
                                     Spell::recharge()));
    

    while let Some(mut fight) = fights.pop_front() {
        let mut player = &mut fight.player;
        let mut opponent = &mut fight.opponent;
        let mut spell = &fight.spell;

        let res = player.fight(*spell, &mut opponent); 

        if res < 0 {
            continue;
        }
        else {
            fight.cost += res;
            if opponent.hit_points <= 0 && fight.cost < min_mana {
                min_mana = fight.cost;
                continue;
            }

            if fight.cost > min_mana {
                continue;
            }

            let mana = player.mana;

            if mana >= Spell::magic_missle().cost {
                fight.spell = Spell::magic_missle();
                fights.push_back(fight.clone());
            }
            if mana >= Spell::drain().cost {
                fight.spell = Spell::drain();
                fights.push_back(fight.clone());
            }
            if mana >= Spell::shield().cost {
                fight.spell = Spell::shield();
                fights.push_back(fight.clone());
            }
            if mana >= Spell::poison().cost {
                fight.spell = Spell::poison();
                fights.push_back(fight.clone());
            }
            if mana >= Spell::recharge().cost {
                fight.spell = Spell::recharge();
                fights.push_back(fight.clone());
            }
        }

    }

    println!("Min mana: {}", min_mana);
}
