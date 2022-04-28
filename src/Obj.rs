pub enum status{
    online,
    offline,
    def
}

pub struct User {
    pub username:String,
    pub email:String,
    pub password:String,
    pub status:status
}

impl User{

}