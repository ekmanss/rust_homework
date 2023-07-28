trait LevelOption {
    fn go_home(&self) -> String;
}

struct Worker;
impl LevelOption for Worker {
    fn go_home(&self) -> String {
        String::from("walk")
    }
}
struct Manager;
impl LevelOption for Manager {
    fn go_home(&self) -> String {
        String::from("bus")
    }
}
struct Boss;
impl LevelOption for Boss {
    fn go_home(&self) -> String {
        String::from("taxi")
    }
}

enum Level {
    Worker(Worker),
    Manager(Manager),
    Boss(Boss),
}

pub fn usingEnum() {
    let vecEnum: Vec<Level> = vec![
        Level::Worker(Worker),
        Level::Manager(Manager),
        Level::Boss(Boss),
    ];

    //使用enum携带不同类型的数据，需要把每个确切的类型都列出来，并写出它们的行为，这样就可以为不同类型的数据提供不同的行为了。
    //优点: 可以在编译时就保证所有的情况都被考虑到了，从而避免出现bug.方法调用是完成的静态分发，这意味着编译器知道在编译时调用哪个方法，性能相对trait object更好。
    //缺点: 代码比较冗长，而且需要在每次添加新类型的时候都要更新代码,且编译后的代码会更大。
    for item in vecEnum {
        match item {
            Level::Worker(x) => println!("Worker go home by: {}", x.go_home()),
            Level::Manager(x) => println!("Manager go home by: {}", x.go_home()),
            Level::Boss(x) => println!("Boss go home by: {}", x.go_home()),
        }
    }
}

pub fn traitObject() {
    let vecEnum: Vec<&dyn LevelOption> = vec![&Worker, &Manager, &Boss];

    //使用trait object，可以在运行时选择不同的类型，而不是在编译时选择。这意味着编译器无法保证所有的情况都被考虑到了，可能会出现bug。方法调用是完成的动态分发，这意味着编译器在运行时才知道调用哪个方法，性能相对enum更差。
    //优点: 代码比较简洁，且编译后的代码会更小。
    //缺点: 无法在编译时保证所有的情况都被考虑到了，可能会出现bug.方法调用是完成的动态分发，这意味着编译器在运行时才知道调用哪个方法，性能相对enum更差。
    for item in vecEnum {
        println!("go home by: {}", item.go_home());
    }
}
