use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<T, U, V>
where 
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy
{
    calculation: T,
    cache: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V> 
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy
{
    pub fn new(calculation: T) -> Cacher<T, U, V> {
        let mut cache: HashMap<U, V> = HashMap::new();
        Cacher {
            calculation,
            cache, 
        }
    }
    pub fn call(&mut self, arg: U) -> V {
        match self.cache.get(&arg) {
            Some(val) => *val,
            None => {
                let res = (self.calculation)(arg);
                self.cache.insert(arg, res);
                res
            }
        }
    }
}

fn expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout_1(intensity: u32, random: u32) {
    let closure = |num| {
        expensive_calculation(num)
    };

    if intensity < 25 {
        println!("Do {} pushups!", closure(intensity));
        println!("Do {} situps!", closure(intensity));
    } else {
        if random == 3 {
            println!("Take a break today, remember to stay hydrated!");
        } else {
            println!("Run for {} minutes!", closure(intensity));
        }
    }
}

fn generate_workout(intensity: u32, random: u32) {
    let mut cacher = Cacher::new(expensive_calculation);

    if intensity < 25 {
        println!("Do {} pushups!", cacher.call(intensity));
        println!("Do {} situps!", cacher.call(intensity));
    } else {
        if random == 3 {
            println!("Take a break today, remember to stay hydrated!");
        } else {
            println!("Run for {} minutes!", cacher.call(intensity));
        }
    }
}
fn main() {
    let intensity = 10;
    let random_num = 7;

    generate_workout(intensity, random_num);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn cacher_diff_vals() {
        let mut cacher = Cacher::new(|num| num);
        assert_eq!(1, cacher.call(1));
        assert_eq!(2, cacher.call(2));
        assert_eq!(1, cacher.call(1));
    }
    
    #[test]
    fn cacher_string() {
        let mut cacher = Cacher::new(|s| s);
        assert_eq!("hello", cacher.call("hello"));
        assert_eq!("world", cacher.call("world"));
    }
}
