struct OneToTen(u32);

fn one_to_ten() -> OneToTen {
    OneToTen(1)
}

impl Iterator for OneToTen {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.0 > 10 {
            None
        } else {
            let res = Some(self.0);
            self.0 += 1;
            res
        }
    }
}


struct Doubler<I>{
    iter: I,
}
impl<I> Iterator for Doubler<I>
    where
    I: Iterator,
    I::Item: std::ops::Mul<Output=I::Item> + From<u8>,  
    {
        type Item = I::Item;
        fn next(&mut self) -> Option<Self::Item> {
            match self.iter.next() {
                None => None,
                Some(x) => Some(x * From::from(2u8)),
            }
        }
    }

fn sum<I>(iter: I) -> I::Item
    where
    I: Iterator,
    I::Item: std::ops::Add<Output=I::Item> + From<u32>,
{
    iter.fold(From::from(0u32), std::ops::Add::add)
}

struct Fibs{
    x: u32,
    y:u32,
}

fn fibs() -> Fibs{
    Fibs{
        x: 0,
        y: 1,
    }
}

impl Iterator for Fibs{
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let origin_y = self.x + self.y;
        self.x = self.y;
        self.y = origin_y;
        Some(self.y)
    }
}

pub fn run() {
    for i in one_to_ten() {
        println!("{}", i);
    }

    let orig_iter = 1..11;
    let doubled_iter = Doubler {
        iter: orig_iter,
    };

    for i in doubled_iter {
        println!("{}", i);
    }


    let res = (1..11).fold(0, |x, y| x + y);
    println!("{}", res);

    //let res = (1..11).fold(0, std::ops::Add::add);
    println!("{}", sum(1..11000u32));


    for i in fibs().take(10) {
        println!("{}", i);
    }


    
}