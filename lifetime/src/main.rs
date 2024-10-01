struct ImportantExcerpt<'a> {
	part: &'a str,
}

// 메서드 라이프타임
impl<'a> ImportantExcerpt<'a> {
    // 라이프타임 생략 규칙
    fn level(&self) -> i32 {
        3
    }    
}

impl<'a> ImportantExcerpt<'a> {
    // fn announce_and_return_part<'b, 'c>(&'b self, announcement: &'c str) -> &'b str {
    fn announce_and_return_part(&self, announcement: &str) -> &str { //< 라이프타임 생략규칙
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if y.len() < x.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

	let novel = String::from("Call me Ishmael. Some years ago...");
	let first_sentence = novel.split('.').next().expect("Could not find a '.'");
	let i = ImportantExcerpt {
		part:first_sentence,
	};
	println!("{}", i.part);

	// 라이프타임 오류
	// let i;
	// {
	// 	let novel = String::from("Call me Ishmael. Some years ago...");
	// 	let first_sentence = novel.split('.').next().expect("Could not find a '.'");
	// 	i = ImportantExcerpt {
	// 		part:first_sentence,
	// 	};
	// }
	// println!("{}", i.part);
}