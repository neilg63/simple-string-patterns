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
  let conditions = [
    StringBounds::StartsWithCi("cat", true),
    StringBounds::ContainsCi("video", false)
  ];
  assert_eq!(source_strs.filter_all_conditional(&conditions), target_strs);
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
  let expected_string = "mysite.com".to_string();
  assert_eq!(domain, expected_string);
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
    StringBounds::StartsWithCi("jan", true),
    StringBounds::EndsWithCi("images", true),
    StringBounds::ContainsCi("2023", true),
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

  let mixed_conditions = [
    StringBounds::ContainsCi("nepal", true),
    StringBounds::EndsWithCi(".psd", false),
  ];

  let file_name_a = file_names[0];
  let file_name_b = file_names[2];

  assert!(file_name_a.match_all_conditional(&mixed_conditions) == false);

  assert!(file_name_b.match_all_conditional(&mixed_conditions));
  
  let nepal_jpg_files: Vec<&str> = file_names.filter_all_conditional(&mixed_conditions);

  assert_eq!(nepal_jpg_files.len(), 2);

  assert_eq!(nepal_jpg_files[0], file_name_b);

  let file_names_vector = file_names.to_strings();

  let nepal_jpg_files_vector: Vec<&str> = file_names_vector.filter_all_conditional(&mixed_conditions);
  
  assert_eq!(nepal_jpg_files_vector.len(), 2);

}