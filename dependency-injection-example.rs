// Our "extractors"
struct Name(String);
struct Age(u32);
struct Country(String);

// Our request type
struct Request {
    name: String,
    age: u32,
    country: String,
}

// Trait for types that can be extracted from our "request"
trait FromRequest {
    fn extract(request: &Request) -> Self;
}

// Implement extraction for each type
impl FromRequest for Name {
    fn extract(request: &Request) -> Self {
        Name(request.name.clone())
    }
}

impl FromRequest for Age {
    fn extract(request: &Request) -> Self {
        Age(request.age)
    }
}

impl FromRequest for Country {
    fn extract(request: &Request) -> Self {
        Country(request.country.clone())
    }
}

// Handler with one parameter
fn handler1(name: Name) {
    println!("Handler1: Name: {}", name.0);
}

// Handler with two parameters in different orders
fn handler2a(name: Name, age: Age) {
    println!("Handler2a: Name: {}, Age: {}", name.0, age.0);
}

fn handler2b(age: Age, name: Name) {
    println!("Handler2b: Name: {}, Age: {}", name.0, age.0);
}

// Handler with three parameters
fn handler3(country: Country, name: Name, age: Age) {
    println!("Handler3: Name: {}, Age: {}, Country: {}", name.0, age.0, country.0);
}

// Generic processors for different numbers of parameters
fn process_request1<F, T1: FromRequest>(
    request: &Request,
    handler: F,
)
where
    F: Fn(T1),
{
    let param1 = T1::extract(request);
    handler(param1);
}

fn process_request2<F, T1: FromRequest, T2: FromRequest>(
    request: &Request,
    handler: F,
)
where
    F: Fn(T1, T2),
{
    let param1 = T1::extract(request);
    let param2 = T2::extract(request);
    handler(param1, param2);
}

fn process_request3<F, T1: FromRequest, T2: FromRequest, T3: FromRequest>(
    request: &Request,
    handler: F,
)
where
    F: Fn(T1, T2, T3),
{
    let param1 = T1::extract(request);
    let param2 = T2::extract(request);
    let param3 = T3::extract(request);
    handler(param1, param2, param3);
}

fn main() {
    let request = Request {
        name: String::from("Alice"),
        age: 30,
        country: String::from("Wonderland"),
    };

    // One parameter
    process_request1(&request, handler1);

    // Two parameters in different orders
    process_request2(&request, handler2a);
    process_request2(&request, handler2b);

    // Three parameters
    process_request3(&request, handler3);
}