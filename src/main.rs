fn delete_last_x(s: std::string::String, x: usize) -> std::string::String {
	let slice: &str = s.as_ref();
    let length = slice.chars().count();
    let max_chars = length - x;
    match slice.char_indices().nth(max_chars) {
        None => slice.to_string(),
        Some((idx, _)) => slice[..idx].to_string(),
    }
}

#[test]
fn test_delete_last_x() {
	assert_eq!(delete_last_x("abcd".to_string(), 2), "ab".to_string());
}


fn find_last_x(s: std::string::String, x: usize) -> std::string::String {
	let slice: &str = s.as_ref();
	let length = slice.chars().count();
	let max_chars = length - x;
	match slice.char_indices().nth(max_chars) {
		None => slice.to_string(),
		Some((idx, _)) => slice[idx..].to_string(),
	}
}

#[test]
fn test_find_last_x() {
	assert_eq!(find_last_x("abcd".to_string(), 2), "cd".to_string());
}


fn replace_last_x(s: std::string::String, x: usize, replace: &str) -> std::string::String {
	let mut new_s: std::string::String = (delete_last_x(s, x)).to_string();
	new_s.push_str(replace);
	new_s
}

#[test]
fn test_replace_last_x() {
	assert_eq!(replace_last_x("abcd".to_string(), 2, "x"), "abx".to_string());
}


fn remove_possessives(s: std::string::String) -> std::string::String {
	let slice: &str = s.as_ref();
	let length = slice.chars().count() as u8;

	match length > 5 {
		true => match find_last_x(slice.to_string(), 2).as_ref() {
			"ov" | "ův" => return delete_last_x(slice.to_string(), 2), 
			"in" => return palatalise(delete_last_x(slice.to_string(), 1)),
			_ => return slice.to_string()
		},
		false => return slice.to_string()
	}
}

#[test]
fn test_remove_possessives() {
	assert_eq!(remove_possessives("karlův".to_string()), "karl".to_string());
	assert_eq!(remove_possessives("matčin".to_string()), "matk".to_string());
	assert_eq!(remove_possessives("wtf".to_string()), "wtf".to_string());
	assert_eq!(remove_possessives("wtfwtf".to_string()), "wtfwtf".to_string());
}


fn remove_comparatives(s: std::string::String) -> std::string::String {
	let slice: &str = s.as_ref();
	let length = slice.chars().count() as u8;

	match length > 5 {
		true => match find_last_x(slice.to_string(), 3).as_ref() {
			"ejš" | "ějš" => return palatalise(delete_last_x(slice.to_string(), 2)),
			_ => return slice.to_string()
		},
		false => return slice.to_string()
	}
}

#[test]
fn test_remove_comparatives() {
	assert_eq!(remove_comparatives("nejvejš".to_string()), "nejv".to_string());
	assert_eq!(remove_comparatives("wtfwtf".to_string()), "wtfwtf".to_string());
}


fn remove_augmentative(s: std::string::String) -> std::string::String {
	let slice: &str = s.as_ref();
	let length = slice.chars().count() as u8;

	let slice = match length > 6 && slice.ends_with("ajzn") {
		true => return delete_last_x(slice.to_string(), 4), 
		false => slice
	};
	let slice = match length > 5 && 
	(slice.ends_with("izn") || slice.ends_with("isk")) {
		true => return palatalise(delete_last_x(slice.to_string(), 2)), 
		false => slice
	};
	let slice = match length > 4 && slice.ends_with("ák") {
		true => return delete_last_x(slice.to_string(), 2), 
		false => slice.to_string()
	};
	slice.to_string()
}

#[test]
fn test_remove_augmentative() {
	assert_eq!(remove_augmentative("xxxajzn".to_string()), "xxx".to_string());
	assert_eq!(remove_augmentative("cizák".to_string()), "ciz".to_string());
}


fn remove_diminutives(s: std::string::String) -> std::string::String {
	let slice: &str = s.as_ref();
	let length = slice.chars().count() as u8;

	let slice = match length > 7 && slice.ends_with("oušek") {
		true => return delete_last_x(slice.to_string(), 5),
		_ => slice 
	};
	let slice = match length > 6 {
		true => match find_last_x(slice.to_string(), 4).as_ref() {
			"eček" | "éček" | "iček" | "íček" | 
			"enek" | "ének" | "inek" | "ínek" => return palatalise(delete_last_x(slice.to_string(), 3)),
			"áček" | "aček" | "oček" | "uček" | 
			"anek" | "onek" | "unek" | "ánek" => return delete_last_x(slice.to_string(), 4),
			_ => slice
		},
		_ => slice
	};
	let slice = match length > 5 {
		true => match find_last_x(slice.to_string(), 3).as_ref() {
			"ečk" | "éčk" | "ičk" | "íčk" | 
			"enk" | "énk" | "ink" | "ínk" => return palatalise(delete_last_x(slice.to_string(), 3)),
			"áčk" | "ačk" | "očk" | "učk" | 
			"ank" | "onk" | "unk" => return delete_last_x(slice.to_string(), 3),
			"átk" | "ánk" | "ušk" => return delete_last_x(slice.to_string(), 3), 
			_ => slice 
		},
		_ => slice
	};
	let slice = match length > 4 {
		true => match find_last_x(slice.to_string(), 2).as_ref() {
			"ek" | "ék" | "ík" | "ik" => return palatalise(delete_last_x(slice.to_string(), 1)),
			"ák" | "ak" | "ok" | "uk" => return delete_last_x(slice.to_string(), 1),
			_ => slice
		}, 
		_ => slice
	};
	match length > 3 && slice.ends_with("k") {
		true => return delete_last_x(slice.to_string(), 1), 
		_ => return slice.to_string()
	}
}

#[test]
fn test_remove_diminutives() {
	assert_eq!(remove_diminutives("fanoušek".to_string()), "fan".to_string());
	assert_eq!(remove_diminutives("pohank".to_string()), "poh".to_string());
}


fn remove_derivational(s: std::string::String) -> std::string::String {
	let slice: &str = s.as_ref();
	let length = slice.chars().count() as u8;

	let slice = match length > 8 && slice.ends_with("obinec") {
		true => return delete_last_x(slice.to_string(), 6), 
		_ => slice
	};
	let slice = match length > 7 {
		true => match find_last_x(slice.to_string(), 5).as_ref() {
			"ionář" => return palatalise(delete_last_x(slice.to_string(), 4)),
			"ovstv" | "ovišt" | "ovník" => return delete_last_x(slice.to_string(), 5),
			_ => slice
		},
		false => slice
	};
	let slice = match length > 6 {
		true => match find_last_x(slice.to_string(), 4).as_ref() {
			"ásek" | "loun" | "nost" | "teln" | "ovec" |
			"ovík" | "ovtv" | "ovin" | "štin" => return delete_last_x(slice.to_string(), 4),
			"enic" | "inec" | "itel" => return palatalise(delete_last_x(slice.to_string(), 3)),
			_ => slice
		},
		false => slice
	};
	let slice = match length > 5 {
		true => match find_last_x(slice.to_string(), 3).as_ref() {
			"árn" => return delete_last_x(slice.to_string(), 3),
			"ěnk" | "ián" | "ist" | "isk" | "išt" | 
			"itb" | "írn" => return palatalise(delete_last_x(slice.to_string(), 2)),
			"och" | "ost" | "ovn" | "oun" | "out" | 
			"ouš" | "ušk" => return delete_last_x(slice.to_string(), 3),
			"kyn" | "čan" | "kář" | "néř" | "ník" | 
			"ctv" | "stv" => return delete_last_x(slice.to_string(), 3),
			_ => slice
		}, 
		_ => slice
	};
	let slice = match length > 4 {
		true => match find_last_x(slice.to_string(), 2).as_ref() {
			"áč" | "ač" | "án" | "an" | 
			"ár" | "as" => return delete_last_x(slice.to_string(), 2),
			"ec" | "en" | "ěn" | "éř" => return palatalise(delete_last_x(slice.to_string(), 1)),
			"íř" | "ic" | "in" | "ín" |
			"it" | "iv" => return palatalise(delete_last_x(slice.to_string(), 1)),
			"ob" | "ot" | "ul" | "yn" |
			"ov" | "oň" | "čk" | "čn" | 
			"dl" | "nk" | "tv" | "tk" | 
			"vk" => return delete_last_x(slice.to_string(), 2),
			_ => slice
		},
		_ => slice
	};
	let slice = match length > 3 {
		true => match find_last_x(slice.to_string(), 1).as_ref() {
			"c" | "č" | "k" | 
			"l" | "n" | "t" => return delete_last_x(slice.to_string(), 1),
			_ => slice
		}, 
		_ => slice
	};
	slice.to_string()
}


#[test]
fn test_remove_derivational() {
	assert_eq!(remove_derivational("chudobinec".to_string()), "chud".to_string());
	assert_eq!(remove_derivational("panovník".to_string()), "pan".to_string());
	assert_eq!(remove_derivational("zevloun".to_string()), "zev".to_string());
	assert_eq!(remove_derivational("pěstírn".to_string()), "pěst".to_string());

}


fn remove_case(s: std::string::String) -> std::string::String {
	let slice: &str = s.as_ref();
	let length = slice.chars().count() as u8;

	let slice = match length > 7 && slice.ends_with("atech") {
		true => return delete_last_x(slice.to_string(), 5),
		false => slice
	};
	let slice = match length > 6 {
		true => match find_last_x(slice.to_string(), 4).as_ref() {
			// -tady imho něco chybí a daly by se sem napasovat další přípony (e.g. zvířetem)
			"ětem" => return palatalise(delete_last_x(slice.to_string(), 3)), // then palatalise
			"atům" => return delete_last_x(slice.to_string(), 4),
			_ => slice
		},
		false => slice
	};
	let slice = match length > 5 {
		true => match find_last_x(slice.to_string(), 3).as_ref() {
			"ech" | "ich" | "ích" => return palatalise(delete_last_x(slice.to_string(), 2)), //then palatalise
			"ého" | "emi" | "ému" | 
			"eti" | "iho" | "ího" | 
			"ími" | "imu" => return palatalise(delete_last_x(slice.to_string(), 2)), //then palatalise,
			"ách" | "ata" | "aty" |
			"ých" | "ama" | "ami" |
			"ové" | "ovi" | "ými" => return delete_last_x(slice.to_string(), 3),
			_ => slice
		},
		false => slice
	};
	let slice = match length > 4 {
		true => match find_last_x(slice.to_string(), 2).as_ref() {
			"em" => return palatalise(delete_last_x(slice.to_string(), 1)), //palatalise
			"es" | "ém" | "ím" => return palatalise(delete_last_x(slice.to_string(), 2)), //palatalise
			"ům" => return delete_last_x(slice.to_string(), 2), 
			"at" | "ám" | "os" | "ou" | 
			"us" | "ým" | "mi" => return delete_last_x(slice.to_string(), 2),
			_ => slice
		},
		false => slice
	};
	let slice = match length > 3 {
		true => match find_last_x(slice.to_string(), 1).as_ref() {
			"e" | "i" | "í" | "ě" => return palatalise(slice.to_string()), //palatalise
			"u" | "y" | "ů" | 
			"a" | "á" | "o" | 
			"ý" | "é" => return delete_last_x(slice.to_string(), 1),
			_ => slice
		}, 
		false => slice
	};
	slice.to_string()
}

#[test]
fn test_remove_case() {
	assert_eq!(remove_case("zvířatech".to_string()), "zvíř".to_string());
	assert_eq!(remove_case("zvířatům".to_string()), "zvíř".to_string());
	assert_eq!(remove_case("zvířaty".to_string()), "zvíř".to_string());
	assert_eq!(remove_case("zvířat".to_string()), "zvíř".to_string());
	assert_eq!(remove_case("zvíx".to_string()), "zvíx".to_string());
}

fn palatalise(s: std::string::String) -> std::string::String {
	let slice: &str = s.as_ref();
 	let slice = match find_last_x(slice.to_string(), 2).as_ref() {
 		"ci" | "ce" | "či" | "če" => return replace_last_x(slice.to_string(), 2, "k"),
 		"zi" | "ze" | "ži" | "že" => return replace_last_x(slice.to_string(), 2, "h"),
 		_ => slice
 	};
 	match find_last_x(slice.to_string(), 3).as_ref() {
 		"čtě" | "čti" | "čtí" => return replace_last_x(slice.to_string(), 3, "ck"),
 		"ště" | "šti" | "ští" => return replace_last_x(slice.to_string(), 3, "sk"),
 		_ => return delete_last_x(slice.to_string(), 1)
 	}
}

#[test]
fn test_palatalise() {
	assert_eq!(palatalise("ptáci".to_string()), "pták");
	assert_eq!(palatalise("němečtí".to_string()), "německ");
}


fn main() {
	let words: Vec<_> = std::env::args().collect();
	for word in words.iter() {
		let output = remove_derivational(remove_augmentative(remove_diminutives(remove_comparatives(remove_possessives(remove_case(word.to_lowercase().to_string()))))));
		println!("{}", output);
	}
}
