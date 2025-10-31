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
        tasks: StorageVec<Task>,
        owner: H160,
    }

    impl Storage2 {
        /// Constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                tasks: StorageVec::new(),
                owner: caller,
            }
        }

        /// Add a new task
        #[ink(message)]
        pub fn add_task(&mut self, description: String) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            let task = Task {
                description,
                completed: false,
            };
            self.tasks.push(&task);
            Ok(())
        }

        /// Get a task by index
        #[ink(message)]
        pub fn get_task(&self, index: u32) -> Option<Task> {
            self.tasks.get(index)
        }

        /// Mark a task as completed
        #[ink(message)]
        pub fn complete_task(&mut self, index: u32) -> Result<()> {
            let caller = self.env().caller();

            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            let mut task = self.tasks.get(index).ok_or(Error::TaskNotFound)?;
            task.completed = true;
            self.tasks.set(index, &task);
            Ok(())
        }

        /// Get the total number of tasks
        #[ink(message)]
        pub fn task_count(&self) -> u32 {
            self.tasks.len()
        }

        /// Get all tasks (be careful with gas costs on large lists!)
        #[ink(message)]
        pub fn get_all_tasks(&self) -> Vec<Task> {
            let len = self.tasks.len();
            let mut all_tasks = Vec::with_capacity(len as usize);
            for i in 0..len {
                if let Some(task) = self.tasks.get(i) {
                    all_tasks.push(task);
                }
            }
            all_tasks
        }

        /// Remove the last task
        #[ink(message)]
        pub fn remove_last_task(&mut self) -> Result<()> {
            let caller = self.env().caller();

            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            match self.tasks.pop() {
                Some(_) => Ok(()),
                None => Err(Error::NoTasks),
            }
        }

        /// Get the owner
        #[ink(message)]
        pub fn get_owner(&self) -> H160 {
            self.owner
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
