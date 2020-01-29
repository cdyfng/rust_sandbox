use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::hash::Hash;
use std::clone::Clone;

struct Cache<T, U, W>
    where T: Fn(U) -> W,
          U: Eq + Hash + Clone {
    calculator: T,
    values: HashMap<U, W>,
}

impl<T, U, W> Cache<T, U, W>
    where T: Fn(U) -> W,
          U: Eq + Hash + Clone {
    fn new(calculator: T) -> Cache<T, U, W> {
        Cache {
            calculator,
            values: HashMap::new(),
        }
    }

    /*
     * The ugliest part in this function is the constraint that we have to clone the input.
     * HashMap::entry() moves the key, which renders it unusable for subsequent calculations.
     * So we can either
     * - always perform the expensive computation
     * (OR)
     * - clone the input so that it can be re-used later
     *   
     * Another problem is that defining a closure in or_insert_with borrows the Cache::calculator, and that creates a problem
     * when we attempt to borrow it again by invoking self.calculator. This is why the following line won't work:
     * self.values.entry(input.clone()).or_insert_with(|| { (self.calculator)(input) })
     */  
    fn get(&mut self, input: U) -> &W {
        let calc = &self.calculator;
        self.values.entry(input.clone()).or_insert_with(|| { (calc)(input) })
    }   
}

pub fn run() {
    println!("Hello, world!");
    let mut c = Cache::new(|x: &str| {
        println!("performing an expensive calculation with input: {}", x);
        thread::sleep(Duration::from_secs(2));
        format!("Original input: {}", x)
    });

    let x = c.get("10");
    println!("x: {}, cacher after getting {}", x, 10);

    let x = c.get("10");
    println!("x: {}, cacher after getting {}", x, 10);

    let x = c.get("20");
    println!("x: {}, cacher after getting {}", x, 20);

    let x = c.get("10");
    println!("x: {}, cacher after getting {}", x, 10);

    let x = c.get("10");
    println!("x: {}, cacher after getting {}", x, 10);

    let x = c.get("20");
    println!("x: {}, cacher after getting {}", x, 20);
}