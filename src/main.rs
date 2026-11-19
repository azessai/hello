#[allow(dead_code)]


fn patt() -> () {
    let age: i32 = 47;

    match age {
        tmp if tmp > 60 => {
            println!("plus tout jeune...");
        }
        q @ 40 ..= 49 => println!("Quardra! {}", q),
        tmp if tmp > 17 => {
            println!("majeur !");
        }
        17 => println!("Presque majeur!"),
        x => {
            println!("mineur {} !", x);
        }
    }
}
// fn entier_et_float() -> (i32, f32) {
//     let entier = 11;
//     (entier, 0.1)
// }
// fn addition(a : i32, b : i32) -> i32 {
//     a + b
// }

// fn use_fct () {
//     let tuple = entier_et_float();
//     println!("Entier {}, Flotant {}", tuple.0, tuple.1);
//     println!("1 + 2 = {}", addition(1, 2))
// }
// fn test_expression(x: i32) -> i32 {
//     if x < 0 {
//         println!("{} < 0", x);
//         -1
//     } else if x == 0 {
//         println!("{} == 0", x);
//         0
//     } else {
//         println!("{} > 0", x);
//         1
//     }
// }
// fn boucle_et_print() {
//     loop {
//         println!("Toujours là !");
//         let mut i = 0i32;

//         loop {
//             println!("sous-boucle !");
//             i += 1;
//             if i > 2 {
//                 // On quitte la fonction "boucle_et_print".
//                 return;
//             }
//         }
//     }
// }
//mod distance;
enum Book {
    TheRustPL,
    ZeroToProd,
    RustInAction,
    Other(String),
}
fn title(book: &Book) -> String {
    match book {
        Book::TheRustPL  => "The Rust Programming Language".into(),
        Book::ZeroToProd => "Zero To Production".into(),
        Book::RustInAction => "Rust In Action".into(),
        Book::Other(title) => title.into(),
    }
}

fn autor(book: &Book) -> Option<String> {
    match book {
        Book::TheRustPL  => Some("Autor Toto".into()),
        Book::ZeroToProd => Some("Autor Titi".into()),
        Book::RustInAction => Some("Autor Tutu".into()),
        Book::Other(_) => None,
    }
}
fn disply(book: &Book) -> String {
    let title =  title(&book);
    match autor(&book) {
        Some(autor) => format!("{} written by {}", title, autor),
        None                => format!("{}", title),

    }
}
fn fais_quelque_chose(i: i32) -> Option<String> {
    if i < 10 {
        Some("variable inférieure à 10".to_owned())
    } else {
        None
    }
}
fn main() {
    let mut v = vec!(1, 2, 3);

    while let Some(x) = v.pop() {
        println!("{}", x);
    }
    match fais_quelque_chose(11) {
        Some(s) => println!("{}", &s),
        None => {} // rien à afficher donc on ne fait rien
    }
if let Some(s) = fais_quelque_chose(1) {
    println!("{}", &s)
} else {
    println!("Rien passé!")
}
    for book in [Book::TheRustPL, Book::ZeroToProd, Book::RustInAction, Book::Other("Truc".into())]  {
        println!("Book {}", disply(&book));
    }
    // let d = distance::Distance::new();
    // println!("distance {}", d.get_metre());

    // // ou
    // let mut d1 = distance::Distance::new_with_value(10);
    // println!("distance {}", d1.get_metre());
    // d1.set_distance(15);
    // println!("distance {}", d1.get_metre());
// struct Point3d {
//     x: i32,
//     y: i32,
//     z: i32,
// }

// let point = Point3d { x: 0, y: 0, z: 0 };
// // et ici on prend x et z de Point3d
// let mut point2 = Point3d { y: 1, .. point };
// println!("({},{},{})", point2.x, point2.y, point2.z);
// point2 = Point3d { y: 1, z: 2 , .. point };
// println!("({},{},{})", point2.x, point2.y, point2.z);



// struct Point {
//     x: i32,
//     y: i32,
// }

// let origin = Point { x: 10, y: 5 };
// println!("({},{})", origin.x, origin.y);
// pattern matching
// match origin {
//     Point { y, .. } => println!("(.., {})", y),
// }
// pattern binding
// let Point { y, .. } = origin;
// println!("( .. , {})", y);

// // Une structure "classique"
// struct Classique {
//     name: String,
//     age: usize,
//     a_un_chat: bool,
// }
// // La structure "classique"
// let mut c = Classique {
//     // On convertit une `&'static str` en `String`
//     name: "Moi".to_owned(),
//     age: 18,
//     a_un_chat: false,
// };
// println!("name : {}, age : {}, a_un_chat : {}", c.name, c.age, c.a_un_chat);
// c.age = 19;
// println!("name : {}, age : {}, a_un_chat : {}", c.name, c.age, c.a_un_chat);



// // Une distance en mètres.
// struct Distance(f32);

// impl Distance {
//     fn to_kilometre(&self) -> f32 {
//         self.0 / 1000.0
//     }
// }

// let distance = Distance(2500.0);

// // On peut récupérer la valeur contenue dans le type de cette façon.
// let Distance(longueur) = distance;
// println!(
//     "La distance est {}m (ou {} km)",
//     longueur,
//     distance.to_kilometre(),
// );

// 'outer désigne le nom ou label de la boucle ci-dessous :
// 'outer: for x in 0..10 {
//     'inner: for y in 0..10 {
//         // on continue la boucle sur x
//         if x*y > 22 { break 'inner; }

//         if x % 2 == 0 { continue 'outer; }

//         // on continue la boucle sur y
//         if y % 2 == 0 { continue 'inner; }

//         println!("x: {}, y: {}", x, y);
//     }
// }
    // for (position, valeur) in (6..10).enumerate() {
    //     println!("position = {} et valeur = {}", position, valeur);
    // }
    // let v = vec![1, 4, 5, 10, 6]; // On crée un vecteur qui contient ces valeurs.

    // for value in v { // Puis on itère sur les valeurs de ce vecteur.
    //     println!("{}", value);
    // }

    // for i in 0..10 {
    //     println!("i vaut : {}", i);
    // }

// boucle_et_print();

    // let mut i : i32 = 0;

    // loop {
    //     if i == 10 {
    //         println!("Fin");
    //         break;
    //     } else {
    //         println!("i {}", i);
    //     }
    //     i += 2;
    // }
    // let mut i : i32 = 0;
    // while i < 10 {
    //     println!("i {}", i);
    //     i += 2;
    // }
    // let my_string = "helloa";

    // let s = match my_string {
    //     "bonjour" => "français",
    //     "ciao" => "italien",
    //     "hello" => "anglais",
    //     "hola" => "espagnol",
    //     _ => "je ne connais pas cette langue..."
    // }; // on met un ';' ici car ce match retourne un type

    // println!("{}", s);
    // let var = test_expression(1);
    // println!("valeur : {}", var);
    // let var: u32 = if 1 > 2 {
    //     1u32
    // } else {
    //     2u32
    // };
    // println!("{}", var);

    // let mut var = 0i32;
    // println!("var : {}", var);
    // let var2 = var = 1i32;
    // println!("var : {}", var);
    // var = 1;
    // println!("var2 : {:?} {}", var2, var);

// use_fct();

if false == true {
patt();

    let my_string = "shello";

    match my_string {
        "bonjour" => println!("français"),
        "ciao" => println!("italien"),
        "hello" => println!("anglais"),
        "hola" => println!("espagnol"),
        _ => println!("je ne connais pas cette langue..."),
    }

    let x=true;
    match x {
        true => println!("OK"),
        false => println!("KO"),
    }

    let x = false;
    if !x {
        println!("OK");
    } else {
        println!("KO");
    }

    println!("Hello, world!");
    let mut i : i32 = 5;
    println!("Valeur {}", i);
    let max: i32 = 10;
    println!("Valeur {}", max);
    i += 1;
    println!("Valeur {}", i);
    if i < max {
        println!("Yes");
    }
    let tab = [3, 2, 0];
    println!("premier élément {}", tab[0]);
    let mut v: Vec<i32> = Vec::new();
    v.push(33);
    v.push(44);
    v.push(55);
    v.push(66);
    v.push(77);
    println!("premier élément {}", v[0]);
    println!("tableau {:?}", v);
    let s = &v[1..3];
    println!("tableau {:?}", s);

}
}