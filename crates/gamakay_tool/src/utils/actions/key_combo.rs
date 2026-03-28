use crate::utils::actions::{KeyAction, KeyCode};

// Struct for Key Combinations (e.g., Ctrl + Alt + Delete)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyCombo<const N: usize>
where
    // KeyCombo with one key does not exist.
    [(); N + 1]:,
{
    pub keys: [KeyCode; N],
}

pub trait Combinable {
    type Next;
    fn with(self, next: KeyCode) -> Self::Next;
}

// Base KeyCode to KeyCombo<2>
impl Combinable for KeyCode {
    type Next = KeyCombo<2>;
    fn with(self, next: KeyCode) -> Self::Next {
        KeyCombo { keys: [self, next] }
    }
}

impl Combinable for KeyCombo<2> {
    type Next = KeyCombo<3>;
    fn with(self, next: KeyCode) -> Self::Next {
        KeyCombo {
            keys: [self.keys[0], self.keys[1], next],
        }
    }
}

impl Combinable for KeyCombo<3> {
    type Next = KeyCombo<4>;
    fn with(self, next: KeyCode) -> Self::Next {
        KeyCombo {
            keys: [self.keys[0], self.keys[1], self.keys[2], next],
        }
    }
}

impl<const N: usize> KeyAction for KeyCombo<N>
where
    [(); N + 1]:,
{
    fn to_bytes(&self) -> [u8; 4] {
        let mut bytes = [0u8; 4];

        // Because we know that N is at most 4 (due to our Combinable constraint),
        // we fill the array from the right (index 3).
        // i = 0 -> bytes[3], i = 1 -> bytes[2], etc.
        for i in 0..N {
            // We pakken de toetsen van achter naar voren uit de combo
            let key = self.keys[N - 1 - i];
            bytes[3 - i] = key as u8;
        }
        bytes
    }

    fn legend(&self) -> String {
        // We map each KeyCode in the array to its own legend() String
        // and group them together with " + ".
        self.keys
            .iter()
            .map(|k| k.legend())
            .collect::<Vec<_>>()
            .join(" + ")
    }
}
