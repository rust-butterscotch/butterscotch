use std::fmt::Write;
use ::f128::*;
use num_traits::{cast::ToPrimitive, Float};
use num_format::{CustomFormat, Grouping, ToFormattedString};
use std::collections::HashMap;

pub enum ResultTypes {
    Object(HashMap<&'static str, ResultTypes>),
    Array(Vec<ResultTypes>),
    Value(f128),
}



fn main() {
    let mut value_map = HashMap::<&'static str, ResultTypes>::new();
    value_map.insert("PI", ResultTypes::Value(f128::PI));
    value_map.insert("G_M3_KG1_S2", ResultTypes::Value(f128::new(667430)/f128::new(1e16)));


    let mut str_val = String::new();
    print_struct(false, 0, ResultTypes::Object(value_map), &mut str_val);
    println!("{}", str_val);
}

fn print_struct(disable_indent: bool, depth: u32, data: ResultTypes, result: &mut String) {
    let indent        = |r: &mut String|{ for _ in 0..depth { r.push_str("\t"); } };
    let bracket_open  = |v: &str, r: &mut String| { write!(r, "{}\n", v).unwrap(); };
    let bracket_close = |v: &str, r: &mut String| {
        if !disable_indent { indent(r); }
        write!(r, "{}\n", v).unwrap();
    };


    if !disable_indent { indent(result); }

    match data {
        ResultTypes::Value(v)  => write_rescale_pair(v, result),
        ResultTypes::Array(v)  => {
            bracket_open("[", result);
            for entry in v {
                print_struct(false, depth+1, entry, result);
                result.push_str(",\n");
            }
            bracket_close("],", result);
        },
        ResultTypes::Object(v) => {
            bracket_open("{", result);
            for (key, value) in v {
                indent(result);
                result.push_str("\t");
                write!(result, "{}: ", key).unwrap();
                print_struct(true, depth+1, value, result);
                result.push_str(",\n");
            }
            bracket_close("},", result);
        },
    }
}

fn write_rescale_pair(raw_value: f128, result:&mut String) {

    let rescale = match raw_value.signum().to_f64().unwrap() {
        v if v > 0.0 => f128::new(i64::MAX)/raw_value,
        v if v < 0.0 => f128::new(i64::MIN)/raw_value,
        _ => f128::ZERO,
    }.min(f128::new(i64::MAX)).max(f128::new(i64::MIN)).trunc();


    let rescaled_value = rescale * raw_value;

    write!(result, "({}, {})", fmt128(rescaled_value), fmt128(rescale)).unwrap();


    /*let scale_value = f128::from(raw_scale)*raw_value;
    let rescaled_val = scale_value.to_i128().unwrap();
    let rescale = match rescaled_val.signum() {
        -1 => (i64::MIN as i128)/(2*rescaled_val),
         1 => (i64::MAX as i128)/(2*rescaled_val),
         _ => 1,
    } as i64; // Need to figure this out.... Why do we need to 2*rescaled?

    write!(result, "({}, {})", fmt128(f128::from(rescale)*scale_value), fmti64(rescale)).unwrap();*/
}

fn fmti64(v: i64) -> String {
    let format = CustomFormat::builder()
            .grouping(Grouping::Standard)
            .minus_sign("-")
            .separator("_")
            .build().expect("");
    v.to_formatted_string(&format)
}

fn fmt128(v: f128) -> String {
    fmti64(v.to_i64().unwrap())
}