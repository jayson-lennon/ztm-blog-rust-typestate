#![allow(dead_code)]
#![allow(unused_variables)]

// We have a toy program that takes a string, splits it into words, and then formats the words
// in a specific way. Typestate is used to keep the data flowing in a single direction and
// to prevent mistakes at each step along the way.

// type alias for clarity
type Word = &'static str;

// States:
// - RawData
// - ParsedData
// - FormattedData
//
// Transitions:
// - RawData -> ParsedData
// - ParsedData -> FormattedData
//
// All these fields are private and cannot be accessed outside this module.
struct RawText(&'static str);
struct ParsedText(Vec<Word>);
struct FormattedText(String);

impl RawText {
    // The only way to create a RawData is through this function.
    pub fn new(raw: &'static str) -> Self {
        Self(raw)
    }

    // The `parse` function transforms the raw data into some parsed data. Nothing
    // special is happening yet.
    pub fn parse(self) -> ParsedText {
        let parsed = self.0.split(' ').collect();
        ParsedText(parsed)
    }
}

impl ParsedText {
    // The only function we have is the `format` function, which formats the parsed
    // data in some way. Since the inner data is private, the only way to create
    // a `ParsedData` is through the `parse` function on `RawData`. This is how the
    // state transitions are created.
    pub fn format(self) -> FormattedText {
        FormattedText(self.0.join("."))
    }
}

impl FormattedText {
    // Get the formatted data as a &str. This function is only defined on `FormattedData`.
    // So there is no way to access an incomplete/raw/parsed data. We can only access the
    // desired formatted data, and only after it has gone through the required state
    // transitions to get to the `FormattedData` state.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

fn main() {
    // We start off with some raw data
    let raw: RawText = RawText::new("this is some data");

    // We then parse it. The raw data is no longer available because we moved it
    // into `parsed`.
    let parsed: ParsedText = raw.parse();

    // Once parsed, we format it. Just like the raw data, we cannot access the parsed
    // data because it was moved into `formatted`.
    let formatted: FormattedText = parsed.format();

    // We get a &str from the formatted data.
    let formatted: &str = formatted.as_str();

    assert_eq!(formatted, "this.is.some.data");
}
