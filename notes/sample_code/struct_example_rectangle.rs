fn main() {
    let rec1 = Rectangle{
        width:30,
        length: 40
    };

    let rec2 = Rectangle{
        width:40,
        length: 50
    };

    println!("The are of rec is {}", rec1.area());
    println!("Can rect1 hold rect2 ? {} ", rec1.can_hold(rec2));
}

struct Rectangle {
    length:u32,
    width:u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, rec2:Rectangle) -> bool {
        self.length > rec2.length && self.width > rec2.width
    }
}
