// Mock exercise data
import type { Exercise } from "./types"

export const exercises: Exercise[] = [
  {
    id: "intro-1",
    title: "Hello Ink!",
    description: "Your first smart contract - learn the basics of ink! syntax",
    difficulty: "Beginner",
    category: "Intro",
    instructions: `Create your first Rust smart contract using ink!

Complete the TODO by implementing a simple getter function that returns "Hello, Ink!".

Key concepts:
- Smart contract structure
- Public functions
- Basic return types`,
    starterCode: `#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod hello_ink {
    #[ink(storage)]
    pub struct HelloInk;

    impl HelloInk {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self
        }

        // TODO: Create a public function that returns "Hello, Ink!"
        #[ink(message)]
        pub fn get_greeting(&self) -> String {
            // Implement this
            todo!()
        }
    }
}`,
    hints: [
      "Use String::from() to create a new string",
      'The function should return the string "Hello, Ink!"',
      "Make sure to use the correct syntax for ink!",
    ],
    tags: ["basic", "functions", "strings"],
  },
  {
    id: "basics-1",
    title: "Storage Basics",
    description: "Learn how to use contract storage to maintain state",
    difficulty: "Beginner",
    category: "Basics",
    instructions: `Learn to use the #[ink(storage)] attribute to store data.

Implement a counter that can be incremented and retrieved.

Key concepts:
- Storage struct fields
- Mutable references
- Getter and setter patterns`,
    starterCode: `#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod counter {
    #[ink(storage)]
    pub struct Counter {
        // TODO: Add a field to store the counter value
        value: u32,
    }

    impl Counter {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: 0 }
        }

        // TODO: Implement the increment function
        #[ink(message)]
        pub fn increment(&mut self) {
            // Implement this
            todo!()
        }

        #[ink(message)]
        pub fn get_value(&self) -> u32 {
            self.value
        }
    }
}`,
    hints: [
      "Use &mut self to make the function mutable",
      "Increment the value by 1",
      "Check the storage struct for the field name",
    ],
    tags: ["storage", "mutable", "state"],
  },
  {
    id: "storage-1",
    title: "Mapping Storage",
    description: "Use mappings to create key-value stores in your contract",
    difficulty: "Intermediate",
    category: "Storage",
    instructions: `Implement a simple key-value storage using ink! mappings.

Create a balances mapping and implement deposit/withdrawal functions.`,
    starterCode: `#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod simple_store {
    use ink::prelude::*;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct SimpleStore {
        balances: Mapping<AccountId, Balance>,
    }

    impl SimpleStore {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                balances: Mapping::new(),
            }
        }

        // TODO: Implement set_balance function
        #[ink(message)]
        pub fn set_balance(&mut self, account: AccountId, amount: Balance) {
            // Implement this
            todo!()
        }

        #[ink(message)]
        pub fn get_balance(&self, account: AccountId) -> Balance {
            self.balances.get(account).unwrap_or(0)
        }
    }
}`,
    hints: [
      "Use mapping.insert() to set a value",
      "The insert method takes a key and value",
      "Remember to pass the account and amount",
    ],
    tags: ["mapping", "storage", "key-value"],
  },
  {
    id: "events-1",
    title: "Emitting Events",
    description: "Learn to emit events for off-chain indexing",
    difficulty: "Intermediate",
    category: "Events",
    instructions: `Implement event emission in your contract.

Create and emit a Transfer event when balance changes occur.`,
    starterCode: `#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod event_emitter {
    use ink::prelude::*;

    // TODO: Define a Transfer event
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        amount: Balance,
    }

    #[ink(storage)]
    pub struct EventEmitter {
        balances: ink::storage::Mapping<AccountId, Balance>,
    }

    impl EventEmitter {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                balances: ink::storage::Mapping::new(),
            }
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: Balance) -> bool {
            // TODO: Emit a Transfer event
            self.env().emit_event(Transfer {
                from: self.env().caller(),
                to,
                amount,
            });
            true
        }
    }
}`,
    hints: [
      "Use #[ink(event)] to define an event struct",
      "Use #[ink(topic)] on fields that should be indexed",
      "Emit events with self.env().emit_event()",
    ],
    tags: ["events", "emit", "indexing"],
  },
]

export const getExercisesByCategory = (category: string) => {
  return exercises.filter((e) => e.category === category)
}

export const getCategories = () => {
  return Array.from(new Set(exercises.map((e) => e.category)))
}

export const getDifficultyStats = () => {
  return {
    beginner: exercises.filter((e) => e.difficulty === "Beginner").length,
    intermediate: exercises.filter((e) => e.difficulty === "Intermediate").length,
    advanced: exercises.filter((e) => e.difficulty === "Advanced").length,
  }
}

export const getExerciseProgress = (exerciseIds: string[]) => {
  const exercises_by_id = Object.fromEntries(exercises.map((e) => [e.id, e]))
  return exerciseIds.map((id) => exercises_by_id[id]).filter(Boolean)
}
