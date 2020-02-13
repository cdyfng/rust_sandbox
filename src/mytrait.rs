pub trait Summary {
    // add code here
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;
}

#[derive(Clone)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.summarize_author(), self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}



pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        String::from("None")
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}


pub fn notify2<T: Summary, T2: Summary>(item: T, item2: T2){
    println!("Breaking news2!  {}", item.summarize());
    println!("Breaking news3!  {}", item2.summarize());
}

pub fn run(){
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());    

    let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
    };
    println!("1 new article: {}", article.summarize());  


    notify(article.clone());
    notify2(article, tweet); 
}
