#[derive(Default, Debug)]
pub struct Rectangle<T> 
{
    x: T,
    y: T,
    width: T,
    height: T
}

pub struct Point<T> 
{
    x: T,
    y: T,
}

trait Rect<T> : Default 
{
    fn new(x: T, y : T, width: T, height: T) -> Self;
    
    fn adjust(&mut self, dx1: T, dy1: T, dx2: T, dy2: T);
    fn adjust_const(&self, dx1: T, dy1: T, dx2: T, dy2: T) -> Self;

    fn left(&self) -> T;
    fn right(&self) -> T;
    fn top(&self) -> T;
    fn bottom(&self) -> T;

    fn x(&self) -> T;
    fn y(&self) -> T;

    fn height(&self) -> T;
    fn width(&self) -> T;

    fn bottom_left(&self) -> Point<T>;
    fn bottom_right(&self) -> Point<T>;
    fn top_left(&self) -> Point<T>;
    fn top_right(&self) -> Point<T>;

    fn center(&self) -> Point<T>;
    fn contains_point(&self, point: &Point<T>) -> bool;
    fn contains_rect(&self, point: &Rectangle<T>) -> bool;

    fn is_intersected(&self, rect: &Rectangle<T>) -> bool;
    fn intersected(&self, rect: &Rectangle<T>) -> Rectangle<T>;
    fn united(&self, rect: &Rectangle<T>) -> Rectangle<T>;

    fn transposed(&self) -> Rectangle<T>;
}


impl <T> Rect<T> for Rectangle<T> 
where 
    T:Default,
    T:Copy,
    T:std::ops::Add<Output=T>,
    T:std::ops::Sub<Output=T>,
    T:std::ops::Div<Output=T>,
    T:std::ops::Mul<Output=T>,
    T:std::cmp::PartialOrd,
    T:From<i32>
{
    fn new(x: T, y : T, width: T, height: T) -> Self{
        let mut x_ = x;
        let mut y_ = y;
        let mut w_ = width;
        let mut h_ = height;
        if width < T::from(0) {
            w_ = T::from(-1) * width;
            x_ = x - w_;
        }
        if height < T::from(0) {
            h_ = T::from(-1) * width;
            y_ = y - h_;
        }
        Self { x: x_, y: y_, width: w_, height: h_ }
    }
    
    fn adjust(&mut self, dx1: T, dy1: T, dx2: T, dy2: T) {
        let new_rect = self.adjust_const(dx1, dy1, dx2, dy2);
        self.x = new_rect.x;
        self.x = new_rect.y;
        self.width = new_rect.width;
        self.height = new_rect.height;
    }
    fn adjust_const(&self, dx1: T, dy1: T, dx2: T, dy2: T) -> Self{
        let mut x_top_right = self.top_right().x + dx2;
        let mut y_top_right = self.top_right().y + dy2;
        let mut x_bot_left = self.bottom_left().x + dx1;
        let mut y_bot_left = self.bottom_left().y + dy1;
        if x_bot_left > x_top_right{
            // let tmp = x_top_right;
            // x_top_right = x_bot_left;
            // x_bot_left = tmp;
            (x_top_right, x_bot_left) = (x_bot_left, x_top_right);
        }
        if y_bot_left > y_top_right{
            // let tmp = y_top_right;
            // y_top_right = y_bot_left;
            // y_bot_left = tmp;
            (y_top_right, y_bot_left) = (y_bot_left, y_top_right);
        }
        let x_ = x_bot_left;
        let y_ = y_bot_left;
        let w_ = x_top_right - x_bot_left;
        let h_ = y_top_right - y_bot_left;
        return Rectangle{ 
            x: (x_), 
            y: (y_), 
            width: (w_), 
            height: (h_) };
    }

    fn left(&self) -> T     { self.x }
    fn right(&self) -> T    { self.x + self.width }
    fn top(&self) -> T      { self.y + self.height }
    fn bottom(&self) -> T   { self.y }

    fn x(&self) -> T        { self.x }
    fn y(&self) -> T        { self.y }
    fn height(&self) -> T   { self.height }
    fn width(&self) -> T    { self.width }

    fn bottom_left(&self) -> Point<T>   { Point{x:self.x, y:self.y} }
    fn bottom_right(&self) -> Point<T>  { Point{x:self.x + self.width, y:self.y} }
    fn top_left(&self) -> Point<T>      { Point{x:self.x, y:self.y + self.height} }
    fn top_right(&self) -> Point<T>     { Point{x:self.x + self.width, y:self.y + self.height} }

    fn center(&self) -> Point<T>{
        let center_x = self.x + self.width / T::from(2);
        let center_y = self.y + self.height / T::from(2);
        Point { x: center_x, y: center_y }
    }

    fn contains_point(&self, point: &Point<T>) -> bool {
        if point.x <= self.x + self.width && point.x >= self.x {
            if point.y <= self.y + self.height && point.y >= self.y {
                return true;
            }
        }
        return false;
    }
    fn contains_rect(&self, point: &Rectangle<T>) -> bool {
        if point.x >= self.x && point.y >= self.y {
            if point.height <= self.height && point.width <= self.width {
                return true;
            }
        }
        return false;
    }

    fn is_intersected(&self, rect: &Rectangle<T>) -> bool{
        if self.contains_point(&rect.bottom_left()) ||
            self.contains_point(&rect.bottom_right()) ||
            self.contains_point(&rect.top_left()) ||
            self.contains_point(&rect.top_right()) || 
            rect.contains_point(&self.bottom_left())
            //если вдруг все точки self лежат внутри rect, достаточно проверить 1 точку
        {
            return true;
        }
        return false;
    }
    
    fn intersected(&self, rect: &Rectangle<T>) -> Rectangle<T> {
        if !self.is_intersected(rect) {
            return Self::new(T::from(0),T::from(0),T::from(0),T::from(0));
        }

        let x_ = if self.x > rect.x { self.x } else { rect.x };
        let y_ = if self.y > rect.y { self.y } else { rect.y };
        let rx1 = self.right();
        let rx2 = rect.right();
        let rx = if rx1 < rx2 { rx1 } else { rx2 };
        let ty1 = self.top();
        let ty2 = rect.top();
        let ty = if ty1 < ty2 { ty1 } else { ty2 };
        let w_ = rx - x_;
        let h_ = ty - y_;

        return Self::new(x_, y_, w_, h_);
    }

    fn united(&self, rect: &Rectangle<T>) -> Rectangle<T> {
        let x_ = if self.x < rect.x { self.x } else { rect.x };
        let y_ = if self.y < rect.y { self.y } else { rect.y };
        let rx1 = self.right();
        let rx2 = rect.right();
        let rx = if rx1 > rx2 { rx1 } else { rx2 };
        let ty1 = self.top();
        let ty2 = rect.top();
        let ty = if ty1 > ty2 { ty1 } else { ty2 };
        let w_ = rx - x_;
        let h_ = ty - y_;

        return Self::new(x_, y_, w_, h_);
    }

    fn transposed(&self) -> Rectangle<T>    { 
        Rectangle 
        {
            x:self.x, 
            y:self.y, 
            width: self.height, 
            height: self.width
        } 
    }
}

impl <T> std::ops::BitAnd for Rectangle<T> 
where
    T:Default,
    T:Copy,
    T:std::ops::Add<Output=T>,
    T:std::ops::Sub<Output=T>,
    T:std::ops::Div<Output=T>,
    T:std::ops::Mul<Output=T>,
    T:std::cmp::PartialOrd,
    T:From<i32>
{
    type Output = Rectangle<T>;
    fn bitand(self, rhs: Rectangle<T>) -> Self::Output {
        self.intersected(&rhs)
    }
}

impl <T> std::ops::BitAndAssign for Rectangle<T> 
where
    T:Default,
    T:Copy,
    T:std::ops::Add<Output=T>,
    T:std::ops::Sub<Output=T>,
    T:std::ops::Div<Output=T>,
    T:std::ops::Mul<Output=T>,
    T:std::cmp::PartialOrd,
    T:From<i32>
{
    fn bitand_assign(self: &mut Rectangle<T>, rhs: Rectangle<T>){
        let tmp_rect = self.intersected(&rhs);
        (self.x, self.y, self.width, self.height) = (tmp_rect.x, tmp_rect.y, tmp_rect.width, tmp_rect.height);
    }
}

// trait std::ops::BitOr
impl <T> std::ops::BitOr for Rectangle<T> 
where
    T:Default,
    T:Copy,
    T:std::ops::Add<Output=T>,
    T:std::ops::Sub<Output=T>,
    T:std::ops::Div<Output=T>,
    T:std::ops::Mul<Output=T>,
    T:std::cmp::PartialOrd,
    T:From<i32>
{
    type Output = Rectangle<T>;
    fn bitor(self, rhs: Rectangle<T>) -> Self::Output {
        self.united(&rhs)
    }
}

impl <T> std::ops::BitOrAssign for Rectangle<T> 
where
    T:Default,
    T:Copy,
    T:std::ops::Add<Output=T>,
    T:std::ops::Sub<Output=T>,
    T:std::ops::Div<Output=T>,
    T:std::ops::Mul<Output=T>,
    T:std::cmp::PartialOrd,
    T:From<i32>
{
    fn bitor_assign(self: &mut Rectangle<T>, rhs: Rectangle<T>){
        let tmp_rect = self.united(&rhs);
        (self.x, self.y, self.width, self.height) = (tmp_rect.x, tmp_rect.y, tmp_rect.width, tmp_rect.height);
    }
}

#[test]
fn test(){
    let r = Rectangle::new(1.0, 2.0, 3.0, 4.0);
    println!("{:?}", r);
    let r = Rectangle::new(1, 2, 3, 4);
}