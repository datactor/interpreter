/// 동물원에는 코끼리, 하마 그리고 사자 세 종류의 동물이 산다. 낮에는 자유롭게 뛰놀지만 
/// 밤이 되면 다시 사육장에 들어가야 한다. 사육장에 들어갈 때 한 통로로 모든 동물이 같이 들어가며
/// 같은 종류의 동물 2 마리가 연달아 들어가게 되면 서로 싸우게 된다.
///
/// 사육장(Cage) 에 들어가는 동물은 모두 고유한 이름을 가지며 한 줄로 
/// 코끼리, 사자 그리고 하마 타입을 push 하고 pop 할 때 같은 타입이 2 번 연달아 나오면 특정 문장을 
/// 출력하는 FIFO queue 를 구현하는 것이 목적이며 아래 코드를 자유롭게 "확장" 해서 구현.
///
/// 예)
/// Hippo1 - Lion1 - Elephant1 - Elephant2 - Hippo1
/// 출력 => "Elephant1 and Elephant2 fight."
use std::collections::VecDeque;
use std::boxed::Box;
use std::io::stdin;
use std::io::Write;


// 다양한 타입(struct)을 벡터에 넣기
// 타입이 다르더라도 같은 trait bound라면 박스로 감싸거나 참조형태로 벡터에 넣을 수 있다
// (dyn dispatch를 사용. 런타임에 생성되는 트레잇 오브젝트(인스턴스)는 사이즈를 컴파일에는 알 수 없는데,
// 참조형태나 박스에 넣어서 사이즈드 되게 해 넣는 방식)
struct Cage {
    lane: VecDeque<Box<dyn Animal>>,
}

impl Cage {
    fn new() -> Self {
        Cage { lane: VecDeque::new() }
    }

    fn push(&mut self, animal: Box<dyn Animal>) {
        self.lane.push_back(animal);
        let len = self.lane.len();
        if len > 1 {
            if self.lane[len - 1].animal_type() == self.lane[len - 2].animal_type() {
                println!("{} and {} fight",
                         self.lane[len - 2].animal_name(),
                         self.lane[len - 1].animal_name())
            }
        }
        println!("{}", len);
    }

    fn pop(&mut self) {
        self.lane.pop_back().unwrap();
    }
}

pub trait Animal {
    fn animal_name(&self) -> String;
    fn animal_type(&self) -> String;
}

struct Elephant {
    name: String,
    types: String,
}

impl Animal for Elephant {
    fn animal_name(&self) -> String {
        format!("{}", self.name)
    }
    fn animal_type(&self) -> String {
        format!("{}", self.types)
    }
}

impl Elephant {
    fn new() -> Self {
        Elephant {name: String::new(), types: String::new()}
    }

    fn id(&mut self, name: String, types: String) {
        self.name = name;
        self.types = types;
    }
}

struct Lion {
    name: String,
    types: String,
}

impl Animal for Lion {
    fn animal_name(&self) -> String {
        format!("{}", self.name)
    }
    fn animal_type(&self) -> String {
        format!("{}", self.types)
    }
}

impl Lion {
    fn new() -> Self {
        Lion {name: String::new(), types: String::new()}
    }

    fn id(&mut self, name: String, types: String) {
        self.name = name;
        self.types = types;
    }
}

struct Hippo {
    name: String,
    types: String,
}

impl Animal for Hippo {
    fn animal_name(&self) -> String {
        format!("{}", self.name)
    }
    fn animal_type(&self) -> String {
        format!("{}", self.types)
    }
}

impl Hippo {
    fn new() -> Self {
        Hippo {name: String::new(), types: String::new()}
    }

    fn id(&mut self, name: String, types: String) {
        self.name = name;
        self.types = types;
    }
}


fn main() {
    let mut cage = Cage::new();

    // let l1 = Lion { name: "Lion1".to_string(), types: "Lion".to_string()};
    // let l2 = Lion { name: "Lion2".to_string(), types: "Lion".to_string()};
    // let h1 = Hippo { name: "Hippo1".to_string(), types: "Hippo".to_string()};
    // let h2 = Hippo { name: "Hippo2".to_string(), types: "Hippo".to_string()};
    // let e1 = Elephant { name: "Elephant1".to_string(), types: "Elephant".to_string()};
    // let e2 = Elephant { name: "Elephant2".to_string(), types: "Elephant".to_string()};

    let mut l1 = Lion::new();
    let mut l2 = Lion::new();
    let mut h1 = Hippo::new();
    let mut h2 = Hippo::new();
    let mut e1 = Elephant::new();
    let mut e2 = Elephant::new();

    // cage.push(Box::new(a));
    // cage.push(Box::new(d));
    // cage.push(Box::new(h2));
    // cage.push(Box::new(l2));
    // cage.push(Box::new(e1));
    // cage.push(Box::new(e2));

    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("input error");
        match buffer.as_str() {
            "1" => {
                let mut l1 = Lion::new();
                l1.id("Lion1".to_string(), "Lion".to_string());
                cage.push(Box::new(l1));
            },
            "2" => {
                let mut l2 = Lion::new();
                l2.id("Lion2".to_string(), "Lion".to_string());
                cage.push(Box::new(l2));
            },
            "3" => {
                let mut h1 = Hippo::new();
                h1.id("Hippo1".to_string(), "Hippo".to_string());
                cage.push(Box::new(h1));
            },
            "4" => {
                let mut h2 = Hippo::new();
                h2.id("Hippo2".to_string(), "Hippo".to_string());
                cage.push(Box::new(h2));
            },
            "5" => {
                let mut e1 = Elephant::new();
                e1.id("Elephant1".to_string(), "Elephant".to_string());
                cage.push(Box::new(e1));
            },
            "6" => {
                let mut e2 = Elephant::new();
                e2.id("Elephant2".to_string(), "Elephant".to_string());
                cage.push(Box::new(e2))
            },
            _ => {},
        }
    }
}