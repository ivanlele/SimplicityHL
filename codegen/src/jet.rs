use simplicityhl::simplicity_unchained::jets::elements::ElementsExtension;

use simplicityhl::simplicity::jet::Elements;
use std::fmt;

#[rustfmt::skip]
pub fn documentation(jet: ElementsExtension) -> &'static str {
    match jet {
        // Multi-bit logic
        ElementsExtension::Elements(Elements::All8) => "Check if the value is [`u8::MAX`].",
        ElementsExtension::Elements(Elements::All16) => "Check if the value is [`u16::MAX`].",
        ElementsExtension::Elements(Elements::All32) => "Check if the value is [`u32::MAX`].",
        ElementsExtension::Elements(Elements::All64) => "Check if the value is [`u64::MAX`].",
        ElementsExtension::Elements(Elements::And1) => "Bitwise AND of two 1-bit values.",
        ElementsExtension::Elements(Elements::And8) => "Bitwise AND of two 8-bit values.",
        ElementsExtension::Elements(Elements::And16) => "Bitwise AND of two 16-bit values.",
        ElementsExtension::Elements(Elements::And32) => "Bitwise AND of two 32-bit values",
        ElementsExtension::Elements(Elements::And64) => "Bitwise AND of two 64-bit values",
        ElementsExtension::Elements(Elements::Ch1) => "Bitwise CHOICE of a bit and two 1-bit values.  If the bit is true, then take the first value, else take the second value.",
        ElementsExtension::Elements(Elements::Ch8) => "Bitwise CHOICE of a bit and two 8-bit values.  If the bit is true, then take the first value, else take the second value.",
        ElementsExtension::Elements(Elements::Ch16) => "Bitwise CHOICE of a bit and two 16-bit values. If the bit is true, then take the first value, else take the second value.",
        ElementsExtension::Elements(Elements::Ch32) => "Bitwise CHOICE of a bit and two 32-bit values. If the bit is true, then take the first value, else take the second value.",
        ElementsExtension::Elements(Elements::Ch64) => "Bitwise CHOICE of a bit and two 64-bit values. If the bit is true, then take the first value, else take the second value.",
        ElementsExtension::Elements(Elements::Complement1) => "Bitwise NOT of a 1-bit  value.",
        ElementsExtension::Elements(Elements::Complement8) => "Bitwise NOT of an 8-bit value.",
        ElementsExtension::Elements(Elements::Complement16) => "Bitwise NOT of a 16-bit value.",
        ElementsExtension::Elements(Elements::Complement32) => "Bitwise NOT of a 32-bit value.",
        ElementsExtension::Elements(Elements::Complement64) => "Bitwise NOT of a 64-bit value.",
        ElementsExtension::Elements(Elements::Eq1) => "Check if two 1-bit values are equal.",
        ElementsExtension::Elements(Elements::Eq8) => "Check if two 8-bit values are equal.",
        ElementsExtension::Elements(Elements::Eq16) => "Check if two 16-bit values are equal.",
        ElementsExtension::Elements(Elements::Eq32) => "Check if two 32-bit values are equal.",
        ElementsExtension::Elements(Elements::Eq64) => "Check if two 64-bit values are equal.",
        ElementsExtension::Elements(Elements::Eq256) => "Check if two 256-bit values are equal.",
        ElementsExtension::Elements(Elements::FullLeftShift8_1) => "Helper for left-shifting  bits. The bits are shifted from a 1-bit  value into a 8-bit  value. Return the shifted value and the 1  bit  that was  shifted out.",
        ElementsExtension::Elements(Elements::FullLeftShift8_2) => "Helper for left-shifting  bits. The bits are shifted from a 2-bit  value into a 8-bit  value. Return the shifted value and the 2  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullLeftShift8_4) => "Helper for left-shifting  bits. The bits are shifted from a 4-bit  value into a 8-bit  value. Return the shifted value and the 4  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullLeftShift16_1) => "Helper for left-shifting  bits. The bits are shifted from a 1-bit  value into a 16-bit value. Return the shifted value and the 1  bit  that was  shifted out.",
        ElementsExtension::Elements(Elements::FullLeftShift16_2) => "Helper for left-shifting  bits. The bits are shifted from a 2-bit  value into a 16-bit value. Return the shifted value and the 2  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullLeftShift16_4) => "Helper for left-shifting  bits. The bits are shifted from a 4-bit  value into a 16-bit value. Return the shifted value and the 4  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullLeftShift16_8) => "Helper for left-shifting  bits. The bits are shifted from a 8-bit  value into a 16-bit value. Return the shifted value and the 8  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullLeftShift32_1) => "Helper for left-shifting  bits. The bits are shifted from a 1-bit  value into a 32-bit value. Return the shifted value and the 1  bit  that was  shifted out.",
        ElementsExtension::Elements(Elements::FullLeftShift32_2) => "Helper for left-shifting  bits. The bits are shifted from a 2-bit  value into a 32-bit value. Return the shifted value and the 2  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullLeftShift32_4) => "Helper for left-shifting  bits. The bits are shifted from a 4-bit  value into a 32-bit value. Return the shifted value and the 4  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullLeftShift32_8) => "Helper for left-shifting  bits. The bits are shifted from a 8-bit  value into a 32-bit value. Return the shifted value and the 8  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullLeftShift32_16) => "Helper for left-shifting  bits. The bits are shifted from a 16-bit value into a 32-bit value. Return the shifted value and the 16 bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullLeftShift64_1) => "Helper for left-shifting  bits. The bits are shifted from a 1-bit  value into a 64-bit value. Return the shifted value and the 1  bit  that was  shifted out.",
        ElementsExtension::Elements(Elements::FullLeftShift64_2) => "Helper for left-shifting  bits. The bits are shifted from a 2-bit  value into a 64-bit value. Return the shifted value and the 2  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullLeftShift64_4) => "Helper for left-shifting  bits. The bits are shifted from a 4-bit  value into a 64-bit value. Return the shifted value and the 4  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullLeftShift64_8) => "Helper for left-shifting  bits. The bits are shifted from a 8-bit  value into a 64-bit value. Return the shifted value and the 8  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullLeftShift64_16) => "Helper for left-shifting  bits. The bits are shifted from a 16-bit value into a 64-bit value. Return the shifted value and the 16 bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullLeftShift64_32) => "Helper for left-shifting  bits. The bits are shifted from a 32-bit value into a 64-bit value. Return the shifted value and the 32 bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift8_1) => "Helper for right-shifting bits. The bits are shifted from a 1-bit  value into a 8-bit  value. Return the shifted value and the 1  bit  that was  shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift8_2) => "Helper for right-shifting bits. The bits are shifted from a 2-bit  value into a 8-bit  value. Return the shifted value and the 2  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift8_4) => "Helper for right-shifting bits. The bits are shifted from a 4-bit  value into a 8-bit  value. Return the shifted value and the 4  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift16_1) => "Helper for right-shifting bits. The bits are shifted from a 1-bit  value into a 16-bit value. Return the shifted value and the 1  bit  that was  shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift16_2) => "Helper for right-shifting bits. The bits are shifted from a 2-bit  value into a 16-bit value. Return the shifted value and the 2  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift16_4) => "Helper for right-shifting bits. The bits are shifted from a 4-bit  value into a 16-bit value. Return the shifted value and the 4  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift16_8) => "Helper for right-shifting bits. The bits are shifted from a 8-bit  value into a 16-bit value. Return the shifted value and the 8  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift32_1) => "Helper for right-shifting bits. The bits are shifted from a 1-bit  value into a 32-bit value. Return the shifted value and the 1  bit  that was  shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift32_2) => "Helper for right-shifting bits. The bits are shifted from a 2-bit  value into a 32-bit value. Return the shifted value and the 2  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift32_4) => "Helper for right-shifting bits. The bits are shifted from a 4-bit  value into a 32-bit value. Return the shifted value and the 4  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift32_8) => "Helper for right-shifting bits. The bits are shifted from a 8-bit  value into a 32-bit value. Return the shifted value and the 8  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift32_16) => "Helper for right-shifting bits. The bits are shifted from a 16-bit value into a 32-bit value. Return the shifted value and the 16 bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift64_1) => "Helper for right-shifting bits. The bits are shifted from a 1-bit  value into a 64-bit value. Return the shifted value and the 1  bit  that was  shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift64_2) => "Helper for right-shifting bits. The bits are shifted from a 2-bit  value into a 64-bit value. Return the shifted value and the 2  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift64_4) => "Helper for right-shifting bits. The bits are shifted from a 4-bit  value into a 64-bit value. Return the shifted value and the 4  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift64_8) => "Helper for right-shifting bits. The bits are shifted from a 8-bit  value into a 64-bit value. Return the shifted value and the 8  bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift64_16) => "Helper for right-shifting bits. The bits are shifted from a 16-bit value into a 64-bit value. Return the shifted value and the 16 bits that were shifted out.",
        ElementsExtension::Elements(Elements::FullRightShift64_32) => "Helper for right-shifting bits. The bits are shifted from a 32-bit value into a 64-bit value. Return the shifted value and the 32 bits that were shifted out.",
        ElementsExtension::Elements(Elements::High1) => "Return `u1::MAX` = 1.",
        ElementsExtension::Elements(Elements::High8) => "Return [`u8::MAX`].",
        ElementsExtension::Elements(Elements::High16) => "Return [`u16::MAX`].",
        ElementsExtension::Elements(Elements::High32) => "Return [`u32::MAX`].",
        ElementsExtension::Elements(Elements::High64) => "Return [`u64::MAX`].",
        ElementsExtension::Elements(Elements::LeftExtend1_8) => "Extend a 1-bit  value to an 8-bit value by padding its left with the MSB.",
        ElementsExtension::Elements(Elements::LeftExtend1_16) => "Extend a 1-bit  value to a 16-bit value by padding its left with the MSB.",
        ElementsExtension::Elements(Elements::LeftExtend1_32) => "Extend a 1-bit  value to a 32-bit value by padding its left with the MSB.",
        ElementsExtension::Elements(Elements::LeftExtend1_64) => "Extend a 1-bit  value to a 64-bit value by padding its left with the MSB.",
        ElementsExtension::Elements(Elements::LeftExtend8_16) => "Extend an 8-bit value to a 16-bit value by padding its left with the MSB.",
        ElementsExtension::Elements(Elements::LeftExtend8_32) => "Extend an 8-bit value to a 32-bit value by padding its left with the MSB.",
        ElementsExtension::Elements(Elements::LeftExtend8_64) => "Extend an 8-bit value to a 64-bit value by padding its left with the MSB.",
        ElementsExtension::Elements(Elements::LeftExtend16_32) => "Extend a 16-bit value to a 32-bit value by padding its left with the MSB.",
        ElementsExtension::Elements(Elements::LeftExtend16_64) => "Extend a 16-bit value to a 64-bit value by padding its left with the MSB.",
        ElementsExtension::Elements(Elements::LeftExtend32_64) => "Extend a 16-bit value to a 64-bit value by padding its left with the MSB.",
        ElementsExtension::Elements(Elements::LeftPadHigh1_8) => "Extend a 1-bit  value to an 8-bit value by padding its left with ones.",
        ElementsExtension::Elements(Elements::LeftPadHigh1_16) => "Extend a 1-bit  value to a 16-bit value by padding its left with ones.",
        ElementsExtension::Elements(Elements::LeftPadHigh1_32) => "Extend a 1-bit  value to a 32-bit value by padding its left with ones.",
        ElementsExtension::Elements(Elements::LeftPadHigh1_64) => "Extend a 1-bit  value to a 64-bit value by padding its left with ones.",
        ElementsExtension::Elements(Elements::LeftPadHigh8_16) => "Extend an 8-bit value to a 16-bit value by padding its left with ones.",
        ElementsExtension::Elements(Elements::LeftPadHigh8_32) => "Extend an 8-bit value to a 32-bit value by padding its left with ones.",
        ElementsExtension::Elements(Elements::LeftPadHigh8_64) => "Extend a 1-bit  value to a 64-bit value by padding its left with ones.",
        ElementsExtension::Elements(Elements::LeftPadHigh16_32) => "Extend a 16-bit value to a 32-bit value by padding its left with ones.",
        ElementsExtension::Elements(Elements::LeftPadHigh16_64) => "Extend a 16-bit value to a 64-bit value by padding its left with ones.",
        ElementsExtension::Elements(Elements::LeftPadHigh32_64) => "Extend a 32-bit value to a 64-bit value by padding its left with ones.",
        ElementsExtension::Elements(Elements::LeftPadLow1_8) => "Extend a 1-bit  value to an 8-bit value by padding its left with zeroes.",
        ElementsExtension::Elements(Elements::LeftPadLow1_16) => "Extend a 1-bit  value to a 16-bit value by padding its left with zeroes.",
        ElementsExtension::Elements(Elements::LeftPadLow1_32) => "Extend a 1-bit  value to a 32-bit value by padding its left with zeroes.",
        ElementsExtension::Elements(Elements::LeftPadLow1_64) => "Extend a 1-bit  value to a 64-bit value by padding its left with zeroes.",
        ElementsExtension::Elements(Elements::LeftPadLow8_16) => "Extend an 8-bit value to a 16-bit value by padding its left with zeroes.",
        ElementsExtension::Elements(Elements::LeftPadLow8_32) => "Extend an 8-bit value to a 32-bit value by padding its left with zeroes.",
        ElementsExtension::Elements(Elements::LeftPadLow8_64) => "Extend an 8-bit value to a 64-bit value by padding its left with zeroes.",
        ElementsExtension::Elements(Elements::LeftPadLow16_32) => "Extend a 16-bit value to a 32-bit value by padding its left with zeroes.",
        ElementsExtension::Elements(Elements::LeftPadLow16_64) => "Extend a 16-bit value to a 64-bit value by padding its left with zeroes.",
        ElementsExtension::Elements(Elements::LeftPadLow32_64) => "Extend a 32-bit value to a 64-bit value by padding its left with zeroes.",
        ElementsExtension::Elements(Elements::LeftRotate8) => "Left-rotate an 8-bit value by the given amount.",
        ElementsExtension::Elements(Elements::LeftRotate16) => "Left-rotate a 16-bit value by the given amount.",
        ElementsExtension::Elements(Elements::LeftRotate32) => "Left-rotate a 32-bit value by the given amount.",
        ElementsExtension::Elements(Elements::LeftRotate64) => "Left-rotate a 64-bit value by the given amount.",
        ElementsExtension::Elements(Elements::LeftShift8) => "Left-shift an 8-bit value by the given amount. Bits are filled with zeroes.",
        ElementsExtension::Elements(Elements::LeftShift16) => "Left-shift a 16-bit value by the given amount. Bits are filled with zeroes.",
        ElementsExtension::Elements(Elements::LeftShift32) => "Left-shift a 32-bit value by the given amount. Bits are filled with zeroes.",
        ElementsExtension::Elements(Elements::LeftShift64) => "Left-shift a 64-bit value by the given amount. Bits are filled with zeroes.",
        ElementsExtension::Elements(Elements::LeftShiftWith8) => "Left-shift an 8-bit value by the given amount. Bits are filled with the given bit.",
        ElementsExtension::Elements(Elements::LeftShiftWith16) => "Left-shift a 16-bit value by the given amount. Bits are filled with the given bit.",
        ElementsExtension::Elements(Elements::LeftShiftWith32) => "Left-shift a 32-bit value by the given amount. Bits are filled with the given bit.",
        ElementsExtension::Elements(Elements::LeftShiftWith64) => "Left-shift a 64-bit value by the given amount. Bits are filled with the given bit.",
        ElementsExtension::Elements(Elements::Leftmost8_1) => "Return the most significant 1  bits of an 8-bit value.",
        ElementsExtension::Elements(Elements::Leftmost8_2) => "Return the most significant 1  bits of an 8-bit value.",
        ElementsExtension::Elements(Elements::Leftmost8_4) => "Return the most significant 1  bits of an 8-bit value.",
        ElementsExtension::Elements(Elements::Leftmost16_1) => "Return the most significant 1  bit  of a 16-bit value.",
        ElementsExtension::Elements(Elements::Leftmost16_2) => "Return the most significant 2  bits of a 16-bit value.",
        ElementsExtension::Elements(Elements::Leftmost16_4) => "Return the most significant 4  bits of a 16-bit value.",
        ElementsExtension::Elements(Elements::Leftmost16_8) => "Return the most significant 8  bits of a 16-bit value.",
        ElementsExtension::Elements(Elements::Leftmost32_1) => "Return the most significant 1  bit  of a 32-bit value.",
        ElementsExtension::Elements(Elements::Leftmost32_2) => "Return the most significant 2  bits of a 32-bit value.",
        ElementsExtension::Elements(Elements::Leftmost32_4) => "Return the most significant 4  bits of a 32-bit value.",
        ElementsExtension::Elements(Elements::Leftmost32_8) => "Return the most significant 8  bits of a 32-bit value.",
        ElementsExtension::Elements(Elements::Leftmost32_16) => "Return the most significant 16 bits of a 32-bit value.",
        ElementsExtension::Elements(Elements::Leftmost64_1) => "Return the most significant 1  bits of a 64-bit value.",
        ElementsExtension::Elements(Elements::Leftmost64_2) => "Return the most significant 2  bits of a 64-bit value.",
        ElementsExtension::Elements(Elements::Leftmost64_4) => "Return the most significant 4  bits of a 64-bit value.",
        ElementsExtension::Elements(Elements::Leftmost64_8) => "Return the most significant 8  bits of a 64-bit value.",
        ElementsExtension::Elements(Elements::Leftmost64_16) => "Return the most significant 16 bits of a 64-bit value.",
        ElementsExtension::Elements(Elements::Leftmost64_32) => "Return the most significant 32 bits of a 64-bit value.",
        ElementsExtension::Elements(Elements::Low1) => "Return `u1::MIN` = 1.",
        ElementsExtension::Elements(Elements::Low8) => "Return [`u8::MIN`].",
        ElementsExtension::Elements(Elements::Low16) => "Return [`u16::MIN`].",
        ElementsExtension::Elements(Elements::Low32) => "Return [`u32::MIN`].",
        ElementsExtension::Elements(Elements::Low64) => "Return [`u64::MIN`].",
        ElementsExtension::Elements(Elements::Maj1) => "Bitwise MAJORITY of three 1-bit values. The output bit is false if two or more input bits are false, and true otherwise.",
        ElementsExtension::Elements(Elements::Maj8) => "Bitwise MAJORITY of three 1-bit values. The output bit is false if two or more input bits are false, and true otherwise.",
        ElementsExtension::Elements(Elements::Maj16) => "Bitwise MAJORITY of three 1-bit values. The output bit is false if two or more input bits are false, and true otherwise.",
        ElementsExtension::Elements(Elements::Maj32) => "Bitwise MAJORITY of three 1-bit values. The output bit is false if two or more input bits are false, and true otherwise.",
        ElementsExtension::Elements(Elements::Maj64) => "Bitwise MAJORITY of three 1-bit values. The output bit is false if two or more input bits are false, and true otherwise.",
        ElementsExtension::Elements(Elements::Or1) => "Bitwise OR of two 1-bit values.",
        ElementsExtension::Elements(Elements::Or8) => "Bitwise OR of two 8-bit values.",
        ElementsExtension::Elements(Elements::Or16) => "Bitwise OR of two 16-bit values.",
        ElementsExtension::Elements(Elements::Or32) => "Bitwise OR of two 32-bit values.",
        ElementsExtension::Elements(Elements::Or64) => "Bitwise OR of two 64-bit values.",
        ElementsExtension::Elements(Elements::RightExtend8_16) => "Extend an 8-bit value to a 16-bit value by padding its right with the MSB.",
        ElementsExtension::Elements(Elements::RightExtend8_32) => "Extend an 8-bit value to a 32-bit value by padding its right with the MSB.",
        ElementsExtension::Elements(Elements::RightExtend8_64) => "Extend an 8-bit value to a 64-bit value by padding its right with the MSB.",
        ElementsExtension::Elements(Elements::RightExtend16_32) => "Extend a 16-bit value to a 32-bit value by padding its right with the MSB.",
        ElementsExtension::Elements(Elements::RightExtend16_64) => "Extend a 16-bit value to a 64-bit value by padding its right with the MSB.",
        ElementsExtension::Elements(Elements::RightExtend32_64) => "Extend a 16-bit value to a 64-bit value by padding its right with the MSB.",
        ElementsExtension::Elements(Elements::RightPadHigh1_8) => "Extend a 1-bit  value to an 8-bit value by padding its right with ones.",
        ElementsExtension::Elements(Elements::RightPadHigh1_16) => "Extend a 1-bit  value to a 16-bit value by padding its right with ones.",
        ElementsExtension::Elements(Elements::RightPadHigh1_32) => "Extend a 1-bit  value to a 32-bit value by padding its right with ones.",
        ElementsExtension::Elements(Elements::RightPadHigh1_64) => "Extend a 1-bit  value to a 64-bit value by padding its right with ones.",
        ElementsExtension::Elements(Elements::RightPadHigh8_16) => "Extend an 8-bit  value to a 16-bit value by padding its right with ones.",
        ElementsExtension::Elements(Elements::RightPadHigh8_32) => "Extend an 8-bit  value to a 32-bit value by padding its right with ones.",
        ElementsExtension::Elements(Elements::RightPadHigh8_64) => "Extend a 1-bit  value to a 64-bit value by padding its right with ones.",
        ElementsExtension::Elements(Elements::RightPadHigh16_32) => "Extend a 16-bit value to a 32-bit value by padding its right with ones.",
        ElementsExtension::Elements(Elements::RightPadHigh16_64) => "Extend a 16-bit value to a 64-bit value by padding its right with ones.",
        ElementsExtension::Elements(Elements::RightPadHigh32_64) => "Extend a 32-bit value to a 64-bit value by padding its right with ones.",
        ElementsExtension::Elements(Elements::RightPadLow1_8) => "Extend a 1-bit  value to an 8-bit value by padding its right with zeroes.",
        ElementsExtension::Elements(Elements::RightPadLow1_16) => "Extend a 1-bit  value to a 16-bit value by padding its right with zeroes.",
        ElementsExtension::Elements(Elements::RightPadLow1_32) => "Extend a 1-bit  value to a 32-bit value by padding its right with zeroes.",
        ElementsExtension::Elements(Elements::RightPadLow1_64) => "Extend a 1-bit  value to a 64-bit value by padding its right with zeroes.",
        ElementsExtension::Elements(Elements::RightPadLow8_16) => "Extend an 8-bit value to a 16-bit value by padding its right with zeroes.",
        ElementsExtension::Elements(Elements::RightPadLow8_32) => "Extend an 8-bit value to a 32-bit value by padding its right with zeroes.",
        ElementsExtension::Elements(Elements::RightPadLow8_64) => "Extend an 8-bit value to a 64-bit value by padding its right with zeroes.",
        ElementsExtension::Elements(Elements::RightPadLow16_32) => "Extend a 16-bit value to a 32-bit value by padding its right with zeroes.",
        ElementsExtension::Elements(Elements::RightPadLow16_64) => "Extend a 16-bit value to a 64-bit value by padding its right with zeroes.",
        ElementsExtension::Elements(Elements::RightPadLow32_64) => "Extend a 32-bit value to a 64-bit value by padding its right with zeroes.",
        ElementsExtension::Elements(Elements::RightRotate8) => "Right-rotate an 8-bit value by the given amount.",
        ElementsExtension::Elements(Elements::RightRotate16) => "Right-rotate a 16-bit value by the given amount.",
        ElementsExtension::Elements(Elements::RightRotate32) => "Right-rotate a 32-bit value by the given amount.",
        ElementsExtension::Elements(Elements::RightRotate64) => "Right-rotate a 64-bit value by the given amount.",
        ElementsExtension::Elements(Elements::RightShift8) => "Right-shift an 8-bit value by the given amount. Bits are filled with zeroes.",
        ElementsExtension::Elements(Elements::RightShift16) => "Right-shift a 16-bit value by the given amount. Bits are filled with zeroes.",
        ElementsExtension::Elements(Elements::RightShift32) => "Right-shift a 32-bit value by the given amount. Bits are filled with zeroes.",
        ElementsExtension::Elements(Elements::RightShift64) => "Right-shift a 64-bit value by the given amount. Bits are filled with zeroes.",
        ElementsExtension::Elements(Elements::RightShiftWith8) => "Right-shift an 8-bit value by the given amount. Bits are filled with the given bit.",
        ElementsExtension::Elements(Elements::RightShiftWith16) => "Right-shift a 16-bit value by the given amount. Bits are filled with the given bit.",
        ElementsExtension::Elements(Elements::RightShiftWith32) => "Right-shift a 32-bit value by the given amount. Bits are filled with the given bit.",
        ElementsExtension::Elements(Elements::RightShiftWith64) => "Right-shift a 64-bit value by the given amount. Bits are filled with the given bit.",
        ElementsExtension::Elements(Elements::Rightmost8_1) => "Return the least significant 1  bits of an 8-bit value.",
        ElementsExtension::Elements(Elements::Rightmost8_2) => "Return the least significant 1  bits of an 8-bit value.",
        ElementsExtension::Elements(Elements::Rightmost8_4) => "Return the least significant 1  bits of an 8-bit value.",
        ElementsExtension::Elements(Elements::Rightmost16_1) => "Return the least significant 1  bit  of a 16-bit value.",
        ElementsExtension::Elements(Elements::Rightmost16_2) => "Return the least significant 2  bits of a 16-bit value.",
        ElementsExtension::Elements(Elements::Rightmost16_4) => "Return the least significant 4  bits of a 16-bit value.",
        ElementsExtension::Elements(Elements::Rightmost16_8) => "Return the least significant 8  bits of a 16-bit value.",
        ElementsExtension::Elements(Elements::Rightmost32_1) => "Return the least significant 1  bit  of a 32-bit value.",
        ElementsExtension::Elements(Elements::Rightmost32_2) => "Return the least significant 2  bits of a 32-bit value.",
        ElementsExtension::Elements(Elements::Rightmost32_4) => "Return the least significant 4  bits of a 32-bit value.",
        ElementsExtension::Elements(Elements::Rightmost32_8) => "Return the least significant 8  bits of a 32-bit value.",
        ElementsExtension::Elements(Elements::Rightmost32_16) => "Return the least significant 16 bits of a 32-bit value.",
        ElementsExtension::Elements(Elements::Rightmost64_1) => "Return the least significant 1  bits of a 64-bit value.",
        ElementsExtension::Elements(Elements::Rightmost64_2) => "Return the least significant 2  bits of a 64-bit value.",
        ElementsExtension::Elements(Elements::Rightmost64_4) => "Return the least significant 4  bits of a 64-bit value.",
        ElementsExtension::Elements(Elements::Rightmost64_8) => "Return the least significant 8  bits of a 64-bit value.",
        ElementsExtension::Elements(Elements::Rightmost64_16) => "Return the least significant 16 bits of a 64-bit value.",
        ElementsExtension::Elements(Elements::Rightmost64_32) => "Return the least significant 32 bits of a 64-bit value.",
        ElementsExtension::Elements(Elements::Some1) => "Check if a 1-bit  value is nonzero.",
        ElementsExtension::Elements(Elements::Some8) => "Check if an 8-bit value is nonzero.",
        ElementsExtension::Elements(Elements::Some16) => "Check if a 16-bit value is nonzero.",
        ElementsExtension::Elements(Elements::Some32) => "Check if a 32-bit value is nonzero.",
        ElementsExtension::Elements(Elements::Some64) => "Check if a 64-bit value is nonzero.",
        ElementsExtension::Elements(Elements::Verify) => r#"Assert that a bit is true.

## Panics
The assertion fails."#,
        ElementsExtension::Elements(Elements::Xor1) => "Bitwise XOR of two 1-bit  values.",
        ElementsExtension::Elements(Elements::Xor8) => "Bitwise XOR of two 8-bit  values.",
        ElementsExtension::Elements(Elements::Xor16) => "Bitwise XOR of two 16-bit values.",
        ElementsExtension::Elements(Elements::Xor32) => "Bitwise XOR of two 32-bit values.",
        ElementsExtension::Elements(Elements::Xor64) => "Bitwise XOR of two 64-bit values.",
        ElementsExtension::Elements(Elements::XorXor1) => "Bitwise XOR of three 1-bit  values.",
        ElementsExtension::Elements(Elements::XorXor8) => "Bitwise XOR of three 8-bit  values.",
        ElementsExtension::Elements(Elements::XorXor16) => "Bitwise XOR of three 16-bit values.",
        ElementsExtension::Elements(Elements::XorXor32) => "Bitwise XOR of three 32-bit values.",
        ElementsExtension::Elements(Elements::XorXor64) => "Bitwise XOR of three 64-bit values.",
        // Arithmetic
        ElementsExtension::Elements(Elements::Add8
        | Elements::Add16
        | Elements::Add32
        | Elements::Add64) => "Add two integers and return the carry.",
        ElementsExtension::Elements(Elements::Decrement8
        | Elements::Decrement16
        | Elements::Decrement32
        | Elements::Decrement64) => "Decrement an integer by one and return the borrow bit.",
        ElementsExtension::Elements(Elements::DivMod8
        | Elements::DivMod16
        | Elements::DivMod32
        | Elements::DivMod64) => "Divide the first integer by the second integer, and return the remainder.",
        ElementsExtension::Elements(Elements::DivMod128_64) => r#"Divide the 128-bit integer `a` by the 64-bit integer `b`.
Return a tuple of the quotient `q` and the remainer `r`.

Use this jet to recursively define wide integer divisions.

## Preconditions
1. `q` < 2^64
2. 2^63 ≤ `b`

Return `(u64::MAX, u64::MAX)` when the preconditions are not satisfied."#,
        ElementsExtension::Elements(Elements::Divide8
        | Elements::Divide16
        | Elements::Divide32
        | Elements::Divide64) => "Divide the first integer by the second integer.",
        ElementsExtension::Elements(Elements::Divides8
        | Elements::Divides16
        | Elements::Divides32
        | Elements::Divides64) => "Check if the first integer is divisible by the second integer.",
        ElementsExtension::Elements(Elements::FullAdd8
        | Elements::FullAdd16
        | Elements::FullAdd32
        | Elements::FullAdd64) => "Add two integers. Take a carry-in and return a carry-out.",
        ElementsExtension::Elements(Elements::FullDecrement8
        | Elements::FullDecrement16
        | Elements::FullDecrement32
        | Elements::FullDecrement64) => "Decrement an integer by one. Take a borrow-in and return a borrow-out.",
        ElementsExtension::Elements(Elements::FullIncrement8
        | Elements::FullIncrement16
        | Elements::FullIncrement32
        | Elements::FullIncrement64) => "Increment an integer by one. Take a carry-in and return a carry-out.",
        ElementsExtension::Elements(Elements::FullMultiply8
        | Elements::FullMultiply16
        | Elements::FullMultiply32
        | Elements::FullMultiply64) => "Helper for multiplying integers. Take the product of the first pair of integers and add the sum of the second pair.",
        ElementsExtension::Elements(Elements::FullSubtract8
        | Elements::FullSubtract16
        | Elements::FullSubtract32
        | Elements::FullSubtract64) => "Subtract the second integer from the first integer. Take a borrow-in and return a borrow-out.",
        ElementsExtension::Elements(Elements::Increment8
        | Elements::Increment16
        | Elements::Increment32
        | Elements::Increment64) => "Increment an integer by one and return the carry.",
        ElementsExtension::Elements(Elements::IsOne8
        | Elements::IsOne16
        | Elements::IsOne32
        | Elements::IsOne64)  => "Check if an integer is one.",
        ElementsExtension::Elements(Elements::IsZero8
        | Elements::IsZero16
        | Elements::IsZero32
        | Elements::IsZero64) => "Check if an integer is zero.",
        ElementsExtension::Elements(Elements::Le8
        | Elements::Le16
        | Elements::Le32
        | Elements::Le64) => "Check if an integer is less than or equal to another integer.",
        ElementsExtension::Elements(Elements::Lt8
        | Elements::Lt16
        | Elements::Lt32
        | Elements::Lt64) => "Check if an integer is less than another integer.",
        ElementsExtension::Elements(Elements::Max8
        | Elements::Max16
        | Elements::Max32
        | Elements::Max64) => "Return the bigger of two integers.",
        ElementsExtension::Elements(Elements::Median8
        | Elements::Median16
        | Elements::Median32
        | Elements::Median64) => "Return the median of three integers.",
        ElementsExtension::Elements(Elements::Min8
        | Elements::Min16
        | Elements::Min32
        | Elements::Min64) => "Return the smaller of two integers.",
        ElementsExtension::Elements(Elements::Modulo8
        |Elements::Modulo16
        |Elements::Modulo32
        |Elements::Modulo64) => "Compute the remainder after dividing both integers.",
        ElementsExtension::Elements(Elements::Multiply8) => "Multiply two integers. The output is a 16-bit integer.",
        ElementsExtension::Elements(Elements::Multiply16) => "Multiply two integers. The output is a 32-bit integer.",
        ElementsExtension::Elements(Elements::Multiply32) => "Multiply two integers. The output is a 64-bit integer.",
        ElementsExtension::Elements(Elements::Multiply64) => "Multiply two integers. The output is a 128-bit integer.",
        ElementsExtension::Elements(Elements::Negate8) => "Negate the integer (modulo 2⁸)  and return the borrow bit.",
        ElementsExtension::Elements(Elements::Negate16) => "Negate the integer (modulo 2¹⁶) and return the borrow bit.",
        ElementsExtension::Elements(Elements::Negate32) => "Negate the integer (modulo 2³²) and return the borrow bit.",
        ElementsExtension::Elements(Elements::Negate64) => "Negate the integer (modulo 2⁶⁴) and return the borrow bit.",
        ElementsExtension::Elements(Elements::One8) => "Return 1 as an 8-bit integer.",
        ElementsExtension::Elements(Elements::One16) => "Return 1 as a 16-bit integer.",
        ElementsExtension::Elements(Elements::One32) => "Return 1 as a 32-bit integer.",
        ElementsExtension::Elements(Elements::One64) => "Return 1 as a 64-bit integer.",
        ElementsExtension::Elements(Elements::Subtract8
        | Elements::Subtract16
        | Elements::Subtract32
        | Elements::Subtract64) => "Subtract the second integer from the first integer, and return the borrow bit.",
        // Hash functions
        ElementsExtension::Elements(Elements::Sha256Block) => "Update the given 256-bit midstate by running the SHA256 block compression function, using the given 512-bit block.",
        ElementsExtension::Elements(Elements::Sha256Ctx8Add1) => "Add 1   byte  to the SHA256 hash engine.",
        ElementsExtension::Elements(Elements::Sha256Ctx8Add2) => "Add 2   bytes to the SHA256 hash engine.",
        ElementsExtension::Elements(Elements::Sha256Ctx8Add4) => "Add 4   bytes to the SHA256 hash engine.",
        ElementsExtension::Elements(Elements::Sha256Ctx8Add8) => "Add 8   bytes to the SHA256 hash engine.",
        ElementsExtension::Elements(Elements::Sha256Ctx8Add16) => "Add 16  bytes to the SHA256 hash engine.",
        ElementsExtension::Elements(Elements::Sha256Ctx8Add32) => "Add 32  bytes to the SHA256 hash engine.",
        ElementsExtension::Elements(Elements::Sha256Ctx8Add64) => "Add 64  bytes to the SHA256 hash engine.",
        ElementsExtension::Elements(Elements::Sha256Ctx8Add128) => "Add 128 bytes to the SHA256 hash engine.",
        ElementsExtension::Elements(Elements::Sha256Ctx8Add256) => "Add 256 bytes to the SHA256 hash engine.",
        ElementsExtension::Elements(Elements::Sha256Ctx8Add512) => "Add 512 bytes to the SHA256 hash engine.",
        ElementsExtension::Elements(Elements::Sha256Ctx8AddBuffer511) => "Add a list of less than 512 bytes to the SHA256 hash engine.",
        ElementsExtension::Elements(Elements::Sha256Ctx8Finalize) => "Produce a hash from the current state of the SHA256 hash engine.",
        ElementsExtension::Elements(Elements::Sha256Ctx8Init) => "Initialize a default SHA256 hash engine.",
        ElementsExtension::Elements(Elements::Sha256Iv) => "Return the SHA256 initial value.",
        // Elliptic curve functions
        ElementsExtension::Elements(Elements::Decompress) => r#"Decompress a point into affine coordinates.

- Return `None` if the x-coordinate is not on the curve.
- Return `Some(ge)` even if the x-coordinate is not normalized."#,
        ElementsExtension::Elements(Elements::FeAdd) => "Add two field elements.",
        ElementsExtension::Elements(Elements::FeInvert) => "Compute the modular inverse of a field element.",
        ElementsExtension::Elements(Elements::FeIsOdd) => "Check if the canonical representative of the field element is odd.",
        ElementsExtension::Elements(Elements::FeIsZero) => "Check if the field element represents zero.",
        ElementsExtension::Elements(Elements::FeMultiply) => "Multiply two field elements.",
        ElementsExtension::Elements(Elements::FeMultiplyBeta) => "Multiply a field element by the canonical primitive cube root of unity (beta).",
        ElementsExtension::Elements(Elements::FeNegate) => "Negate a field element.",
        ElementsExtension::Elements(Elements::FeNormalize) => "Return the canonical representation of a field element.",
        ElementsExtension::Elements(Elements::FeSquare) => "Square a field element.",
        ElementsExtension::Elements(Elements::FeSquareRoot) => "Compute the modular square root of a field element if it exists.",
        ElementsExtension::Elements(Elements::GeIsOnCurve
        | Elements::GejIsOnCurve) => "Check if the given point satisfies the curve equation y² = x³ + 7.",
        ElementsExtension::Elements(Elements::GeNegate
        | Elements::GejNegate) => "Negate a point.",
        ElementsExtension::Elements(Elements::GejAdd) => "Add two points.",
        ElementsExtension::Elements(Elements::GejDouble) => "Double a point. If the result is the point at infinity, it is returned in canonical form.",
        ElementsExtension::Elements(Elements::GejEquiv) => "Check if two points represent the same point.",
        ElementsExtension::Elements(Elements::GejGeAdd) => "Add two points. If the result is the point at infinity, it is returned in canonical form.",
        ElementsExtension::Elements(Elements::GejGeAddEx) => "Add two points. Also return the ration of the `a`s z-coordinate and the result's z-coordinate. If the result is the point at infinity, it is returned in canonical form.",
        ElementsExtension::Elements(Elements::GejGeEquiv) => "Check if two points represent the same point.",
        ElementsExtension::Elements(Elements::GejInfinity) => "Return the canonical representation of the point at infinity.",
        ElementsExtension::Elements(Elements::GejIsInfinity) => "Check if the point represents infinity.",
        ElementsExtension::Elements(Elements::GejNormalize) => "Convert the point into affine coordinates with canonical field representatives. If the result is the point at infinity, it is returned in canonical form.",
        ElementsExtension::Elements(Elements::GejRescale) => "Change the representatives of a point by multiplying the z-coefficient by the given value.",
        ElementsExtension::Elements(Elements::GejXEquiv) => "Check if the point represents an affine point with the given x-coordinate.",
        ElementsExtension::Elements(Elements::GejYIsOdd) => "Check if the point represents an affine point with odd y-coordinate.",
        ElementsExtension::Elements(Elements::Generate) => "Multiply the generator point with the given scalar.",
        ElementsExtension::Elements(Elements::LinearCombination1) => "Compute the linear combination `b * a + c * g` for point `b` and scalars `a` and `c`, where `g` is the generator point.",
        ElementsExtension::Elements(Elements::LinearVerify1) => r#"Assert that a point `b` is equal to the linear combination `a.0 * a.1 + a.2 * g`, where `g` is the generator point.

## Panics
The assertion fails."#,
        ElementsExtension::Elements(Elements::PointVerify1) => r#"Assert that a point `b` is equal to the linear combination `a.0 * a.1 + a.2 * g`, where `g` is the generator point.

## Panics
- The assertion fails.
- Fails if the points cannot be decompressed."#,
        ElementsExtension::Elements(Elements::ScalarAdd) => "Add two scalars.",
        ElementsExtension::Elements(Elements::ScalarInvert) => "Compute the modular inverse of a scalar.",
        ElementsExtension::Elements(Elements::ScalarIsZero) => "Check if the scalar represents zero.",
        ElementsExtension::Elements(Elements::ScalarMultiply) => "Multiply two scalars.",
        ElementsExtension::Elements(Elements::ScalarMultiplyLambda) => "Multiply a scalar with the canonical primitive cube of unity (lambda)",
        ElementsExtension::Elements(Elements::ScalarNegate) => "Negate a scalar.",
        ElementsExtension::Elements(Elements::ScalarNormalize) => "Return the canonical representation of the scalar.",
        ElementsExtension::Elements(Elements::ScalarSquare) => "Square a scalar.",
        ElementsExtension::Elements(Elements::Scale) => "Multiply a point by a scalar.",
        ElementsExtension::Elements(Elements::HashToCurve) => r#"A cryptographic hash function that results in a point on the secp256k1 curve.

This matches the hash function used to map asset IDs to asset commitments."#,
        ElementsExtension::Elements(Elements::Swu) => r#"Algebraically distribute a field element over the secp256k1 curve as defined in
["Indifferentiable Hashing to Barreto-Naehrig Curves" by Pierre-Alain Fouque, Mehdi Tibouchi](https://inria.hal.science/hal-01094321/file/FT12.pdf).

While this by itself is not a cryptographic hash function, it can be used as a subroutine
in a [`hash_to_curve`] function. However, the distribution only approaches uniformity when it is called twice."#,
        // Digital Signatures
        ElementsExtension::Elements(Elements::Bip0340Verify) => r#"Assert that a Schnorr signature matches a public key and message.

## Panics
The assertion fails."#,
        ElementsExtension::Elements(Elements::CheckSigVerify) => r#"Assert that a Schnorr signature matches a public key and message, using a custom sighash mode.

## Panics
The assertion fails.

## Safety
This jet should not be used directly."#,
        // Bitcoin (without primitives)
        ElementsExtension::Elements(Elements::ParseLock) => "Parse an integer as a consensus-encoded Bitcoin lock time.",
        ElementsExtension::Elements(Elements::ParseSequence) => "Parse an integer as a consensus-encoded Bitcoin sequence number.",
        ElementsExtension::Elements(Elements::TapdataInit) => r#"Create a SHA256 context, initialized with a "TapData" tag."#,
        // Signature hash modes
        ElementsExtension::Elements(Elements::AnnexHash) => r#"Continue a SHA256 hash with an optional hash by appending the following:
- If there is no hash, then the byte `0x00`.
- If there is a hash, then the byte `0x01` followed by the given hash (32 bytes)."#,
        ElementsExtension::Elements(Elements::AssetAmountHash) => "Continue a SHA256 hash with the serialization of a confidential asset followed by the serialization of a amount.",
        ElementsExtension::Elements(Elements::BuildTapbranch) => r#"Return the SHA256 hash of the following:
- The hash of the ASCII string `TapBranch/elements` (32 bytes).
- The lexicographically smaller of the two inputs (32 bytes).
- The hash of the ASCII string `TapBranch/elements` again (32 bytes).
- The lexicographically larger of the two inputs (32 bytes).

This builds a taproot from two branches."#,
        ElementsExtension::Elements(Elements::BuildTapleafSimplicity) => r#"Return the SHA256 hash of the following:
- The hash of the ASCII string `TapBranch/elements` (32 bytes).
- The hash of the ASCII string `TapBranch/elements` again (32 bytes).
- The lexicographically smaller of the two inputs (32 bytes).
- The lexicographically larger of the two inputs (32 bytes).

This builds a taproot from two branches."#,
        ElementsExtension::Elements(Elements::BuildTaptweak) => r#"Implementation of `taproot_tweak_pubkey` from BIP-0341.

## Panics
1. The input x-only public key is off curve or exceeds the field size.
2. The internal hash value `t` exceeds the secp256k1 group order.
3. The generated tweaked point is infinity, and thus has no valid x-only public key.

Note that situations 2 and 3 are cryptographically impossible to occur."#,
        ElementsExtension::Elements(Elements::InputAmountsHash) => "Return the SHA256 hash of the serialization of each input UTXO's asset and amount fields.",
        ElementsExtension::Elements(Elements::InputAnnexesHash) => r#"Return the SHA256 hash of the concatenation of the following for every input:
- If the input has no annex, or isn't a taproot spend, then the byte `0x00`.
- If the input has an annex, then the byte `0x01` followed by the SHA256 hash of the annex (32 bytes)."#,
        ElementsExtension::Elements(Elements::InputOutpointsHash) => r#"Return the SHA256 hash of the concatenation of the following for every input:
- If the input is not a pegin, then the byte `0x00`.
- If the input is a pegin, then the byte `0x01` followed by the parent chain's genesis hash (32 bytes).
- The input's serialized previous transaction id (32 bytes).
- The input's previous transaction index in big endian format (4 bytes).

IMPORTANT: the index is serialized in big endian format rather than little endian format."#,
        ElementsExtension::Elements(Elements::InputScriptSigsHash) => r#"Return the SHA256 hash of the concatenation of the SHA256 hash of each input's scriptSig.

Note that if an input's UTXO uses segwit, then it's scriptSig will necessarily be the empty string. In
such cases we still use the SHA256 hash of the empty string."#,
        ElementsExtension::Elements(Elements::InputScriptsHash) => "Return the SHA256 hash of the concatenation of the SHA256 hash of each input UTXO's scriptPubKey.",
        ElementsExtension::Elements(Elements::InputSequencesHash) => r#"Return the SHA256 hash of the concatenation of the following for every input:
- The input's sequence number in big endian format (4 bytes).

IMPORTANT, the sequence number is serialized in big endian format rather than little endian format."#,
        ElementsExtension::Elements(Elements::InputUtxoHash) => r#"Return the SHA256 hash of the following:
- The serialization of the input UTXO's asset and amount fields.
- The SHA256 hash of the input UTXO's scriptPubKey.

Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::InputUtxosHash) => r#"Return the SHA256 hash of the following:
- The result of [`input_amounts_hash`] (32 bytes).
- The result of [`input_scripts_hash`] (32 bytes)."#,
        ElementsExtension::Elements(Elements::InputHash) => r#"Return the SHA256 hash of the following:
- If the input is not a pegin, then the byte `0x00`.
- If the input is a pegin, then the byte `0x01` followed by the parent chain's genesis hash (32 bytes).
- The input's serialized previous transaction id (32 bytes).
- The input's previous transaction index in big endian format (4 bytes).
- The input's sequence number in big endian format (4 bytes).
- If the input has no annex, or isn't a taproot spend, then the byte `0x00`.
- If the input has an annex, then the byte `0x01` followed by the SHA256 hash of the annex (32 bytes).

Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::InputsHash) => r#"Return the SHA256 hash of the following:
- The result of [`input_outpoints_hash`] (32 bytes).
- The result of [`input_sequences_hash`] (32 bytes).
- The result of [`input_annexes_hash`] (32 bytes)."#,
        ElementsExtension::Elements(Elements::IssuanceAssetAmountsHash) => r#"Return the SHA256 hash of the concatenation of the following for every input:
- If the input has no issuance then two bytes `0x00 0x00`.
- If the input is has a new issuance then the byte `0x01` followed by a serialization of the calculated issued
asset id (32 bytes) followed by the serialization of the (possibly confidential) issued asset amount (9
bytes or 33 bytes).
- If the input is has a reissuance then the byte `0x01` followed by a serialization of the issued asset id
(32 bytes), followed by the serialization of the (possibly confidential) issued asset amount (9 bytes or
33 bytes).

IMPORTANT: If there is an issuance but there are no asset issued (i.e. the amount is null) we serialize
the vase as the explicit 0 amount, (i.e. `0x01 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00`).

Note, the issuance asset id is serialized in the same format as an explicit asset id would be."#,
        ElementsExtension::Elements(Elements::IssuanceBlindingEntropyHash) => r#"Return the SHA256 hash of the concatenation of the following for every input:
- If the input has no issuance then the byte `0x00`.
- If the input is has a new issuance then the byte `0x01` followed by 32 `0x00` bytes and the new issuance's
contract hash field (32 bytes).
- If the input is has reissuance then the byte `0x01` followed by a serializaiton of the reissuance's blinding
nonce field (32 bytes) and the reissuance's entropy field (32 bytes).

Note that if the issuance is a new issuance then the blinding nonce field is 32 `0x00` bytes and new issuance's
contract hash."#,
        ElementsExtension::Elements(Elements::IssuanceRangeProofsHash) => r#"Return the SHA256 hash of the concatenation of the following for every input:
- The SHA256 hash of the range proof of the input's issuance asset amount (32 bytes).
- The SHA256 hash of the range proof of the input's issuance token amount (32 bytes).

Note that each the range proof is considered to be the empty string in the case there is no issuance, or if the
asset or token amount doesn't exist (i.e is null). The SHA256 hash of the empty string is still used in these
cases."#,
        ElementsExtension::Elements(Elements::IssuanceTokenAmountsHash) => r#"Return the SHA256 hash of the concatenation of the following for every input:
- If the input has no issuance then two bytes `0x00 0x00`.
- If the input is has a new issuance then the byte `0x01` followed by a serialization of the calculated issued
token id (32 bytes) followed by the serialization of the (possibly confidential) issued token amount (9
bytes or 33 bytes).
- If the input is has a reissuance then the byte `0x01` followed by a serialization of the issued token id
(32 bytes), followed by the serialization of the explicit 0 amount (i.e `0x01 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00`) (9 bytes).

IMPORTANT: If there is an issuance but there are no tokens issued (i.e. the amount is null) we serialize
the vase as the explicit 0 amount, (i.e. `0x01 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00`).

Note, the issuance token id is serialized in the same format as an explicit asset id would be."#,
        ElementsExtension::Elements(Elements::IssuanceHash) => r#"Return the SHA256 hash of the following:
1. The asset issuance:
    - If the input has no issuance then two bytes `0x00 0x00`.
    - If the input is has a new issuance then the byte `0x01` followed by a serialization of the calculated issued
    asset id (32 bytes) followed by the serialization of the (possibly confidential) issued asset amount (9 bytes or 33 bytes).
    - If the input is has a reissuance then the byte `0x01` followed by a serialization of the issued asset id
    (32 bytes), followed by the serialization of the (possibly confidential) issued asset amount (9 bytes or 33 bytes).
2. The token issuance:
    - If the input has no issuance then two bytes `0x00 0x00`.
    - If the input is has a new issuance then the byte `0x01` followed by a serialization of the calculated issued
    token id (32 bytes) followed by the serialization of the (possibly confidential) issued token amount (9 bytes or 33 bytes).
    - If the input is has a reissuance then the byte `0x01` followed by a serialization of the issued token id (32 bytes),
    followed by the serialization of the explicit 0 amount (i.e `0x01 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00`) (9 bytes).
3. The range proofs:
    - The SHA256 hash of the range proof of the input's issuance asset amount (32 bytes).
    - The SHA256 hash of the range proof of the input's issuance token amount (32 bytes).
4. The blinding entropy:
    - If the input has no issuance then the byte `0x00`.
    - If the input is has a new issuance then the byte `0x01` followed by 32 `0x00` bytes and the new issuance's
    contract hash field (32 bytes).
    - If the input is has reissuance then the byte `0x01` followed by a serializaiton of the reissuance's blinding
    nonce field (32 bytes) and the reissuance's entropy field (32 bytes).

Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::IssuancesHash) => r#"Return the SHA256 hash of the following:
- The result of [`issuance_asset_amounts_hash`] (32 bytes).
- The result of [`issuance_token_amounts_hash`] (32 bytes).
- The result of [`issuance_range_proofs_hash`] (32 bytes).
- The result of [`issuance_blinding_entropy_hash`] (32 bytes)."#,
        ElementsExtension::Elements(Elements::NonceHash) => "Continue the SHA256 hash with the serialization of an optional nonce.",
        ElementsExtension::Elements(Elements::OutpointHash) => r#"Continue the SHA256 hash with an optional pegin and an outpoint by appending the following:
- If the input is not a pegin, then the byte `0x00`.
- If the input is a pegin, then the byte `0x01` followed by the given parent genesis hash (32 bytes).
- The input's previous transaction id (32 bytes).
- The input's previous transaction index in big endian format (4 bytes)."#,
        ElementsExtension::Elements(Elements::OutputAmountsHash) => "Return the SHA256 hash of the serialization of each output's asset and amount fields.",
        ElementsExtension::Elements(Elements::OutputNoncesHash) => "Return the SHA256 hash of the serialization of each output's nonce field.",
        ElementsExtension::Elements(Elements::OutputRangeProofsHash) => r#"Return the SHA256 hash of the concatenation of the SHA256 hash of each output's range proof.

Note that if the output's amount is explicit then the range proof is considered the empty string."#,
        ElementsExtension::Elements(Elements::OutputScriptsHash) => "Return the SHA256 hash of the concatenation of the SHA256 hash of each output's scriptPubKey.",
        ElementsExtension::Elements(Elements::OutputSurjectionProofsHash) => r#"Return the SHA256 hash of the concatenation of the SHA256 hash of each output's surjection proof.

Note that if the output's asset is explicit then the surjection proof is considered the empty string."#,
        ElementsExtension::Elements(Elements::OutputHash) => r#"Return the SHA256 hash of the following:
- The serialization of the output's asset and amount fields.
- The serialization of the output's nonce field.
- The SHA256 hash of the output's scriptPubKey.
- The SHA256 hash of the output's range proof.

Return `None` if the output does not exist.

Note: the result of [`output_surjection_proofs_hash`] is specifically excluded because surjection proofs are dependent on the inputs as well as the output."#,
        ElementsExtension::Elements(Elements::OutputsHash) => r#"Return the SHA256 hash of the following:
- The result of [`output_amounts_hash`] (32 bytes).
- The result of [`output_nonces_hash`] (32 bytes).
- The result of [`output_scripts_hash`] (32 bytes).
- The result of [`output_range_proofs_hash`] (32 bytes).

Note: the result of [`output_surjection_proofs_hash`] is specifically excluded because surjection proofs are dependent on the inputs as well as the output. See also [`tx_hash`]."#,
        ElementsExtension::Elements(Elements::SigAllHash) => r#"Return the SHA256 hash of the following:
- The result of [`genesis_block_hash`] (32 bytes).
- The result of [`genesis_block_hash`] again (32 bytes).
- The result of [`tx_hash`] (32 bytes).
- The result of [`tap_env_hash`] (32 bytes).
- The result of [`current_index`] (Note: this is in big endian format) (4 bytes).

Note: the two copies of the [`genesis_block_hash`] values effectively makes this result a BIP-340 style tagged hash."#,
        ElementsExtension::Elements(Elements::TapEnvHash) => r#"Return the SHA256 hash of the following:
- The result of [`tapleaf_hash`] (32 bytes).
- The result of [`tappath_hash`] (32 bytes).
- The result of [`internal_key`] (32 bytes)."#,
        ElementsExtension::Elements(Elements::TapleafHash) => r#"Return the SHA256 hash of the following:
- The hash of the ASCII string `TapLeaf/elements` (32 bytes).
- The hash of the ASCII string `TapLeaf/elements` again (32 bytes).
- The result of [`tapleaf_version`] (1 byte).
- The byte `0x20` (1 byte).
- The result of [`script_cmr`] (32 bytes).

Note: this matches Element's modified BIP-0341 definition of tapleaf hash."#,
        ElementsExtension::Elements(Elements::TappathHash) => r#"Return a hash of the current input's control block excluding the leaf version and the taproot internal key.

Using the notation of BIP-0341, it returns the SHA256 hash of c[33: 33 + 32m]."#,
        ElementsExtension::Elements(Elements::TxHash) => r#"Return the SHA256 hash of the following:
- The result of [`version`] (Note: this is in big endian format) (4 bytes).
- The result of [`tx_lock_time`] (Note: this is in big endian format) (4 bytes).
- The result of [`inputs_hash`] (32 bytes).
- The result of [`outputs_hash`] (32 bytes).
- The result of [`issuances_hash`] (32 bytes).
- The result of [`output_surjection_proofs_hash`] (32 bytes).
- The result of [`input_utxos_hash`] (32 bytes)."#,
        // Time locks
        ElementsExtension::Elements(Elements::CheckLockDistance) => r#"Assert that the value returned by [`tx_lock_distance`] is greater than or equal to the given value.

## Panics
The assertion fails."#,
        ElementsExtension::Elements(Elements::CheckLockDuration) => r#"Assert that the value returned by [`tx_lock_duration`] is greater than or equal to the given value.

## Panics
The assertion fails"#,
        ElementsExtension::Elements(Elements::CheckLockHeight) => r#"Assert that the value returned by [`tx_lock_height`]   is greater than or equal to the given value.

## Panics
The assertion fails."#,
        ElementsExtension::Elements(Elements::CheckLockTime) => r#"Assert that the value returned by [`tx_lock_time`]     is greater than or equal to the given value.

## Panics
The assertion fails."#,
        ElementsExtension::Elements(Elements::TxIsFinal) => "Check if the sequence numbers of all transaction inputs are at their maximum value.",
        ElementsExtension::Elements(Elements::TxLockDistance) => "If [`version`] returns 2 or greater, then return the greatest valid [`Distance`] value of any transaction input. Return zeroes otherwise.",
        ElementsExtension::Elements(Elements::TxLockDuration) => "If [`version`] returns 2 or greater, then return the greatest valid [`Duration`] value of any transaction input. Return zeroes otherwise.",
        ElementsExtension::Elements(Elements::TxLockHeight) => "If [`tx_is_final`] returns false, then try to parse the transaction's lock time as a [`Height`] value. Return zeroes otherwise.",
        ElementsExtension::Elements(Elements::TxLockTime) => "If [`tx_is_final`] returns false, then try to parse the transaction's lock time as a [`Time`] value. Return zeroes otherwise.",
        // Issuance
        ElementsExtension::Elements(Elements::CalculateAsset) => "Calculate the issued asset id from a given entropy value.",
        ElementsExtension::Elements(Elements::CalculateConfidentialToken) => "Calculate the reissuance token id from a given entropy value for assets with confidential issued amounts.",
        ElementsExtension::Elements(Elements::CalculateExplicitToken) => "Calculate the reissuance token id from a given entropy value for assets with explicit issued amounts.",
        ElementsExtension::Elements(Elements::CalculateIssuanceEntropy) => r#"Calculate the entropy value from a given outpoint and contract hash.

This entropy value is used to compute issued asset and token IDs."#,
        ElementsExtension::Elements(Elements::Issuance) => r#"Return the kind of issuance of the input at the given index:
- Return `Some(Some(false))` if the input has new issuance.
- Return `Some(Some(true))` if the input as reissuance.
- Return `Some(None)` if the input has no issuance.
- Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::IssuanceAsset) => r#"Return the ID of the issued asset of the input at the given index:
- Return `Some(Some(x))` if the input has issuance with asset id `x`.
- Return `Some(None)` if the input has no issuance.
- Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::IssuanceEntropy) => r#"Return the issuance entropy of the input at the given index:
- Return `Some(Some(x))` if the input has reissuance with entropy `x` or if there is new issuance whose computed entropy is `x`.
- Return `Some(Some(x))` if the input has no issuance.
- Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::IssuanceToken) => r#"Return the reissuance token of the input at the given index:
- Return `Some(Some(x))` if the input has issuance with the reissuance token ID `x`.
- Return `Some(None)` if the input has no issuance.
- Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::LbtcAsset) => "Return the asset for Liquid Bitcoin.",
        // Transaction
        ElementsExtension::Elements(Elements::CurrentAmount) => "Return the [`input_amount`] at the [`current_index`].",
        ElementsExtension::Elements(Elements::CurrentAnnexHash) => "Return the [`input_annex_hash`] at th [`current_index`].",
        ElementsExtension::Elements(Elements::CurrentAsset) => "Return the [`input_asset`] at the [`current_index`].",
        ElementsExtension::Elements(Elements::CurrentIndex) => "Return the index of the current txin.",
        ElementsExtension::Elements(Elements::CurrentIssuanceAssetAmount) => "Return the [`issuance_asset_amount`] at the [`current_index`].",
        ElementsExtension::Elements(Elements::CurrentIssuanceAssetProof) => "Return the [`issuance_asset_proof`]  at the [`current_index`].",
        ElementsExtension::Elements(Elements::CurrentIssuanceTokenAmount) => "Return the [`issuance_token_amount`] at the [`current_index`].",
        ElementsExtension::Elements(Elements::CurrentIssuanceTokenProof) => "Return the [`issuance_token_proof`]  at the [`current_index`].",
        ElementsExtension::Elements(Elements::CurrentNewIssuanceContract) => "Return the [`new_issuance_contract`] at the [`current_index`].",
        ElementsExtension::Elements(Elements::CurrentPegin) => "Return the [`input_pegin`] at the [`current_index`].",
        ElementsExtension::Elements(Elements::CurrentPrevOutpoint) => "Return the previous outpoint of the current txin.",
        ElementsExtension::Elements(Elements::CurrentReissuanceBlinding) => "Return the [`reissuance_blinding`] at the [`current_index`].",
        ElementsExtension::Elements(Elements::CurrentReissuanceEntropy) => "Return the [`reissuance_entropy`]  at the [`current_index`].",
        ElementsExtension::Elements(Elements::CurrentScriptHash) => "Return the SHA256 hash of the scriptPubKey of the UTXO of the current txin.",
        ElementsExtension::Elements(Elements::CurrentScriptSigHash) => r#"Return the SHA256 hash of the scriptSig of the current txin.

SegWit UTXOs enforce scriptSig to be the empty string. In such cases, we return the SHA256 hash of the empty string."#,
        ElementsExtension::Elements(Elements::CurrentSequence) => r#"Return the nSequence of the current txin.

Use this jet to obtain the raw, encoded sequence number.
Use [`tx_lock_distance`] to obtain a relative block height, or [`tx_lock_duration`] to obtain a relative UNIX timestamp, in a safe manner."#,
        ElementsExtension::Elements(Elements::GenesisBlockHash) => "Return the SHA256 hash of the genesis block.",
        ElementsExtension::Elements(Elements::InputAmount) => r#"Return the asset id and the asset amount at the given input index.

Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::InputAnnexHash) => r#"Return the SHA256 hash of the annex at the given input:
- Return `Some(Some(x))` if the input has an annex that hashes to `x`.
- Return `Some(None`) if the input has no annex.
- Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::InputAsset) => r#"Return the asset id of the input at the given index.

Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::InputPegin) => r#"Return the parent genesis block hash if the input at the given index is a peg-in.

- Return `Some(None)` if the input is not a peg-in.
- Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::InputPrevOutpoint) => r#"Return the previous outpoint of the input at the given index.

Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::InputScriptHash) => r#"Return the SHA256 hash of the scriptPubKey of the UTXO of the input at the given index.

Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::InputScriptSigHash) => r#"Return the SHA256 hash of the scriptSigKey of the input at the given index.

Return `None` if the input does not exist.

SegWit UTXOs enforce scriptSig to be the empty string. In such cases, we return the SHA256 hash of the empty string."#,
        ElementsExtension::Elements(Elements::InputSequence) => r#"Return the nSequence of the input at the given index.

Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::InternalKey) => r#"Return the internal key of the current input.

We assume that Simplicity can be spent in Taproot outputs only, so there always exists an internal key."#,
        ElementsExtension::Elements(Elements::IssuanceAssetAmount) => r#"Return the possibly confidential amount of the issuance if the input at the given index has an issuance.

- Return `Some(None)` if the input does not have an issuance.
- Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::IssuanceAssetProof) => r#"Return the SHA256 hash of the range proof for the amount of the issuance at the given input index.

- Return the hash of the empty string if the input does not have an issuance.
- Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::IssuanceTokenAmount) => r#"Return the possibly confidential amount of the reissuance tokens if the input at the given index has an issuance.

- Return `Some(Some(Right(0)))` if the input is itself a reissuance.
- Return `Some(None)` if the input does not have an issuance.
- Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::IssuanceTokenProof) => r#"Return the SHA256 hash of the range proof for the amount of the reissuance tokens at the given input index.

- Return the hash of the empty string if the input does not have an issuance.
- Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::LockTime) => "Return the lock time of the transaction.",
        ElementsExtension::Elements(Elements::NewIssuanceContract) => r#"Return the contract hash for the new issuance at the given input index.

- Return `Some(None)` if the input does not have a new issuance.
- Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::NumInputs) => "Return the number of inputs of the transaction.",
        ElementsExtension::Elements(Elements::NumOutputs) => "Return the number of outputs of the transaction.",
        ElementsExtension::Elements(Elements::OutputAmount) => r#"Return the asset amount of the output at the given index.

Return `None` if the output does not exist."#,
        ElementsExtension::Elements(Elements::OutputAsset) => r#"Return the asset id of the output at the given index.

Return `None` if the output does not exist."#,
        ElementsExtension::Elements(Elements::OutputIsFee) => r#"Check if the output at the given index is a fee output.

Return `None` if the output does not exist."#,
        ElementsExtension::Elements(Elements::OutputNonce) => r#"Return the nonce of the output at the given index.

- Return `Some(None)` if the output does not have a nonce.
- Return `None` if the output does not exist."#,
        ElementsExtension::Elements(Elements::OutputNullDatum) => r#"Return the `b`-th entry of a null data (`OP_RETURN`) output at index `a`.

- Return `Some(Some(Right(Right(x-1))))` if the entry is `OP_x` for `x` in the range 1..=16.
- Return `Some(Some(Right(Left(0))))` if the entry is `OP_1NEGATE`.
- Return `Some(Some(Right(Left(1))))` if the entry is `OP_RESERVED`.
- Return `Some(Some(Left((x, hash))))` if the entry is pushed data. `hash` is the SHA256 hash of the data pushed and `x` indicates how the data was pushed:
    - `x == 0` means the push was an immediate 0 to 75 bytes.
    - `x == 1` means the push was an `OP_PUSHDATA1`.
    - `x == 2` means the push was an `OP_PUSHDATA2`.
    - `x == 3` means the push was an `OP_PUSHDATA4`.
- Return `Some(None)` if the null data has fewer than `b` entries.
- Return `None` if the output is not a null data output.

Use this jet to read peg-out data from an output."#,
        ElementsExtension::Elements(Elements::OutputRangeProof) => r#"Return the SHA256 hash of the range proof of the output at the given index.

Return `None` if the output does not exist."#,
        ElementsExtension::Elements(Elements::OutputScriptHash) => r#"Return the SHA256 hash of the scriptPubKey of the output at the given index.

Return `None` if the output does not exist."#,
        ElementsExtension::Elements(Elements::OutputSurjectionProof) => r#"Return the SHA256 hash of the surjection proof of the output at the given index.

Return `None` if the output does not exist."#,
        ElementsExtension::Elements(Elements::ReissuanceBlinding) => r#"Return the blinding factor used for the reissuance at the given input index.

- Return `Some(None)` if the input does not have a reissuance.
- Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::ReissuanceEntropy) => r#"Return the entropy used for the reissuance at the given input index.

- Return `Some(None)` if the input does not have a reissuance.
- Return `None` if the input does not exist."#,
        ElementsExtension::Elements(Elements::ScriptCMR) => r#"Return the CMR of the Simplicity program in the current input.

This is the CMR of the currently executed Simplicity program."#,
        ElementsExtension::Elements(Elements::TapleafVersion) => r#"Return the tap leaf version of the current input.

We assume that Simplicity can be spent in Taproot outputs only, so there always exists a tap leaf."#,
        ElementsExtension::Elements(Elements::Tappath) => r#"Return the SHA256 hash of the tap path of the current input.

We assume that Simplicity can be spent in Taproot outputs only, so there always exists a tap path."#,
        ElementsExtension::Elements(Elements::TotalFee) => r#"Return the total amount of fees paid to the given asset id.

Return zero for any asset without fees."#,
        ElementsExtension::Elements(Elements::TransactionId) => "Return the transaction ID.",
        ElementsExtension::Elements(Elements::Version) => "Return the version number of the transaction.",
        ElementsExtension::GetOpcodeFromScript => "given an index, return the opcode at that index.",
        ElementsExtension::GetPubkeyFromScript => r#"Each pubkey is encoded as: [OP_PUSHBYTES_33][0x02 or 0x03][32 bytes X coordinate].
Returns X only pubkeys from a script at the given index.
Index should point to OP_PUSHBYTES_33 opcode in the script."#,
    }
}

/// Category of an Elements jet.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Category {
    MultiBitLogic,
    Arithmetic,
    HashFunctions,
    EllipticCurveFunctions,
    DigitalSignatures,
    BitcoinWithoutPrimitives,
    SignatureHashModes,
    TimeLocks,
    Issuance,
    Transaction,
    Script,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::MultiBitLogic => f.write_str("multi_bit_logic"),
            Category::Arithmetic => f.write_str("arithmetic"),
            Category::HashFunctions => f.write_str("hash_functions"),
            Category::EllipticCurveFunctions => f.write_str("elliptic_curve_functions"),
            Category::DigitalSignatures => f.write_str("digital_signatures"),
            Category::BitcoinWithoutPrimitives => f.write_str("bitcoin_without_primitives"),
            Category::SignatureHashModes => f.write_str("signature_hash_modes"),
            Category::TimeLocks => f.write_str("time_locks"),
            Category::Issuance => f.write_str("issuance"),
            Category::Transaction => f.write_str("transaction"),
            Category::Script => f.write_str("script"),
        }
    }
}

impl Category {
    pub const ALL: [Self; 11] = [
        Self::MultiBitLogic,
        Self::Arithmetic,
        Self::HashFunctions,
        Self::EllipticCurveFunctions,
        Self::DigitalSignatures,
        Self::BitcoinWithoutPrimitives,
        Self::SignatureHashModes,
        Self::TimeLocks,
        Self::Issuance,
        Self::Transaction,
        Self::Script,
    ];

    pub const fn documentation(self) -> &'static str {
        match self {
            Self::MultiBitLogic => {
                r#"//! # Multi-bit logic
//!
//! This module defines jets that operate on strings of bits."#
            }
            Self::Arithmetic => {
                r#"//! # Arithmetic
//!
//! This module defines jets that compute arithmetic functions."#
            }
            Self::HashFunctions => {
                r#"//! # Hash functions
//!
//! This module defines jets for computing SHA-256 hashes.
//! Be aware that SHA-256 padding isn't provided and messages should be manually padded."#
            }
            Self::EllipticCurveFunctions => {
                r#"//! # Elliptic curve functions
//!
//! This module defines jets that replicate the functional behavior of (a specific version of) libsecp256k1's elliptic curve operations <https://github.com/bitcoin-core/secp256k1/tree/v0.3.0>.
//! The functions defined here return precisely the same field and point representatives that the corresponding libsecp256k1's functions do, with a few exceptions with the way the point at infinity is handled."#
            }
            Self::DigitalSignatures => {
                r#"//! # Digital signatures
//!
//! This module defines jets for verifying digital signatures."#
            }
            Self::BitcoinWithoutPrimitives => {
                r#"//! # Bitcoin (without primitives)
//!
//! This module defines jets for Bitcoin that work without the Bitcoin transaction environmnent.
//! These jets are not recommended for non-Bitcoin(-like) applications."#
            }
            Self::SignatureHashModes => {
                r#"//! # Elements signature hash modes
//!
//! This module defines jets for computing signature hashes of Elements transactions."#
            }
            Self::TimeLocks => {
                r#"//! # Time locks
//!
//! This module defines jets for checking time locks of Elements transactions."#
            }
            Self::Issuance => {
                r#"//! # Issuance
//!
//! This module defines jets for handling issuance of Elements assets or tokens."#
            }
            Self::Transaction => {
                r#"//! Transaction
//!
//! This module defines jets to introspect the contents of an Elements transaction."#
            }
            Self::Script => {
                r#"//! # Script
//!
//! This module defines jets for handling Elements scripts."#
            }
        }
    }

    /// Check if a jet is contained in the category.
    #[cfg(test)]
    pub fn contains(&self, jet: &ElementsExtension) -> bool {
        match self {
            Self::MultiBitLogic => MULTI_BIT_LOGIC.contains(jet),
            Self::Arithmetic => ARITHMETIC.contains(jet),
            Self::HashFunctions => HASH_FUNCTIONS.contains(jet),
            Self::EllipticCurveFunctions => ELLIPTIC_CURVE_FUNCTIONS.contains(jet),
            Self::DigitalSignatures => DIGITAL_SIGNATURES.contains(jet),
            Self::BitcoinWithoutPrimitives => BITCOIN.contains(jet),
            Self::SignatureHashModes => SIGNATURE_HASH_MODES.contains(jet),
            Self::TimeLocks => TIME_LOCKS.contains(jet),
            Self::Issuance => ISSUANCE.contains(jet),
            Self::Transaction => TRANSACTION.contains(jet),
            Self::Script => SCRIPT.contains(jet),
        }
    }

    /// Iterate over all jets in the category.
    pub fn iter(&self) -> impl Iterator<Item = &ElementsExtension> {
        match self {
            Self::MultiBitLogic => MULTI_BIT_LOGIC.iter(),
            Self::Arithmetic => ARITHMETIC.iter(),
            Self::HashFunctions => HASH_FUNCTIONS.iter(),
            Self::EllipticCurveFunctions => ELLIPTIC_CURVE_FUNCTIONS.iter(),
            Self::DigitalSignatures => DIGITAL_SIGNATURES.iter(),
            Self::BitcoinWithoutPrimitives => BITCOIN.iter(),
            Self::SignatureHashModes => SIGNATURE_HASH_MODES.iter(),
            Self::TimeLocks => TIME_LOCKS.iter(),
            Self::Issuance => ISSUANCE.iter(),
            Self::Transaction => TRANSACTION.iter(),
            Self::Script => SCRIPT.iter(),
        }
    }
}

// Core
#[rustfmt::skip]
const MULTI_BIT_LOGIC: [ElementsExtension; 212] = [
    ElementsExtension::Elements(Elements::All8), ElementsExtension::Elements(Elements::All16), ElementsExtension::Elements(Elements::All32), ElementsExtension::Elements(Elements::All64), ElementsExtension::Elements(Elements::And1), ElementsExtension::Elements(Elements::And8), ElementsExtension::Elements(Elements::And16), ElementsExtension::Elements(Elements::And32), ElementsExtension::Elements(Elements::And64), ElementsExtension::Elements(Elements::Ch1), ElementsExtension::Elements(Elements::Ch8), ElementsExtension::Elements(Elements::Ch16), ElementsExtension::Elements(Elements::Ch32), ElementsExtension::Elements(Elements::Ch64), ElementsExtension::Elements(Elements::Complement1), ElementsExtension::Elements(Elements::Complement8), ElementsExtension::Elements(Elements::Complement16), ElementsExtension::Elements(Elements::Complement32), ElementsExtension::Elements(Elements::Complement64), ElementsExtension::Elements(Elements::Eq1), ElementsExtension::Elements(Elements::Eq8), ElementsExtension::Elements(Elements::Eq16), ElementsExtension::Elements(Elements::Eq32), ElementsExtension::Elements(Elements::Eq64), ElementsExtension::Elements(Elements::Eq256), ElementsExtension::Elements(Elements::FullLeftShift16_1), ElementsExtension::Elements(Elements::FullLeftShift16_2), ElementsExtension::Elements(Elements::FullLeftShift16_4), ElementsExtension::Elements(Elements::FullLeftShift16_8), ElementsExtension::Elements(Elements::FullLeftShift32_1), ElementsExtension::Elements(Elements::FullLeftShift32_2), ElementsExtension::Elements(Elements::FullLeftShift32_4), ElementsExtension::Elements(Elements::FullLeftShift32_8), ElementsExtension::Elements(Elements::FullLeftShift32_16), ElementsExtension::Elements(Elements::FullLeftShift64_1), ElementsExtension::Elements(Elements::FullLeftShift64_2), ElementsExtension::Elements(Elements::FullLeftShift64_4), ElementsExtension::Elements(Elements::FullLeftShift64_8), ElementsExtension::Elements(Elements::FullLeftShift64_16), ElementsExtension::Elements(Elements::FullLeftShift64_32), ElementsExtension::Elements(Elements::FullLeftShift8_1), ElementsExtension::Elements(Elements::FullLeftShift8_2), ElementsExtension::Elements(Elements::FullLeftShift8_4), ElementsExtension::Elements(Elements::FullRightShift16_1), ElementsExtension::Elements(Elements::FullRightShift16_2), ElementsExtension::Elements(Elements::FullRightShift16_4), ElementsExtension::Elements(Elements::FullRightShift16_8), ElementsExtension::Elements(Elements::FullRightShift32_1), ElementsExtension::Elements(Elements::FullRightShift32_2), ElementsExtension::Elements(Elements::FullRightShift32_4), ElementsExtension::Elements(Elements::FullRightShift32_8), ElementsExtension::Elements(Elements::FullRightShift32_16), ElementsExtension::Elements(Elements::FullRightShift64_1), ElementsExtension::Elements(Elements::FullRightShift64_2), ElementsExtension::Elements(Elements::FullRightShift64_4), ElementsExtension::Elements(Elements::FullRightShift64_8), ElementsExtension::Elements(Elements::FullRightShift64_16), ElementsExtension::Elements(Elements::FullRightShift64_32), ElementsExtension::Elements(Elements::FullRightShift8_1), ElementsExtension::Elements(Elements::FullRightShift8_2), ElementsExtension::Elements(Elements::FullRightShift8_4), ElementsExtension::Elements(Elements::High1), ElementsExtension::Elements(Elements::High8), ElementsExtension::Elements(Elements::High16), ElementsExtension::Elements(Elements::High32), ElementsExtension::Elements(Elements::High64), ElementsExtension::Elements(Elements::LeftExtend16_32), ElementsExtension::Elements(Elements::LeftExtend16_64), ElementsExtension::Elements(Elements::LeftExtend1_8), ElementsExtension::Elements(Elements::LeftExtend1_16), ElementsExtension::Elements(Elements::LeftExtend1_32), ElementsExtension::Elements(Elements::LeftExtend1_64), ElementsExtension::Elements(Elements::LeftExtend32_64), ElementsExtension::Elements(Elements::LeftExtend8_16), ElementsExtension::Elements(Elements::LeftExtend8_32), ElementsExtension::Elements(Elements::LeftExtend8_64), ElementsExtension::Elements(Elements::LeftPadHigh16_32), ElementsExtension::Elements(Elements::LeftPadHigh16_64), ElementsExtension::Elements(Elements::LeftPadHigh1_8), ElementsExtension::Elements(Elements::LeftPadHigh1_16), ElementsExtension::Elements(Elements::LeftPadHigh1_32), ElementsExtension::Elements(Elements::LeftPadHigh1_64), ElementsExtension::Elements(Elements::LeftPadHigh32_64), ElementsExtension::Elements(Elements::LeftPadHigh8_16), ElementsExtension::Elements(Elements::LeftPadHigh8_32), ElementsExtension::Elements(Elements::LeftPadHigh8_64), ElementsExtension::Elements(Elements::LeftPadLow16_32), ElementsExtension::Elements(Elements::LeftPadLow16_64), ElementsExtension::Elements(Elements::LeftPadLow1_8), ElementsExtension::Elements(Elements::LeftPadLow1_16), ElementsExtension::Elements(Elements::LeftPadLow1_32), ElementsExtension::Elements(Elements::LeftPadLow1_64), ElementsExtension::Elements(Elements::LeftPadLow32_64), ElementsExtension::Elements(Elements::LeftPadLow8_16), ElementsExtension::Elements(Elements::LeftPadLow8_32), ElementsExtension::Elements(Elements::LeftPadLow8_64), ElementsExtension::Elements(Elements::LeftRotate8), ElementsExtension::Elements(Elements::LeftRotate16), ElementsExtension::Elements(Elements::LeftRotate32), ElementsExtension::Elements(Elements::LeftRotate64), ElementsExtension::Elements(Elements::LeftShift8), ElementsExtension::Elements(Elements::LeftShift16), ElementsExtension::Elements(Elements::LeftShift32), ElementsExtension::Elements(Elements::LeftShift64), ElementsExtension::Elements(Elements::LeftShiftWith8), ElementsExtension::Elements(Elements::LeftShiftWith16), ElementsExtension::Elements(Elements::LeftShiftWith32), ElementsExtension::Elements(Elements::LeftShiftWith64), ElementsExtension::Elements(Elements::Leftmost16_1), ElementsExtension::Elements(Elements::Leftmost16_2), ElementsExtension::Elements(Elements::Leftmost16_4), ElementsExtension::Elements(Elements::Leftmost16_8), ElementsExtension::Elements(Elements::Leftmost32_1), ElementsExtension::Elements(Elements::Leftmost32_2), ElementsExtension::Elements(Elements::Leftmost32_4), ElementsExtension::Elements(Elements::Leftmost32_8), ElementsExtension::Elements(Elements::Leftmost32_16), ElementsExtension::Elements(Elements::Leftmost64_1), ElementsExtension::Elements(Elements::Leftmost64_2), ElementsExtension::Elements(Elements::Leftmost64_4), ElementsExtension::Elements(Elements::Leftmost64_8), ElementsExtension::Elements(Elements::Leftmost64_16), ElementsExtension::Elements(Elements::Leftmost64_32), ElementsExtension::Elements(Elements::Leftmost8_1), ElementsExtension::Elements(Elements::Leftmost8_2), ElementsExtension::Elements(Elements::Leftmost8_4), ElementsExtension::Elements(Elements::Low1), ElementsExtension::Elements(Elements::Low8), ElementsExtension::Elements(Elements::Low16), ElementsExtension::Elements(Elements::Low32), ElementsExtension::Elements(Elements::Low64), ElementsExtension::Elements(Elements::Maj1), ElementsExtension::Elements(Elements::Maj8), ElementsExtension::Elements(Elements::Maj16), ElementsExtension::Elements(Elements::Maj32), ElementsExtension::Elements(Elements::Maj64), ElementsExtension::Elements(Elements::Or1), ElementsExtension::Elements(Elements::Or8), ElementsExtension::Elements(Elements::Or16), ElementsExtension::Elements(Elements::Or32), ElementsExtension::Elements(Elements::Or64), ElementsExtension::Elements(Elements::RightExtend16_32), ElementsExtension::Elements(Elements::RightExtend16_64), ElementsExtension::Elements(Elements::RightExtend32_64), ElementsExtension::Elements(Elements::RightExtend8_16), ElementsExtension::Elements(Elements::RightExtend8_32), ElementsExtension::Elements(Elements::RightExtend8_64), ElementsExtension::Elements(Elements::RightPadHigh16_32), ElementsExtension::Elements(Elements::RightPadHigh16_64), ElementsExtension::Elements(Elements::RightPadHigh1_8), ElementsExtension::Elements(Elements::RightPadHigh1_16), ElementsExtension::Elements(Elements::RightPadHigh1_32), ElementsExtension::Elements(Elements::RightPadHigh1_64), ElementsExtension::Elements(Elements::RightPadHigh32_64), ElementsExtension::Elements(Elements::RightPadHigh8_16), ElementsExtension::Elements(Elements::RightPadHigh8_32), ElementsExtension::Elements(Elements::RightPadHigh8_64), ElementsExtension::Elements(Elements::RightPadLow16_32), ElementsExtension::Elements(Elements::RightPadLow16_64), ElementsExtension::Elements(Elements::RightPadLow1_8), ElementsExtension::Elements(Elements::RightPadLow1_16), ElementsExtension::Elements(Elements::RightPadLow1_32), ElementsExtension::Elements(Elements::RightPadLow1_64), ElementsExtension::Elements(Elements::RightPadLow32_64), ElementsExtension::Elements(Elements::RightPadLow8_16), ElementsExtension::Elements(Elements::RightPadLow8_32), ElementsExtension::Elements(Elements::RightPadLow8_64), ElementsExtension::Elements(Elements::RightRotate8), ElementsExtension::Elements(Elements::RightRotate16), ElementsExtension::Elements(Elements::RightRotate32), ElementsExtension::Elements(Elements::RightRotate64), ElementsExtension::Elements(Elements::RightShift8), ElementsExtension::Elements(Elements::RightShift16), ElementsExtension::Elements(Elements::RightShift32), ElementsExtension::Elements(Elements::RightShift64), ElementsExtension::Elements(Elements::RightShiftWith8), ElementsExtension::Elements(Elements::RightShiftWith16), ElementsExtension::Elements(Elements::RightShiftWith32), ElementsExtension::Elements(Elements::RightShiftWith64), ElementsExtension::Elements(Elements::Rightmost16_1), ElementsExtension::Elements(Elements::Rightmost16_2), ElementsExtension::Elements(Elements::Rightmost16_4), ElementsExtension::Elements(Elements::Rightmost16_8), ElementsExtension::Elements(Elements::Rightmost32_1), ElementsExtension::Elements(Elements::Rightmost32_2), ElementsExtension::Elements(Elements::Rightmost32_4), ElementsExtension::Elements(Elements::Rightmost32_8), ElementsExtension::Elements(Elements::Rightmost32_16), ElementsExtension::Elements(Elements::Rightmost64_1), ElementsExtension::Elements(Elements::Rightmost64_2), ElementsExtension::Elements(Elements::Rightmost64_4), ElementsExtension::Elements(Elements::Rightmost64_8), ElementsExtension::Elements(Elements::Rightmost64_16), ElementsExtension::Elements(Elements::Rightmost64_32), ElementsExtension::Elements(Elements::Rightmost8_1), ElementsExtension::Elements(Elements::Rightmost8_2), ElementsExtension::Elements(Elements::Rightmost8_4), ElementsExtension::Elements(Elements::Some1), ElementsExtension::Elements(Elements::Some8), ElementsExtension::Elements(Elements::Some16), ElementsExtension::Elements(Elements::Some32), ElementsExtension::Elements(Elements::Some64), ElementsExtension::Elements(Elements::Xor1), ElementsExtension::Elements(Elements::Xor8), ElementsExtension::Elements(Elements::Xor16), ElementsExtension::Elements(Elements::Xor32), ElementsExtension::Elements(Elements::Xor64), ElementsExtension::Elements(Elements::XorXor1), ElementsExtension::Elements(Elements::XorXor8), ElementsExtension::Elements(Elements::XorXor16), ElementsExtension::Elements(Elements::XorXor32), ElementsExtension::Elements(Elements::XorXor64)
];
#[rustfmt::skip]
const ARITHMETIC: [ElementsExtension; 93] = [
    ElementsExtension::Elements(Elements::Add8), ElementsExtension::Elements(Elements::Add16), ElementsExtension::Elements(Elements::Add32), ElementsExtension::Elements(Elements::Add64), ElementsExtension::Elements(Elements::Decrement8), ElementsExtension::Elements(Elements::Decrement16), ElementsExtension::Elements(Elements::Decrement32), ElementsExtension::Elements(Elements::Decrement64), ElementsExtension::Elements(Elements::DivMod8), ElementsExtension::Elements(Elements::DivMod16), ElementsExtension::Elements(Elements::DivMod32), ElementsExtension::Elements(Elements::DivMod64), ElementsExtension::Elements(Elements::DivMod128_64), ElementsExtension::Elements(Elements::Divide8), ElementsExtension::Elements(Elements::Divide16), ElementsExtension::Elements(Elements::Divide32), ElementsExtension::Elements(Elements::Divide64), ElementsExtension::Elements(Elements::Divides8), ElementsExtension::Elements(Elements::Divides16), ElementsExtension::Elements(Elements::Divides32), ElementsExtension::Elements(Elements::Divides64), ElementsExtension::Elements(Elements::FullAdd8), ElementsExtension::Elements(Elements::FullAdd16), ElementsExtension::Elements(Elements::FullAdd32), ElementsExtension::Elements(Elements::FullAdd64), ElementsExtension::Elements(Elements::FullDecrement8), ElementsExtension::Elements(Elements::FullDecrement16), ElementsExtension::Elements(Elements::FullDecrement32), ElementsExtension::Elements(Elements::FullDecrement64), ElementsExtension::Elements(Elements::FullIncrement8), ElementsExtension::Elements(Elements::FullIncrement16), ElementsExtension::Elements(Elements::FullIncrement32), ElementsExtension::Elements(Elements::FullIncrement64), ElementsExtension::Elements(Elements::FullMultiply8), ElementsExtension::Elements(Elements::FullMultiply16), ElementsExtension::Elements(Elements::FullMultiply32), ElementsExtension::Elements(Elements::FullMultiply64), ElementsExtension::Elements(Elements::FullSubtract8), ElementsExtension::Elements(Elements::FullSubtract16), ElementsExtension::Elements(Elements::FullSubtract32), ElementsExtension::Elements(Elements::FullSubtract64), ElementsExtension::Elements(Elements::Increment8), ElementsExtension::Elements(Elements::Increment16), ElementsExtension::Elements(Elements::Increment32), ElementsExtension::Elements(Elements::Increment64), ElementsExtension::Elements(Elements::IsOne8), ElementsExtension::Elements(Elements::IsOne16), ElementsExtension::Elements(Elements::IsOne32), ElementsExtension::Elements(Elements::IsOne64), ElementsExtension::Elements(Elements::IsZero8), ElementsExtension::Elements(Elements::IsZero16), ElementsExtension::Elements(Elements::IsZero32), ElementsExtension::Elements(Elements::IsZero64), ElementsExtension::Elements(Elements::Le8), ElementsExtension::Elements(Elements::Le16), ElementsExtension::Elements(Elements::Le32), ElementsExtension::Elements(Elements::Le64), ElementsExtension::Elements(Elements::Lt8), ElementsExtension::Elements(Elements::Lt16), ElementsExtension::Elements(Elements::Lt32), ElementsExtension::Elements(Elements::Lt64), ElementsExtension::Elements(Elements::Max8), ElementsExtension::Elements(Elements::Max16), ElementsExtension::Elements(Elements::Max32), ElementsExtension::Elements(Elements::Max64), ElementsExtension::Elements(Elements::Median8), ElementsExtension::Elements(Elements::Median16), ElementsExtension::Elements(Elements::Median32), ElementsExtension::Elements(Elements::Median64), ElementsExtension::Elements(Elements::Min8), ElementsExtension::Elements(Elements::Min16), ElementsExtension::Elements(Elements::Min32), ElementsExtension::Elements(Elements::Min64), ElementsExtension::Elements(Elements::Modulo8), ElementsExtension::Elements(Elements::Modulo16), ElementsExtension::Elements(Elements::Modulo32), ElementsExtension::Elements(Elements::Modulo64), ElementsExtension::Elements(Elements::Multiply8), ElementsExtension::Elements(Elements::Multiply16), ElementsExtension::Elements(Elements::Multiply32), ElementsExtension::Elements(Elements::Multiply64), ElementsExtension::Elements(Elements::Negate8), ElementsExtension::Elements(Elements::Negate16), ElementsExtension::Elements(Elements::Negate32), ElementsExtension::Elements(Elements::Negate64), ElementsExtension::Elements(Elements::One8), ElementsExtension::Elements(Elements::One16), ElementsExtension::Elements(Elements::One32), ElementsExtension::Elements(Elements::One64), ElementsExtension::Elements(Elements::Subtract8), ElementsExtension::Elements(Elements::Subtract16), ElementsExtension::Elements(Elements::Subtract32), ElementsExtension::Elements(Elements::Subtract64)
];
#[rustfmt::skip]
const HASH_FUNCTIONS: [ElementsExtension; 15] = [
    ElementsExtension::Elements(Elements::Sha256Block), ElementsExtension::Elements(Elements::Sha256Ctx8Add1), ElementsExtension::Elements(Elements::Sha256Ctx8Add2), ElementsExtension::Elements(Elements::Sha256Ctx8Add4), ElementsExtension::Elements(Elements::Sha256Ctx8Add8), ElementsExtension::Elements(Elements::Sha256Ctx8Add16), ElementsExtension::Elements(Elements::Sha256Ctx8Add32), ElementsExtension::Elements(Elements::Sha256Ctx8Add64), ElementsExtension::Elements(Elements::Sha256Ctx8Add128), ElementsExtension::Elements(Elements::Sha256Ctx8Add256), ElementsExtension::Elements(Elements::Sha256Ctx8Add512), ElementsExtension::Elements(Elements::Sha256Ctx8AddBuffer511), ElementsExtension::Elements(Elements::Sha256Ctx8Finalize), ElementsExtension::Elements(Elements::Sha256Ctx8Init), ElementsExtension::Elements(Elements::Sha256Iv)
];
#[rustfmt::skip]
const ELLIPTIC_CURVE_FUNCTIONS: [ElementsExtension; 42] = [
    ElementsExtension::Elements(Elements::Decompress), ElementsExtension::Elements(Elements::FeAdd), ElementsExtension::Elements(Elements::FeInvert), ElementsExtension::Elements(Elements::FeIsOdd), ElementsExtension::Elements(Elements::FeIsZero), ElementsExtension::Elements(Elements::FeMultiply), ElementsExtension::Elements(Elements::FeMultiplyBeta), ElementsExtension::Elements(Elements::FeNegate), ElementsExtension::Elements(Elements::FeNormalize), ElementsExtension::Elements(Elements::FeSquare), ElementsExtension::Elements(Elements::FeSquareRoot), ElementsExtension::Elements(Elements::GeIsOnCurve), ElementsExtension::Elements(Elements::GeNegate), ElementsExtension::Elements(Elements::GejAdd), ElementsExtension::Elements(Elements::GejDouble), ElementsExtension::Elements(Elements::GejEquiv), ElementsExtension::Elements(Elements::GejGeAdd), ElementsExtension::Elements(Elements::GejGeAddEx), ElementsExtension::Elements(Elements::GejGeEquiv), ElementsExtension::Elements(Elements::GejInfinity), ElementsExtension::Elements(Elements::GejIsInfinity), ElementsExtension::Elements(Elements::GejIsOnCurve), ElementsExtension::Elements(Elements::GejNegate), ElementsExtension::Elements(Elements::GejNormalize), ElementsExtension::Elements(Elements::GejRescale), ElementsExtension::Elements(Elements::GejXEquiv), ElementsExtension::Elements(Elements::GejYIsOdd), ElementsExtension::Elements(Elements::Generate), ElementsExtension::Elements(Elements::HashToCurve), ElementsExtension::Elements(Elements::LinearCombination1), ElementsExtension::Elements(Elements::LinearVerify1), ElementsExtension::Elements(Elements::PointVerify1), ElementsExtension::Elements(Elements::ScalarAdd), ElementsExtension::Elements(Elements::ScalarInvert), ElementsExtension::Elements(Elements::ScalarIsZero), ElementsExtension::Elements(Elements::ScalarMultiply), ElementsExtension::Elements(Elements::ScalarMultiplyLambda), ElementsExtension::Elements(Elements::ScalarNegate), ElementsExtension::Elements(Elements::ScalarNormalize), ElementsExtension::Elements(Elements::ScalarSquare), ElementsExtension::Elements(Elements::Scale), ElementsExtension::Elements(Elements::Swu)
];
#[rustfmt::skip]
const DIGITAL_SIGNATURES: [ElementsExtension; 1] = [
    ElementsExtension::Elements(Elements::Bip0340Verify)
];
#[rustfmt::skip]
const BITCOIN: [ElementsExtension; 3] = [
    ElementsExtension::Elements(Elements::ParseLock), ElementsExtension::Elements(Elements::ParseSequence), ElementsExtension::Elements(Elements::TapdataInit),
];
// Elements
#[rustfmt::skip]
const SIGNATURE_HASH_MODES: [ElementsExtension; 35] = [
    ElementsExtension::Elements(Elements::AnnexHash), ElementsExtension::Elements(Elements::AssetAmountHash), ElementsExtension::Elements(Elements::BuildTapbranch), ElementsExtension::Elements(Elements::BuildTapleafSimplicity), ElementsExtension::Elements(Elements::BuildTaptweak), ElementsExtension::Elements(Elements::InputAmountsHash), ElementsExtension::Elements(Elements::InputAnnexesHash), ElementsExtension::Elements(Elements::InputHash), ElementsExtension::Elements(Elements::InputOutpointsHash), ElementsExtension::Elements(Elements::InputScriptSigsHash), ElementsExtension::Elements(Elements::InputScriptsHash), ElementsExtension::Elements(Elements::InputSequencesHash), ElementsExtension::Elements(Elements::InputUtxoHash), ElementsExtension::Elements(Elements::InputUtxosHash), ElementsExtension::Elements(Elements::InputsHash), ElementsExtension::Elements(Elements::IssuanceAssetAmountsHash), ElementsExtension::Elements(Elements::IssuanceBlindingEntropyHash), ElementsExtension::Elements(Elements::IssuanceHash), ElementsExtension::Elements(Elements::IssuanceRangeProofsHash), ElementsExtension::Elements(Elements::IssuanceTokenAmountsHash), ElementsExtension::Elements(Elements::IssuancesHash), ElementsExtension::Elements(Elements::NonceHash), ElementsExtension::Elements(Elements::OutpointHash), ElementsExtension::Elements(Elements::OutputAmountsHash), ElementsExtension::Elements(Elements::OutputHash), ElementsExtension::Elements(Elements::OutputNoncesHash), ElementsExtension::Elements(Elements::OutputRangeProofsHash), ElementsExtension::Elements(Elements::OutputScriptsHash), ElementsExtension::Elements(Elements::OutputSurjectionProofsHash), ElementsExtension::Elements(Elements::OutputsHash), ElementsExtension::Elements(Elements::SigAllHash), ElementsExtension::Elements(Elements::TapEnvHash), ElementsExtension::Elements(Elements::TapleafHash), ElementsExtension::Elements(Elements::TappathHash), ElementsExtension::Elements(Elements::TxHash)
];
#[rustfmt::skip]
const TIME_LOCKS: [ElementsExtension; 9] = [
    ElementsExtension::Elements(Elements::CheckLockDistance), ElementsExtension::Elements(Elements::CheckLockDuration), ElementsExtension::Elements(Elements::CheckLockHeight), ElementsExtension::Elements(Elements::CheckLockTime), ElementsExtension::Elements(Elements::TxIsFinal), ElementsExtension::Elements(Elements::TxLockDistance), ElementsExtension::Elements(Elements::TxLockDuration), ElementsExtension::Elements(Elements::TxLockHeight), ElementsExtension::Elements(Elements::TxLockTime)
];
#[rustfmt::skip]
const ISSUANCE: [ElementsExtension; 9] = [
    ElementsExtension::Elements(Elements::CalculateAsset), ElementsExtension::Elements(Elements::CalculateConfidentialToken), ElementsExtension::Elements(Elements::CalculateExplicitToken), ElementsExtension::Elements(Elements::CalculateIssuanceEntropy), ElementsExtension::Elements(Elements::Issuance), ElementsExtension::Elements(Elements::IssuanceAsset), ElementsExtension::Elements(Elements::IssuanceEntropy), ElementsExtension::Elements(Elements::IssuanceToken), ElementsExtension::Elements(Elements::LbtcAsset)
];
#[rustfmt::skip]
const TRANSACTION: [ElementsExtension; 50] = [
    ElementsExtension::Elements(Elements::CurrentAmount), ElementsExtension::Elements(Elements::CurrentAnnexHash), ElementsExtension::Elements(Elements::CurrentAsset), ElementsExtension::Elements(Elements::CurrentIndex), ElementsExtension::Elements(Elements::CurrentIssuanceAssetAmount), ElementsExtension::Elements(Elements::CurrentIssuanceAssetProof), ElementsExtension::Elements(Elements::CurrentIssuanceTokenAmount), ElementsExtension::Elements(Elements::CurrentIssuanceTokenProof), ElementsExtension::Elements(Elements::CurrentNewIssuanceContract), ElementsExtension::Elements(Elements::CurrentPegin), ElementsExtension::Elements(Elements::CurrentPrevOutpoint), ElementsExtension::Elements(Elements::CurrentReissuanceBlinding), ElementsExtension::Elements(Elements::CurrentReissuanceEntropy), ElementsExtension::Elements(Elements::CurrentScriptHash), ElementsExtension::Elements(Elements::CurrentScriptSigHash), ElementsExtension::Elements(Elements::CurrentSequence), ElementsExtension::Elements(Elements::GenesisBlockHash), ElementsExtension::Elements(Elements::InputAmount), ElementsExtension::Elements(Elements::InputAnnexHash), ElementsExtension::Elements(Elements::InputAsset), ElementsExtension::Elements(Elements::InputPegin), ElementsExtension::Elements(Elements::InputPrevOutpoint), ElementsExtension::Elements(Elements::InputScriptHash), ElementsExtension::Elements(Elements::InputScriptSigHash), ElementsExtension::Elements(Elements::InputSequence), ElementsExtension::Elements(Elements::InternalKey), ElementsExtension::Elements(Elements::IssuanceAssetAmount), ElementsExtension::Elements(Elements::IssuanceAssetProof), ElementsExtension::Elements(Elements::IssuanceTokenAmount), ElementsExtension::Elements(Elements::IssuanceTokenProof), ElementsExtension::Elements(Elements::LockTime), ElementsExtension::Elements(Elements::NewIssuanceContract), ElementsExtension::Elements(Elements::NumInputs), ElementsExtension::Elements(Elements::NumOutputs), ElementsExtension::Elements(Elements::OutputAmount), ElementsExtension::Elements(Elements::OutputAsset), ElementsExtension::Elements(Elements::OutputIsFee), ElementsExtension::Elements(Elements::OutputNonce), ElementsExtension::Elements(Elements::OutputNullDatum), ElementsExtension::Elements(Elements::OutputRangeProof), ElementsExtension::Elements(Elements::OutputScriptHash), ElementsExtension::Elements(Elements::OutputSurjectionProof), ElementsExtension::Elements(Elements::ReissuanceBlinding), ElementsExtension::Elements(Elements::ReissuanceEntropy), ElementsExtension::Elements(Elements::ScriptCMR), ElementsExtension::Elements(Elements::TapleafVersion), ElementsExtension::Elements(Elements::Tappath), ElementsExtension::Elements(Elements::TotalFee), ElementsExtension::Elements(Elements::TransactionId), ElementsExtension::Elements(Elements::Version)
];
#[rustfmt::skip]
const SCRIPT: [ElementsExtension; 2] = [
    ElementsExtension::GetOpcodeFromScript, ElementsExtension::GetPubkeyFromScript
];

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const DISABLED: [ElementsExtension; 2] = [
        ElementsExtension::Elements(Elements::CheckSigVerify), ElementsExtension::Elements(Elements::Verify)
    ];

    #[test]
    fn correct_categorization() {
        for jet in ElementsExtension::ALL {
            match Category::ALL.iter().find(|c| c.contains(&jet)) {
                Some(category) => {
                    if let Some(other) = Category::ALL
                        .into_iter()
                        .filter(|other| other != category)
                        .find(|other| other.contains(&jet))
                    {
                        panic!("{jet} is assigned conflicting categories {category} and {other}");
                    }
                }
                None => {
                    assert!(DISABLED.contains(&jet), "{jet} is not categorized");
                }
            }
        }
    }
}
