use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Foo {
    i: u32,
}

#[derive(Serialize, Deserialize)]
struct Bar<K> {
    f: K,
}

fn main() {
    let bf = Bar::<Foo> { f: Foo { i: 9 } };

    let serialized = serde_json::to_string(&bf).unwrap();

    println!("serialized = {}", serialized);

    let _deserialized: Bar<Foo> = serde_json::from_str(&serialized).unwrap();
}
