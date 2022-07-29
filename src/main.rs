trait Animal {
    // trait 공통 메서드
    // 위 trait을 가지는 각각의 구조체들이 공통적으로 가질 필요가 있는 동작은 정리되어 명시되어야 하며,
    // 각 인스턴스 상에서 아래 메소드를 호출함으로써 해당 정리를 얻어낼 수 있어야 한다
    fn fields(&self) -> String;
}

struct Lion {
    name: String
}

// impl [trait] for [struct(type)] {}
impl Animal for Lion {
    fn fields(&self) -> String {
        format!("{}", self.name)
    }
}

struct Hippo {
    name: String
}

impl Animal for Hippo {
    fn fields(&self) -> String {
        format!("{}", self.name)
    }
}

struct Elephant {
    name: String
}

impl Animal for Elephant {
    fn fields(&self) -> String {
        format!("{}", self.name)
    }
}


// 여러 개의 제네릭 타입 파라미터를 가진 함수들에 대하여, 각 제네릭은 고유의 trait bound를 가짐.
// 꺾쇠 괄호 내에 많은 수의 trait bound를 특정할 수 있지만, 가독성을 떨어트릴 수 있으므로
// 함수 시그니처 뒤에 where 절 뒤로 trait bound를 옮겨서 특정하도록 해주는 대안 문법이 있음
//
// e.g. fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
// -> fn some_function<T, U>(t: T, u: U) -> i32
//       where T: Display + Clone,
//             U: Clone + Debug
//   {}
struct Mystruct<T: Animal>  // bound Animal을 where절을 사용해 대체할 수 있음.
// where
//     T: Animal
{
    inner: T
}

impl<T: Animal> Mystruct<T> {
    pub fn new(t: T) -> Self {
        Self {
            inner: t
        }
    }
}

fn get_animal_name<T: Animal> (t: &Mystruct<T>) -> String {
    t.inner.fields()
}

// Dynamic dispatch? run-time에 어떤 메소드를 실행할 지 결정하는 것(default: 정적 디스패치 - compile-time)
// trait objects? set of traits을 구현한 타입의 인스턴스(unsized value)
// 'dyn Animal'을 참조하거나 Box<T>와 같이 포인터로 사용하는 이유?
// compile-time에 Animal trait의 크기를 알 수 없기 때문(sized 조건 불충족)
// 제네릭 타입 파라미터는 한 번에 하나의 구체 타입으로만 대입될 수 있는 반면,
// trait object를 사용하면 run-time에 여러 구체 타입을 채워넣는 것도 가능해짐
// 결론적으로, compile-time에 모든 타입을 알 필요는 없게 된다
// todo!증명 할 것
fn get_animal(t: &dyn Animal) -> String {
    t.fields()
}

fn main() {
    let a = Mystruct::new(Lion {name: "Lion".to_string()});
    let b = Mystruct::new(Hippo {name: "Hippo".to_string()});
    let c = Mystruct::new(Elephant {name: "Elephant".to_string()});
    println!("{}", &a.inner.name);
    dbg!(get_animal_name(&a));
    dbg!(get_animal_name(&b));
    dbg!(get_animal_name(&c));
    dbg!(get_animal(&a.inner));
    dbg!(get_animal(&b.inner));
    dbg!(get_animal(&c.inner));
}