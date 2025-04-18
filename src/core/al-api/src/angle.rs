use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "../../js/libs/astro/coo.js")]
extern "C" {
    #[wasm_bindgen(js_name = Format)]
    pub type Format;

    /**
     * Convert a decimal coordinate into sexagesimal string, according to the given precision<br>
     * 8: 1/1000th sec, 7: 1/100th sec, 6: 1/10th sec, 5: sec, 4: 1/10th min, 3: min, 2: 1/10th deg, 1: deg
     * @param num number (integer or decimal)
     * @param prec precision (= number of decimal digit to keep or append)
     * @param plus if true, the '+' sign is displayed
     * @return a string with the formatted sexagesimal number
     */
    #[wasm_bindgen(static_method_of = Format)]
    pub fn toSexagesimal(num: f64, prec: u8, plus: bool) -> String;
    /**
     * Convert a decimal coordinate into a decimal string, according to the given precision
     * @param num number (integer or decimal)
     * @param prec precision (= number of decimal digit to keep or append)
     * @return a string with the formatted sexagesimal number
     */
    #[wasm_bindgen(static_method_of = Format)]
    pub fn toDecimal(num: f64, prec: u8) -> String;
}

use std::cmp::Eq;
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen]
pub enum Formatter {
    Sexagesimal,
    Decimal
}