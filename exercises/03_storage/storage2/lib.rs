// storage2
//
// This exercise teaches you about StorageVec<T> in ink!
//
// StorageVec<T> is optimized for storing collections on-chain.
// Unlike regular Vec<T> which loads all elements into memory at once<Packed storage>
// StorageVec<T> loads elements lazily, saving gas.
// Theoretically it can grow to infinite size.
// But currently the length limit is 2 ^ 32 elements
//
// Use cases:
// - Lists of items (todos, records, etc.)
// - Dynamic arrays that grow over time
// - Collections where you access one element at a time
//
// Execute `inklings verify 03_storage/storage2` when you think you're done!

#[ink::contract]
mod storage2 {
    use ink::prelude::vec::Vec;
    use ink::primitives::H160;
    use ink::storage::traits::StorageLayout;
    use ink::storage::StorageVec;
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        /// Caller is not the owner
        NotOwner,
        /// Task not found at the given index
        TaskNotFound,
        /// No tasks available
        NoTasks,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(StorageLayout)]
    pub struct Task {
        pub description: String,
        pub completed: bool,
    }

    #[ink(storage)]
    pub struct Storage2 {
        // TODO: Add a StorageVec<Task> field called 'tasks'
        // TODO: Add an owner field of type Account Address of type H160
        todo!()
    
    }

    impl Storage2 {
        /// Constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();

            // TODO: Initialize with:
            // - tasks: StorageVec::new()
            // - owner: caller
            todo!()
           
        }

        /// Add a new task
        #[ink(message)]
        pub fn add_task(&mut self, description: String) -> Result<()> {
            let caller = self.env().caller();

            // TODO: Check if caller is owner
            // If not, return Error::NotOwner

            // TODO: Create a new Task with completed = false
            // TODO: Push the task to the StorageVec using self.tasks.push(&task)
            todo!()
            
        }

        /// Get a task by index
        #[ink(message)]
        pub fn get_task(&self, index: u32) -> Option<Task> {
            // TODO: Get task from StorageVec
            // Hint: self.tasks.get(index)
            // This returns Option<Task>, perfect for our return type!
            todo!()
            
        }

        /// Mark a task as completed
        #[ink(message)]
        pub fn complete_task(&mut self, index: u32) -> Result<()> {
            let caller = self.env().caller();

            // TODO: Check if caller is owner

            // TODO: Get the task at index
            // If it doesn't exist, return Error::TaskNotFound

            // TODO: Update the task's completed field to true
            // TODO: Set the updated task back to the StorageVec at the same index
            // Hint: self.tasks.set(index, &updated_task);
            todo!()
           
        }

        /// Get the total number of tasks
        #[ink(message)]
        pub fn task_count(&self) -> u32 {
            // TODO: Return the length of the StorageVec
            // Hint: self.tasks.len()
            todo!()
           
        }

        /// Get all tasks (be careful with gas costs on large lists!)
        #[ink(message)]
        pub fn get_all_tasks(&self) -> Vec<Task> {
            // TODO: Create an empty Vec
            // TODO: Iterate from 0 to self.tasks.len()
            // TODO: For each index, get the task and push it to the Vec
            // TODO: Return the Vec
            //
            // Hint: Use a for loop with self.tasks.get(i)
            todo!()
            
        }

        /// Remove the last task
        #[ink(message)]
        pub fn remove_last_task(&mut self) -> Result<()> {
            let caller = self.env().caller();

            // TODO: Check if caller is owner

            // TODO: Use self.tasks.pop() to remove the last task
            // If pop() returns None, return Error::NoTasks
            // Otherwise return Ok(())
            todo!()
          
        }

        /// Get the owner
        #[ink(message)]
        pub fn get_owner(&self) -> H160 {
            // TODO: Return the owner
            todo!()
           
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);
            let contract = Storage2::new();

            assert_eq!(contract.task_count(), 0);
            assert_eq!(contract.get_owner(), accounts.alice);
        }

        #[ink::test]
        fn add_task_works() {
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);
            let mut contract = Storage2::new();

            assert_eq!(contract.add_task(String::from("Learn ink!")), Ok(()));
            assert_eq!(contract.task_count(), 1);

            let task = contract.get_task(0).unwrap();
            assert_eq!(task.description, String::from("Learn ink!"));
            assert_eq!(task.completed, false);
        }

        #[ink::test]
        fn add_task_fails_not_owner() {
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);
            let mut contract = Storage2::new();

            // Bob tries to add task
            ink::env::test::set_caller(accounts.bob);
            assert_eq!(
                contract.add_task(String::from("Task")),
                Err(Error::NotOwner)
            );
            assert_eq!(contract.task_count(), 0);
        }

        #[ink::test]
        fn complete_task_works() {
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);
            let mut contract = Storage2::new();

            contract.add_task(String::from("Task 1")).unwrap();
            assert_eq!(contract.complete_task(0), Ok(()));

            let task = contract.get_task(0).unwrap();
            assert_eq!(task.completed, true);
        }

        #[ink::test]
        fn complete_task_not_found() {
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);
            let mut contract = Storage2::new();

            assert_eq!(contract.complete_task(0), Err(Error::TaskNotFound));
        }

        #[ink::test]
        fn get_all_tasks_works() {
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);
            let mut contract = Storage2::new();

            contract.add_task(String::from("Task 1")).unwrap();
            contract.add_task(String::from("Task 2")).unwrap();
            contract.add_task(String::from("Task 3")).unwrap();

            let all_tasks = contract.get_all_tasks();
            assert_eq!(all_tasks.len(), 3);
            assert_eq!(all_tasks[0].description, String::from("Task 1"));
            assert_eq!(all_tasks[1].description, String::from("Task 2"));
            assert_eq!(all_tasks[2].description, String::from("Task 3"));
        }

        #[ink::test]
        fn remove_last_task_works() {
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);
            let mut contract = Storage2::new();

            contract.add_task(String::from("Task 1")).unwrap();
            contract.add_task(String::from("Task 2")).unwrap();
            assert_eq!(contract.task_count(), 2);

            assert_eq!(contract.remove_last_task(), Ok(()));
            assert_eq!(contract.task_count(), 1);

            let task = contract.get_task(0).unwrap();
            assert_eq!(task.description, String::from("Task 1"));
        }

        #[ink::test]
        fn remove_last_task_empty() {
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);
            let mut contract = Storage2::new();

            assert_eq!(contract.remove_last_task(), Err(Error::NoTasks));
        }

        #[ink::test]
        fn multiple_tasks_workflow() {
            let accounts = ink::env::test::default_accounts();
            ink::env::test::set_caller(accounts.alice);
            let mut contract = Storage2::new();

            // Add tasks
            contract.add_task(String::from("Buy milk")).unwrap();
            contract.add_task(String::from("Write code")).unwrap();
            contract.add_task(String::from("Deploy contract")).unwrap();

            // Complete middle task
            contract.complete_task(1).unwrap();

            // Check states
            assert_eq!(contract.get_task(0).unwrap().completed, false);
            assert_eq!(contract.get_task(1).unwrap().completed, true);
            assert_eq!(contract.get_task(2).unwrap().completed, false);

            assert_eq!(contract.task_count(), 3);
        }
    }
}
