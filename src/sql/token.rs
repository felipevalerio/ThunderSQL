struct Symbol {
	name: String,
	length: u32,
	token: Token,
	group: Group
}


enum Group {
	Keyword,
	Function
}


enum Token {
	
}