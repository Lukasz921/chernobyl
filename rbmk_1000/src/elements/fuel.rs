use crate::elements::uranium::*;
use crate::elements::waste::*;
use crate::elements::xenon::*;
pub enum Fuel {
    Uranium(Uranium),
    Xenon(Xenon),
    Waste(Waste),
}