use crate::num::NonZeroPow2Usize;
use crate::types::BuiltinAlias::*;
use crate::types::UIntType::*;
use crate::types::*;

use simplicity_unchained::jets::unchained::ElementsExtension;

use simplicity::jet::Elements;

fn tuple<A: Into<AliasedType>, I: IntoIterator<Item = A>>(elements: I) -> AliasedType {
    AliasedType::tuple(elements.into_iter().map(A::into))
}

fn array<A: Into<AliasedType>>(element: A, size: usize) -> AliasedType {
    AliasedType::array(element.into(), size)
}

fn list<A: Into<AliasedType>>(element: A, bound: usize) -> AliasedType {
    AliasedType::list(element.into(), NonZeroPow2Usize::new(bound).unwrap())
}

fn bool() -> AliasedType {
    AliasedType::boolean()
}

fn either<A: Into<AliasedType>, B: Into<AliasedType>>(left: A, right: B) -> AliasedType {
    AliasedType::either(left.into(), right.into())
}

fn option<A: Into<AliasedType>>(inner: A) -> AliasedType {
    AliasedType::option(inner.into())
}

pub fn source_type(jet: ElementsExtension) -> Vec<AliasedType> {
    match jet {
        /*
         * ==============================
         *          Core jets
         * ==============================
         *
         * Multi-bit logic
         */
        ElementsExtension::Elements(Elements::Low1)
        | ElementsExtension::Elements(Elements::Low8)
        | ElementsExtension::Elements(Elements::Low16)
        | ElementsExtension::Elements(Elements::Low32)
        | ElementsExtension::Elements(Elements::Low64)
        | ElementsExtension::Elements(Elements::High1)
        | ElementsExtension::Elements(Elements::High8)
        | ElementsExtension::Elements(Elements::High16)
        | ElementsExtension::Elements(Elements::High32)
        | ElementsExtension::Elements(Elements::High64) => vec![],
        ElementsExtension::Elements(Elements::Verify) => vec![bool()],
        ElementsExtension::Elements(Elements::Complement1)
        | ElementsExtension::Elements(Elements::Some1)
        | ElementsExtension::Elements(Elements::LeftPadLow1_8)
        | ElementsExtension::Elements(Elements::LeftPadLow1_16)
        | ElementsExtension::Elements(Elements::LeftPadLow1_32)
        | ElementsExtension::Elements(Elements::LeftPadLow1_64)
        | ElementsExtension::Elements(Elements::LeftPadHigh1_8)
        | ElementsExtension::Elements(Elements::LeftPadHigh1_16)
        | ElementsExtension::Elements(Elements::LeftPadHigh1_32)
        | ElementsExtension::Elements(Elements::LeftPadHigh1_64)
        | ElementsExtension::Elements(Elements::LeftExtend1_8)
        | ElementsExtension::Elements(Elements::LeftExtend1_16)
        | ElementsExtension::Elements(Elements::LeftExtend1_32)
        | ElementsExtension::Elements(Elements::LeftExtend1_64)
        | ElementsExtension::Elements(Elements::RightPadLow1_8)
        | ElementsExtension::Elements(Elements::RightPadLow1_16)
        | ElementsExtension::Elements(Elements::RightPadLow1_32)
        | ElementsExtension::Elements(Elements::RightPadLow1_64)
        | ElementsExtension::Elements(Elements::RightPadHigh1_8)
        | ElementsExtension::Elements(Elements::RightPadHigh1_16)
        | ElementsExtension::Elements(Elements::RightPadHigh1_32)
        | ElementsExtension::Elements(Elements::RightPadHigh1_64) => vec![U1.into()],
        ElementsExtension::Elements(Elements::Complement8)
        | ElementsExtension::Elements(Elements::Some8)
        | ElementsExtension::Elements(Elements::All8)
        | ElementsExtension::Elements(Elements::Leftmost8_1)
        | ElementsExtension::Elements(Elements::Leftmost8_2)
        | ElementsExtension::Elements(Elements::Leftmost8_4)
        | ElementsExtension::Elements(Elements::Rightmost8_1)
        | ElementsExtension::Elements(Elements::Rightmost8_2)
        | ElementsExtension::Elements(Elements::Rightmost8_4)
        | ElementsExtension::Elements(Elements::LeftPadLow8_16)
        | ElementsExtension::Elements(Elements::LeftPadLow8_32)
        | ElementsExtension::Elements(Elements::LeftPadLow8_64)
        | ElementsExtension::Elements(Elements::LeftPadHigh8_16)
        | ElementsExtension::Elements(Elements::LeftPadHigh8_32)
        | ElementsExtension::Elements(Elements::LeftPadHigh8_64)
        | ElementsExtension::Elements(Elements::LeftExtend8_16)
        | ElementsExtension::Elements(Elements::LeftExtend8_32)
        | ElementsExtension::Elements(Elements::LeftExtend8_64)
        | ElementsExtension::Elements(Elements::RightPadLow8_16)
        | ElementsExtension::Elements(Elements::RightPadLow8_32)
        | ElementsExtension::Elements(Elements::RightPadLow8_64)
        | ElementsExtension::Elements(Elements::RightPadHigh8_16)
        | ElementsExtension::Elements(Elements::RightPadHigh8_32)
        | ElementsExtension::Elements(Elements::RightPadHigh8_64)
        | ElementsExtension::Elements(Elements::RightExtend8_16)
        | ElementsExtension::Elements(Elements::RightExtend8_32)
        | ElementsExtension::Elements(Elements::RightExtend8_64) => vec![U8.into()],
        ElementsExtension::Elements(Elements::Complement16)
        | ElementsExtension::Elements(Elements::Some16)
        | ElementsExtension::Elements(Elements::All16)
        | ElementsExtension::Elements(Elements::Leftmost16_1)
        | ElementsExtension::Elements(Elements::Leftmost16_2)
        | ElementsExtension::Elements(Elements::Leftmost16_4)
        | ElementsExtension::Elements(Elements::Leftmost16_8)
        | ElementsExtension::Elements(Elements::Rightmost16_1)
        | ElementsExtension::Elements(Elements::Rightmost16_2)
        | ElementsExtension::Elements(Elements::Rightmost16_4)
        | ElementsExtension::Elements(Elements::Rightmost16_8)
        | ElementsExtension::Elements(Elements::LeftPadLow16_32)
        | ElementsExtension::Elements(Elements::LeftPadLow16_64)
        | ElementsExtension::Elements(Elements::LeftPadHigh16_32)
        | ElementsExtension::Elements(Elements::LeftPadHigh16_64)
        | ElementsExtension::Elements(Elements::LeftExtend16_32)
        | ElementsExtension::Elements(Elements::LeftExtend16_64)
        | ElementsExtension::Elements(Elements::RightPadLow16_32)
        | ElementsExtension::Elements(Elements::RightPadLow16_64)
        | ElementsExtension::Elements(Elements::RightPadHigh16_32)
        | ElementsExtension::Elements(Elements::RightPadHigh16_64)
        | ElementsExtension::Elements(Elements::RightExtend16_32)
        | ElementsExtension::Elements(Elements::RightExtend16_64) => vec![U16.into()],
        ElementsExtension::Elements(Elements::Complement32)
        | ElementsExtension::Elements(Elements::Some32)
        | ElementsExtension::Elements(Elements::All32)
        | ElementsExtension::Elements(Elements::Leftmost32_1)
        | ElementsExtension::Elements(Elements::Leftmost32_2)
        | ElementsExtension::Elements(Elements::Leftmost32_4)
        | ElementsExtension::Elements(Elements::Leftmost32_8)
        | ElementsExtension::Elements(Elements::Leftmost32_16)
        | ElementsExtension::Elements(Elements::Rightmost32_1)
        | ElementsExtension::Elements(Elements::Rightmost32_2)
        | ElementsExtension::Elements(Elements::Rightmost32_4)
        | ElementsExtension::Elements(Elements::Rightmost32_8)
        | ElementsExtension::Elements(Elements::Rightmost32_16)
        | ElementsExtension::Elements(Elements::LeftPadLow32_64)
        | ElementsExtension::Elements(Elements::LeftPadHigh32_64)
        | ElementsExtension::Elements(Elements::LeftExtend32_64)
        | ElementsExtension::Elements(Elements::RightPadLow32_64)
        | ElementsExtension::Elements(Elements::RightPadHigh32_64)
        | ElementsExtension::Elements(Elements::RightExtend32_64) => vec![U32.into()],
        ElementsExtension::Elements(Elements::Complement64)
        | ElementsExtension::Elements(Elements::Some64)
        | ElementsExtension::Elements(Elements::All64)
        | ElementsExtension::Elements(Elements::Leftmost64_1)
        | ElementsExtension::Elements(Elements::Leftmost64_2)
        | ElementsExtension::Elements(Elements::Leftmost64_4)
        | ElementsExtension::Elements(Elements::Leftmost64_8)
        | ElementsExtension::Elements(Elements::Leftmost64_16)
        | ElementsExtension::Elements(Elements::Leftmost64_32)
        | ElementsExtension::Elements(Elements::Rightmost64_1)
        | ElementsExtension::Elements(Elements::Rightmost64_2)
        | ElementsExtension::Elements(Elements::Rightmost64_4)
        | ElementsExtension::Elements(Elements::Rightmost64_8)
        | ElementsExtension::Elements(Elements::Rightmost64_16)
        | ElementsExtension::Elements(Elements::Rightmost64_32) => vec![U64.into()],
        ElementsExtension::Elements(Elements::And1)
        | ElementsExtension::Elements(Elements::Or1)
        | ElementsExtension::Elements(Elements::Xor1)
        | ElementsExtension::Elements(Elements::Eq1) => {
            vec![U1.into(), U1.into()]
        }
        ElementsExtension::Elements(Elements::And8)
        | ElementsExtension::Elements(Elements::Or8)
        | ElementsExtension::Elements(Elements::Xor8)
        | ElementsExtension::Elements(Elements::Eq8) => {
            vec![U8.into(), U8.into()]
        }
        ElementsExtension::Elements(Elements::And16)
        | ElementsExtension::Elements(Elements::Or16)
        | ElementsExtension::Elements(Elements::Xor16)
        | ElementsExtension::Elements(Elements::Eq16) => {
            vec![U16.into(), U16.into()]
        }
        ElementsExtension::Elements(Elements::And32)
        | ElementsExtension::Elements(Elements::Or32)
        | ElementsExtension::Elements(Elements::Xor32)
        | ElementsExtension::Elements(Elements::Eq32) => {
            vec![U32.into(), U32.into()]
        }
        ElementsExtension::Elements(Elements::And64)
        | ElementsExtension::Elements(Elements::Or64)
        | ElementsExtension::Elements(Elements::Xor64)
        | ElementsExtension::Elements(Elements::Eq64) => {
            vec![U64.into(), U64.into()]
        }
        ElementsExtension::Elements(Elements::Eq256) => vec![U256.into(), U256.into()],
        ElementsExtension::Elements(Elements::Maj1)
        | ElementsExtension::Elements(Elements::XorXor1)
        | ElementsExtension::Elements(Elements::Ch1) => vec![U1.into(), U1.into(), U1.into()],
        ElementsExtension::Elements(Elements::Maj8)
        | ElementsExtension::Elements(Elements::XorXor8)
        | ElementsExtension::Elements(Elements::Ch8) => vec![U8.into(), U8.into(), U8.into()],
        ElementsExtension::Elements(Elements::Maj16)
        | ElementsExtension::Elements(Elements::XorXor16)
        | ElementsExtension::Elements(Elements::Ch16) => {
            vec![U16.into(), tuple([U16, U16])]
        }
        ElementsExtension::Elements(Elements::Maj32)
        | ElementsExtension::Elements(Elements::XorXor32)
        | ElementsExtension::Elements(Elements::Ch32) => {
            vec![U32.into(), tuple([U32, U32])]
        }
        ElementsExtension::Elements(Elements::Maj64)
        | ElementsExtension::Elements(Elements::XorXor64)
        | ElementsExtension::Elements(Elements::Ch64) => {
            vec![U64.into(), tuple([U64, U64])]
        }
        ElementsExtension::Elements(Elements::FullLeftShift8_1) => vec![U8.into(), U1.into()],
        ElementsExtension::Elements(Elements::FullLeftShift8_2) => vec![U8.into(), U2.into()],
        ElementsExtension::Elements(Elements::FullLeftShift8_4) => vec![U8.into(), U4.into()],
        ElementsExtension::Elements(Elements::FullLeftShift16_1) => vec![U16.into(), U1.into()],
        ElementsExtension::Elements(Elements::FullLeftShift16_2) => vec![U16.into(), U2.into()],
        ElementsExtension::Elements(Elements::FullLeftShift16_4) => vec![U16.into(), U4.into()],
        ElementsExtension::Elements(Elements::FullLeftShift16_8) => vec![U16.into(), U8.into()],
        ElementsExtension::Elements(Elements::FullLeftShift32_1) => vec![U32.into(), U1.into()],
        ElementsExtension::Elements(Elements::FullLeftShift32_2) => vec![U32.into(), U2.into()],
        ElementsExtension::Elements(Elements::FullLeftShift32_4) => vec![U32.into(), U4.into()],
        ElementsExtension::Elements(Elements::FullLeftShift32_8) => vec![U32.into(), U8.into()],
        ElementsExtension::Elements(Elements::FullLeftShift32_16) => vec![U32.into(), U16.into()],
        ElementsExtension::Elements(Elements::FullLeftShift64_1) => vec![U64.into(), U1.into()],
        ElementsExtension::Elements(Elements::FullLeftShift64_2) => vec![U64.into(), U2.into()],
        ElementsExtension::Elements(Elements::FullLeftShift64_4) => vec![U64.into(), U4.into()],
        ElementsExtension::Elements(Elements::FullLeftShift64_8) => vec![U64.into(), U8.into()],
        ElementsExtension::Elements(Elements::FullLeftShift64_16) => vec![U64.into(), U16.into()],
        ElementsExtension::Elements(Elements::FullLeftShift64_32) => vec![U64.into(), U32.into()],
        ElementsExtension::Elements(Elements::FullRightShift8_1) => vec![U1.into(), U8.into()],
        ElementsExtension::Elements(Elements::FullRightShift8_2) => vec![U2.into(), U8.into()],
        ElementsExtension::Elements(Elements::FullRightShift8_4) => vec![U4.into(), U8.into()],
        ElementsExtension::Elements(Elements::FullRightShift16_1) => vec![U1.into(), U16.into()],
        ElementsExtension::Elements(Elements::FullRightShift16_2) => vec![U2.into(), U16.into()],
        ElementsExtension::Elements(Elements::FullRightShift16_4) => vec![U4.into(), U16.into()],
        ElementsExtension::Elements(Elements::FullRightShift16_8) => vec![U8.into(), U16.into()],
        ElementsExtension::Elements(Elements::FullRightShift32_1) => vec![U1.into(), U32.into()],
        ElementsExtension::Elements(Elements::FullRightShift32_2) => vec![U2.into(), U32.into()],
        ElementsExtension::Elements(Elements::FullRightShift32_4) => vec![U4.into(), U32.into()],
        ElementsExtension::Elements(Elements::FullRightShift32_8) => vec![U8.into(), U32.into()],
        ElementsExtension::Elements(Elements::FullRightShift32_16) => vec![U16.into(), U32.into()],
        ElementsExtension::Elements(Elements::FullRightShift64_1) => vec![U1.into(), U64.into()],
        ElementsExtension::Elements(Elements::FullRightShift64_2) => vec![U2.into(), U64.into()],
        ElementsExtension::Elements(Elements::FullRightShift64_4) => vec![U4.into(), U64.into()],
        ElementsExtension::Elements(Elements::FullRightShift64_8) => vec![U8.into(), U64.into()],
        ElementsExtension::Elements(Elements::FullRightShift64_16) => vec![U16.into(), U64.into()],
        ElementsExtension::Elements(Elements::FullRightShift64_32) => vec![U32.into(), U64.into()],
        ElementsExtension::Elements(Elements::LeftShiftWith8)
        | ElementsExtension::Elements(Elements::RightShiftWith8) => {
            vec![U1.into(), U4.into(), U8.into()]
        }
        ElementsExtension::Elements(Elements::LeftShiftWith16)
        | ElementsExtension::Elements(Elements::RightShiftWith16) => {
            vec![U1.into(), U4.into(), U16.into()]
        }
        ElementsExtension::Elements(Elements::LeftShiftWith32)
        | ElementsExtension::Elements(Elements::RightShiftWith32) => {
            vec![U1.into(), U8.into(), U32.into()]
        }
        ElementsExtension::Elements(Elements::LeftShiftWith64)
        | ElementsExtension::Elements(Elements::RightShiftWith64) => {
            vec![U1.into(), U8.into(), U64.into()]
        }
        ElementsExtension::Elements(Elements::LeftShift8)
        | ElementsExtension::Elements(Elements::RightShift8)
        | ElementsExtension::Elements(Elements::LeftRotate8)
        | ElementsExtension::Elements(Elements::RightRotate8) => vec![U4.into(), U8.into()],
        ElementsExtension::Elements(Elements::LeftShift16)
        | ElementsExtension::Elements(Elements::RightShift16)
        | ElementsExtension::Elements(Elements::LeftRotate16)
        | ElementsExtension::Elements(Elements::RightRotate16) => vec![U4.into(), U16.into()],
        ElementsExtension::Elements(Elements::LeftShift32)
        | ElementsExtension::Elements(Elements::RightShift32)
        | ElementsExtension::Elements(Elements::LeftRotate32)
        | ElementsExtension::Elements(Elements::RightRotate32) => vec![U8.into(), U32.into()],
        ElementsExtension::Elements(Elements::LeftShift64)
        | ElementsExtension::Elements(Elements::RightShift64)
        | ElementsExtension::Elements(Elements::LeftRotate64)
        | ElementsExtension::Elements(Elements::RightRotate64) => vec![U8.into(), U64.into()],
        /*
         * Arithmetic
         */
        ElementsExtension::Elements(Elements::One8)
        | ElementsExtension::Elements(Elements::One16)
        | ElementsExtension::Elements(Elements::One32)
        | ElementsExtension::Elements(Elements::One64) => vec![],
        ElementsExtension::Elements(Elements::Increment8)
        | ElementsExtension::Elements(Elements::Negate8)
        | ElementsExtension::Elements(Elements::Decrement8)
        | ElementsExtension::Elements(Elements::IsZero8)
        | ElementsExtension::Elements(Elements::IsOne8) => vec![U8.into()],
        ElementsExtension::Elements(Elements::Increment16)
        | ElementsExtension::Elements(Elements::Negate16)
        | ElementsExtension::Elements(Elements::Decrement16)
        | ElementsExtension::Elements(Elements::IsZero16)
        | ElementsExtension::Elements(Elements::IsOne16) => vec![U16.into()],
        ElementsExtension::Elements(Elements::Increment32)
        | ElementsExtension::Elements(Elements::Negate32)
        | ElementsExtension::Elements(Elements::Decrement32)
        | ElementsExtension::Elements(Elements::IsZero32)
        | ElementsExtension::Elements(Elements::IsOne32) => vec![U32.into()],
        ElementsExtension::Elements(Elements::Increment64)
        | ElementsExtension::Elements(Elements::Negate64)
        | ElementsExtension::Elements(Elements::Decrement64)
        | ElementsExtension::Elements(Elements::IsZero64)
        | ElementsExtension::Elements(Elements::IsOne64) => vec![U64.into()],
        ElementsExtension::Elements(Elements::Add8)
        | ElementsExtension::Elements(Elements::Subtract8)
        | ElementsExtension::Elements(Elements::Multiply8)
        | ElementsExtension::Elements(Elements::Le8)
        | ElementsExtension::Elements(Elements::Lt8)
        | ElementsExtension::Elements(Elements::Min8)
        | ElementsExtension::Elements(Elements::Max8)
        | ElementsExtension::Elements(Elements::DivMod8)
        | ElementsExtension::Elements(Elements::Divide8)
        | ElementsExtension::Elements(Elements::Modulo8)
        | ElementsExtension::Elements(Elements::Divides8) => vec![U8.into(), U8.into()],
        ElementsExtension::Elements(Elements::Add16)
        | ElementsExtension::Elements(Elements::Subtract16)
        | ElementsExtension::Elements(Elements::Multiply16)
        | ElementsExtension::Elements(Elements::Le16)
        | ElementsExtension::Elements(Elements::Lt16)
        | ElementsExtension::Elements(Elements::Min16)
        | ElementsExtension::Elements(Elements::Max16)
        | ElementsExtension::Elements(Elements::DivMod16)
        | ElementsExtension::Elements(Elements::Divide16)
        | ElementsExtension::Elements(Elements::Modulo16)
        | ElementsExtension::Elements(Elements::Divides16) => vec![U16.into(), U16.into()],
        ElementsExtension::Elements(Elements::Add32)
        | ElementsExtension::Elements(Elements::Subtract32)
        | ElementsExtension::Elements(Elements::Multiply32)
        | ElementsExtension::Elements(Elements::Le32)
        | ElementsExtension::Elements(Elements::Lt32)
        | ElementsExtension::Elements(Elements::Min32)
        | ElementsExtension::Elements(Elements::Max32)
        | ElementsExtension::Elements(Elements::DivMod32)
        | ElementsExtension::Elements(Elements::Divide32)
        | ElementsExtension::Elements(Elements::Modulo32)
        | ElementsExtension::Elements(Elements::Divides32) => vec![U32.into(), U32.into()],
        ElementsExtension::Elements(Elements::Add64)
        | ElementsExtension::Elements(Elements::Subtract64)
        | ElementsExtension::Elements(Elements::Multiply64)
        | ElementsExtension::Elements(Elements::Le64)
        | ElementsExtension::Elements(Elements::Lt64)
        | ElementsExtension::Elements(Elements::Min64)
        | ElementsExtension::Elements(Elements::Max64)
        | ElementsExtension::Elements(Elements::DivMod64)
        | ElementsExtension::Elements(Elements::Divide64)
        | ElementsExtension::Elements(Elements::Modulo64)
        | ElementsExtension::Elements(Elements::Divides64) => vec![U64.into(), U64.into()],
        ElementsExtension::Elements(Elements::DivMod128_64) => vec![U128.into(), U64.into()],
        ElementsExtension::Elements(Elements::FullAdd8)
        | ElementsExtension::Elements(Elements::FullSubtract8) => {
            vec![bool(), U8.into(), U8.into()]
        }
        ElementsExtension::Elements(Elements::FullAdd16)
        | ElementsExtension::Elements(Elements::FullSubtract16) => {
            vec![bool(), U16.into(), U16.into()]
        }
        ElementsExtension::Elements(Elements::FullAdd32)
        | ElementsExtension::Elements(Elements::FullSubtract32) => {
            vec![bool(), U32.into(), U32.into()]
        }
        ElementsExtension::Elements(Elements::FullAdd64)
        | ElementsExtension::Elements(Elements::FullSubtract64) => {
            vec![bool(), U64.into(), U64.into()]
        }
        ElementsExtension::Elements(Elements::FullIncrement8)
        | ElementsExtension::Elements(Elements::FullDecrement8) => vec![bool(), U8.into()],
        ElementsExtension::Elements(Elements::FullIncrement16)
        | ElementsExtension::Elements(Elements::FullDecrement16) => vec![bool(), U16.into()],
        ElementsExtension::Elements(Elements::FullIncrement32)
        | ElementsExtension::Elements(Elements::FullDecrement32) => vec![bool(), U32.into()],
        ElementsExtension::Elements(Elements::FullIncrement64)
        | ElementsExtension::Elements(Elements::FullDecrement64) => vec![bool(), U64.into()],
        ElementsExtension::Elements(Elements::FullMultiply8) => {
            vec![tuple([U8, U8]), tuple([U8, U8])]
        }
        ElementsExtension::Elements(Elements::FullMultiply16) => {
            vec![tuple([U16, U16]), tuple([U16, U16])]
        }
        ElementsExtension::Elements(Elements::FullMultiply32) => {
            vec![tuple([U32, U32]), tuple([U32, U32])]
        }
        ElementsExtension::Elements(Elements::FullMultiply64) => {
            vec![tuple([U64, U64]), tuple([U64, U64])]
        }
        ElementsExtension::Elements(Elements::Median8) => vec![U8.into(), U8.into(), U8.into()],
        ElementsExtension::Elements(Elements::Median16) => vec![U16.into(), U16.into(), U16.into()],
        ElementsExtension::Elements(Elements::Median32) => vec![U32.into(), U32.into(), U32.into()],
        ElementsExtension::Elements(Elements::Median64) => vec![U64.into(), U64.into(), U64.into()],
        /*
         * Hash functions
         */
        ElementsExtension::Elements(Elements::Sha256Iv)
        | ElementsExtension::Elements(Elements::Sha256Ctx8Init) => vec![],
        ElementsExtension::Elements(Elements::Sha256Block) => {
            vec![U256.into(), U256.into(), U256.into()]
        }
        ElementsExtension::Elements(Elements::Sha256Ctx8Add1) => vec![Ctx8.into(), U8.into()],
        ElementsExtension::Elements(Elements::Sha256Ctx8Add2) => vec![Ctx8.into(), U16.into()],
        ElementsExtension::Elements(Elements::Sha256Ctx8Add4) => vec![Ctx8.into(), U32.into()],
        ElementsExtension::Elements(Elements::Sha256Ctx8Add8) => vec![Ctx8.into(), U64.into()],
        ElementsExtension::Elements(Elements::Sha256Ctx8Add16) => vec![Ctx8.into(), U128.into()],
        ElementsExtension::Elements(Elements::Sha256Ctx8Add32) => vec![Ctx8.into(), U256.into()],
        ElementsExtension::Elements(Elements::Sha256Ctx8Add64) => vec![Ctx8.into(), array(U8, 64)],
        ElementsExtension::Elements(Elements::Sha256Ctx8Add128) => {
            vec![Ctx8.into(), array(U8, 128)]
        }
        ElementsExtension::Elements(Elements::Sha256Ctx8Add256) => {
            vec![Ctx8.into(), array(U8, 256)]
        }
        ElementsExtension::Elements(Elements::Sha256Ctx8Add512) => {
            vec![Ctx8.into(), array(U8, 512)]
        }
        ElementsExtension::Elements(Elements::Sha256Ctx8AddBuffer511) => {
            vec![Ctx8.into(), list(U8, 512)]
        }
        ElementsExtension::Elements(Elements::Sha256Ctx8Finalize) => vec![Ctx8.into()],
        /*
         * Elliptic curve functions
         */
        // XXX: Nonstandard tuple
        ElementsExtension::Elements(Elements::PointVerify1) => {
            vec![tuple([tuple([Scalar, Point]), Scalar.into()]), Point.into()]
        }
        ElementsExtension::Elements(Elements::Decompress) => vec![Point.into()],
        // XXX: Nonstandard tuple
        ElementsExtension::Elements(Elements::LinearVerify1) => {
            vec![tuple([tuple([Scalar, Ge]), Scalar.into()]), Ge.into()]
        }
        // XXX: Nonstandard tuple
        ElementsExtension::Elements(Elements::LinearCombination1) => {
            vec![tuple([Scalar, Gej]), Scalar.into()]
        }
        ElementsExtension::Elements(Elements::Scale) => vec![Scalar.into(), Gej.into()],
        ElementsExtension::Elements(Elements::Generate) => vec![Scalar.into()],
        ElementsExtension::Elements(Elements::GejInfinity) => vec![],
        ElementsExtension::Elements(Elements::GejNormalize)
        | ElementsExtension::Elements(Elements::GejNegate)
        | ElementsExtension::Elements(Elements::GejDouble)
        | ElementsExtension::Elements(Elements::GejIsInfinity)
        | ElementsExtension::Elements(Elements::GejYIsOdd)
        | ElementsExtension::Elements(Elements::GejIsOnCurve) => vec![Gej.into()],
        ElementsExtension::Elements(Elements::GeNegate)
        | ElementsExtension::Elements(Elements::GeIsOnCurve) => vec![Ge.into()],
        ElementsExtension::Elements(Elements::GejAdd)
        | ElementsExtension::Elements(Elements::GejEquiv) => vec![Gej.into(), Gej.into()],
        ElementsExtension::Elements(Elements::GejGeAddEx)
        | ElementsExtension::Elements(Elements::GejGeAdd)
        | ElementsExtension::Elements(Elements::GejGeEquiv) => {
            vec![Gej.into(), Ge.into()]
        }
        ElementsExtension::Elements(Elements::GejRescale) => vec![Gej.into(), Fe.into()],
        ElementsExtension::Elements(Elements::GejXEquiv) => vec![Fe.into(), Gej.into()],
        ElementsExtension::Elements(Elements::ScalarAdd)
        | ElementsExtension::Elements(Elements::ScalarMultiply) => {
            vec![Scalar.into(), Scalar.into()]
        }
        ElementsExtension::Elements(Elements::ScalarNormalize)
        | ElementsExtension::Elements(Elements::ScalarNegate)
        | ElementsExtension::Elements(Elements::ScalarSquare)
        | ElementsExtension::Elements(Elements::ScalarInvert)
        | ElementsExtension::Elements(Elements::ScalarMultiplyLambda)
        | ElementsExtension::Elements(Elements::ScalarIsZero) => vec![Scalar.into()],
        ElementsExtension::Elements(Elements::FeNormalize)
        | ElementsExtension::Elements(Elements::FeNegate)
        | ElementsExtension::Elements(Elements::FeSquare)
        | ElementsExtension::Elements(Elements::FeMultiplyBeta)
        | ElementsExtension::Elements(Elements::FeInvert)
        | ElementsExtension::Elements(Elements::FeSquareRoot)
        | ElementsExtension::Elements(Elements::FeIsZero)
        | ElementsExtension::Elements(Elements::FeIsOdd)
        | ElementsExtension::Elements(Elements::Swu) => vec![Fe.into()],
        ElementsExtension::Elements(Elements::FeAdd)
        | ElementsExtension::Elements(Elements::FeMultiply) => vec![Fe.into(), Fe.into()],
        ElementsExtension::Elements(Elements::HashToCurve) => vec![U256.into()],
        /*
         * Digital signatures
         */
        // XXX: Nonstandard tuple
        ElementsExtension::Elements(Elements::CheckSigVerify) => {
            vec![tuple([Pubkey, Message64]), Signature.into()]
        }
        // XXX: Nonstandard tuple
        ElementsExtension::Elements(Elements::Bip0340Verify) => {
            vec![tuple([Pubkey, Message]), Signature.into()]
        }
        /*
         * Bitcoin (without primitives)
         */
        ElementsExtension::Elements(Elements::TapdataInit) => vec![],
        ElementsExtension::Elements(Elements::ParseLock)
        | ElementsExtension::Elements(Elements::ParseSequence) => vec![U32.into()],
        /*
         * ==============================
         *         Elements jets
         * ==============================
         *
         * Signature hash modes
         */
        ElementsExtension::Elements(Elements::SigAllHash)
        | ElementsExtension::Elements(Elements::TxHash)
        | ElementsExtension::Elements(Elements::TapEnvHash)
        | ElementsExtension::Elements(Elements::InputsHash)
        | ElementsExtension::Elements(Elements::OutputsHash)
        | ElementsExtension::Elements(Elements::IssuancesHash)
        | ElementsExtension::Elements(Elements::InputUtxosHash)
        | ElementsExtension::Elements(Elements::OutputAmountsHash)
        | ElementsExtension::Elements(Elements::OutputScriptsHash)
        | ElementsExtension::Elements(Elements::OutputNoncesHash)
        | ElementsExtension::Elements(Elements::OutputRangeProofsHash)
        | ElementsExtension::Elements(Elements::OutputSurjectionProofsHash)
        | ElementsExtension::Elements(Elements::InputOutpointsHash)
        | ElementsExtension::Elements(Elements::InputAnnexesHash)
        | ElementsExtension::Elements(Elements::InputSequencesHash)
        | ElementsExtension::Elements(Elements::InputScriptSigsHash)
        | ElementsExtension::Elements(Elements::IssuanceAssetAmountsHash)
        | ElementsExtension::Elements(Elements::IssuanceTokenAmountsHash)
        | ElementsExtension::Elements(Elements::IssuanceRangeProofsHash)
        | ElementsExtension::Elements(Elements::IssuanceBlindingEntropyHash)
        | ElementsExtension::Elements(Elements::InputAmountsHash)
        | ElementsExtension::Elements(Elements::InputScriptsHash)
        | ElementsExtension::Elements(Elements::TapleafHash)
        | ElementsExtension::Elements(Elements::TappathHash) => vec![],
        ElementsExtension::Elements(Elements::OutpointHash) => {
            vec![Ctx8.into(), option(U256), Outpoint.into()]
        }
        ElementsExtension::Elements(Elements::AssetAmountHash) => {
            vec![Ctx8.into(), Asset1.into(), Amount1.into()]
        }
        ElementsExtension::Elements(Elements::NonceHash) => vec![Ctx8.into(), option(Nonce)],
        ElementsExtension::Elements(Elements::AnnexHash) => vec![Ctx8.into(), option(U256)],
        ElementsExtension::Elements(Elements::BuildTapleafSimplicity) => vec![U256.into()],
        ElementsExtension::Elements(Elements::BuildTapbranch) => vec![U256.into(), U256.into()],
        ElementsExtension::Elements(Elements::BuildTaptweak) => vec![Pubkey.into(), U256.into()],
        /*
         * Time locks
         */
        ElementsExtension::Elements(Elements::CheckLockTime) => vec![Time.into()],
        ElementsExtension::Elements(Elements::CheckLockDistance) => vec![Distance.into()],
        ElementsExtension::Elements(Elements::CheckLockDuration) => vec![Duration.into()],
        ElementsExtension::Elements(Elements::CheckLockHeight) => vec![Height.into()],
        ElementsExtension::Elements(Elements::TxLockTime)
        | ElementsExtension::Elements(Elements::TxLockDistance)
        | ElementsExtension::Elements(Elements::TxLockDuration)
        | ElementsExtension::Elements(Elements::TxLockHeight)
        | ElementsExtension::Elements(Elements::TxIsFinal) => vec![],
        /*
         * Issuance
         */
        ElementsExtension::Elements(Elements::Issuance)
        | ElementsExtension::Elements(Elements::IssuanceAsset)
        | ElementsExtension::Elements(Elements::IssuanceToken)
        | ElementsExtension::Elements(Elements::IssuanceEntropy) => vec![U32.into()],
        ElementsExtension::Elements(Elements::CalculateIssuanceEntropy) => {
            vec![Outpoint.into(), U256.into()]
        }
        ElementsExtension::Elements(Elements::CalculateAsset)
        | ElementsExtension::Elements(Elements::CalculateExplicitToken)
        | ElementsExtension::Elements(Elements::CalculateConfidentialToken) => vec![U256.into()],
        /*
         * Transaction
         */
        ElementsExtension::Elements(Elements::ScriptCMR)
        | ElementsExtension::Elements(Elements::InternalKey)
        | ElementsExtension::Elements(Elements::CurrentIndex)
        | ElementsExtension::Elements(Elements::NumInputs)
        | ElementsExtension::Elements(Elements::NumOutputs)
        | ElementsExtension::Elements(Elements::LockTime)
        | ElementsExtension::Elements(Elements::CurrentPegin)
        | ElementsExtension::Elements(Elements::CurrentPrevOutpoint)
        | ElementsExtension::Elements(Elements::CurrentAsset)
        | ElementsExtension::Elements(Elements::CurrentAmount)
        | ElementsExtension::Elements(Elements::CurrentScriptHash)
        | ElementsExtension::Elements(Elements::CurrentSequence)
        | ElementsExtension::Elements(Elements::CurrentAnnexHash)
        | ElementsExtension::Elements(Elements::CurrentScriptSigHash)
        | ElementsExtension::Elements(Elements::CurrentReissuanceBlinding)
        | ElementsExtension::Elements(Elements::CurrentNewIssuanceContract)
        | ElementsExtension::Elements(Elements::CurrentReissuanceEntropy)
        | ElementsExtension::Elements(Elements::CurrentIssuanceTokenAmount)
        | ElementsExtension::Elements(Elements::CurrentIssuanceAssetAmount)
        | ElementsExtension::Elements(Elements::CurrentIssuanceAssetProof)
        | ElementsExtension::Elements(Elements::CurrentIssuanceTokenProof)
        | ElementsExtension::Elements(Elements::TapleafVersion)
        | ElementsExtension::Elements(Elements::Version)
        | ElementsExtension::Elements(Elements::GenesisBlockHash)
        | ElementsExtension::Elements(Elements::LbtcAsset)
        | ElementsExtension::Elements(Elements::TransactionId) => vec![],
        ElementsExtension::Elements(Elements::OutputAsset)
        | ElementsExtension::Elements(Elements::OutputAmount)
        | ElementsExtension::Elements(Elements::OutputNonce)
        | ElementsExtension::Elements(Elements::OutputScriptHash)
        | ElementsExtension::Elements(Elements::OutputIsFee)
        | ElementsExtension::Elements(Elements::OutputSurjectionProof)
        | ElementsExtension::Elements(Elements::OutputRangeProof)
        | ElementsExtension::Elements(Elements::OutputHash)
        | ElementsExtension::Elements(Elements::InputPegin)
        | ElementsExtension::Elements(Elements::InputPrevOutpoint)
        | ElementsExtension::Elements(Elements::InputAsset)
        | ElementsExtension::Elements(Elements::InputAmount)
        | ElementsExtension::Elements(Elements::InputScriptHash)
        | ElementsExtension::Elements(Elements::InputSequence)
        | ElementsExtension::Elements(Elements::InputAnnexHash)
        | ElementsExtension::Elements(Elements::InputScriptSigHash)
        | ElementsExtension::Elements(Elements::InputHash)
        | ElementsExtension::Elements(Elements::InputUtxoHash)
        | ElementsExtension::Elements(Elements::ReissuanceBlinding)
        | ElementsExtension::Elements(Elements::NewIssuanceContract)
        | ElementsExtension::Elements(Elements::ReissuanceEntropy)
        | ElementsExtension::Elements(Elements::IssuanceAssetAmount)
        | ElementsExtension::Elements(Elements::IssuanceTokenAmount)
        | ElementsExtension::Elements(Elements::IssuanceAssetProof)
        | ElementsExtension::Elements(Elements::IssuanceTokenProof)
        | ElementsExtension::Elements(Elements::IssuanceHash) => vec![U32.into()],
        ElementsExtension::Elements(Elements::OutputNullDatum) => vec![U32.into(), U32.into()],
        ElementsExtension::Elements(Elements::TotalFee) => vec![ExplicitAsset.into()],
        ElementsExtension::Elements(Elements::Tappath) => vec![U8.into()],
        /*
         * Script operations
         */
        ElementsExtension::GetOpcodeFromScript => vec![U8.into()],
        ElementsExtension::GetPubkeyFromScript => vec![U8.into()],
    }
}

pub fn target_type(jet: ElementsExtension) -> AliasedType {
    match jet {
        /*
         * ==============================
         *          Core jets
         * ==============================
         *
         * Multi-bit logic
         */
        ElementsExtension::Elements(Elements::Verify) => AliasedType::unit(),
        ElementsExtension::Elements(Elements::Some1)
        | ElementsExtension::Elements(Elements::Some8)
        | ElementsExtension::Elements(Elements::Some16)
        | ElementsExtension::Elements(Elements::Some32)
        | ElementsExtension::Elements(Elements::Some64)
        | ElementsExtension::Elements(Elements::All8)
        | ElementsExtension::Elements(Elements::All16)
        | ElementsExtension::Elements(Elements::All32)
        | ElementsExtension::Elements(Elements::All64)
        | ElementsExtension::Elements(Elements::Eq1)
        | ElementsExtension::Elements(Elements::Eq8)
        | ElementsExtension::Elements(Elements::Eq16)
        | ElementsExtension::Elements(Elements::Eq32)
        | ElementsExtension::Elements(Elements::Eq64)
        | ElementsExtension::Elements(Elements::Eq256) => bool(),
        ElementsExtension::Elements(Elements::Low1)
        | ElementsExtension::Elements(Elements::High1)
        | ElementsExtension::Elements(Elements::Complement1)
        | ElementsExtension::Elements(Elements::And1)
        | ElementsExtension::Elements(Elements::Or1)
        | ElementsExtension::Elements(Elements::Xor1)
        | ElementsExtension::Elements(Elements::Maj1)
        | ElementsExtension::Elements(Elements::XorXor1)
        | ElementsExtension::Elements(Elements::Ch1)
        | ElementsExtension::Elements(Elements::Leftmost8_1)
        | ElementsExtension::Elements(Elements::Rightmost8_1)
        | ElementsExtension::Elements(Elements::Leftmost16_1)
        | ElementsExtension::Elements(Elements::Rightmost16_1)
        | ElementsExtension::Elements(Elements::Leftmost32_1)
        | ElementsExtension::Elements(Elements::Rightmost32_1)
        | ElementsExtension::Elements(Elements::Leftmost64_1)
        | ElementsExtension::Elements(Elements::Rightmost64_1) => U1.into(),
        ElementsExtension::Elements(Elements::Leftmost8_2)
        | ElementsExtension::Elements(Elements::Rightmost8_2)
        | ElementsExtension::Elements(Elements::Leftmost16_2)
        | ElementsExtension::Elements(Elements::Rightmost16_2)
        | ElementsExtension::Elements(Elements::Leftmost32_2)
        | ElementsExtension::Elements(Elements::Rightmost32_2)
        | ElementsExtension::Elements(Elements::Leftmost64_2)
        | ElementsExtension::Elements(Elements::Rightmost64_2) => U2.into(),
        ElementsExtension::Elements(Elements::Leftmost8_4)
        | ElementsExtension::Elements(Elements::Rightmost8_4)
        | ElementsExtension::Elements(Elements::Leftmost16_4)
        | ElementsExtension::Elements(Elements::Rightmost16_4)
        | ElementsExtension::Elements(Elements::Leftmost32_4)
        | ElementsExtension::Elements(Elements::Rightmost32_4)
        | ElementsExtension::Elements(Elements::Leftmost64_4)
        | ElementsExtension::Elements(Elements::Rightmost64_4) => U4.into(),
        ElementsExtension::Elements(Elements::Low8)
        | ElementsExtension::Elements(Elements::High8)
        | ElementsExtension::Elements(Elements::Complement8)
        | ElementsExtension::Elements(Elements::And8)
        | ElementsExtension::Elements(Elements::Or8)
        | ElementsExtension::Elements(Elements::Xor8)
        | ElementsExtension::Elements(Elements::Maj8)
        | ElementsExtension::Elements(Elements::XorXor8)
        | ElementsExtension::Elements(Elements::Ch8)
        | ElementsExtension::Elements(Elements::Leftmost16_8)
        | ElementsExtension::Elements(Elements::Rightmost16_8)
        | ElementsExtension::Elements(Elements::Leftmost32_8)
        | ElementsExtension::Elements(Elements::Rightmost32_8)
        | ElementsExtension::Elements(Elements::Leftmost64_8)
        | ElementsExtension::Elements(Elements::Rightmost64_8)
        | ElementsExtension::Elements(Elements::LeftPadLow1_8)
        | ElementsExtension::Elements(Elements::LeftPadHigh1_8)
        | ElementsExtension::Elements(Elements::LeftExtend1_8)
        | ElementsExtension::Elements(Elements::RightPadLow1_8)
        | ElementsExtension::Elements(Elements::RightPadHigh1_8)
        | ElementsExtension::Elements(Elements::LeftShiftWith8)
        | ElementsExtension::Elements(Elements::RightShiftWith8)
        | ElementsExtension::Elements(Elements::LeftShift8)
        | ElementsExtension::Elements(Elements::RightShift8)
        | ElementsExtension::Elements(Elements::LeftRotate8)
        | ElementsExtension::Elements(Elements::RightRotate8) => U8.into(),
        ElementsExtension::Elements(Elements::Low16)
        | ElementsExtension::Elements(Elements::High16)
        | ElementsExtension::Elements(Elements::Complement16)
        | ElementsExtension::Elements(Elements::And16)
        | ElementsExtension::Elements(Elements::Or16)
        | ElementsExtension::Elements(Elements::Xor16)
        | ElementsExtension::Elements(Elements::Maj16)
        | ElementsExtension::Elements(Elements::XorXor16)
        | ElementsExtension::Elements(Elements::Ch16)
        | ElementsExtension::Elements(Elements::Leftmost32_16)
        | ElementsExtension::Elements(Elements::Rightmost32_16)
        | ElementsExtension::Elements(Elements::Leftmost64_16)
        | ElementsExtension::Elements(Elements::Rightmost64_16)
        | ElementsExtension::Elements(Elements::LeftPadLow1_16)
        | ElementsExtension::Elements(Elements::LeftPadHigh1_16)
        | ElementsExtension::Elements(Elements::LeftExtend1_16)
        | ElementsExtension::Elements(Elements::RightPadLow1_16)
        | ElementsExtension::Elements(Elements::RightPadHigh1_16)
        | ElementsExtension::Elements(Elements::LeftPadLow8_16)
        | ElementsExtension::Elements(Elements::LeftPadHigh8_16)
        | ElementsExtension::Elements(Elements::LeftExtend8_16)
        | ElementsExtension::Elements(Elements::RightPadLow8_16)
        | ElementsExtension::Elements(Elements::RightPadHigh8_16)
        | ElementsExtension::Elements(Elements::RightExtend8_16)
        | ElementsExtension::Elements(Elements::LeftShiftWith16)
        | ElementsExtension::Elements(Elements::RightShiftWith16)
        | ElementsExtension::Elements(Elements::LeftShift16)
        | ElementsExtension::Elements(Elements::RightShift16)
        | ElementsExtension::Elements(Elements::LeftRotate16)
        | ElementsExtension::Elements(Elements::RightRotate16) => U16.into(),
        ElementsExtension::Elements(Elements::Low32)
        | ElementsExtension::Elements(Elements::High32)
        | ElementsExtension::Elements(Elements::Complement32)
        | ElementsExtension::Elements(Elements::And32)
        | ElementsExtension::Elements(Elements::Or32)
        | ElementsExtension::Elements(Elements::Xor32)
        | ElementsExtension::Elements(Elements::Maj32)
        | ElementsExtension::Elements(Elements::XorXor32)
        | ElementsExtension::Elements(Elements::Ch32)
        | ElementsExtension::Elements(Elements::Leftmost64_32)
        | ElementsExtension::Elements(Elements::Rightmost64_32)
        | ElementsExtension::Elements(Elements::LeftPadLow1_32)
        | ElementsExtension::Elements(Elements::LeftPadHigh1_32)
        | ElementsExtension::Elements(Elements::LeftExtend1_32)
        | ElementsExtension::Elements(Elements::RightPadLow1_32)
        | ElementsExtension::Elements(Elements::RightPadHigh1_32)
        | ElementsExtension::Elements(Elements::LeftPadLow8_32)
        | ElementsExtension::Elements(Elements::LeftPadHigh8_32)
        | ElementsExtension::Elements(Elements::LeftExtend8_32)
        | ElementsExtension::Elements(Elements::RightPadLow8_32)
        | ElementsExtension::Elements(Elements::RightPadHigh8_32)
        | ElementsExtension::Elements(Elements::RightExtend8_32)
        | ElementsExtension::Elements(Elements::LeftPadLow16_32)
        | ElementsExtension::Elements(Elements::LeftPadHigh16_32)
        | ElementsExtension::Elements(Elements::LeftExtend16_32)
        | ElementsExtension::Elements(Elements::RightPadLow16_32)
        | ElementsExtension::Elements(Elements::RightPadHigh16_32)
        | ElementsExtension::Elements(Elements::RightExtend16_32)
        | ElementsExtension::Elements(Elements::LeftShiftWith32)
        | ElementsExtension::Elements(Elements::RightShiftWith32)
        | ElementsExtension::Elements(Elements::LeftShift32)
        | ElementsExtension::Elements(Elements::RightShift32)
        | ElementsExtension::Elements(Elements::LeftRotate32)
        | ElementsExtension::Elements(Elements::RightRotate32) => U32.into(),
        ElementsExtension::Elements(Elements::Low64)
        | ElementsExtension::Elements(Elements::High64)
        | ElementsExtension::Elements(Elements::Complement64)
        | ElementsExtension::Elements(Elements::And64)
        | ElementsExtension::Elements(Elements::Or64)
        | ElementsExtension::Elements(Elements::Xor64)
        | ElementsExtension::Elements(Elements::Maj64)
        | ElementsExtension::Elements(Elements::XorXor64)
        | ElementsExtension::Elements(Elements::Ch64)
        | ElementsExtension::Elements(Elements::LeftPadLow1_64)
        | ElementsExtension::Elements(Elements::LeftPadHigh1_64)
        | ElementsExtension::Elements(Elements::LeftExtend1_64)
        | ElementsExtension::Elements(Elements::RightPadLow1_64)
        | ElementsExtension::Elements(Elements::RightPadHigh1_64)
        | ElementsExtension::Elements(Elements::LeftPadLow8_64)
        | ElementsExtension::Elements(Elements::LeftPadHigh8_64)
        | ElementsExtension::Elements(Elements::LeftExtend8_64)
        | ElementsExtension::Elements(Elements::RightPadLow8_64)
        | ElementsExtension::Elements(Elements::RightPadHigh8_64)
        | ElementsExtension::Elements(Elements::RightExtend8_64)
        | ElementsExtension::Elements(Elements::LeftPadLow16_64)
        | ElementsExtension::Elements(Elements::LeftPadHigh16_64)
        | ElementsExtension::Elements(Elements::LeftExtend16_64)
        | ElementsExtension::Elements(Elements::RightPadLow16_64)
        | ElementsExtension::Elements(Elements::RightPadHigh16_64)
        | ElementsExtension::Elements(Elements::RightExtend16_64)
        | ElementsExtension::Elements(Elements::LeftPadLow32_64)
        | ElementsExtension::Elements(Elements::LeftPadHigh32_64)
        | ElementsExtension::Elements(Elements::LeftExtend32_64)
        | ElementsExtension::Elements(Elements::RightPadLow32_64)
        | ElementsExtension::Elements(Elements::RightPadHigh32_64)
        | ElementsExtension::Elements(Elements::RightExtend32_64)
        | ElementsExtension::Elements(Elements::LeftShiftWith64)
        | ElementsExtension::Elements(Elements::RightShiftWith64)
        | ElementsExtension::Elements(Elements::LeftShift64)
        | ElementsExtension::Elements(Elements::RightShift64)
        | ElementsExtension::Elements(Elements::LeftRotate64)
        | ElementsExtension::Elements(Elements::RightRotate64) => U64.into(),
        ElementsExtension::Elements(Elements::FullLeftShift8_1) => tuple([U1, U8]),
        ElementsExtension::Elements(Elements::FullLeftShift8_2) => tuple([U2, U8]),
        ElementsExtension::Elements(Elements::FullLeftShift8_4) => tuple([U4, U8]),
        ElementsExtension::Elements(Elements::FullLeftShift16_1) => tuple([U1, U16]),
        ElementsExtension::Elements(Elements::FullLeftShift16_2) => tuple([U2, U16]),
        ElementsExtension::Elements(Elements::FullLeftShift16_4) => tuple([U4, U16]),
        ElementsExtension::Elements(Elements::FullLeftShift16_8) => tuple([U8, U16]),
        ElementsExtension::Elements(Elements::FullLeftShift32_1) => tuple([U1, U32]),
        ElementsExtension::Elements(Elements::FullLeftShift32_2) => tuple([U2, U32]),
        ElementsExtension::Elements(Elements::FullLeftShift32_4) => tuple([U4, U32]),
        ElementsExtension::Elements(Elements::FullLeftShift32_8) => tuple([U8, U32]),
        ElementsExtension::Elements(Elements::FullLeftShift32_16) => tuple([U16, U32]),
        ElementsExtension::Elements(Elements::FullLeftShift64_1) => tuple([U1, U64]),
        ElementsExtension::Elements(Elements::FullLeftShift64_2) => tuple([U2, U64]),
        ElementsExtension::Elements(Elements::FullLeftShift64_4) => tuple([U4, U64]),
        ElementsExtension::Elements(Elements::FullLeftShift64_8) => tuple([U8, U64]),
        ElementsExtension::Elements(Elements::FullLeftShift64_16) => tuple([U16, U64]),
        ElementsExtension::Elements(Elements::FullLeftShift64_32) => tuple([U32, U64]),
        ElementsExtension::Elements(Elements::FullRightShift8_1) => tuple([U8, U1]),
        ElementsExtension::Elements(Elements::FullRightShift8_2) => tuple([U8, U2]),
        ElementsExtension::Elements(Elements::FullRightShift8_4) => tuple([U8, U4]),
        ElementsExtension::Elements(Elements::FullRightShift16_1) => tuple([U16, U1]),
        ElementsExtension::Elements(Elements::FullRightShift16_2) => tuple([U16, U2]),
        ElementsExtension::Elements(Elements::FullRightShift16_4) => tuple([U16, U4]),
        ElementsExtension::Elements(Elements::FullRightShift16_8) => tuple([U16, U8]),
        ElementsExtension::Elements(Elements::FullRightShift32_1) => tuple([U32, U1]),
        ElementsExtension::Elements(Elements::FullRightShift32_2) => tuple([U32, U2]),
        ElementsExtension::Elements(Elements::FullRightShift32_4) => tuple([U32, U4]),
        ElementsExtension::Elements(Elements::FullRightShift32_8) => tuple([U32, U8]),
        ElementsExtension::Elements(Elements::FullRightShift32_16) => tuple([U32, U16]),
        ElementsExtension::Elements(Elements::FullRightShift64_1) => tuple([U64, U1]),
        ElementsExtension::Elements(Elements::FullRightShift64_2) => tuple([U64, U2]),
        ElementsExtension::Elements(Elements::FullRightShift64_4) => tuple([U64, U4]),
        ElementsExtension::Elements(Elements::FullRightShift64_8) => tuple([U64, U8]),
        ElementsExtension::Elements(Elements::FullRightShift64_16) => tuple([U64, U16]),
        ElementsExtension::Elements(Elements::FullRightShift64_32) => tuple([U64, U32]),
        /*
         * Arithmetic
         */
        ElementsExtension::Elements(Elements::Le8)
        | ElementsExtension::Elements(Elements::Lt8)
        | ElementsExtension::Elements(Elements::Le16)
        | ElementsExtension::Elements(Elements::Lt16)
        | ElementsExtension::Elements(Elements::Le32)
        | ElementsExtension::Elements(Elements::Lt32)
        | ElementsExtension::Elements(Elements::Le64)
        | ElementsExtension::Elements(Elements::Lt64)
        | ElementsExtension::Elements(Elements::IsZero8)
        | ElementsExtension::Elements(Elements::IsOne8)
        | ElementsExtension::Elements(Elements::IsZero16)
        | ElementsExtension::Elements(Elements::IsOne16)
        | ElementsExtension::Elements(Elements::IsZero32)
        | ElementsExtension::Elements(Elements::IsOne32)
        | ElementsExtension::Elements(Elements::IsZero64)
        | ElementsExtension::Elements(Elements::IsOne64)
        | ElementsExtension::Elements(Elements::Divides8)
        | ElementsExtension::Elements(Elements::Divides16)
        | ElementsExtension::Elements(Elements::Divides32)
        | ElementsExtension::Elements(Elements::Divides64) => bool(),
        ElementsExtension::Elements(Elements::One8)
        | ElementsExtension::Elements(Elements::Min8)
        | ElementsExtension::Elements(Elements::Max8)
        | ElementsExtension::Elements(Elements::Divide8)
        | ElementsExtension::Elements(Elements::Modulo8)
        | ElementsExtension::Elements(Elements::Median8) => U8.into(),
        ElementsExtension::Elements(Elements::One16)
        | ElementsExtension::Elements(Elements::Min16)
        | ElementsExtension::Elements(Elements::Max16)
        | ElementsExtension::Elements(Elements::Divide16)
        | ElementsExtension::Elements(Elements::Modulo16)
        | ElementsExtension::Elements(Elements::Multiply8)
        | ElementsExtension::Elements(Elements::FullMultiply8)
        | ElementsExtension::Elements(Elements::Median16) => U16.into(),
        ElementsExtension::Elements(Elements::One32)
        | ElementsExtension::Elements(Elements::Min32)
        | ElementsExtension::Elements(Elements::Max32)
        | ElementsExtension::Elements(Elements::Divide32)
        | ElementsExtension::Elements(Elements::Modulo32)
        | ElementsExtension::Elements(Elements::Multiply16)
        | ElementsExtension::Elements(Elements::FullMultiply16)
        | ElementsExtension::Elements(Elements::Median32) => U32.into(),
        ElementsExtension::Elements(Elements::One64)
        | ElementsExtension::Elements(Elements::Min64)
        | ElementsExtension::Elements(Elements::Max64)
        | ElementsExtension::Elements(Elements::Divide64)
        | ElementsExtension::Elements(Elements::Modulo64)
        | ElementsExtension::Elements(Elements::Multiply32)
        | ElementsExtension::Elements(Elements::FullMultiply32)
        | ElementsExtension::Elements(Elements::Median64) => U64.into(),
        ElementsExtension::Elements(Elements::Multiply64)
        | ElementsExtension::Elements(Elements::FullMultiply64) => U128.into(),
        ElementsExtension::Elements(Elements::Increment8)
        | ElementsExtension::Elements(Elements::Negate8)
        | ElementsExtension::Elements(Elements::Decrement8)
        | ElementsExtension::Elements(Elements::Add8)
        | ElementsExtension::Elements(Elements::Subtract8)
        | ElementsExtension::Elements(Elements::FullAdd8)
        | ElementsExtension::Elements(Elements::FullSubtract8)
        | ElementsExtension::Elements(Elements::FullIncrement8)
        | ElementsExtension::Elements(Elements::FullDecrement8) => tuple([bool(), U8.into()]),
        ElementsExtension::Elements(Elements::Increment16)
        | ElementsExtension::Elements(Elements::Negate16)
        | ElementsExtension::Elements(Elements::Decrement16)
        | ElementsExtension::Elements(Elements::Add16)
        | ElementsExtension::Elements(Elements::Subtract16)
        | ElementsExtension::Elements(Elements::FullAdd16)
        | ElementsExtension::Elements(Elements::FullSubtract16)
        | ElementsExtension::Elements(Elements::FullIncrement16)
        | ElementsExtension::Elements(Elements::FullDecrement16) => tuple([bool(), U16.into()]),
        ElementsExtension::Elements(Elements::Increment32)
        | ElementsExtension::Elements(Elements::Negate32)
        | ElementsExtension::Elements(Elements::Decrement32)
        | ElementsExtension::Elements(Elements::Add32)
        | ElementsExtension::Elements(Elements::Subtract32)
        | ElementsExtension::Elements(Elements::FullAdd32)
        | ElementsExtension::Elements(Elements::FullSubtract32)
        | ElementsExtension::Elements(Elements::FullIncrement32)
        | ElementsExtension::Elements(Elements::FullDecrement32) => tuple([bool(), U32.into()]),
        ElementsExtension::Elements(Elements::Increment64)
        | ElementsExtension::Elements(Elements::Negate64)
        | ElementsExtension::Elements(Elements::Decrement64)
        | ElementsExtension::Elements(Elements::Add64)
        | ElementsExtension::Elements(Elements::Subtract64)
        | ElementsExtension::Elements(Elements::FullAdd64)
        | ElementsExtension::Elements(Elements::FullSubtract64)
        | ElementsExtension::Elements(Elements::FullIncrement64)
        | ElementsExtension::Elements(Elements::FullDecrement64) => tuple([bool(), U64.into()]),
        ElementsExtension::Elements(Elements::DivMod8) => tuple([U8, U8]),
        ElementsExtension::Elements(Elements::DivMod16) => tuple([U16, U16]),
        ElementsExtension::Elements(Elements::DivMod32) => tuple([U32, U32]),
        ElementsExtension::Elements(Elements::DivMod64) => tuple([U64, U64]),
        ElementsExtension::Elements(Elements::DivMod128_64) => tuple([U64, U64]),
        /*
         * Hash functions
         */
        ElementsExtension::Elements(Elements::Sha256Iv)
        | ElementsExtension::Elements(Elements::Sha256Block)
        | ElementsExtension::Elements(Elements::Sha256Ctx8Finalize) => U256.into(),
        ElementsExtension::Elements(Elements::Sha256Ctx8Init)
        | ElementsExtension::Elements(Elements::Sha256Ctx8Add1)
        | ElementsExtension::Elements(Elements::Sha256Ctx8Add2)
        | ElementsExtension::Elements(Elements::Sha256Ctx8Add4)
        | ElementsExtension::Elements(Elements::Sha256Ctx8Add8)
        | ElementsExtension::Elements(Elements::Sha256Ctx8Add16)
        | ElementsExtension::Elements(Elements::Sha256Ctx8Add32)
        | ElementsExtension::Elements(Elements::Sha256Ctx8Add64)
        | ElementsExtension::Elements(Elements::Sha256Ctx8Add128)
        | ElementsExtension::Elements(Elements::Sha256Ctx8Add256)
        | ElementsExtension::Elements(Elements::Sha256Ctx8Add512)
        | ElementsExtension::Elements(Elements::Sha256Ctx8AddBuffer511) => Ctx8.into(),
        /*
         * Elliptic curve functions
         */
        ElementsExtension::Elements(Elements::PointVerify1)
        | ElementsExtension::Elements(Elements::LinearVerify1) => AliasedType::unit(),
        ElementsExtension::Elements(Elements::GejIsInfinity)
        | ElementsExtension::Elements(Elements::GejEquiv)
        | ElementsExtension::Elements(Elements::GejGeEquiv)
        | ElementsExtension::Elements(Elements::GejXEquiv)
        | ElementsExtension::Elements(Elements::GejYIsOdd)
        | ElementsExtension::Elements(Elements::GejIsOnCurve)
        | ElementsExtension::Elements(Elements::GeIsOnCurve)
        | ElementsExtension::Elements(Elements::ScalarIsZero)
        | ElementsExtension::Elements(Elements::FeIsZero)
        | ElementsExtension::Elements(Elements::FeIsOdd) => bool(),
        ElementsExtension::Elements(Elements::GeNegate)
        | ElementsExtension::Elements(Elements::HashToCurve)
        | ElementsExtension::Elements(Elements::Swu) => Ge.into(),
        ElementsExtension::Elements(Elements::Decompress)
        | ElementsExtension::Elements(Elements::GejNormalize) => option(Ge),
        ElementsExtension::Elements(Elements::LinearCombination1)
        | ElementsExtension::Elements(Elements::Scale)
        | ElementsExtension::Elements(Elements::Generate)
        | ElementsExtension::Elements(Elements::GejInfinity)
        | ElementsExtension::Elements(Elements::GejNegate)
        | ElementsExtension::Elements(Elements::GejDouble)
        | ElementsExtension::Elements(Elements::GejAdd)
        | ElementsExtension::Elements(Elements::GejGeAdd)
        | ElementsExtension::Elements(Elements::GejRescale) => Gej.into(),
        ElementsExtension::Elements(Elements::GejGeAddEx) => tuple([Fe, Gej]),
        ElementsExtension::Elements(Elements::ScalarNormalize)
        | ElementsExtension::Elements(Elements::ScalarNegate)
        | ElementsExtension::Elements(Elements::ScalarAdd)
        | ElementsExtension::Elements(Elements::ScalarSquare)
        | ElementsExtension::Elements(Elements::ScalarMultiply)
        | ElementsExtension::Elements(Elements::ScalarMultiplyLambda)
        | ElementsExtension::Elements(Elements::ScalarInvert) => Scalar.into(),
        ElementsExtension::Elements(Elements::FeNormalize)
        | ElementsExtension::Elements(Elements::FeNegate)
        | ElementsExtension::Elements(Elements::FeAdd)
        | ElementsExtension::Elements(Elements::FeSquare)
        | ElementsExtension::Elements(Elements::FeMultiply)
        | ElementsExtension::Elements(Elements::FeMultiplyBeta)
        | ElementsExtension::Elements(Elements::FeInvert) => Fe.into(),
        ElementsExtension::Elements(Elements::FeSquareRoot) => option(Fe),
        /*
         * Digital signatures
         */
        ElementsExtension::Elements(Elements::CheckSigVerify)
        | ElementsExtension::Elements(Elements::Bip0340Verify) => AliasedType::unit(),
        /*
         * Bitcoin (without primitives)
         */
        ElementsExtension::Elements(Elements::ParseLock) => either(Height, Time),
        ElementsExtension::Elements(Elements::ParseSequence) => option(either(Distance, Duration)),
        ElementsExtension::Elements(Elements::TapdataInit) => Ctx8.into(),
        /*
         * ==============================
         *         Elements jets
         * ==============================
         *
         * Signature hash modes
         */
        ElementsExtension::Elements(Elements::SigAllHash)
        | ElementsExtension::Elements(Elements::TxHash)
        | ElementsExtension::Elements(Elements::TapEnvHash)
        | ElementsExtension::Elements(Elements::InputsHash)
        | ElementsExtension::Elements(Elements::OutputsHash)
        | ElementsExtension::Elements(Elements::IssuancesHash)
        | ElementsExtension::Elements(Elements::InputUtxosHash)
        | ElementsExtension::Elements(Elements::OutputAmountsHash)
        | ElementsExtension::Elements(Elements::OutputScriptsHash)
        | ElementsExtension::Elements(Elements::OutputNoncesHash)
        | ElementsExtension::Elements(Elements::OutputRangeProofsHash)
        | ElementsExtension::Elements(Elements::OutputSurjectionProofsHash)
        | ElementsExtension::Elements(Elements::InputOutpointsHash)
        | ElementsExtension::Elements(Elements::InputAnnexesHash)
        | ElementsExtension::Elements(Elements::InputSequencesHash)
        | ElementsExtension::Elements(Elements::InputScriptSigsHash)
        | ElementsExtension::Elements(Elements::IssuanceAssetAmountsHash)
        | ElementsExtension::Elements(Elements::IssuanceTokenAmountsHash)
        | ElementsExtension::Elements(Elements::IssuanceRangeProofsHash)
        | ElementsExtension::Elements(Elements::IssuanceBlindingEntropyHash)
        | ElementsExtension::Elements(Elements::InputAmountsHash)
        | ElementsExtension::Elements(Elements::InputScriptsHash)
        | ElementsExtension::Elements(Elements::TapleafHash)
        | ElementsExtension::Elements(Elements::TappathHash)
        | ElementsExtension::Elements(Elements::BuildTapleafSimplicity)
        | ElementsExtension::Elements(Elements::BuildTapbranch)
        | ElementsExtension::Elements(Elements::BuildTaptweak) => U256.into(),
        ElementsExtension::Elements(Elements::OutpointHash)
        | ElementsExtension::Elements(Elements::AssetAmountHash)
        | ElementsExtension::Elements(Elements::NonceHash)
        | ElementsExtension::Elements(Elements::AnnexHash) => Ctx8.into(),
        /*
         * Time locks
         */
        ElementsExtension::Elements(Elements::CheckLockTime)
        | ElementsExtension::Elements(Elements::CheckLockDistance)
        | ElementsExtension::Elements(Elements::CheckLockDuration)
        | ElementsExtension::Elements(Elements::CheckLockHeight) => AliasedType::unit(),
        ElementsExtension::Elements(Elements::TxIsFinal) => bool(),
        ElementsExtension::Elements(Elements::TxLockTime) => Time.into(),
        ElementsExtension::Elements(Elements::TxLockDistance) => Distance.into(),
        ElementsExtension::Elements(Elements::TxLockDuration) => Duration.into(),
        ElementsExtension::Elements(Elements::TxLockHeight) => Height.into(),
        /*
         * Issuance
         */
        ElementsExtension::Elements(Elements::Issuance) => option(option(bool())),
        ElementsExtension::Elements(Elements::IssuanceAsset)
        | ElementsExtension::Elements(Elements::IssuanceToken) => option(option(ExplicitAsset)),
        ElementsExtension::Elements(Elements::IssuanceEntropy) => option(option(U256)),
        ElementsExtension::Elements(Elements::CalculateIssuanceEntropy) => U256.into(),
        ElementsExtension::Elements(Elements::CalculateAsset)
        | ElementsExtension::Elements(Elements::CalculateExplicitToken)
        | ElementsExtension::Elements(Elements::CalculateConfidentialToken) => ExplicitAsset.into(),
        /*
         * Transaction
         */
        ElementsExtension::Elements(Elements::TapleafVersion) => U8.into(),
        ElementsExtension::Elements(Elements::CurrentIndex)
        | ElementsExtension::Elements(Elements::NumInputs)
        | ElementsExtension::Elements(Elements::NumOutputs)
        | ElementsExtension::Elements(Elements::CurrentSequence)
        | ElementsExtension::Elements(Elements::Version) => U32.into(),
        ElementsExtension::Elements(Elements::ScriptCMR)
        | ElementsExtension::Elements(Elements::CurrentScriptHash)
        | ElementsExtension::Elements(Elements::CurrentScriptSigHash)
        | ElementsExtension::Elements(Elements::CurrentIssuanceAssetProof)
        | ElementsExtension::Elements(Elements::CurrentIssuanceTokenProof)
        | ElementsExtension::Elements(Elements::GenesisBlockHash)
        | ElementsExtension::Elements(Elements::LbtcAsset)
        | ElementsExtension::Elements(Elements::TransactionId) => U256.into(),
        ElementsExtension::Elements(Elements::InternalKey) => Pubkey.into(),
        ElementsExtension::Elements(Elements::LockTime) => Lock.into(),
        ElementsExtension::Elements(Elements::InputSequence) => option(U32),
        ElementsExtension::Elements(Elements::OutputAsset) => option(Asset1),
        ElementsExtension::Elements(Elements::OutputAmount) => option(tuple([Asset1, Amount1])),
        ElementsExtension::Elements(Elements::OutputNonce) => option(option(Nonce)),
        ElementsExtension::Elements(Elements::OutputScriptHash)
        | ElementsExtension::Elements(Elements::OutputSurjectionProof)
        | ElementsExtension::Elements(Elements::OutputRangeProof)
        | ElementsExtension::Elements(Elements::OutputHash)
        | ElementsExtension::Elements(Elements::CurrentPegin)
        | ElementsExtension::Elements(Elements::CurrentAnnexHash)
        | ElementsExtension::Elements(Elements::CurrentNewIssuanceContract)
        | ElementsExtension::Elements(Elements::CurrentReissuanceEntropy)
        | ElementsExtension::Elements(Elements::InputScriptHash)
        | ElementsExtension::Elements(Elements::InputScriptSigHash)
        | ElementsExtension::Elements(Elements::InputHash)
        | ElementsExtension::Elements(Elements::InputUtxoHash)
        | ElementsExtension::Elements(Elements::IssuanceAssetProof)
        | ElementsExtension::Elements(Elements::IssuanceTokenProof)
        | ElementsExtension::Elements(Elements::IssuanceHash)
        | ElementsExtension::Elements(Elements::Tappath) => option(U256),
        ElementsExtension::Elements(Elements::OutputNullDatum) => {
            option(option(either(tuple([U2, U256]), either(U1, U4))))
        }
        ElementsExtension::Elements(Elements::OutputIsFee) => option(bool()),
        ElementsExtension::Elements(Elements::TotalFee) => ExplicitAmount.into(),
        ElementsExtension::Elements(Elements::CurrentPrevOutpoint) => Outpoint.into(),
        ElementsExtension::Elements(Elements::CurrentAsset) => Asset1.into(),
        ElementsExtension::Elements(Elements::CurrentAmount) => tuple([Asset1, Amount1]),
        ElementsExtension::Elements(Elements::CurrentReissuanceBlinding) => option(ExplicitNonce),
        ElementsExtension::Elements(Elements::CurrentIssuanceAssetAmount) => option(Amount1),
        ElementsExtension::Elements(Elements::CurrentIssuanceTokenAmount) => option(TokenAmount1),
        ElementsExtension::Elements(Elements::InputPegin)
        | ElementsExtension::Elements(Elements::InputAnnexHash)
        | ElementsExtension::Elements(Elements::NewIssuanceContract)
        | ElementsExtension::Elements(Elements::ReissuanceEntropy) => option(option(U256)),
        ElementsExtension::Elements(Elements::InputPrevOutpoint) => option(Outpoint),
        ElementsExtension::Elements(Elements::InputAsset) => option(Asset1),
        ElementsExtension::Elements(Elements::InputAmount) => option(tuple([Asset1, Amount1])),
        ElementsExtension::Elements(Elements::ReissuanceBlinding) => option(option(ExplicitNonce)),
        ElementsExtension::Elements(Elements::IssuanceAssetAmount) => option(option(Amount1)),
        ElementsExtension::Elements(Elements::IssuanceTokenAmount) => option(option(TokenAmount1)),
        /*
         * Script operations
         */
        ElementsExtension::GetOpcodeFromScript => U8.into(),
        ElementsExtension::GetPubkeyFromScript => Pubkey.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simplicity::jet::Jet;

    #[test]
    fn compatible_source_type() {
        for jet in ElementsExtension::ALL {
            let resolved_ty = ResolvedType::tuple(
                source_type(jet)
                    .into_iter()
                    .map(|t| t.resolve_builtin().unwrap()),
            );
            let structural_ty = StructuralType::from(&resolved_ty);
            let simplicity_ty = jet.source_ty().to_final();

            println!("{jet}");
            assert_eq!(structural_ty.as_ref(), simplicity_ty.as_ref());
        }
    }

    #[test]
    fn compatible_target_type() {
        for jet in ElementsExtension::ALL {
            let resolved_ty = target_type(jet).resolve_builtin().unwrap();
            let structural_ty = StructuralType::from(&resolved_ty);
            let simplicity_ty = jet.target_ty().to_final();

            println!("{jet}");
            assert_eq!(structural_ty.as_ref(), simplicity_ty.as_ref());
        }
    }
}
