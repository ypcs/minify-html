use ::phf::{phf_set, Set};

// Difference to MDN's inline text semantics list: -br, +del, +ins
static FORMATTING_TAGS: Set<&'static str> = phf_set! {
	"a",
	"abbr",
	"b",
	"bdi",
	"bdo",
	"cite",
	"data",
	"del",
	"dfn",
	"em",
	"i",
	"ins",
	"kbd",
	"mark",
	"q",
	"rp",
	"rt",
	"rtc",
	"ruby",
	"s",
	"samp",
	"small",
	"span",
	"strong",
	"sub",
	"sup",
	"time",
	"u",
	"var",
	"wbr",
};