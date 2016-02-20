pub type Token = String;
pub type TokenList = Vec<Token>;

pub fn tokenize<'a>(input: &str) -> TokenList {
    let mut out = TokenList::new();

    out.push(input.to_string());

    out = split_on_spaces(out);

    out
}

pub fn split_on_spaces<'a>(input: TokenList) -> TokenList {
    let mut out = TokenList::new();

    for token in input {
        let split = token.split(" ");
        for item in split {
            out.push(item.to_string());
        }
    }

    out
}

#[test]
fn test_with_good_muffins() {
  let tokens = tokenize("Good muffins cost $3.88\\nin New York.  Please buy me\\ntwo of them.\\nThanks.");
  assert_eq!(tokens, vec!["Good", "muffins", "cost", "$3.88\\nin", "New", "York.", "", "Please", "buy", "me\\ntwo", "of", "them.\\nThanks."]);
}
