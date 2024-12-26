pub fn demonstrate() {
    let (tweet, article) = basic_trait();

    // 'trait bound' syntax
    notify(&tweet);
    notify2(&article);

    // 'trait bound' with multiple traits
    notify3(&tweet);
    notify4(&article);
    
    // 'trait bound' with where clause
    notify5(&tweet);

    // return type of a trait
    let summ = returns_summarizable();
    println!("summ: {}", summ.summarize());
}

fn basic_trait() -> (Tweet, NewsArticle) {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };

    println!("summerize: {}", tweet.summarize());
    println!("topic: {}", tweet.topic());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
    };

    println!("summerize: {}", article.summarize());
    println!("topic: {}", article.topic());

    (tweet, article)
}

trait Display {
    fn display(&self) -> String;
}

trait Summary {
    fn summarize(&self) -> String;

    // Default implementation
    fn topic(&self) -> String {
        format!("(Read more from {}...)", self.summarize())
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn topic(&self) -> String {
        format!("{} says...", self.username)
    }
}

impl Display for Tweet {
    fn display(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Display for NewsArticle {
    fn display(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// 'trait bound' syntax
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 'trait bound' with multiple traits
fn notify3(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}
fn notify4<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 'trait bound' with where clause
fn notify5<T>(item: &T)
where
    T: Summary + Display,
{
    println!("Breaking news! {}", item.display());
}

// return type of a trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    }
}

// blanket implementations
// impl<T: Display> ToString for T {
//     // --snip--
// }