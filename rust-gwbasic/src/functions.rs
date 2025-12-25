//! Built-in functions for GW-BASIC

use crate::error::{Error, Result};
use crate::value::Value;

/// Math functions
pub fn abs_fn(val: Value) -> Result<Value> {
    Ok(Value::Double(val.as_double()?.abs()))
}

pub fn int_fn(val: Value) -> Result<Value> {
    Ok(Value::Integer(val.as_double()?.floor() as i32))
}

pub fn sqr_fn(val: Value) -> Result<Value> {
    let v = val.as_double()?;
    if v < 0.0 {
        return Err(Error::RuntimeError("Square root of negative number".to_string()));
    }
    Ok(Value::Double(v.sqrt()))
}

pub fn sin_fn(val: Value) -> Result<Value> {
    Ok(Value::Double(val.as_double()?.sin()))
}

pub fn cos_fn(val: Value) -> Result<Value> {
    Ok(Value::Double(val.as_double()?.cos()))
}

pub fn tan_fn(val: Value) -> Result<Value> {
    Ok(Value::Double(val.as_double()?.tan()))
}

pub fn atn_fn(val: Value) -> Result<Value> {
    Ok(Value::Double(val.as_double()?.atan()))
}

pub fn exp_fn(val: Value) -> Result<Value> {
    Ok(Value::Double(val.as_double()?.exp()))
}

pub fn log_fn(val: Value) -> Result<Value> {
    let v = val.as_double()?;
    if v <= 0.0 {
        return Err(Error::RuntimeError("Logarithm of non-positive number".to_string()));
    }
    Ok(Value::Double(v.ln()))
}

pub fn sgn_fn(val: Value) -> Result<Value> {
    let v = val.as_double()?;
    let sign = if v > 0.0 { 1 } else if v < 0.0 { -1 } else { 0 };
    Ok(Value::Integer(sign))
}

pub fn fix_fn(val: Value) -> Result<Value> {
    Ok(Value::Integer(val.as_double()?.trunc() as i32))
}

pub fn cint_fn(val: Value) -> Result<Value> {
    Ok(Value::Integer(val.as_double()?.round() as i32))
}

pub fn csng_fn(val: Value) -> Result<Value> {
    Ok(Value::Single(val.as_double()? as f32))
}

pub fn cdbl_fn(val: Value) -> Result<Value> {
    Ok(Value::Double(val.as_double()?))
}

/// String functions
pub fn len_fn(val: Value) -> Result<Value> {
    Ok(Value::Integer(val.as_string().len() as i32))
}

pub fn asc_fn(val: Value) -> Result<Value> {
    let s = val.as_string();
    if s.is_empty() {
        return Err(Error::RuntimeError("ASC on empty string".to_string()));
    }
    Ok(Value::Integer(s.chars().next().unwrap() as i32))
}

pub fn chr_fn(val: Value) -> Result<Value> {
    let code = val.as_integer()?;
    if code < 0 || code > 255 {
        return Err(Error::RuntimeError(format!("CHR$ code out of range: {}", code)));
    }
    Ok(Value::String((code as u8 as char).to_string()))
}

pub fn str_fn(val: Value) -> Result<Value> {
    Ok(Value::String(val.to_string()))
}

pub fn val_fn(val: Value) -> Result<Value> {
    let string = val.as_string();
    let s = string.trim();
    if let Ok(i) = s.parse::<i32>() {
        Ok(Value::Integer(i))
    } else if let Ok(f) = s.parse::<f64>() {
        Ok(Value::Double(f))
    } else {
        Ok(Value::Integer(0))
    }
}

pub fn left_fn(s: Value, n: Value) -> Result<Value> {
    let string = s.as_string();
    let count = n.as_integer()? as usize;
    Ok(Value::String(string.chars().take(count).collect()))
}

pub fn right_fn(s: Value, n: Value) -> Result<Value> {
    let string = s.as_string();
    let count = n.as_integer()? as usize;
    let chars: Vec<char> = string.chars().collect();
    let start = if count > chars.len() { 0 } else { chars.len() - count };
    Ok(Value::String(chars[start..].iter().collect()))
}

pub fn mid_fn(s: Value, start: Value, len: Option<Value>) -> Result<Value> {
    let string = s.as_string();
    let start_pos = (start.as_integer()? - 1).max(0) as usize;
    let chars: Vec<char> = string.chars().collect();
    
    if start_pos >= chars.len() {
        return Ok(Value::String(String::new()));
    }
    
    let result = if let Some(length) = len {
        let count = length.as_integer()? as usize;
        chars[start_pos..].iter().take(count).collect()
    } else {
        chars[start_pos..].iter().collect()
    };
    
    Ok(Value::String(result))
}

pub fn space_fn(n: Value) -> Result<Value> {
    let count = n.as_integer()?;
    if count < 0 {
        return Err(Error::RuntimeError("SPACE$ count cannot be negative".to_string()));
    }
    Ok(Value::String(" ".repeat(count as usize)))
}

pub fn string_fn(n: Value, ch: Value) -> Result<Value> {
    let count = n.as_integer()?;
    if count < 0 {
        return Err(Error::RuntimeError("STRING$ count cannot be negative".to_string()));
    }
    
    let char_code = if ch.is_string() {
        let s = ch.as_string();
        if s.is_empty() {
            return Err(Error::RuntimeError("STRING$ character cannot be empty".to_string()));
        }
        s.chars().next().unwrap()
    } else {
        let code = ch.as_integer()?;
        if code < 0 || code > 255 {
            return Err(Error::RuntimeError("STRING$ code out of range".to_string()));
        }
        code as u8 as char
    };
    
    Ok(Value::String(char_code.to_string().repeat(count as usize)))
}

pub fn instr_fn(start: Option<Value>, haystack: Value, needle: Value) -> Result<Value> {
    let start_pos = if let Some(s) = start {
        (s.as_integer()? - 1).max(0) as usize
    } else {
        0
    };
    
    let hay = haystack.as_string();
    let need = needle.as_string();
    
    if start_pos >= hay.len() {
        return Ok(Value::Integer(0));
    }
    
    if let Some(pos) = hay[start_pos..].find(&need) {
        Ok(Value::Integer((start_pos + pos + 1) as i32))
    } else {
        Ok(Value::Integer(0))
    }
}

pub fn hex_fn(val: Value) -> Result<Value> {
    Ok(Value::String(format!("{:X}", val.as_integer()?)))
}

pub fn oct_fn(val: Value) -> Result<Value> {
    Ok(Value::String(format!("{:o}", val.as_integer()?)))
}

/// Conversion functions
pub fn peek_fn(_addr: Value) -> Result<Value> {
    // Simulated - returns 0
    Ok(Value::Integer(0))
}

pub fn inp_fn(_port: Value) -> Result<Value> {
    // Simulated - returns 0
    Ok(Value::Integer(0))
}

/// System functions
pub fn rnd_fn(seed: Option<Value>) -> Result<Value> {
    use std::cell::RefCell;
    thread_local! {
        static RNG_STATE: RefCell<u64> = RefCell::new(12345);
    }
    
    RNG_STATE.with(|state| {
        let mut s = state.borrow_mut();
        
        if let Some(seed_val) = seed {
            let sv = seed_val.as_double()?;
            if sv < 0.0 {
                *s = (sv.abs() * 1000000.0) as u64;
            } else if sv == 0.0 {
                // Return last random number (simplified)
                return Ok(Value::Single((*s % 1000) as f32 / 1000.0));
            }
        }
        
        // Simple LCG
        *s = (*s * 1103515245 + 12345) & 0x7fffffff;
        Ok(Value::Single((*s % 1000000) as f32 / 1000000.0))
    })
}

pub fn timer_fn() -> Result<Value> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    let seconds_since_midnight = (now.as_secs() % 86400) as f32;
    Ok(Value::Single(seconds_since_midnight))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_functions() {
        assert_eq!(abs_fn(Value::Integer(-5)).unwrap().as_integer().unwrap(), 5);
        assert_eq!(int_fn(Value::Double(3.7)).unwrap().as_integer().unwrap(), 3);
        assert!((sqr_fn(Value::Integer(16)).unwrap().as_double().unwrap() - 4.0).abs() < 0.001);
    }

    #[test]
    fn test_string_functions() {
        assert_eq!(len_fn(Value::String("Hello".to_string())).unwrap().as_integer().unwrap(), 5);
        assert_eq!(asc_fn(Value::String("A".to_string())).unwrap().as_integer().unwrap(), 65);
        assert_eq!(chr_fn(Value::Integer(65)).unwrap().as_string(), "A");
    }

    #[test]
    fn test_left_right_mid() {
        let s = Value::String("HELLO".to_string());
        assert_eq!(left_fn(s.clone(), Value::Integer(2)).unwrap().as_string(), "HE");
        assert_eq!(right_fn(s.clone(), Value::Integer(2)).unwrap().as_string(), "LO");
        assert_eq!(mid_fn(s, Value::Integer(2), Some(Value::Integer(3))).unwrap().as_string(), "ELL");
    }
}