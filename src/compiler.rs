use std::fmt::format;
use crate::data_types::{YAL, VariableSingleValued, Py, FunctionCall};


fn py_var_single_value(var: VariableSingleValued) -> String
{
    let mut value: String = "".to_string();
    for val in &var.value
    {
        value += &val.true_value();
    }
    format!("{}: {} = {}\n", var.name.value, var.data_type.value, value)
}

fn py_function_call(func_call: FunctionCall) -> String
{
    let mut val = format!("{}(", func_call.name.value);
    for value in &func_call.values {
       val += &format!("{},", value.value);
    }
    val = (&val[(0 as usize)..(val.len() - 1 as usize)]).to_string() + ")";
    val
}

fn py_py(var: Py) -> String
{
    var.value + "\n"
}

pub fn compyle(yal: Vec<YAL>) -> String
{
    let mut whole_file: String = "".to_string();
    for thing in yal
    {
        match thing
        {
            // var_name: type = value;
            YAL::VariableSingleValue(var) => whole_file += &py_var_single_value(var),
            YAL::Py(var) => whole_file += &py_py(var),
            YAL::FuncCall(var) => whole_file += &py_function_call(var),
        }
    }
    whole_file
}