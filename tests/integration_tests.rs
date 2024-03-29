use simple_string_patterns::{enums::StringBounds, *};

#[cfg(test)]

#[test]
fn test_simple_filter_all() {
  let source_strs = [
    "Cat image",
    "dog picture",
    "elephant image",
    "CAT_Video",
    "cat Picture",
  ];
  let target_strs = [
    "Cat image",
    "cat Picture",
  ];

  let conditions = bounds_builder()
      .starting_with_ci("cat")
      .not_containing_ci("video").as_vec();
  assert_eq!(source_strs.filter_all_conditional(&conditions), target_strs);
}

#[test]
fn test_nested_rules_with_filter_all() {
  let source_strs = [
    "_Cat image.jpg",
    "-dog picture.png",
    "_DOG pc.jpg",
    "elephant image.psd",
    "CAT_Video.mp4",
    "lion Picture.jpg",
  ];
  let target_strs = [
    "_Cat image.jpg",
    "_DOG pc.jpg",
  ];
  let patterns = vec!["cat", "dog"];
  let conditions = bounds_builder()
      .or_starting_with_ci_alphanum(&patterns)
      .ending_with_ci(".jpg");
  assert_eq!(source_strs.filter_all_rules(&conditions), target_strs);
}


#[test]
fn test_nested_rules_with_filter_any() {
  let source_strs = [
    "_Cat image.jpg",
    "-dog picture.png",
    "_DOG pc.jpg",
    "elephant image.psd",
    "CAT_Video.mp4",
    "lion Picture.jpg",
  ];
  let target_strs = [
    "_Cat image.jpg",
    "elephant image.psd",
  ];
  let conditions = bounds_builder()
      .and(
        bounds_builder()
        .starting_with_ci_alphanum("cat")
        .ending_with_ci(".jpg")
      )
      .ending_with_ci(".psd");
  assert_eq!(source_strs.filter_any_rules(&conditions), target_strs);
}



#[test]
fn test_strip_non_chars() {
  let source_str = "Cañon, Zürich, Москва".to_string();
  let target_str = "CañonZürichМосква".to_string();
  assert_eq!(source_str.strip_non_alphanum(),target_str );
}

#[test]
fn test_segment_match() {
  let path_string = "/var/www/mysite.com/web/uploads";
  // ignore leading slash
  let domain = path_string.to_segment("/",2).unwrap_or("".to_string()); 
  let expected_string_1 = "mysite.com".to_string();
  assert_eq!(domain, expected_string_1);
  let part = path_string.to_part("/",2).unwrap_or("".to_string()); 
  let expected_string_2 = "www".to_string();
  assert_eq!(part, expected_string_2);
}

#[test]
fn test_to_string_vector() {
  // ignore leading slash
  let path_string = "/var/www/mysite.com/web/uploads";
  let segments = path_string.to_segments("/"); 
  assert_eq!(segments.len(), 5);
  let fourth_element = segments.get(3).unwrap().to_owned();
  let expected_string = "web".to_owned();
  assert_eq!(fourth_element, expected_string);
}

#[test]
fn test_to_segments() {
  let path_string = "/var/www/mysite.com/web/uploads/";
  // should extract only non-empty segments
  let segments = path_string.to_segments("/"); 
  let expected_segments = ["var", "www", "mysite.com", "web", "uploads"].to_strings();
  assert_eq!(segments, expected_segments);
  // convert all parts split by a separator whether empty or not
  let parts = path_string.to_parts("/"); 
  let expected_parts = ["", "var", "www", "mysite.com", "web", "uploads", ""].to_strings();
  assert_eq!(parts, expected_parts);
}

#[test]
fn test_to_tail() {
  let source_str = "long/path/with-a-long-title/details";
  let target_str = "long".to_string();
  assert_eq!(source_str.to_inner_segment(&[("/", 2), ("-", 2)]), Some(target_str) );
}

#[test]
fn test_to_inner_segment() {
  let source_str = "long/path/with-a-long-title/details";
  let target_str = "long".to_string();
  assert_eq!(source_str.to_inner_segment(&[("/", 2), ("-", 2)]), Some(target_str) );
  let source_str2 = "complex/pattern/with-many-nested|embedded-words";
  let target_str2 = "embedded".to_string();
  let pairs = [("/", 2), ("-", 2), ("|", 1)];
  assert_eq!(source_str2.to_inner_segment(&pairs), Some(target_str2) );

 let groups = [("/", 1), ("-", 2)];
 let file_path = "pictures/holiday-france-1983/originals";
 let current_year: i32 = 2024;  
 let invalid_age: i32 = 0;
 let expected_age: i32 = 41;
 let matched_year = if let Some(year_string) = file_path.to_inner_segment(&groups) {
  // only parse age if matched, standard parse() is fine, but to_first_number() will strip any characters before or after the first number.
  year_string.parse::<i32>().unwrap_or(invalid_age)
 } else {
  invalid_age
 }; // should yield 1983
 assert_eq!(current_year - matched_year, expected_age);
}

#[test]
fn test_to_first() {
  let source_str = "/path/with/a/leading/slash";
  let target_str = "path".to_string();
  assert_eq!(source_str.to_first("/"), target_str );
  let source_str2 = "path/without/a/leading/slash";
  assert_eq!(source_str2.to_first("/"), target_str );
}

#[test]
fn test_to_last() {
  let source_str = "/path/with/a/trailing/slash/";
  let target_str = "slash".to_string();
  assert_eq!(source_str.to_last("/"), target_str );
  let source_str2 = "/path/without/a/trailing/slash";
  assert_eq!(source_str2.to_last("/"), target_str );
}

#[test]
fn test_to_head_tail() {
  let source_str = "comma,separated,string";
  let start = "comma";
  let end = "separated,string";
  assert_eq!(source_str.to_head_tail(","), (start.to_string(), end.to_string()) );
  let head = source_str.to_head(",");
  assert_eq!(head, start.to_string() );

  let end_part = source_str.to_end(",");
  assert_eq!(end_part, "string".to_string() );
}

#[test]
fn test_to_start_end() {
  let source_str = "comma,separated,string";
  let start = "comma,separated".to_string();
  let end = "string".to_string();
  assert_eq!(source_str.to_start_end(","), (start, end) );
  let source_str = "one-item".to_string();
  let empty_end = "".to_string();
  assert_eq!(source_str.to_start_end(","), (source_str, empty_end) );
}

#[test]
fn test_array_str_to_vec_string() {
  let source_strs = [
    "one",
    "two",
    "three"
  ].to_strings();
  let target_vec = [
    "one",
    "two",
    "three"
  ].to_strings();
  assert_eq!(source_strs, target_vec );
}

#[test]
fn test_char_group_matches() {
  let str1 = "I spent £12.50 on wine".to_string();

  assert!(str1.has_alphabetic());

  assert!(str1.has_digits());
  let str2 = "I bought a bottle of champagne for twenty pounds".to_string();
  // Deoes not contain digits
  assert!(str2.has_digits() == false);

  let str3 = "{-; _)(:-)}".to_string();
  // Does not contain letters os numbers
  assert!(str3.has_alphanumeric() == false);
  
}

#[test]
fn test_simple_pattern_matches() {
  let str1 = "Picture_of my cat-2018.PNG";

  let pattern_1 = "pictureof";
  assert!(str1.starts_with_ci_alphanum(pattern_1));

  let pattern_2 = "mycat";
  assert!(str1.contains_ci_alphanum(pattern_2));

  // Ends with .png with upper, lower or mixed case letters
  assert!(str1.ends_with_ci(".png"));
  
}

#[test]
fn test_is_numeric() {
  let num_str_1 = "-1227.75";
  assert!(num_str_1.is_numeric());
  
  let num_str_2 = "-1,227.75"; // will not validate with commas, unless corrected
  assert_eq!(num_str_2.is_numeric(), false);
  // &str has to be cast to an owned String first
  assert!(num_str_2.correct_numeric_string(false).is_numeric());

  let num_str_3 = "-1.227,75"; // European-style with commas as decimal separators
  assert!(num_str_3.correct_numeric_string(true).is_numeric());

  let num_str_4 = "$19.99 each"; // Should fail, as this will not parse directly to a float
  assert!(!num_str_4.is_numeric());
}

#[test]
fn test_has_digits() {
  // Does this have a valid decimal digit sequence that may be extracted as a valid number
  let num_str_1 = "serial number: 93025371";
  assert!(num_str_1.has_digits());

  // Is this a valid decimal digit sequence that may be cast to an integer
  let num_str_1 = "93025371";
  assert!(num_str_1.is_digits_only());
  
  // Is this a valid hexadecimal string that may be cast to a float via from_str_radix(16)
  let num_str_2 = "1ec9F9a";
  assert!(num_str_2.is_digits_only_radix(16));
}

#[test]
fn test_match_ocurrences() {
  // As this works on literal strs/Strings only it may only match a set number of characters
  let str = "The fox jumped out of the box into the mixing bowl.";
  
  let x_indices = str.find_matched_indices("x");
  let expected_x_indices: Vec<usize> = vec![6, 28, 41];
  assert_eq!(x_indices, expected_x_indices);

  let ox_indices = str.find_matched_indices("ox");
  let expected_ox_indices: Vec<usize> = vec![5, 27];
  assert_eq!(ox_indices, expected_ox_indices);
}

#[test]
fn test_strip_non_numeric() {
  let source_str = "I spent £9999.99 on 2 motorbikes at the age of 72.";
  let target_str = "9999.99 2 72".to_string();
  assert_eq!(source_str.strip_non_numeric(), target_str);

  
  let target_str = "Ispent999999on2motorbikesattheageof72".to_string();
  assert_eq!(source_str.strip_non_alphanum(), target_str);
  // check if ythe above numbers parse successfully to numbers
  assert_eq!(source_str.to_numbers::<f64>(), vec![9999.99f64, 2f64, 72f64]);

  assert_eq!(source_str.to_first_number::<f32>().unwrap_or(0f32), 9999.99f32);

  let input_text = "I'd like 2.5lb of flour please";

  assert_eq!(input_text.to_first_number::<f32>().unwrap_or(0f32), 2.5f32);
  
  // Standard European price format. This is not ambiguous because both a dot and comma are both present
  let input_text = "Il conto è del 1.999,50€. Come vuole pagare?";
  assert_eq!(input_text.to_first_number::<f32>().unwrap_or(0f32), 1999.5f32);

  // Rounded amount in the European format. The absence of a secondary separator makes this
  // value ambigiuous
  let input_text = "Il furgone pesa 1.500kg";
  assert_eq!(input_text.to_first_number_euro::<u32>().unwrap_or(0), 1500);
}

#[test]
fn test_correct_floats() {
  let source_str = "Ho pagato 15,00€ per l'ingresso.".to_string();
  // with numbers that can be corrected parsed and cast to floats
  let target_str = "Ho pagato 15.00€ per l'ingresso.".to_string(); 
  // Correct a euro-style number and always interpret commas as decimal separators.
  assert_eq!(source_str.correct_numeric_string(true), target_str);

  let source_str_2 = "Pesa 1.678 grammi".to_string(); 
  let target_str_2 = "Pesa 1678 grammi".to_string(); // do not use in longer phrases with commas and dots as punctuation
  // Correct a euro-style number and always interpret commas as decimal separators.
  assert_eq!(source_str_2.correct_numeric_string(true), target_str_2);

  let sample_str = "Ho pagato 12,50€ per 1.500 grammi di sale.".to_string();
  // with numbers that can be corrected parsed and cast to floats
  let target_numbers = vec![12.5f32, 1500f32]; 
  // Correct euro-style numbers and convert to 32-bit floats
  assert_eq!(sample_str.to_numbers_euro::<f32>(), target_numbers);
}

#[test]
fn test_matched_conditional() {
  let conditions = [
    StringBounds::StartsWith("jan", true, CaseMatchMode::Insensitive),
    StringBounds::EndsWith("images", true, CaseMatchMode::Insensitive),
    StringBounds::Contains("2023", true, CaseMatchMode::Insensitive),
  ];

  let folder_1 = "Jan_2023_IMAGES";

  let folder_2 = "january_2024_Images";

  assert_eq!(folder_1.matched_conditional(&conditions), vec![true, true, true]);

  assert!(folder_1.match_all_conditional(&conditions));

  assert_eq!(folder_2.matched_conditional(&conditions), vec![true, true, false]);

  let test_strs = ["image", "cat", "garden"];

  let folder_3 = "cat-IMAGES_Garden";
  let folder_4 = "images-of-cats-and-dogs-in-the-park";

  assert!(folder_3.contains_all_conditional_ci(&test_strs));
  // the second folder should not match all conditions
  assert_eq!(folder_4.contains_all_conditional_ci(&test_strs), false);

  let file_names = [
    "edited-img-Nepal-Feb-2003.psd",
    "image-Thailand-Mar-2003.jpg",
    "photo_Nepal_Jan-2005.jpg",
    "image-India-Mar-2003.jpg",
    "pic_nepal_Dec-2004.png"
  ];


  let mixed_conditions = bounds_builder()
    .containing_ci("nepal")
    .not_ending_with_ci(".psd")
    .as_vec();

  let file_name_a = file_names[0];
  let file_name_b = file_names[2];

  assert!(file_name_a.match_all_conditional(&mixed_conditions) == false);

  assert!(file_name_b.match_all_conditional(&mixed_conditions));
  
  let nepal_jpg_files: Vec<&str> = file_names.filter_all_conditional(&mixed_conditions);

  assert_eq!(nepal_jpg_files.len(), 2);

  assert_eq!(nepal_jpg_files[0], file_name_b);

  let file_names_vector = file_names.to_strings();

  let nepal_jpg_files_vector: Vec<String> = file_names_vector.filter_all_conditional(&mixed_conditions);
  
  assert_eq!(nepal_jpg_files_vector.len(), 2);
  
  assert_eq!(nepal_jpg_files_vector.len(), 2);

}

#[test]
fn test_matched_any_conditional() {
  let false_conditions = [
    StringBounds::Whole("no", true, CaseMatchMode::Insensitive),
    StringBounds::Whole("false", true, CaseMatchMode::Insensitive),
    StringBounds::Contains("not", true, CaseMatchMode::Insensitive),
    StringBounds::Contains("negative", true, CaseMatchMode::Insensitive),
  ];

  let boolean_strs_1 = [
    "NO",
    "FALSE",
    "not at all",
    "negative result",
    "Noon"
  ];

  let falsy_strs = [
    "NO",
    "FALSE",
    "not at all",
    "negative result"
  ];

  assert_eq!(boolean_strs_1.filter_any_conditional(&false_conditions), falsy_strs);

  let true_conditions = bounds_builder()
    .is_ci("yes")
    .is_ci("y")
    .starting_with_ci("ok")
    .containing_ci("positive")
    .as_vec();
  let boolean_strs_2 = [
    "Yes",
    "y",
    "Yep",
    "okay",
    "positive result",
    "good"
  ];

  let truthy_strs = [
    "Yes",
    "y",
    "okay",
    "positive result"
  ];

  assert_eq!(boolean_strs_2.filter_any_conditional(&true_conditions), truthy_strs);

}

#[test]
fn test_enclose_in_chars() {
  // As this works on literal strs/Strings only it may only match a set number of characters
  let sample_str = "purple";
  
  let out_str_1 = "(purple)";

  let out_str_2 = "<purple>";

  let out_str_3 = "‟purple„";

  let out_str_4 = "(?=purple)";

  assert_eq!(sample_str.parenthesize(), out_str_1.to_owned());

  assert_eq!(sample_str.wrap('<'), out_str_2.to_owned());

  assert_eq!(sample_str.enclose('‟', '„'), out_str_3.to_owned());

  assert_eq!(sample_str.in_parentheses(Some("?=")), out_str_4.to_owned());

}

#[test]
fn test_enclose_escaped_in_chars() {
  // As this works on literal strs/Strings only it may only match a set number of characters
  let sample_str = r#"Tom whispered "I love you" as he gazed into Jennifer's eyes only inches away."#;
  
  let expected_quoted_str = r#""Tom whispered \"I love you\" as he gazed into Jennifer's eyes only inches away.""#;

  assert_eq!(sample_str.double_quotes_safe(), expected_quoted_str.to_owned());


  let sample_str_2 = r#"Bee's wax and \'organic honey\'"#;

  let expected_quoted_str_2 = r#"'Bee\'s wax and \'organic honey\''"#;

  assert_eq!(sample_str_2.single_quotes_safe(), expected_quoted_str_2.to_owned());

  let sample_str_3 = r#"She wrote "From Antarctica with a Cold Heart""#;

  let expected_quoted_str_3 = r#""She wrote ""From Antarctica with a Cold Heart""""#;

  assert_eq!(sample_str_3.wrap_escaped('"', Some('"')), expected_quoted_str_3.to_owned());

}

#[test]
fn test_filter_by_character_type() {
  // Nonsense text with miscellaneous letters, numbers and punctuation
  let sample_str = "Products: $9.99 per unit, £19.50 each, €15 only. Zürich café cañon";
  
  let vowels_only = sample_str.filter_by_type(CharType::Chars(&['a','e','i','o', 'u', 'é', 'ü', 'y']));
  let expected_vowel_sequence = "oueuieaoyüiaéao";
  assert_eq!(vowels_only, expected_vowel_sequence);

  let lower_case_letters_a_to_m_only = sample_str.filter_by_type(CharType::Range('a'..'m'));
  let expected_letter_sequence = "dceieachlichcafca";
  assert_eq!(lower_case_letters_a_to_m_only, expected_letter_sequence);

  let sample_without_punctuation = sample_str.strip_by_type(CharType::Punctuation);
  let expected_string = "Products 999 per unit £1950 each €15 only Zürich café cañon";
  assert_eq!(sample_without_punctuation, expected_string);

  let sample_without_spaces_and_punct = sample_str.strip_by_types(&[CharType::Spaces, CharType::Punctuation]);
  let expected_string = "Products999perunit£1950each€15onlyZürichcafécañon";
  assert_eq!(sample_without_spaces_and_punct, expected_string);

  let sample_with_upper_case_accented_chars = sample_str.filter_by_types(&[CharType::Upper, CharType::Chars(&['é', 'ü'])]);
  let expected_string = "PZüé";
  assert_eq!(sample_with_upper_case_accented_chars, expected_string);

  let sample_with_lower_case_chars_and_spaces = sample_str.filter_by_types(&[CharType::Lower, CharType::Spaces]);
  let expected_string = "roducts  per unit  each  only ürich café cañon";
  assert_eq!(sample_with_lower_case_chars_and_spaces, expected_string);


  let sample_str_2 = "Orange: #ff9900";
  let hexadecimal_digits_only = sample_str_2.filter_by_type(CharType::Digit(16));
  let expected_letter_sequence = "aeff9900";
  assert_eq!(hexadecimal_digits_only, expected_letter_sequence);

}

#[test]
fn test_split_on_characters() {
  // Sample string with different, but predictable seprators
  let sample_strs = [
    "jazz-and-blues-music",
    "rock_and_roll-music",
    "classical music-hall"
  ];

  let split_chars = ['-','_', ' '];
  let words = sample_strs.into_iter().map(|s| s.split_on_any_char(&split_chars)).collect::<Vec<Vec<String>>>();
  let first_line_words = words.get(0);
  assert_eq!(first_line_words.unwrap().last().unwrap().to_owned(), "music".to_owned());

  let last_line_words = words.last();
  assert_eq!(last_line_words.unwrap().get(0).unwrap().to_owned(), "classical".to_owned());

  let sample_str = "jazz-and-blues_music/section";
  let parts = sample_str.split_on_any_char(&['-','_', '/']);
  let expected_parts = ["jazz", "and", "blues", "music", "section"].to_strings();
  assert_eq!(parts, expected_parts);

}

#[test]
fn test_bounds_builder() {
  // Nonsense text with miscellaneous letters, numbers and punctuation
  let rules = bounds_builder()
      .starting_with_ci("cat")
      .not_ending_with_ci(".jpg");
  
  let sample_strs = [
    "cat-picture.jpg",
    "Dog-picture.png",
    "CAT-image.png",
    "rabbit-photo.png",
    "cAt-pic.webp"
  ];

  let filtered_lines = sample_strs.filter_all_rules(&rules);
  let expected_lines = vec![
    "CAT-image.png",
    "cAt-pic.webp"
  ];
  assert_eq!(filtered_lines, expected_lines);

  let rules_2 = bounds_builder()
  .starting_with_ci("cat")
  .or_ending_with_ci(&[".jpg", ".png"]);

  let filtered_lines = sample_strs.filter_all_rules(&rules_2);
  let expected_lines = vec![
    "cat-picture.jpg",
    "CAT-image.png",
  ];
  assert_eq!(filtered_lines, expected_lines);

}