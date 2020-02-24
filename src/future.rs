use std::time::Duration;
use std::thread::sleep;

fn run_ext<F>(mut f: F) where F: Future<Item=(), Error=()> {
    loop {
        match f.poll() {
            Ok(Async::Ready(_)) => break,
            Ok(Async::NotReady) => (),
            Err(_) => break,
        }
        sleep(Duration::from_millis(1000));
    }
}


enum Async<T>{
    Ready(T),
    NotReady
}

type Poll<T, E> = Result<Async<T>, E>;

trait Future {
    type Item;
    type Error;
    
    fn poll(&mut self) -> Poll<Self::Item, Self::Error>;
}

struct MyStruct {
    count: u32,
}

impl Future for MyStruct {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error>{
        println!("Count: {}", self.count);

        match self.count {
            3 => Ok(Async::Ready(())),
            _=> {
                self.count +=1;
                Ok(Async::NotReady)
            }
        }
    }
}

pub fn run(){
    let s = MyStruct{count:0};
    run_ext(s); 
}