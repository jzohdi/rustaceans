struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn z(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MixedPoint<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

impl<T, U> MixedPoint<T, U> {
    fn mixin<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn basics_1() {
    let inter_p = Point { x: 5, y: 10 };
//    println!("int z: {}", inter_p.z());
    let float_p = Point { x: 1.0, y: 4.0 };
    println!("float z: {}", float_p.z());
    let mixed_p = MixedPoint { x: 5, y: 2.0 };
    println!("mixed_p x: {}", mixed_p.get_x());
    let other_mixed = MixedPoint { x: "hello", y: "world" };
    let new_mixed = mixed_p.mixin(other_mixed);
    println!("new mixed x: {}, y: {}", new_mixed.x, new_mixed.y);
}

fn main() {
    basics_1();
    traits();
}

fn traits() {
    let tweet = Tweet { 
        username: String::from("jzohdi"),
        content: String::from("Test post please ignore"),
        reply: false,
        retweet: false,
    };
    println!("tweet: {}", tweet.summarize());
}


trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
