
use regex::{Captures, Regex};
use lazy_static::lazy_static;

lazy_static! {
    static ref PROCESS_RANGE: Regex = Regex::new(r#"(?s)%\{.*?%\}"#).expect("Could not construct regex");
    static ref SPLIT_VALUE: Regex = Regex::new("%SV").expect("Could not construct regex");

    static ref TYPE_RAW: Regex = Regex::new("%TR").expect("Could not construct regex");

    static ref VALUE_RAW:     Regex = Regex::new("%VR").expect("Could not construct regex");
    static ref VALUE_INDEX:   Regex = Regex::new("%VI").expect("Could not construct regex");

    static ref VARIANT:  Regex = Regex::new(r"(?s)%VAR\[\((.*?)\)\|\((.*?)\)\]").expect("Could not construct regex");
}

pub fn process_tuple_string<const N: usize>(template: &str) -> String {
    let has_variant = VARIANT.is_match(template);
    let mut result = "".to_owned();
    for i in 0..N {
        result.push_str(&generate_tuple_string(template, i+1, has_variant));
    }
    return result;
}

fn generate_tuple_string(input: &str, tuple_n: usize, has_variant: bool) -> String {
    let mut result = "".to_owned();

    let input = &SPLIT_VALUE.replace_all(input, |_: &Captures|{
        let mut result = "".to_owned();
        for i in 0..tuple_n {
            result.push_str(&format!("v{},", i))
        }
        return result;
    });

    for j in if has_variant { 0 } else { tuple_n }..=tuple_n {
        let mut last = 0;
        for range in PROCESS_RANGE.find_iter(input) {
            let sub_string = &input[(range.start()+2)..(range.end()-2)];
            result.push_str(&input[last..range.start()]);
            result.push_str(&generate_tuple_string_loop(sub_string, tuple_n, tuple_n - j, has_variant));
            last = range.end();
        }
        
        result.push_str(if last == 0 { input } else { &input[last..] });
    }

    return result;
}

fn generate_tuple_string_loop(input: &str, tuple_n: usize, variant_n: usize, has_variant: bool) -> String {
    let mut result = "".to_owned();



    //let match_split_value   = Regex::new("%SV").expect("Could not construct regex");

    for i in 0..tuple_n {

        let is_variant = i >= variant_n;
        let tmp =    TYPE_RAW.replace_all(input, &format!("T{}",     i) as &str);
        let tmp =   VALUE_RAW.replace_all(&tmp,  &format!("self.{}", i) as &str);
        let tmp = VALUE_INDEX.replace_all(&tmp,  &format!("v{}",     i) as &str);

        let tmp = if has_variant { VARIANT.replace_all(
            &tmp,
            |v: &Captures| { ( v[if is_variant { 2 } else { 1 }]).to_owned() }
        )} else { tmp };

        result.push_str(&tmp);
    }

    return result;
}