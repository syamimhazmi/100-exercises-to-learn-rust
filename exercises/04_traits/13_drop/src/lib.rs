// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

pub struct DropBomb {
    is_defused: bool,
}

impl DropBomb {
    fn new() -> Self {
        DropBomb { is_defused: false }
    }

    pub fn defuse(&mut self) {
        self.is_defused = true;
    }

    pub fn is_defused(self) -> bool {
        self.is_defused
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::panic;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped

        if bomb.is_defused() == false {
            panic!("Bomb is dropped")
        }
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
