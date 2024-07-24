fn main() {
    println!("Hello, world!");
}

struct Post{
    content: String,
    state: Option<Box<dyn State>>,
}

impl Post{
    pub fn new() -> Post{
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self,text:&str){
        self.content.push_str(text);
    }
    pub fn content(&self)->&str{
        ""
    }
    pub fn request_review(&mut self){
        if let Some(s)=self.state.take(){
            self.state=Some(s.request_revew())
        }
    }
}

trait State{
    fn request_revew(self)-> Box<dyn State>;
}
struct Draft;
struct PendingReview{}

impl State for Draft{
    fn request_revew(self) -> Box<dyn State> {
        Box::new(PendingReview{})
    }
}

impl State for PendingReview{
    fn request_revew(self:Box<Self>) -> Box<dyn State> {
        self
    }
}