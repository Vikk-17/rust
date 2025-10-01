pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// Normally we just use impl with the name of the struct to define a funtion 
// and under that we function
// Example
// impl NewsArticle {
//    fn name() {} 
// }
// Defining trait along with for keyword
// Name of the trait is Summary
/*
 *  Within the impl block we put the method signatures that the trait definition has defined.
 *  Instead of Semicolon after each signature, we use curly brackets and fill in the method body
 *  with the specific behavior that we want the methods of the trait to have the particular type 
 *  */
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}


pub struct SocialPost {
    pub usename: String,
    pub content: String,
    pub reply: String,
    pub repost: String,
}

// Defining trait along with for keyword
// Name of the trait is Summary
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

