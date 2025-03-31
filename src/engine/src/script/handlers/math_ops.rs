use std::collections::HashMap;
use std::mem::offset_of;
use std::sync::OnceLock;
use crate::script::script_opcode::ScriptOpcode;
use crate::script::script_runner::CommandHandlers;
use crate::script::script_state::ScriptState;
use crate::util::bits::{bitcount, clear_bit_range, set_bit_range, MASK};

pub fn get_math_ops() -> &'static CommandHandlers {
    static HANDLERS: OnceLock<CommandHandlers> = OnceLock::new();

    HANDLERS.get_or_init(|| {
        let mut handlers: CommandHandlers = HashMap::with_capacity(28);

        handlers.insert(
            ScriptOpcode::ADD as i32,
            |state: &mut ScriptState| {
                let b = state.pop_int();
                let a = state.pop_int();

                state.push_int(a + b);
            }
        );

        handlers.insert(
            ScriptOpcode::SUB as i32,
            |state: &mut ScriptState| {
                let b = state.pop_int();
                let a = state.pop_int();

                state.push_int(a - b);
            }
        );

        handlers.insert(
            ScriptOpcode::MULTIPLY as i32,
            |state: &mut ScriptState| {
                let b = state.pop_int();
                let a = state.pop_int();

                state.push_int(a * b);
            }
        );

        handlers.insert(
            ScriptOpcode::DIVIDE as i32,
            |state: &mut ScriptState| {
                let b = state.pop_int();
                let a = state.pop_int();

                state.push_int(a / b);
            }
        );

        handlers.insert(
            ScriptOpcode::RANDOM as i32,
            |state: &mut ScriptState| {
                use rand::rngs::ThreadRng;
                use rand::Rng;

                thread_local! {
                    static RNG: std::cell::RefCell<ThreadRng> = std::cell::RefCell::new(rand::thread_rng());
                }

                let a = state.pop_int();

                let random_value = if a <= 0 {
                    0
                } else {
                    RNG.with(|rng| rng.borrow_mut().random_range(0..a))
                };

                state.push_int(random_value);
            }
        );

        handlers.insert(
            ScriptOpcode::RANDOMINC as i32,
            |state: &mut ScriptState| {
                use rand::rngs::ThreadRng;
                use rand::Rng;

                thread_local! {
                    static RNG: std::cell::RefCell<ThreadRng> = std::cell::RefCell::new(rand::thread_rng());
                }

                let a = state.pop_int();

                let random_value = if a < 0 {
                    0
                } else {
                    RNG.with(|rng| rng.borrow_mut().gen_range(0..=a))
                };

                state.push_int(random_value);
            }
        );

        handlers.insert(
            ScriptOpcode::INTERPOLATE as i32,
            |state: &mut ScriptState| {
                let x = state.pop_int();
                let x1 = state.pop_int();
                let x0 = state.pop_int();
                let y1 = state.pop_int();
                let y0 = state.pop_int();

                // Avoid division by zero
                let lerp = if x1 == x0 {
                    y0
                } else {
                    // Calculate linear interpolation
                    let slope = (y1 - y0) / (x1 / x0);
                    (slope * (x - x0)) + y0
                };

                state.push_int(lerp);
            }
        );

        handlers.insert(
            ScriptOpcode::ADDPERCENT as i32,
            |state: &mut ScriptState| {
                let percent = state.pop_int();
                let num = state.pop_int();

                let result = ((num * percent) / 100) + num;

                state.push_int(result);
            }
        );

        handlers.insert(
            ScriptOpcode::SETBIT as i32,
            |state: &mut ScriptState| {
                let bit = state.pop_int();
                let value = state.pop_int();

                state.push_int(value | (1 << bit));
            }
        );

        handlers.insert(
            ScriptOpcode::CLEARBIT as i32,
            |state: &mut ScriptState| {
                let bit = state.pop_int();
                let value = state.pop_int();

                state.push_int(value & !(1 << bit));
            }
        );

        handlers.insert(
            ScriptOpcode::TESTBIT as i32,
            |state: &mut ScriptState| {
                let bit = state.pop_int();
                let value = state.pop_int();

                // Test if the bit is set, return 1 if set, 0 if not
                state.push_int(if (value & (1 << bit)) != 0 { 1 } else { 0 });
            }
        );

        handlers.insert(
            ScriptOpcode::MODULO as i32,
            |state: &mut ScriptState| {
                let values = state.pop_ints(2);
                state.push_int(values[0] % values[1]);
            }
        );

        handlers.insert(
            ScriptOpcode::POW as i32,
            |state: &mut ScriptState| {
                let exponent = state.pop_int();
                let base = state.pop_int();

                let result = (base as f64).powf(exponent as f64) as i32;
                state.push_int(result);
            }
        );

        handlers.insert(
            ScriptOpcode::INVPOW as i32,
            |state: &mut ScriptState| {
                let n2 = state.pop_int();
                let n1 = state.pop_int();

                if n1 == 0 || n2 == 0 {
                    state.push_int(0)
                } else {
                    match n2 {
                        1 => state.push_int(n1),
                        2 => state.push_int((n1 as f64).sqrt() as i32),
                        3 => state.push_int((n1 as f64).cbrt() as i32),
                        4 => state.push_int((n1 as f64).sqrt().cbrt() as i32),

                        _ => {
                            let result = (n1 as f64).powf(1.0 / n2 as f64) as i32;
                            state.push_int(result);
                        }
                    }
                }
            }
        );

        handlers.insert(
            ScriptOpcode::AND as i32,
            |state: &mut ScriptState| {
                let values = state.pop_ints(2);
                state.push_int(values[0] & values[1]);
            }
        );

        handlers.insert(
            ScriptOpcode::OR as i32,
            |state: &mut ScriptState| {
                let values = state.pop_ints(2);
                state.push_int(values[0] | values[1]);
            }
        );

        handlers.insert(
            ScriptOpcode::MIN as i32,
            |state: &mut ScriptState| {
                let values = state.pop_ints(2);
                state.push_int(std::cmp::min(values[0], values[1]));
            }
        );

        handlers.insert(
            ScriptOpcode::MAX as i32,
            |state: &mut ScriptState| {
                let values = state.pop_ints(2);
                state.push_int(std::cmp::max(values[0], values[1]));
            }
        );

        handlers.insert(
            ScriptOpcode::SCALE as i32,
            |state: &mut ScriptState| {
                let values = state.pop_ints(3);
                state.push_int((values[0] * values[1]) / values[2]);
            }
        );

        handlers.insert(
            ScriptOpcode::BITCOUNT as i32,
            |state: &mut ScriptState| {
                let value = state.pop_int();
                state.push_int(bitcount(value));
            }
        );

        handlers.insert(
            ScriptOpcode::TOGGLEBIT as i32,
            |state: &mut ScriptState| {
                let values = state.pop_ints(2);
                state.push_int(values[0] ^ (1 << values[1]));
            }
        );

        handlers.insert(
            ScriptOpcode::SETBIT_RANGE as i32,
            |state: &mut ScriptState| {
                let values = state.pop_ints(3);
                state.push_int(set_bit_range(values[0], values[1], values[2]));
            }
        );

        handlers.insert(
            ScriptOpcode::CLEARBIT as i32,
            |state: &mut ScriptState| {
                let values = state.pop_ints(3);
                state.push_int(clear_bit_range(values[0], values[1], values[2]));
            }
        );

        handlers.insert(
            ScriptOpcode::GETBIT_RANGE as i32,
            |state: &mut ScriptState| {
                let values = state.pop_ints(3);
                let a = 31 - values[2];

                let shifted_left = (values[0] << a) as u32;
                let result = (shifted_left >> (values[1] + a)) as i32;

                state.push_int(result);
            }
        );

        handlers.insert(
            ScriptOpcode::SETBIT_RANGE_TOINT as i32,
            |state: &mut ScriptState| {
                let values = state.pop_ints(4);
                let cleared_bit_range = clear_bit_range(values[0], values[2], values[3]);
                let max_value = MASK[(values[3] - values[2] + 1) as usize];
                let mut assign_value = values[1];
                if values[1] > max_value {
                    assign_value = max_value;
                }
                state.push_int(cleared_bit_range | (assign_value << values[2]));
            }
        );

        handlers.insert(
            ScriptOpcode::SIN_DEG as i32,
            |state: &mut ScriptState| {
                // TODO
            }
        );
        
        handlers.insert(
            ScriptOpcode::COS_DEG as i32,
            |state: &mut ScriptState| {
                // TODO
            }
        );
        
        handlers.insert(
            ScriptOpcode::ATAN2_DEG as i32,
            |state: &mut ScriptState| {
                // TODO
            }
        );

        handlers.insert(
            ScriptOpcode::ABS as i32,
            |state: &mut ScriptState| {
                let value = state.pop_int();
                state.push_int(value.abs());
            }
        );

        handlers
    })
}