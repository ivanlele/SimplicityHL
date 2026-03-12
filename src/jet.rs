use std::str::FromStr;
use std::sync::Arc;

use crate::num::NonZeroPow2Usize;
use crate::types::BuiltinAlias::*;
use crate::types::UIntType::*;
use crate::types::*;

use simplicity::elements::Transaction;
use simplicity::jet::elements::ElementsEnv;
use simplicity::jet::Core;
use simplicity_unchained::jets::custom_jet::CustomJet;

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

pub fn source_type<E>(jet: CustomJet<E>) -> Vec<AliasedType> {
    let env_type = std::any::TypeId::of::<E>();
    if env_type == std::any::TypeId::of::<ElementsEnv<Arc<Transaction>>>() {
        let jet_casted = unsafe { std::mem::transmute(jet) };
        source_type_elements(jet_casted)
    } else if env_type == std::any::TypeId::of::<()>() {
        let jet_casted = unsafe { std::mem::transmute(jet) };
        source_type_core(jet_casted)
    } else {
        panic!("Unsupported type of jet")
    }
}

pub fn target_type<E>(jet: CustomJet<E>) -> AliasedType {
    let env_type = std::any::TypeId::of::<E>();
    if env_type == std::any::TypeId::of::<ElementsEnv<Arc<Transaction>>>() {
        let jet_casted = unsafe { std::mem::transmute(jet) };
        target_type_elements(jet_casted)
    } else if env_type == std::any::TypeId::of::<()>() {
        let jet_casted = unsafe { std::mem::transmute(jet) };
        target_type_core(jet_casted)
    } else {
        panic!("Unsupported type of jet")
    }
}

fn source_type_elements(jet: CustomJet<ElementsEnv<Arc<Transaction>>>) -> Vec<AliasedType> {
    let try_base_jet: Option<Elements> = unsafe { jet.to_base_jet() };
    match try_base_jet {
        /*
         * ==============================
         *          Core jets
         * ==============================
         *
         * Multi-bit logic
         */
        Some(Elements::Low1)
        | Some(Elements::Low8)
        | Some(Elements::Low16)
        | Some(Elements::Low32)
        | Some(Elements::Low64)
        | Some(Elements::High1)
        | Some(Elements::High8)
        | Some(Elements::High16)
        | Some(Elements::High32)
        | Some(Elements::High64) => vec![],
        Some(Elements::Verify) => vec![bool()],
        Some(Elements::Complement1)
        | Some(Elements::Some1)
        | Some(Elements::LeftPadLow1_8)
        | Some(Elements::LeftPadLow1_16)
        | Some(Elements::LeftPadLow1_32)
        | Some(Elements::LeftPadLow1_64)
        | Some(Elements::LeftPadHigh1_8)
        | Some(Elements::LeftPadHigh1_16)
        | Some(Elements::LeftPadHigh1_32)
        | Some(Elements::LeftPadHigh1_64)
        | Some(Elements::LeftExtend1_8)
        | Some(Elements::LeftExtend1_16)
        | Some(Elements::LeftExtend1_32)
        | Some(Elements::LeftExtend1_64)
        | Some(Elements::RightPadLow1_8)
        | Some(Elements::RightPadLow1_16)
        | Some(Elements::RightPadLow1_32)
        | Some(Elements::RightPadLow1_64)
        | Some(Elements::RightPadHigh1_8)
        | Some(Elements::RightPadHigh1_16)
        | Some(Elements::RightPadHigh1_32)
        | Some(Elements::RightPadHigh1_64) => vec![U1.into()],
        Some(Elements::Complement8)
        | Some(Elements::Some8)
        | Some(Elements::All8)
        | Some(Elements::Leftmost8_1)
        | Some(Elements::Leftmost8_2)
        | Some(Elements::Leftmost8_4)
        | Some(Elements::Rightmost8_1)
        | Some(Elements::Rightmost8_2)
        | Some(Elements::Rightmost8_4)
        | Some(Elements::LeftPadLow8_16)
        | Some(Elements::LeftPadLow8_32)
        | Some(Elements::LeftPadLow8_64)
        | Some(Elements::LeftPadHigh8_16)
        | Some(Elements::LeftPadHigh8_32)
        | Some(Elements::LeftPadHigh8_64)
        | Some(Elements::LeftExtend8_16)
        | Some(Elements::LeftExtend8_32)
        | Some(Elements::LeftExtend8_64)
        | Some(Elements::RightPadLow8_16)
        | Some(Elements::RightPadLow8_32)
        | Some(Elements::RightPadLow8_64)
        | Some(Elements::RightPadHigh8_16)
        | Some(Elements::RightPadHigh8_32)
        | Some(Elements::RightPadHigh8_64)
        | Some(Elements::RightExtend8_16)
        | Some(Elements::RightExtend8_32)
        | Some(Elements::RightExtend8_64) => vec![U8.into()],
        Some(Elements::Complement16)
        | Some(Elements::Some16)
        | Some(Elements::All16)
        | Some(Elements::Leftmost16_1)
        | Some(Elements::Leftmost16_2)
        | Some(Elements::Leftmost16_4)
        | Some(Elements::Leftmost16_8)
        | Some(Elements::Rightmost16_1)
        | Some(Elements::Rightmost16_2)
        | Some(Elements::Rightmost16_4)
        | Some(Elements::Rightmost16_8)
        | Some(Elements::LeftPadLow16_32)
        | Some(Elements::LeftPadLow16_64)
        | Some(Elements::LeftPadHigh16_32)
        | Some(Elements::LeftPadHigh16_64)
        | Some(Elements::LeftExtend16_32)
        | Some(Elements::LeftExtend16_64)
        | Some(Elements::RightPadLow16_32)
        | Some(Elements::RightPadLow16_64)
        | Some(Elements::RightPadHigh16_32)
        | Some(Elements::RightPadHigh16_64)
        | Some(Elements::RightExtend16_32)
        | Some(Elements::RightExtend16_64) => vec![U16.into()],
        Some(Elements::Complement32)
        | Some(Elements::Some32)
        | Some(Elements::All32)
        | Some(Elements::Leftmost32_1)
        | Some(Elements::Leftmost32_2)
        | Some(Elements::Leftmost32_4)
        | Some(Elements::Leftmost32_8)
        | Some(Elements::Leftmost32_16)
        | Some(Elements::Rightmost32_1)
        | Some(Elements::Rightmost32_2)
        | Some(Elements::Rightmost32_4)
        | Some(Elements::Rightmost32_8)
        | Some(Elements::Rightmost32_16)
        | Some(Elements::LeftPadLow32_64)
        | Some(Elements::LeftPadHigh32_64)
        | Some(Elements::LeftExtend32_64)
        | Some(Elements::RightPadLow32_64)
        | Some(Elements::RightPadHigh32_64)
        | Some(Elements::RightExtend32_64) => vec![U32.into()],
        Some(Elements::Complement64)
        | Some(Elements::Some64)
        | Some(Elements::All64)
        | Some(Elements::Leftmost64_1)
        | Some(Elements::Leftmost64_2)
        | Some(Elements::Leftmost64_4)
        | Some(Elements::Leftmost64_8)
        | Some(Elements::Leftmost64_16)
        | Some(Elements::Leftmost64_32)
        | Some(Elements::Rightmost64_1)
        | Some(Elements::Rightmost64_2)
        | Some(Elements::Rightmost64_4)
        | Some(Elements::Rightmost64_8)
        | Some(Elements::Rightmost64_16)
        | Some(Elements::Rightmost64_32) => vec![U64.into()],
        Some(Elements::And1) | Some(Elements::Or1) | Some(Elements::Xor1) | Some(Elements::Eq1) => {
            vec![U1.into(), U1.into()]
        }
        Some(Elements::And8) | Some(Elements::Or8) | Some(Elements::Xor8) | Some(Elements::Eq8) => {
            vec![U8.into(), U8.into()]
        }
        Some(Elements::And16)
        | Some(Elements::Or16)
        | Some(Elements::Xor16)
        | Some(Elements::Eq16) => {
            vec![U16.into(), U16.into()]
        }
        Some(Elements::And32)
        | Some(Elements::Or32)
        | Some(Elements::Xor32)
        | Some(Elements::Eq32) => {
            vec![U32.into(), U32.into()]
        }
        Some(Elements::And64)
        | Some(Elements::Or64)
        | Some(Elements::Xor64)
        | Some(Elements::Eq64) => {
            vec![U64.into(), U64.into()]
        }
        Some(Elements::Eq256) => vec![U256.into(), U256.into()],
        Some(Elements::Maj1) | Some(Elements::XorXor1) | Some(Elements::Ch1) => {
            vec![U1.into(), U1.into(), U1.into()]
        }
        Some(Elements::Maj8) | Some(Elements::XorXor8) | Some(Elements::Ch8) => {
            vec![U8.into(), U8.into(), U8.into()]
        }
        Some(Elements::Maj16) | Some(Elements::XorXor16) | Some(Elements::Ch16) => {
            vec![U16.into(), tuple([U16, U16])]
        }
        Some(Elements::Maj32) | Some(Elements::XorXor32) | Some(Elements::Ch32) => {
            vec![U32.into(), tuple([U32, U32])]
        }
        Some(Elements::Maj64) | Some(Elements::XorXor64) | Some(Elements::Ch64) => {
            vec![U64.into(), tuple([U64, U64])]
        }
        Some(Elements::FullLeftShift8_1) => vec![U8.into(), U1.into()],
        Some(Elements::FullLeftShift8_2) => vec![U8.into(), U2.into()],
        Some(Elements::FullLeftShift8_4) => vec![U8.into(), U4.into()],
        Some(Elements::FullLeftShift16_1) => vec![U16.into(), U1.into()],
        Some(Elements::FullLeftShift16_2) => vec![U16.into(), U2.into()],
        Some(Elements::FullLeftShift16_4) => vec![U16.into(), U4.into()],
        Some(Elements::FullLeftShift16_8) => vec![U16.into(), U8.into()],
        Some(Elements::FullLeftShift32_1) => vec![U32.into(), U1.into()],
        Some(Elements::FullLeftShift32_2) => vec![U32.into(), U2.into()],
        Some(Elements::FullLeftShift32_4) => vec![U32.into(), U4.into()],
        Some(Elements::FullLeftShift32_8) => vec![U32.into(), U8.into()],
        Some(Elements::FullLeftShift32_16) => vec![U32.into(), U16.into()],
        Some(Elements::FullLeftShift64_1) => vec![U64.into(), U1.into()],
        Some(Elements::FullLeftShift64_2) => vec![U64.into(), U2.into()],
        Some(Elements::FullLeftShift64_4) => vec![U64.into(), U4.into()],
        Some(Elements::FullLeftShift64_8) => vec![U64.into(), U8.into()],
        Some(Elements::FullLeftShift64_16) => vec![U64.into(), U16.into()],
        Some(Elements::FullLeftShift64_32) => vec![U64.into(), U32.into()],
        Some(Elements::FullRightShift8_1) => vec![U1.into(), U8.into()],
        Some(Elements::FullRightShift8_2) => vec![U2.into(), U8.into()],
        Some(Elements::FullRightShift8_4) => vec![U4.into(), U8.into()],
        Some(Elements::FullRightShift16_1) => vec![U1.into(), U16.into()],
        Some(Elements::FullRightShift16_2) => vec![U2.into(), U16.into()],
        Some(Elements::FullRightShift16_4) => vec![U4.into(), U16.into()],
        Some(Elements::FullRightShift16_8) => vec![U8.into(), U16.into()],
        Some(Elements::FullRightShift32_1) => vec![U1.into(), U32.into()],
        Some(Elements::FullRightShift32_2) => vec![U2.into(), U32.into()],
        Some(Elements::FullRightShift32_4) => vec![U4.into(), U32.into()],
        Some(Elements::FullRightShift32_8) => vec![U8.into(), U32.into()],
        Some(Elements::FullRightShift32_16) => vec![U16.into(), U32.into()],
        Some(Elements::FullRightShift64_1) => vec![U1.into(), U64.into()],
        Some(Elements::FullRightShift64_2) => vec![U2.into(), U64.into()],
        Some(Elements::FullRightShift64_4) => vec![U4.into(), U64.into()],
        Some(Elements::FullRightShift64_8) => vec![U8.into(), U64.into()],
        Some(Elements::FullRightShift64_16) => vec![U16.into(), U64.into()],
        Some(Elements::FullRightShift64_32) => vec![U32.into(), U64.into()],
        Some(Elements::LeftShiftWith8) | Some(Elements::RightShiftWith8) => {
            vec![U1.into(), U4.into(), U8.into()]
        }
        Some(Elements::LeftShiftWith16) | Some(Elements::RightShiftWith16) => {
            vec![U1.into(), U4.into(), U16.into()]
        }
        Some(Elements::LeftShiftWith32) | Some(Elements::RightShiftWith32) => {
            vec![U1.into(), U8.into(), U32.into()]
        }
        Some(Elements::LeftShiftWith64) | Some(Elements::RightShiftWith64) => {
            vec![U1.into(), U8.into(), U64.into()]
        }
        Some(Elements::LeftShift8)
        | Some(Elements::RightShift8)
        | Some(Elements::LeftRotate8)
        | Some(Elements::RightRotate8) => vec![U4.into(), U8.into()],
        Some(Elements::LeftShift16)
        | Some(Elements::RightShift16)
        | Some(Elements::LeftRotate16)
        | Some(Elements::RightRotate16) => vec![U4.into(), U16.into()],
        Some(Elements::LeftShift32)
        | Some(Elements::RightShift32)
        | Some(Elements::LeftRotate32)
        | Some(Elements::RightRotate32) => vec![U8.into(), U32.into()],
        Some(Elements::LeftShift64)
        | Some(Elements::RightShift64)
        | Some(Elements::LeftRotate64)
        | Some(Elements::RightRotate64) => vec![U8.into(), U64.into()],
        /*
         * Arithmetic
         */
        Some(Elements::One8)
        | Some(Elements::One16)
        | Some(Elements::One32)
        | Some(Elements::One64) => vec![],
        Some(Elements::Increment8)
        | Some(Elements::Negate8)
        | Some(Elements::Decrement8)
        | Some(Elements::IsZero8)
        | Some(Elements::IsOne8) => vec![U8.into()],
        Some(Elements::Increment16)
        | Some(Elements::Negate16)
        | Some(Elements::Decrement16)
        | Some(Elements::IsZero16)
        | Some(Elements::IsOne16) => vec![U16.into()],
        Some(Elements::Increment32)
        | Some(Elements::Negate32)
        | Some(Elements::Decrement32)
        | Some(Elements::IsZero32)
        | Some(Elements::IsOne32) => vec![U32.into()],
        Some(Elements::Increment64)
        | Some(Elements::Negate64)
        | Some(Elements::Decrement64)
        | Some(Elements::IsZero64)
        | Some(Elements::IsOne64) => vec![U64.into()],
        Some(Elements::Add8)
        | Some(Elements::Subtract8)
        | Some(Elements::Multiply8)
        | Some(Elements::Le8)
        | Some(Elements::Lt8)
        | Some(Elements::Min8)
        | Some(Elements::Max8)
        | Some(Elements::DivMod8)
        | Some(Elements::Divide8)
        | Some(Elements::Modulo8)
        | Some(Elements::Divides8) => vec![U8.into(), U8.into()],
        Some(Elements::Add16)
        | Some(Elements::Subtract16)
        | Some(Elements::Multiply16)
        | Some(Elements::Le16)
        | Some(Elements::Lt16)
        | Some(Elements::Min16)
        | Some(Elements::Max16)
        | Some(Elements::DivMod16)
        | Some(Elements::Divide16)
        | Some(Elements::Modulo16)
        | Some(Elements::Divides16) => vec![U16.into(), U16.into()],
        Some(Elements::Add32)
        | Some(Elements::Subtract32)
        | Some(Elements::Multiply32)
        | Some(Elements::Le32)
        | Some(Elements::Lt32)
        | Some(Elements::Min32)
        | Some(Elements::Max32)
        | Some(Elements::DivMod32)
        | Some(Elements::Divide32)
        | Some(Elements::Modulo32)
        | Some(Elements::Divides32) => vec![U32.into(), U32.into()],
        Some(Elements::Add64)
        | Some(Elements::Subtract64)
        | Some(Elements::Multiply64)
        | Some(Elements::Le64)
        | Some(Elements::Lt64)
        | Some(Elements::Min64)
        | Some(Elements::Max64)
        | Some(Elements::DivMod64)
        | Some(Elements::Divide64)
        | Some(Elements::Modulo64)
        | Some(Elements::Divides64) => vec![U64.into(), U64.into()],
        Some(Elements::DivMod128_64) => vec![U128.into(), U64.into()],
        Some(Elements::FullAdd8) | Some(Elements::FullSubtract8) => {
            vec![bool(), U8.into(), U8.into()]
        }
        Some(Elements::FullAdd16) | Some(Elements::FullSubtract16) => {
            vec![bool(), U16.into(), U16.into()]
        }
        Some(Elements::FullAdd32) | Some(Elements::FullSubtract32) => {
            vec![bool(), U32.into(), U32.into()]
        }
        Some(Elements::FullAdd64) | Some(Elements::FullSubtract64) => {
            vec![bool(), U64.into(), U64.into()]
        }
        Some(Elements::FullIncrement8) | Some(Elements::FullDecrement8) => vec![bool(), U8.into()],
        Some(Elements::FullIncrement16) | Some(Elements::FullDecrement16) => {
            vec![bool(), U16.into()]
        }
        Some(Elements::FullIncrement32) | Some(Elements::FullDecrement32) => {
            vec![bool(), U32.into()]
        }
        Some(Elements::FullIncrement64) | Some(Elements::FullDecrement64) => {
            vec![bool(), U64.into()]
        }
        Some(Elements::FullMultiply8) => {
            vec![tuple([U8, U8]), tuple([U8, U8])]
        }
        Some(Elements::FullMultiply16) => {
            vec![tuple([U16, U16]), tuple([U16, U16])]
        }
        Some(Elements::FullMultiply32) => {
            vec![tuple([U32, U32]), tuple([U32, U32])]
        }
        Some(Elements::FullMultiply64) => {
            vec![tuple([U64, U64]), tuple([U64, U64])]
        }
        Some(Elements::Median8) => vec![U8.into(), U8.into(), U8.into()],
        Some(Elements::Median16) => vec![U16.into(), U16.into(), U16.into()],
        Some(Elements::Median32) => vec![U32.into(), U32.into(), U32.into()],
        Some(Elements::Median64) => vec![U64.into(), U64.into(), U64.into()],
        /*
         * Hash functions
         */
        Some(Elements::Sha256Iv) | Some(Elements::Sha256Ctx8Init) => {
            vec![]
        }
        Some(Elements::Sha256Block) => {
            vec![U256.into(), U256.into(), U256.into()]
        }
        Some(Elements::Sha256Ctx8Add1) => vec![Ctx8.into(), U8.into()],
        Some(Elements::Sha256Ctx8Add2) => vec![Ctx8.into(), U16.into()],
        Some(Elements::Sha256Ctx8Add4) => vec![Ctx8.into(), U32.into()],
        Some(Elements::Sha256Ctx8Add8) => vec![Ctx8.into(), U64.into()],
        Some(Elements::Sha256Ctx8Add16) => vec![Ctx8.into(), U128.into()],
        Some(Elements::Sha256Ctx8Add32) => vec![Ctx8.into(), U256.into()],
        Some(Elements::Sha256Ctx8Add64) => vec![Ctx8.into(), array(U8, 64)],
        Some(Elements::Sha256Ctx8Add128) => {
            vec![Ctx8.into(), array(U8, 128)]
        }
        Some(Elements::Sha256Ctx8Add256) => {
            vec![Ctx8.into(), array(U8, 256)]
        }
        Some(Elements::Sha256Ctx8Add512) => {
            vec![Ctx8.into(), array(U8, 512)]
        }
        Some(Elements::Sha256Ctx8AddBuffer511) => {
            vec![Ctx8.into(), list(U8, 512)]
        }
        Some(Elements::Sha256Ctx8Finalize) => vec![Ctx8.into()],
        /*
         * Elliptic curve functions
         */
        // XXX: Nonstandard tuple
        Some(Elements::PointVerify1) => {
            vec![tuple([tuple([Scalar, Point]), Scalar.into()]), Point.into()]
        }
        Some(Elements::Decompress) => vec![Point.into()],
        // XXX: Nonstandard tuple
        Some(Elements::LinearVerify1) => {
            vec![tuple([tuple([Scalar, Ge]), Scalar.into()]), Ge.into()]
        }
        // XXX: Nonstandard tuple
        Some(Elements::LinearCombination1) => {
            vec![tuple([Scalar, Gej]), Scalar.into()]
        }
        Some(Elements::Scale) => vec![Scalar.into(), Gej.into()],
        Some(Elements::Generate) => vec![Scalar.into()],
        Some(Elements::GejInfinity) => vec![],
        Some(Elements::GejNormalize)
        | Some(Elements::GejNegate)
        | Some(Elements::GejDouble)
        | Some(Elements::GejIsInfinity)
        | Some(Elements::GejYIsOdd)
        | Some(Elements::GejIsOnCurve) => vec![Gej.into()],
        Some(Elements::GeNegate) | Some(Elements::GeIsOnCurve) => {
            vec![Ge.into()]
        }
        Some(Elements::GejAdd) | Some(Elements::GejEquiv) => {
            vec![Gej.into(), Gej.into()]
        }
        Some(Elements::GejGeAddEx) | Some(Elements::GejGeAdd) | Some(Elements::GejGeEquiv) => {
            vec![Gej.into(), Ge.into()]
        }
        Some(Elements::GejRescale) => vec![Gej.into(), Fe.into()],
        Some(Elements::GejXEquiv) => vec![Fe.into(), Gej.into()],
        Some(Elements::ScalarAdd) | Some(Elements::ScalarMultiply) => {
            vec![Scalar.into(), Scalar.into()]
        }
        Some(Elements::ScalarNormalize)
        | Some(Elements::ScalarNegate)
        | Some(Elements::ScalarSquare)
        | Some(Elements::ScalarInvert)
        | Some(Elements::ScalarMultiplyLambda)
        | Some(Elements::ScalarIsZero) => vec![Scalar.into()],
        Some(Elements::FeNormalize)
        | Some(Elements::FeNegate)
        | Some(Elements::FeSquare)
        | Some(Elements::FeMultiplyBeta)
        | Some(Elements::FeInvert)
        | Some(Elements::FeSquareRoot)
        | Some(Elements::FeIsZero)
        | Some(Elements::FeIsOdd)
        | Some(Elements::Swu) => vec![Fe.into()],
        Some(Elements::FeAdd) | Some(Elements::FeMultiply) => {
            vec![Fe.into(), Fe.into()]
        }
        Some(Elements::HashToCurve) => vec![U256.into()],
        /*
         * Digital signatures
         */
        // XXX: Nonstandard tuple
        Some(Elements::CheckSigVerify) => {
            vec![tuple([Pubkey, Message64]), Signature.into()]
        }
        // XXX: Nonstandard tuple
        Some(Elements::Bip0340Verify) => {
            vec![tuple([Pubkey, Message]), Signature.into()]
        }
        /*
         * Bitcoin (without primitives)
         */
        Some(Elements::TapdataInit) => vec![],
        Some(Elements::ParseLock) | Some(Elements::ParseSequence) => {
            vec![U32.into()]
        }
        /*
         * ==============================
         *         Elements jets
         * ==============================
         *
         * Signature hash modes
         */
        Some(Elements::SigAllHash)
        | Some(Elements::TxHash)
        | Some(Elements::TapEnvHash)
        | Some(Elements::InputsHash)
        | Some(Elements::OutputsHash)
        | Some(Elements::IssuancesHash)
        | Some(Elements::InputUtxosHash)
        | Some(Elements::OutputAmountsHash)
        | Some(Elements::OutputScriptsHash)
        | Some(Elements::OutputNoncesHash)
        | Some(Elements::OutputRangeProofsHash)
        | Some(Elements::OutputSurjectionProofsHash)
        | Some(Elements::InputOutpointsHash)
        | Some(Elements::InputAnnexesHash)
        | Some(Elements::InputSequencesHash)
        | Some(Elements::InputScriptSigsHash)
        | Some(Elements::IssuanceAssetAmountsHash)
        | Some(Elements::IssuanceTokenAmountsHash)
        | Some(Elements::IssuanceRangeProofsHash)
        | Some(Elements::IssuanceBlindingEntropyHash)
        | Some(Elements::InputAmountsHash)
        | Some(Elements::InputScriptsHash)
        | Some(Elements::TapleafHash)
        | Some(Elements::TappathHash) => vec![],
        Some(Elements::OutpointHash) => {
            vec![Ctx8.into(), option(U256), Outpoint.into()]
        }
        Some(Elements::AssetAmountHash) => {
            vec![Ctx8.into(), Asset1.into(), Amount1.into()]
        }
        Some(Elements::NonceHash) => vec![Ctx8.into(), option(Nonce)],
        Some(Elements::AnnexHash) => vec![Ctx8.into(), option(U256)],
        Some(Elements::BuildTapleafSimplicity) => vec![U256.into()],
        Some(Elements::BuildTapbranch) => vec![U256.into(), U256.into()],
        Some(Elements::BuildTaptweak) => vec![Pubkey.into(), U256.into()],
        /*
         * Time locks
         */
        Some(Elements::CheckLockTime) => vec![Time.into()],
        Some(Elements::CheckLockDistance) => vec![Distance.into()],
        Some(Elements::CheckLockDuration) => vec![Duration.into()],
        Some(Elements::CheckLockHeight) => vec![Height.into()],
        Some(Elements::TxLockTime)
        | Some(Elements::TxLockDistance)
        | Some(Elements::TxLockDuration)
        | Some(Elements::TxLockHeight)
        | Some(Elements::TxIsFinal) => vec![],
        /*
         * Issuance
         */
        Some(Elements::Issuance)
        | Some(Elements::IssuanceAsset)
        | Some(Elements::IssuanceToken)
        | Some(Elements::IssuanceEntropy) => vec![U32.into()],
        Some(Elements::CalculateIssuanceEntropy) => {
            vec![Outpoint.into(), U256.into()]
        }
        Some(Elements::CalculateAsset)
        | Some(Elements::CalculateExplicitToken)
        | Some(Elements::CalculateConfidentialToken) => vec![U256.into()],
        /*
         * Transaction
         */
        Some(Elements::ScriptCMR)
        | Some(Elements::InternalKey)
        | Some(Elements::CurrentIndex)
        | Some(Elements::NumInputs)
        | Some(Elements::NumOutputs)
        | Some(Elements::LockTime)
        | Some(Elements::CurrentPegin)
        | Some(Elements::CurrentPrevOutpoint)
        | Some(Elements::CurrentAsset)
        | Some(Elements::CurrentAmount)
        | Some(Elements::CurrentScriptHash)
        | Some(Elements::CurrentSequence)
        | Some(Elements::CurrentAnnexHash)
        | Some(Elements::CurrentScriptSigHash)
        | Some(Elements::CurrentReissuanceBlinding)
        | Some(Elements::CurrentNewIssuanceContract)
        | Some(Elements::CurrentReissuanceEntropy)
        | Some(Elements::CurrentIssuanceTokenAmount)
        | Some(Elements::CurrentIssuanceAssetAmount)
        | Some(Elements::CurrentIssuanceAssetProof)
        | Some(Elements::CurrentIssuanceTokenProof)
        | Some(Elements::TapleafVersion)
        | Some(Elements::Version)
        | Some(Elements::GenesisBlockHash)
        | Some(Elements::LbtcAsset)
        | Some(Elements::TransactionId) => vec![],
        Some(Elements::OutputAsset)
        | Some(Elements::OutputAmount)
        | Some(Elements::OutputNonce)
        | Some(Elements::OutputScriptHash)
        | Some(Elements::OutputIsFee)
        | Some(Elements::OutputSurjectionProof)
        | Some(Elements::OutputRangeProof)
        | Some(Elements::OutputHash)
        | Some(Elements::InputPegin)
        | Some(Elements::InputPrevOutpoint)
        | Some(Elements::InputAsset)
        | Some(Elements::InputAmount)
        | Some(Elements::InputScriptHash)
        | Some(Elements::InputSequence)
        | Some(Elements::InputAnnexHash)
        | Some(Elements::InputScriptSigHash)
        | Some(Elements::InputHash)
        | Some(Elements::InputUtxoHash)
        | Some(Elements::ReissuanceBlinding)
        | Some(Elements::NewIssuanceContract)
        | Some(Elements::ReissuanceEntropy)
        | Some(Elements::IssuanceAssetAmount)
        | Some(Elements::IssuanceTokenAmount)
        | Some(Elements::IssuanceAssetProof)
        | Some(Elements::IssuanceTokenProof)
        | Some(Elements::IssuanceHash) => vec![U32.into()],
        Some(Elements::OutputNullDatum) => vec![U32.into(), U32.into()],
        Some(Elements::TotalFee) => vec![ExplicitAsset.into()],
        Some(Elements::Tappath) => vec![U8.into()],
        None => vec![U8.into()],
    }
}

fn target_type_elements(jet: CustomJet<ElementsEnv<Arc<Transaction>>>) -> AliasedType {
    let try_base_jet: Option<Elements> = unsafe { jet.to_base_jet() };

    match try_base_jet {
        /*
         * ==============================
         *          Core jets
         * ==============================
         *
         * Multi-bit logic
         */
        Some(Elements::Verify) => AliasedType::unit(),
        Some(Elements::Some1)
        | Some(Elements::Some8)
        | Some(Elements::Some16)
        | Some(Elements::Some32)
        | Some(Elements::Some64)
        | Some(Elements::All8)
        | Some(Elements::All16)
        | Some(Elements::All32)
        | Some(Elements::All64)
        | Some(Elements::Eq1)
        | Some(Elements::Eq8)
        | Some(Elements::Eq16)
        | Some(Elements::Eq32)
        | Some(Elements::Eq64)
        | Some(Elements::Eq256) => bool(),
        Some(Elements::Low1)
        | Some(Elements::High1)
        | Some(Elements::Complement1)
        | Some(Elements::And1)
        | Some(Elements::Or1)
        | Some(Elements::Xor1)
        | Some(Elements::Maj1)
        | Some(Elements::XorXor1)
        | Some(Elements::Ch1)
        | Some(Elements::Leftmost8_1)
        | Some(Elements::Rightmost8_1)
        | Some(Elements::Leftmost16_1)
        | Some(Elements::Rightmost16_1)
        | Some(Elements::Leftmost32_1)
        | Some(Elements::Rightmost32_1)
        | Some(Elements::Leftmost64_1)
        | Some(Elements::Rightmost64_1) => U1.into(),
        Some(Elements::Leftmost8_2)
        | Some(Elements::Rightmost8_2)
        | Some(Elements::Leftmost16_2)
        | Some(Elements::Rightmost16_2)
        | Some(Elements::Leftmost32_2)
        | Some(Elements::Rightmost32_2)
        | Some(Elements::Leftmost64_2)
        | Some(Elements::Rightmost64_2) => U2.into(),
        Some(Elements::Leftmost8_4)
        | Some(Elements::Rightmost8_4)
        | Some(Elements::Leftmost16_4)
        | Some(Elements::Rightmost16_4)
        | Some(Elements::Leftmost32_4)
        | Some(Elements::Rightmost32_4)
        | Some(Elements::Leftmost64_4)
        | Some(Elements::Rightmost64_4) => U4.into(),
        Some(Elements::Low8)
        | Some(Elements::High8)
        | Some(Elements::Complement8)
        | Some(Elements::And8)
        | Some(Elements::Or8)
        | Some(Elements::Xor8)
        | Some(Elements::Maj8)
        | Some(Elements::XorXor8)
        | Some(Elements::Ch8)
        | Some(Elements::Leftmost16_8)
        | Some(Elements::Rightmost16_8)
        | Some(Elements::Leftmost32_8)
        | Some(Elements::Rightmost32_8)
        | Some(Elements::Leftmost64_8)
        | Some(Elements::Rightmost64_8)
        | Some(Elements::LeftPadLow1_8)
        | Some(Elements::LeftPadHigh1_8)
        | Some(Elements::LeftExtend1_8)
        | Some(Elements::RightPadLow1_8)
        | Some(Elements::RightPadHigh1_8)
        | Some(Elements::LeftShiftWith8)
        | Some(Elements::RightShiftWith8)
        | Some(Elements::LeftShift8)
        | Some(Elements::RightShift8)
        | Some(Elements::LeftRotate8)
        | Some(Elements::RightRotate8) => U8.into(),
        Some(Elements::Low16)
        | Some(Elements::High16)
        | Some(Elements::Complement16)
        | Some(Elements::And16)
        | Some(Elements::Or16)
        | Some(Elements::Xor16)
        | Some(Elements::Maj16)
        | Some(Elements::XorXor16)
        | Some(Elements::Ch16)
        | Some(Elements::Leftmost32_16)
        | Some(Elements::Rightmost32_16)
        | Some(Elements::Leftmost64_16)
        | Some(Elements::Rightmost64_16)
        | Some(Elements::LeftPadLow1_16)
        | Some(Elements::LeftPadHigh1_16)
        | Some(Elements::LeftExtend1_16)
        | Some(Elements::RightPadLow1_16)
        | Some(Elements::RightPadHigh1_16)
        | Some(Elements::LeftPadLow8_16)
        | Some(Elements::LeftPadHigh8_16)
        | Some(Elements::LeftExtend8_16)
        | Some(Elements::RightPadLow8_16)
        | Some(Elements::RightPadHigh8_16)
        | Some(Elements::RightExtend8_16)
        | Some(Elements::LeftShiftWith16)
        | Some(Elements::RightShiftWith16)
        | Some(Elements::LeftShift16)
        | Some(Elements::RightShift16)
        | Some(Elements::LeftRotate16)
        | Some(Elements::RightRotate16) => U16.into(),
        Some(Elements::Low32)
        | Some(Elements::High32)
        | Some(Elements::Complement32)
        | Some(Elements::And32)
        | Some(Elements::Or32)
        | Some(Elements::Xor32)
        | Some(Elements::Maj32)
        | Some(Elements::XorXor32)
        | Some(Elements::Ch32)
        | Some(Elements::Leftmost64_32)
        | Some(Elements::Rightmost64_32)
        | Some(Elements::LeftPadLow1_32)
        | Some(Elements::LeftPadHigh1_32)
        | Some(Elements::LeftExtend1_32)
        | Some(Elements::RightPadLow1_32)
        | Some(Elements::RightPadHigh1_32)
        | Some(Elements::LeftPadLow8_32)
        | Some(Elements::LeftPadHigh8_32)
        | Some(Elements::LeftExtend8_32)
        | Some(Elements::RightPadLow8_32)
        | Some(Elements::RightPadHigh8_32)
        | Some(Elements::RightExtend8_32)
        | Some(Elements::LeftPadLow16_32)
        | Some(Elements::LeftPadHigh16_32)
        | Some(Elements::LeftExtend16_32)
        | Some(Elements::RightPadLow16_32)
        | Some(Elements::RightPadHigh16_32)
        | Some(Elements::RightExtend16_32)
        | Some(Elements::LeftShiftWith32)
        | Some(Elements::RightShiftWith32)
        | Some(Elements::LeftShift32)
        | Some(Elements::RightShift32)
        | Some(Elements::LeftRotate32)
        | Some(Elements::RightRotate32) => U32.into(),
        Some(Elements::Low64)
        | Some(Elements::High64)
        | Some(Elements::Complement64)
        | Some(Elements::And64)
        | Some(Elements::Or64)
        | Some(Elements::Xor64)
        | Some(Elements::Maj64)
        | Some(Elements::XorXor64)
        | Some(Elements::Ch64)
        | Some(Elements::LeftPadLow1_64)
        | Some(Elements::LeftPadHigh1_64)
        | Some(Elements::LeftExtend1_64)
        | Some(Elements::RightPadLow1_64)
        | Some(Elements::RightPadHigh1_64)
        | Some(Elements::LeftPadLow8_64)
        | Some(Elements::LeftPadHigh8_64)
        | Some(Elements::LeftExtend8_64)
        | Some(Elements::RightPadLow8_64)
        | Some(Elements::RightPadHigh8_64)
        | Some(Elements::RightExtend8_64)
        | Some(Elements::LeftPadLow16_64)
        | Some(Elements::LeftPadHigh16_64)
        | Some(Elements::LeftExtend16_64)
        | Some(Elements::RightPadLow16_64)
        | Some(Elements::RightPadHigh16_64)
        | Some(Elements::RightExtend16_64)
        | Some(Elements::LeftPadLow32_64)
        | Some(Elements::LeftPadHigh32_64)
        | Some(Elements::LeftExtend32_64)
        | Some(Elements::RightPadLow32_64)
        | Some(Elements::RightPadHigh32_64)
        | Some(Elements::RightExtend32_64)
        | Some(Elements::LeftShiftWith64)
        | Some(Elements::RightShiftWith64)
        | Some(Elements::LeftShift64)
        | Some(Elements::RightShift64)
        | Some(Elements::LeftRotate64)
        | Some(Elements::RightRotate64) => U64.into(),
        Some(Elements::FullLeftShift8_1) => tuple([U1, U8]),
        Some(Elements::FullLeftShift8_2) => tuple([U2, U8]),
        Some(Elements::FullLeftShift8_4) => tuple([U4, U8]),
        Some(Elements::FullLeftShift16_1) => tuple([U1, U16]),
        Some(Elements::FullLeftShift16_2) => tuple([U2, U16]),
        Some(Elements::FullLeftShift16_4) => tuple([U4, U16]),
        Some(Elements::FullLeftShift16_8) => tuple([U8, U16]),
        Some(Elements::FullLeftShift32_1) => tuple([U1, U32]),
        Some(Elements::FullLeftShift32_2) => tuple([U2, U32]),
        Some(Elements::FullLeftShift32_4) => tuple([U4, U32]),
        Some(Elements::FullLeftShift32_8) => tuple([U8, U32]),
        Some(Elements::FullLeftShift32_16) => tuple([U16, U32]),
        Some(Elements::FullLeftShift64_1) => tuple([U1, U64]),
        Some(Elements::FullLeftShift64_2) => tuple([U2, U64]),
        Some(Elements::FullLeftShift64_4) => tuple([U4, U64]),
        Some(Elements::FullLeftShift64_8) => tuple([U8, U64]),
        Some(Elements::FullLeftShift64_16) => tuple([U16, U64]),
        Some(Elements::FullLeftShift64_32) => tuple([U32, U64]),
        Some(Elements::FullRightShift8_1) => tuple([U8, U1]),
        Some(Elements::FullRightShift8_2) => tuple([U8, U2]),
        Some(Elements::FullRightShift8_4) => tuple([U8, U4]),
        Some(Elements::FullRightShift16_1) => tuple([U16, U1]),
        Some(Elements::FullRightShift16_2) => tuple([U16, U2]),
        Some(Elements::FullRightShift16_4) => tuple([U16, U4]),
        Some(Elements::FullRightShift16_8) => tuple([U16, U8]),
        Some(Elements::FullRightShift32_1) => tuple([U32, U1]),
        Some(Elements::FullRightShift32_2) => tuple([U32, U2]),
        Some(Elements::FullRightShift32_4) => tuple([U32, U4]),
        Some(Elements::FullRightShift32_8) => tuple([U32, U8]),
        Some(Elements::FullRightShift32_16) => tuple([U32, U16]),
        Some(Elements::FullRightShift64_1) => tuple([U64, U1]),
        Some(Elements::FullRightShift64_2) => tuple([U64, U2]),
        Some(Elements::FullRightShift64_4) => tuple([U64, U4]),
        Some(Elements::FullRightShift64_8) => tuple([U64, U8]),
        Some(Elements::FullRightShift64_16) => tuple([U64, U16]),
        Some(Elements::FullRightShift64_32) => tuple([U64, U32]),
        /*
         * Arithmetic
         */
        Some(Elements::Le8)
        | Some(Elements::Lt8)
        | Some(Elements::Le16)
        | Some(Elements::Lt16)
        | Some(Elements::Le32)
        | Some(Elements::Lt32)
        | Some(Elements::Le64)
        | Some(Elements::Lt64)
        | Some(Elements::IsZero8)
        | Some(Elements::IsOne8)
        | Some(Elements::IsZero16)
        | Some(Elements::IsOne16)
        | Some(Elements::IsZero32)
        | Some(Elements::IsOne32)
        | Some(Elements::IsZero64)
        | Some(Elements::IsOne64)
        | Some(Elements::Divides8)
        | Some(Elements::Divides16)
        | Some(Elements::Divides32)
        | Some(Elements::Divides64) => bool(),
        Some(Elements::One8)
        | Some(Elements::Min8)
        | Some(Elements::Max8)
        | Some(Elements::Divide8)
        | Some(Elements::Modulo8)
        | Some(Elements::Median8) => U8.into(),
        Some(Elements::One16)
        | Some(Elements::Min16)
        | Some(Elements::Max16)
        | Some(Elements::Divide16)
        | Some(Elements::Modulo16)
        | Some(Elements::Multiply8)
        | Some(Elements::FullMultiply8)
        | Some(Elements::Median16) => U16.into(),
        Some(Elements::One32)
        | Some(Elements::Min32)
        | Some(Elements::Max32)
        | Some(Elements::Divide32)
        | Some(Elements::Modulo32)
        | Some(Elements::Multiply16)
        | Some(Elements::FullMultiply16)
        | Some(Elements::Median32) => U32.into(),
        Some(Elements::One64)
        | Some(Elements::Min64)
        | Some(Elements::Max64)
        | Some(Elements::Divide64)
        | Some(Elements::Modulo64)
        | Some(Elements::Multiply32)
        | Some(Elements::FullMultiply32)
        | Some(Elements::Median64) => U64.into(),
        Some(Elements::Multiply64) | Some(Elements::FullMultiply64) => U128.into(),
        Some(Elements::Increment8)
        | Some(Elements::Negate8)
        | Some(Elements::Decrement8)
        | Some(Elements::Add8)
        | Some(Elements::Subtract8)
        | Some(Elements::FullAdd8)
        | Some(Elements::FullSubtract8)
        | Some(Elements::FullIncrement8)
        | Some(Elements::FullDecrement8) => tuple([bool(), U8.into()]),
        Some(Elements::Increment16)
        | Some(Elements::Negate16)
        | Some(Elements::Decrement16)
        | Some(Elements::Add16)
        | Some(Elements::Subtract16)
        | Some(Elements::FullAdd16)
        | Some(Elements::FullSubtract16)
        | Some(Elements::FullIncrement16)
        | Some(Elements::FullDecrement16) => tuple([bool(), U16.into()]),
        Some(Elements::Increment32)
        | Some(Elements::Negate32)
        | Some(Elements::Decrement32)
        | Some(Elements::Add32)
        | Some(Elements::Subtract32)
        | Some(Elements::FullAdd32)
        | Some(Elements::FullSubtract32)
        | Some(Elements::FullIncrement32)
        | Some(Elements::FullDecrement32) => tuple([bool(), U32.into()]),
        Some(Elements::Increment64)
        | Some(Elements::Negate64)
        | Some(Elements::Decrement64)
        | Some(Elements::Add64)
        | Some(Elements::Subtract64)
        | Some(Elements::FullAdd64)
        | Some(Elements::FullSubtract64)
        | Some(Elements::FullIncrement64)
        | Some(Elements::FullDecrement64) => tuple([bool(), U64.into()]),
        Some(Elements::DivMod8) => tuple([U8, U8]),
        Some(Elements::DivMod16) => tuple([U16, U16]),
        Some(Elements::DivMod32) => tuple([U32, U32]),
        Some(Elements::DivMod64) => tuple([U64, U64]),
        Some(Elements::DivMod128_64) => tuple([U64, U64]),
        /*
         * Hash functions
         */
        Some(Elements::Sha256Iv)
        | Some(Elements::Sha256Block)
        | Some(Elements::Sha256Ctx8Finalize) => U256.into(),
        Some(Elements::Sha256Ctx8Init)
        | Some(Elements::Sha256Ctx8Add1)
        | Some(Elements::Sha256Ctx8Add2)
        | Some(Elements::Sha256Ctx8Add4)
        | Some(Elements::Sha256Ctx8Add8)
        | Some(Elements::Sha256Ctx8Add16)
        | Some(Elements::Sha256Ctx8Add32)
        | Some(Elements::Sha256Ctx8Add64)
        | Some(Elements::Sha256Ctx8Add128)
        | Some(Elements::Sha256Ctx8Add256)
        | Some(Elements::Sha256Ctx8Add512)
        | Some(Elements::Sha256Ctx8AddBuffer511) => Ctx8.into(),
        /*
         * Elliptic curve functions
         */
        Some(Elements::PointVerify1) | Some(Elements::LinearVerify1) => AliasedType::unit(),
        Some(Elements::GejIsInfinity)
        | Some(Elements::GejEquiv)
        | Some(Elements::GejGeEquiv)
        | Some(Elements::GejXEquiv)
        | Some(Elements::GejYIsOdd)
        | Some(Elements::GejIsOnCurve)
        | Some(Elements::GeIsOnCurve)
        | Some(Elements::ScalarIsZero)
        | Some(Elements::FeIsZero)
        | Some(Elements::FeIsOdd) => bool(),
        Some(Elements::GeNegate) | Some(Elements::HashToCurve) | Some(Elements::Swu) => Ge.into(),
        Some(Elements::Decompress) | Some(Elements::GejNormalize) => option(Ge),
        Some(Elements::LinearCombination1)
        | Some(Elements::Scale)
        | Some(Elements::Generate)
        | Some(Elements::GejInfinity)
        | Some(Elements::GejNegate)
        | Some(Elements::GejDouble)
        | Some(Elements::GejAdd)
        | Some(Elements::GejGeAdd)
        | Some(Elements::GejRescale) => Gej.into(),
        Some(Elements::GejGeAddEx) => tuple([Fe, Gej]),
        Some(Elements::ScalarNormalize)
        | Some(Elements::ScalarNegate)
        | Some(Elements::ScalarAdd)
        | Some(Elements::ScalarSquare)
        | Some(Elements::ScalarMultiply)
        | Some(Elements::ScalarMultiplyLambda)
        | Some(Elements::ScalarInvert) => Scalar.into(),
        Some(Elements::FeNormalize)
        | Some(Elements::FeNegate)
        | Some(Elements::FeAdd)
        | Some(Elements::FeSquare)
        | Some(Elements::FeMultiply)
        | Some(Elements::FeMultiplyBeta)
        | Some(Elements::FeInvert) => Fe.into(),
        Some(Elements::FeSquareRoot) => option(Fe),
        /*
         * Digital signatures
         */
        Some(Elements::CheckSigVerify) | Some(Elements::Bip0340Verify) => AliasedType::unit(),
        /*
         * Bitcoin (without primitives)
         */
        Some(Elements::ParseLock) => either(Height, Time),
        Some(Elements::ParseSequence) => option(either(Distance, Duration)),
        Some(Elements::TapdataInit) => Ctx8.into(),
        /*
         * ==============================
         *         Elements jets
         * ==============================
         *
         * Signature hash modes
         */
        Some(Elements::SigAllHash)
        | Some(Elements::TxHash)
        | Some(Elements::TapEnvHash)
        | Some(Elements::InputsHash)
        | Some(Elements::OutputsHash)
        | Some(Elements::IssuancesHash)
        | Some(Elements::InputUtxosHash)
        | Some(Elements::OutputAmountsHash)
        | Some(Elements::OutputScriptsHash)
        | Some(Elements::OutputNoncesHash)
        | Some(Elements::OutputRangeProofsHash)
        | Some(Elements::OutputSurjectionProofsHash)
        | Some(Elements::InputOutpointsHash)
        | Some(Elements::InputAnnexesHash)
        | Some(Elements::InputSequencesHash)
        | Some(Elements::InputScriptSigsHash)
        | Some(Elements::IssuanceAssetAmountsHash)
        | Some(Elements::IssuanceTokenAmountsHash)
        | Some(Elements::IssuanceRangeProofsHash)
        | Some(Elements::IssuanceBlindingEntropyHash)
        | Some(Elements::InputAmountsHash)
        | Some(Elements::InputScriptsHash)
        | Some(Elements::TapleafHash)
        | Some(Elements::TappathHash)
        | Some(Elements::BuildTapleafSimplicity)
        | Some(Elements::BuildTapbranch)
        | Some(Elements::BuildTaptweak) => U256.into(),
        Some(Elements::OutpointHash)
        | Some(Elements::AssetAmountHash)
        | Some(Elements::NonceHash)
        | Some(Elements::AnnexHash) => Ctx8.into(),
        /*
         * Time locks
         */
        Some(Elements::CheckLockTime)
        | Some(Elements::CheckLockDistance)
        | Some(Elements::CheckLockDuration)
        | Some(Elements::CheckLockHeight) => AliasedType::unit(),
        Some(Elements::TxIsFinal) => bool(),
        Some(Elements::TxLockTime) => Time.into(),
        Some(Elements::TxLockDistance) => Distance.into(),
        Some(Elements::TxLockDuration) => Duration.into(),
        Some(Elements::TxLockHeight) => Height.into(),
        /*
         * Issuance
         */
        Some(Elements::Issuance) => option(option(bool())),
        Some(Elements::IssuanceAsset) | Some(Elements::IssuanceToken) => {
            option(option(ExplicitAsset))
        }
        Some(Elements::IssuanceEntropy) => option(option(U256)),
        Some(Elements::CalculateIssuanceEntropy) => U256.into(),
        Some(Elements::CalculateAsset)
        | Some(Elements::CalculateExplicitToken)
        | Some(Elements::CalculateConfidentialToken) => ExplicitAsset.into(),
        /*
         * Transaction
         */
        Some(Elements::TapleafVersion) => U8.into(),
        Some(Elements::CurrentIndex)
        | Some(Elements::NumInputs)
        | Some(Elements::NumOutputs)
        | Some(Elements::CurrentSequence)
        | Some(Elements::Version) => U32.into(),
        Some(Elements::ScriptCMR)
        | Some(Elements::CurrentScriptHash)
        | Some(Elements::CurrentScriptSigHash)
        | Some(Elements::CurrentIssuanceAssetProof)
        | Some(Elements::CurrentIssuanceTokenProof)
        | Some(Elements::GenesisBlockHash)
        | Some(Elements::LbtcAsset)
        | Some(Elements::TransactionId) => U256.into(),
        Some(Elements::InternalKey) => Pubkey.into(),
        Some(Elements::LockTime) => Lock.into(),
        Some(Elements::InputSequence) => option(U32),
        Some(Elements::OutputAsset) => option(Asset1),
        Some(Elements::OutputAmount) => option(tuple([Asset1, Amount1])),
        Some(Elements::OutputNonce) => option(option(Nonce)),
        Some(Elements::OutputScriptHash)
        | Some(Elements::OutputSurjectionProof)
        | Some(Elements::OutputRangeProof)
        | Some(Elements::OutputHash)
        | Some(Elements::CurrentPegin)
        | Some(Elements::CurrentAnnexHash)
        | Some(Elements::CurrentNewIssuanceContract)
        | Some(Elements::CurrentReissuanceEntropy)
        | Some(Elements::InputScriptHash)
        | Some(Elements::InputScriptSigHash)
        | Some(Elements::InputHash)
        | Some(Elements::InputUtxoHash)
        | Some(Elements::IssuanceAssetProof)
        | Some(Elements::IssuanceTokenProof)
        | Some(Elements::IssuanceHash)
        | Some(Elements::Tappath) => option(U256),
        Some(Elements::OutputNullDatum) => {
            option(option(either(tuple([U2, U256]), either(U1, U4))))
        }
        Some(Elements::OutputIsFee) => option(bool()),
        Some(Elements::TotalFee) => ExplicitAmount.into(),
        Some(Elements::CurrentPrevOutpoint) => Outpoint.into(),
        Some(Elements::CurrentAsset) => Asset1.into(),
        Some(Elements::CurrentAmount) => tuple([Asset1, Amount1]),
        Some(Elements::CurrentReissuanceBlinding) => option(ExplicitNonce),
        Some(Elements::CurrentIssuanceAssetAmount) => option(Amount1),
        Some(Elements::CurrentIssuanceTokenAmount) => option(TokenAmount1),
        Some(Elements::InputPegin)
        | Some(Elements::InputAnnexHash)
        | Some(Elements::NewIssuanceContract)
        | Some(Elements::ReissuanceEntropy) => option(option(U256)),
        Some(Elements::InputPrevOutpoint) => option(Outpoint),
        Some(Elements::InputAsset) => option(Asset1),
        Some(Elements::InputAmount) => option(tuple([Asset1, Amount1])),
        Some(Elements::ReissuanceBlinding) => option(option(ExplicitNonce)),
        Some(Elements::IssuanceAssetAmount) => option(option(Amount1)),
        Some(Elements::IssuanceTokenAmount) => option(option(TokenAmount1)),
        None => {
            let (get_opcode, get_pubkey) = (
                CustomJet::from_str("get_opcode_from_script").unwrap(),
                CustomJet::from_str("get_pubkey_from_script").unwrap(),
            );

            if jet == get_opcode {
                U8.into()
            } else if jet == get_pubkey {
                Pubkey.into()
            } else {
                unreachable!()
            }
        }
    }
}

fn source_type_core(jet: CustomJet<()>) -> Vec<AliasedType> {
    let try_base_jet: Option<Core> = unsafe { jet.to_base_jet() };

    match try_base_jet {
        /*
         * ==============================
         *          Core jets
         * ==============================
         *
         * Multi-bit logic
         */
        Some(Core::Low1) | Some(Core::Low8) | Some(Core::Low16) | Some(Core::Low32)
        | Some(Core::Low64) | Some(Core::High1) | Some(Core::High8) | Some(Core::High16)
        | Some(Core::High32) | Some(Core::High64) => vec![],
        Some(Core::Verify) => vec![bool()],
        Some(Core::Complement1)
        | Some(Core::Some1)
        | Some(Core::LeftPadLow1_8)
        | Some(Core::LeftPadLow1_16)
        | Some(Core::LeftPadLow1_32)
        | Some(Core::LeftPadLow1_64)
        | Some(Core::LeftPadHigh1_8)
        | Some(Core::LeftPadHigh1_16)
        | Some(Core::LeftPadHigh1_32)
        | Some(Core::LeftPadHigh1_64)
        | Some(Core::LeftExtend1_8)
        | Some(Core::LeftExtend1_16)
        | Some(Core::LeftExtend1_32)
        | Some(Core::LeftExtend1_64)
        | Some(Core::RightPadLow1_8)
        | Some(Core::RightPadLow1_16)
        | Some(Core::RightPadLow1_32)
        | Some(Core::RightPadLow1_64)
        | Some(Core::RightPadHigh1_8)
        | Some(Core::RightPadHigh1_16)
        | Some(Core::RightPadHigh1_32)
        | Some(Core::RightPadHigh1_64) => vec![U1.into()],
        Some(Core::Complement8)
        | Some(Core::Some8)
        | Some(Core::All8)
        | Some(Core::Leftmost8_1)
        | Some(Core::Leftmost8_2)
        | Some(Core::Leftmost8_4)
        | Some(Core::Rightmost8_1)
        | Some(Core::Rightmost8_2)
        | Some(Core::Rightmost8_4)
        | Some(Core::LeftPadLow8_16)
        | Some(Core::LeftPadLow8_32)
        | Some(Core::LeftPadLow8_64)
        | Some(Core::LeftPadHigh8_16)
        | Some(Core::LeftPadHigh8_32)
        | Some(Core::LeftPadHigh8_64)
        | Some(Core::LeftExtend8_16)
        | Some(Core::LeftExtend8_32)
        | Some(Core::LeftExtend8_64)
        | Some(Core::RightPadLow8_16)
        | Some(Core::RightPadLow8_32)
        | Some(Core::RightPadLow8_64)
        | Some(Core::RightPadHigh8_16)
        | Some(Core::RightPadHigh8_32)
        | Some(Core::RightPadHigh8_64)
        | Some(Core::RightExtend8_16)
        | Some(Core::RightExtend8_32)
        | Some(Core::RightExtend8_64) => vec![U8.into()],
        Some(Core::Complement16)
        | Some(Core::Some16)
        | Some(Core::All16)
        | Some(Core::Leftmost16_1)
        | Some(Core::Leftmost16_2)
        | Some(Core::Leftmost16_4)
        | Some(Core::Leftmost16_8)
        | Some(Core::Rightmost16_1)
        | Some(Core::Rightmost16_2)
        | Some(Core::Rightmost16_4)
        | Some(Core::Rightmost16_8)
        | Some(Core::LeftPadLow16_32)
        | Some(Core::LeftPadLow16_64)
        | Some(Core::LeftPadHigh16_32)
        | Some(Core::LeftPadHigh16_64)
        | Some(Core::LeftExtend16_32)
        | Some(Core::LeftExtend16_64)
        | Some(Core::RightPadLow16_32)
        | Some(Core::RightPadLow16_64)
        | Some(Core::RightPadHigh16_32)
        | Some(Core::RightPadHigh16_64)
        | Some(Core::RightExtend16_32)
        | Some(Core::RightExtend16_64) => vec![U16.into()],
        Some(Core::Complement32)
        | Some(Core::Some32)
        | Some(Core::All32)
        | Some(Core::Leftmost32_1)
        | Some(Core::Leftmost32_2)
        | Some(Core::Leftmost32_4)
        | Some(Core::Leftmost32_8)
        | Some(Core::Leftmost32_16)
        | Some(Core::Rightmost32_1)
        | Some(Core::Rightmost32_2)
        | Some(Core::Rightmost32_4)
        | Some(Core::Rightmost32_8)
        | Some(Core::Rightmost32_16)
        | Some(Core::LeftPadLow32_64)
        | Some(Core::LeftPadHigh32_64)
        | Some(Core::LeftExtend32_64)
        | Some(Core::RightPadLow32_64)
        | Some(Core::RightPadHigh32_64)
        | Some(Core::RightExtend32_64) => vec![U32.into()],
        Some(Core::Complement64)
        | Some(Core::Some64)
        | Some(Core::All64)
        | Some(Core::Leftmost64_1)
        | Some(Core::Leftmost64_2)
        | Some(Core::Leftmost64_4)
        | Some(Core::Leftmost64_8)
        | Some(Core::Leftmost64_16)
        | Some(Core::Leftmost64_32)
        | Some(Core::Rightmost64_1)
        | Some(Core::Rightmost64_2)
        | Some(Core::Rightmost64_4)
        | Some(Core::Rightmost64_8)
        | Some(Core::Rightmost64_16)
        | Some(Core::Rightmost64_32) => vec![U64.into()],
        Some(Core::And1) | Some(Core::Or1) | Some(Core::Xor1) | Some(Core::Eq1) => {
            vec![U1.into(), U1.into()]
        }
        Some(Core::And8) | Some(Core::Or8) | Some(Core::Xor8) | Some(Core::Eq8) => {
            vec![U8.into(), U8.into()]
        }
        Some(Core::And16) | Some(Core::Or16) | Some(Core::Xor16) | Some(Core::Eq16) => {
            vec![U16.into(), U16.into()]
        }
        Some(Core::And32) | Some(Core::Or32) | Some(Core::Xor32) | Some(Core::Eq32) => {
            vec![U32.into(), U32.into()]
        }
        Some(Core::And64) | Some(Core::Or64) | Some(Core::Xor64) | Some(Core::Eq64) => {
            vec![U64.into(), U64.into()]
        }
        Some(Core::Eq256) => vec![U256.into(), U256.into()],
        Some(Core::Maj1) | Some(Core::XorXor1) | Some(Core::Ch1) => {
            vec![U1.into(), U1.into(), U1.into()]
        }
        Some(Core::Maj8) | Some(Core::XorXor8) | Some(Core::Ch8) => {
            vec![U8.into(), U8.into(), U8.into()]
        }
        Some(Core::Maj16) | Some(Core::XorXor16) | Some(Core::Ch16) => {
            vec![U16.into(), tuple([U16, U16])]
        }
        Some(Core::Maj32) | Some(Core::XorXor32) | Some(Core::Ch32) => {
            vec![U32.into(), tuple([U32, U32])]
        }
        Some(Core::Maj64) | Some(Core::XorXor64) | Some(Core::Ch64) => {
            vec![U64.into(), tuple([U64, U64])]
        }
        Some(Core::FullLeftShift8_1) => vec![U8.into(), U1.into()],
        Some(Core::FullLeftShift8_2) => vec![U8.into(), U2.into()],
        Some(Core::FullLeftShift8_4) => vec![U8.into(), U4.into()],
        Some(Core::FullLeftShift16_1) => vec![U16.into(), U1.into()],
        Some(Core::FullLeftShift16_2) => vec![U16.into(), U2.into()],
        Some(Core::FullLeftShift16_4) => vec![U16.into(), U4.into()],
        Some(Core::FullLeftShift16_8) => vec![U16.into(), U8.into()],
        Some(Core::FullLeftShift32_1) => vec![U32.into(), U1.into()],
        Some(Core::FullLeftShift32_2) => vec![U32.into(), U2.into()],
        Some(Core::FullLeftShift32_4) => vec![U32.into(), U4.into()],
        Some(Core::FullLeftShift32_8) => vec![U32.into(), U8.into()],
        Some(Core::FullLeftShift32_16) => vec![U32.into(), U16.into()],
        Some(Core::FullLeftShift64_1) => vec![U64.into(), U1.into()],
        Some(Core::FullLeftShift64_2) => vec![U64.into(), U2.into()],
        Some(Core::FullLeftShift64_4) => vec![U64.into(), U4.into()],
        Some(Core::FullLeftShift64_8) => vec![U64.into(), U8.into()],
        Some(Core::FullLeftShift64_16) => vec![U64.into(), U16.into()],
        Some(Core::FullLeftShift64_32) => vec![U64.into(), U32.into()],
        Some(Core::FullRightShift8_1) => vec![U1.into(), U8.into()],
        Some(Core::FullRightShift8_2) => vec![U2.into(), U8.into()],
        Some(Core::FullRightShift8_4) => vec![U4.into(), U8.into()],
        Some(Core::FullRightShift16_1) => vec![U1.into(), U16.into()],
        Some(Core::FullRightShift16_2) => vec![U2.into(), U16.into()],
        Some(Core::FullRightShift16_4) => vec![U4.into(), U16.into()],
        Some(Core::FullRightShift16_8) => vec![U8.into(), U16.into()],
        Some(Core::FullRightShift32_1) => vec![U1.into(), U32.into()],
        Some(Core::FullRightShift32_2) => vec![U2.into(), U32.into()],
        Some(Core::FullRightShift32_4) => vec![U4.into(), U32.into()],
        Some(Core::FullRightShift32_8) => vec![U8.into(), U32.into()],
        Some(Core::FullRightShift32_16) => vec![U16.into(), U32.into()],
        Some(Core::FullRightShift64_1) => vec![U1.into(), U64.into()],
        Some(Core::FullRightShift64_2) => vec![U2.into(), U64.into()],
        Some(Core::FullRightShift64_4) => vec![U4.into(), U64.into()],
        Some(Core::FullRightShift64_8) => vec![U8.into(), U64.into()],
        Some(Core::FullRightShift64_16) => vec![U16.into(), U64.into()],
        Some(Core::FullRightShift64_32) => vec![U32.into(), U64.into()],
        Some(Core::LeftShiftWith8) | Some(Core::RightShiftWith8) => {
            vec![U1.into(), U4.into(), U8.into()]
        }
        Some(Core::LeftShiftWith16) | Some(Core::RightShiftWith16) => {
            vec![U1.into(), U4.into(), U16.into()]
        }
        Some(Core::LeftShiftWith32) | Some(Core::RightShiftWith32) => {
            vec![U1.into(), U8.into(), U32.into()]
        }
        Some(Core::LeftShiftWith64) | Some(Core::RightShiftWith64) => {
            vec![U1.into(), U8.into(), U64.into()]
        }
        Some(Core::LeftShift8)
        | Some(Core::RightShift8)
        | Some(Core::LeftRotate8)
        | Some(Core::RightRotate8) => vec![U4.into(), U8.into()],
        Some(Core::LeftShift16)
        | Some(Core::RightShift16)
        | Some(Core::LeftRotate16)
        | Some(Core::RightRotate16) => vec![U4.into(), U16.into()],
        Some(Core::LeftShift32)
        | Some(Core::RightShift32)
        | Some(Core::LeftRotate32)
        | Some(Core::RightRotate32) => vec![U8.into(), U32.into()],
        Some(Core::LeftShift64)
        | Some(Core::RightShift64)
        | Some(Core::LeftRotate64)
        | Some(Core::RightRotate64) => vec![U8.into(), U64.into()],
        /*
         * Arithmetic
         */
        Some(Core::One8) | Some(Core::One16) | Some(Core::One32) | Some(Core::One64) => vec![],
        Some(Core::Increment8)
        | Some(Core::Negate8)
        | Some(Core::Decrement8)
        | Some(Core::IsZero8)
        | Some(Core::IsOne8) => vec![U8.into()],
        Some(Core::Increment16)
        | Some(Core::Negate16)
        | Some(Core::Decrement16)
        | Some(Core::IsZero16)
        | Some(Core::IsOne16) => vec![U16.into()],
        Some(Core::Increment32)
        | Some(Core::Negate32)
        | Some(Core::Decrement32)
        | Some(Core::IsZero32)
        | Some(Core::IsOne32) => vec![U32.into()],
        Some(Core::Increment64)
        | Some(Core::Negate64)
        | Some(Core::Decrement64)
        | Some(Core::IsZero64)
        | Some(Core::IsOne64) => vec![U64.into()],
        Some(Core::Add8)
        | Some(Core::Subtract8)
        | Some(Core::Multiply8)
        | Some(Core::Le8)
        | Some(Core::Lt8)
        | Some(Core::Min8)
        | Some(Core::Max8)
        | Some(Core::DivMod8)
        | Some(Core::Divide8)
        | Some(Core::Modulo8)
        | Some(Core::Divides8) => vec![U8.into(), U8.into()],
        Some(Core::Add16)
        | Some(Core::Subtract16)
        | Some(Core::Multiply16)
        | Some(Core::Le16)
        | Some(Core::Lt16)
        | Some(Core::Min16)
        | Some(Core::Max16)
        | Some(Core::DivMod16)
        | Some(Core::Divide16)
        | Some(Core::Modulo16)
        | Some(Core::Divides16) => vec![U16.into(), U16.into()],
        Some(Core::Add32)
        | Some(Core::Subtract32)
        | Some(Core::Multiply32)
        | Some(Core::Le32)
        | Some(Core::Lt32)
        | Some(Core::Min32)
        | Some(Core::Max32)
        | Some(Core::DivMod32)
        | Some(Core::Divide32)
        | Some(Core::Modulo32)
        | Some(Core::Divides32) => vec![U32.into(), U32.into()],
        Some(Core::Add64)
        | Some(Core::Subtract64)
        | Some(Core::Multiply64)
        | Some(Core::Le64)
        | Some(Core::Lt64)
        | Some(Core::Min64)
        | Some(Core::Max64)
        | Some(Core::DivMod64)
        | Some(Core::Divide64)
        | Some(Core::Modulo64)
        | Some(Core::Divides64) => vec![U64.into(), U64.into()],
        Some(Core::DivMod128_64) => vec![U128.into(), U64.into()],
        Some(Core::FullAdd8) | Some(Core::FullSubtract8) => {
            vec![bool(), U8.into(), U8.into()]
        }
        Some(Core::FullAdd16) | Some(Core::FullSubtract16) => {
            vec![bool(), U16.into(), U16.into()]
        }
        Some(Core::FullAdd32) | Some(Core::FullSubtract32) => {
            vec![bool(), U32.into(), U32.into()]
        }
        Some(Core::FullAdd64) | Some(Core::FullSubtract64) => {
            vec![bool(), U64.into(), U64.into()]
        }
        Some(Core::FullIncrement8) | Some(Core::FullDecrement8) => {
            vec![bool(), U8.into()]
        }
        Some(Core::FullIncrement16) | Some(Core::FullDecrement16) => {
            vec![bool(), U16.into()]
        }
        Some(Core::FullIncrement32) | Some(Core::FullDecrement32) => {
            vec![bool(), U32.into()]
        }
        Some(Core::FullIncrement64) | Some(Core::FullDecrement64) => {
            vec![bool(), U64.into()]
        }
        Some(Core::FullMultiply8) => {
            vec![tuple([U8, U8]), tuple([U8, U8])]
        }
        Some(Core::FullMultiply16) => {
            vec![tuple([U16, U16]), tuple([U16, U16])]
        }
        Some(Core::FullMultiply32) => {
            vec![tuple([U32, U32]), tuple([U32, U32])]
        }
        Some(Core::FullMultiply64) => {
            vec![tuple([U64, U64]), tuple([U64, U64])]
        }
        Some(Core::Median8) => vec![U8.into(), U8.into(), U8.into()],
        Some(Core::Median16) => vec![U16.into(), U16.into(), U16.into()],
        Some(Core::Median32) => vec![U32.into(), U32.into(), U32.into()],
        Some(Core::Median64) => vec![U64.into(), U64.into(), U64.into()],
        /*
         * Hash functions
         */
        Some(Core::Sha256Iv) | Some(Core::Sha256Ctx8Init) => vec![],
        Some(Core::Sha256Block) => {
            vec![U256.into(), U256.into(), U256.into()]
        }
        Some(Core::Sha256Ctx8Add1) => vec![Ctx8.into(), U8.into()],
        Some(Core::Sha256Ctx8Add2) => vec![Ctx8.into(), U16.into()],
        Some(Core::Sha256Ctx8Add4) => vec![Ctx8.into(), U32.into()],
        Some(Core::Sha256Ctx8Add8) => vec![Ctx8.into(), U64.into()],
        Some(Core::Sha256Ctx8Add16) => vec![Ctx8.into(), U128.into()],
        Some(Core::Sha256Ctx8Add32) => vec![Ctx8.into(), U256.into()],
        Some(Core::Sha256Ctx8Add64) => vec![Ctx8.into(), array(U8, 64)],
        Some(Core::Sha256Ctx8Add128) => {
            vec![Ctx8.into(), array(U8, 128)]
        }
        Some(Core::Sha256Ctx8Add256) => {
            vec![Ctx8.into(), array(U8, 256)]
        }
        Some(Core::Sha256Ctx8Add512) => {
            vec![Ctx8.into(), array(U8, 512)]
        }
        Some(Core::Sha256Ctx8AddBuffer511) => {
            vec![Ctx8.into(), list(U8, 512)]
        }
        Some(Core::Sha256Ctx8Finalize) => vec![Ctx8.into()],
        /*
         * Elliptic curve functions
         */
        // XXX: Nonstandard tuple
        Some(Core::PointVerify1) => {
            vec![tuple([tuple([Scalar, Point]), Scalar.into()]), Point.into()]
        }
        Some(Core::Decompress) => vec![Point.into()],
        // XXX: Nonstandard tuple
        Some(Core::LinearVerify1) => {
            vec![tuple([tuple([Scalar, Ge]), Scalar.into()]), Ge.into()]
        }
        // XXX: Nonstandard tuple
        Some(Core::LinearCombination1) => {
            vec![tuple([Scalar, Gej]), Scalar.into()]
        }
        Some(Core::Scale) => vec![Scalar.into(), Gej.into()],
        Some(Core::Generate) => vec![Scalar.into()],
        Some(Core::GejInfinity) => vec![],
        Some(Core::GejNormalize)
        | Some(Core::GejNegate)
        | Some(Core::GejDouble)
        | Some(Core::GejIsInfinity)
        | Some(Core::GejYIsOdd)
        | Some(Core::GejIsOnCurve) => vec![Gej.into()],
        Some(Core::GeNegate) | Some(Core::GeIsOnCurve) => {
            vec![Ge.into()]
        }
        Some(Core::GejAdd) | Some(Core::GejEquiv) => {
            vec![Gej.into(), Gej.into()]
        }
        Some(Core::GejGeAddEx) | Some(Core::GejGeAdd) | Some(Core::GejGeEquiv) => {
            vec![Gej.into(), Ge.into()]
        }
        Some(Core::GejRescale) => vec![Gej.into(), Fe.into()],
        Some(Core::GejXEquiv) => vec![Fe.into(), Gej.into()],
        Some(Core::ScalarAdd) | Some(Core::ScalarMultiply) => {
            vec![Scalar.into(), Scalar.into()]
        }
        Some(Core::ScalarNormalize)
        | Some(Core::ScalarNegate)
        | Some(Core::ScalarSquare)
        | Some(Core::ScalarInvert)
        | Some(Core::ScalarMultiplyLambda)
        | Some(Core::ScalarIsZero) => vec![Scalar.into()],
        Some(Core::FeNormalize)
        | Some(Core::FeNegate)
        | Some(Core::FeSquare)
        | Some(Core::FeMultiplyBeta)
        | Some(Core::FeInvert)
        | Some(Core::FeSquareRoot)
        | Some(Core::FeIsZero)
        | Some(Core::FeIsOdd)
        | Some(Core::Swu) => vec![Fe.into()],
        Some(Core::FeAdd) | Some(Core::FeMultiply) => {
            vec![Fe.into(), Fe.into()]
        }
        Some(Core::HashToCurve) => vec![U256.into()],
        /*
         * Digital signatures
         */
        // XXX: Nonstandard tuple
        Some(Core::CheckSigVerify) => {
            vec![tuple([Pubkey, Message64]), Signature.into()]
        }
        // XXX: Nonstandard tuple
        Some(Core::Bip0340Verify) => {
            vec![tuple([Pubkey, Message]), Signature.into()]
        }
        /*
         * Bitcoin (without primitives)
         */
        Some(Core::TapdataInit) => vec![],
        Some(Core::ParseLock) | Some(Core::ParseSequence) => {
            vec![U32.into()]
        }
        /*
         * Script operations
         */
        None => vec![U8.into()],
    }
}

fn target_type_core(jet: CustomJet<()>) -> AliasedType {
    let try_base_jet: Option<Core> = unsafe { jet.to_base_jet() };

    match try_base_jet {
        /*
         * ==============================
         *          Core jets
         * ==============================
         *
         * Multi-bit logic
         */
        Some(Core::Verify) => AliasedType::unit(),
        Some(Core::Some1) | Some(Core::Some8) | Some(Core::Some16) | Some(Core::Some32)
        | Some(Core::Some64) | Some(Core::All8) | Some(Core::All16) | Some(Core::All32)
        | Some(Core::All64) | Some(Core::Eq1) | Some(Core::Eq8) | Some(Core::Eq16)
        | Some(Core::Eq32) | Some(Core::Eq64) | Some(Core::Eq256) => bool(),
        Some(Core::Low1)
        | Some(Core::High1)
        | Some(Core::Complement1)
        | Some(Core::And1)
        | Some(Core::Or1)
        | Some(Core::Xor1)
        | Some(Core::Maj1)
        | Some(Core::XorXor1)
        | Some(Core::Ch1)
        | Some(Core::Leftmost8_1)
        | Some(Core::Rightmost8_1)
        | Some(Core::Leftmost16_1)
        | Some(Core::Rightmost16_1)
        | Some(Core::Leftmost32_1)
        | Some(Core::Rightmost32_1)
        | Some(Core::Leftmost64_1)
        | Some(Core::Rightmost64_1) => U1.into(),
        Some(Core::Leftmost8_2)
        | Some(Core::Rightmost8_2)
        | Some(Core::Leftmost16_2)
        | Some(Core::Rightmost16_2)
        | Some(Core::Leftmost32_2)
        | Some(Core::Rightmost32_2)
        | Some(Core::Leftmost64_2)
        | Some(Core::Rightmost64_2) => U2.into(),
        Some(Core::Leftmost8_4)
        | Some(Core::Rightmost8_4)
        | Some(Core::Leftmost16_4)
        | Some(Core::Rightmost16_4)
        | Some(Core::Leftmost32_4)
        | Some(Core::Rightmost32_4)
        | Some(Core::Leftmost64_4)
        | Some(Core::Rightmost64_4) => U4.into(),
        Some(Core::Low8)
        | Some(Core::High8)
        | Some(Core::Complement8)
        | Some(Core::And8)
        | Some(Core::Or8)
        | Some(Core::Xor8)
        | Some(Core::Maj8)
        | Some(Core::XorXor8)
        | Some(Core::Ch8)
        | Some(Core::Leftmost16_8)
        | Some(Core::Rightmost16_8)
        | Some(Core::Leftmost32_8)
        | Some(Core::Rightmost32_8)
        | Some(Core::Leftmost64_8)
        | Some(Core::Rightmost64_8)
        | Some(Core::LeftPadLow1_8)
        | Some(Core::LeftPadHigh1_8)
        | Some(Core::LeftExtend1_8)
        | Some(Core::RightPadLow1_8)
        | Some(Core::RightPadHigh1_8)
        | Some(Core::LeftShiftWith8)
        | Some(Core::RightShiftWith8)
        | Some(Core::LeftShift8)
        | Some(Core::RightShift8)
        | Some(Core::LeftRotate8)
        | Some(Core::RightRotate8) => U8.into(),
        Some(Core::Low16)
        | Some(Core::High16)
        | Some(Core::Complement16)
        | Some(Core::And16)
        | Some(Core::Or16)
        | Some(Core::Xor16)
        | Some(Core::Maj16)
        | Some(Core::XorXor16)
        | Some(Core::Ch16)
        | Some(Core::Leftmost32_16)
        | Some(Core::Rightmost32_16)
        | Some(Core::Leftmost64_16)
        | Some(Core::Rightmost64_16)
        | Some(Core::LeftPadLow1_16)
        | Some(Core::LeftPadHigh1_16)
        | Some(Core::LeftExtend1_16)
        | Some(Core::RightPadLow1_16)
        | Some(Core::RightPadHigh1_16)
        | Some(Core::LeftPadLow8_16)
        | Some(Core::LeftPadHigh8_16)
        | Some(Core::LeftExtend8_16)
        | Some(Core::RightPadLow8_16)
        | Some(Core::RightPadHigh8_16)
        | Some(Core::RightExtend8_16)
        | Some(Core::LeftShiftWith16)
        | Some(Core::RightShiftWith16)
        | Some(Core::LeftShift16)
        | Some(Core::RightShift16)
        | Some(Core::LeftRotate16)
        | Some(Core::RightRotate16) => U16.into(),
        Some(Core::Low32)
        | Some(Core::High32)
        | Some(Core::Complement32)
        | Some(Core::And32)
        | Some(Core::Or32)
        | Some(Core::Xor32)
        | Some(Core::Maj32)
        | Some(Core::XorXor32)
        | Some(Core::Ch32)
        | Some(Core::Leftmost64_32)
        | Some(Core::Rightmost64_32)
        | Some(Core::LeftPadLow1_32)
        | Some(Core::LeftPadHigh1_32)
        | Some(Core::LeftExtend1_32)
        | Some(Core::RightPadLow1_32)
        | Some(Core::RightPadHigh1_32)
        | Some(Core::LeftPadLow8_32)
        | Some(Core::LeftPadHigh8_32)
        | Some(Core::LeftExtend8_32)
        | Some(Core::RightPadLow8_32)
        | Some(Core::RightPadHigh8_32)
        | Some(Core::RightExtend8_32)
        | Some(Core::LeftPadLow16_32)
        | Some(Core::LeftPadHigh16_32)
        | Some(Core::LeftExtend16_32)
        | Some(Core::RightPadLow16_32)
        | Some(Core::RightPadHigh16_32)
        | Some(Core::RightExtend16_32)
        | Some(Core::LeftShiftWith32)
        | Some(Core::RightShiftWith32)
        | Some(Core::LeftShift32)
        | Some(Core::RightShift32)
        | Some(Core::LeftRotate32)
        | Some(Core::RightRotate32) => U32.into(),
        Some(Core::Low64)
        | Some(Core::High64)
        | Some(Core::Complement64)
        | Some(Core::And64)
        | Some(Core::Or64)
        | Some(Core::Xor64)
        | Some(Core::Maj64)
        | Some(Core::XorXor64)
        | Some(Core::Ch64)
        | Some(Core::LeftPadLow1_64)
        | Some(Core::LeftPadHigh1_64)
        | Some(Core::LeftExtend1_64)
        | Some(Core::RightPadLow1_64)
        | Some(Core::RightPadHigh1_64)
        | Some(Core::LeftPadLow8_64)
        | Some(Core::LeftPadHigh8_64)
        | Some(Core::LeftExtend8_64)
        | Some(Core::RightPadLow8_64)
        | Some(Core::RightPadHigh8_64)
        | Some(Core::RightExtend8_64)
        | Some(Core::LeftPadLow16_64)
        | Some(Core::LeftPadHigh16_64)
        | Some(Core::LeftExtend16_64)
        | Some(Core::RightPadLow16_64)
        | Some(Core::RightPadHigh16_64)
        | Some(Core::RightExtend16_64)
        | Some(Core::LeftPadLow32_64)
        | Some(Core::LeftPadHigh32_64)
        | Some(Core::LeftExtend32_64)
        | Some(Core::RightPadLow32_64)
        | Some(Core::RightPadHigh32_64)
        | Some(Core::RightExtend32_64)
        | Some(Core::LeftShiftWith64)
        | Some(Core::RightShiftWith64)
        | Some(Core::LeftShift64)
        | Some(Core::RightShift64)
        | Some(Core::LeftRotate64)
        | Some(Core::RightRotate64) => U64.into(),
        Some(Core::FullLeftShift8_1) => tuple([U1, U8]),
        Some(Core::FullLeftShift8_2) => tuple([U2, U8]),
        Some(Core::FullLeftShift8_4) => tuple([U4, U8]),
        Some(Core::FullLeftShift16_1) => tuple([U1, U16]),
        Some(Core::FullLeftShift16_2) => tuple([U2, U16]),
        Some(Core::FullLeftShift16_4) => tuple([U4, U16]),
        Some(Core::FullLeftShift16_8) => tuple([U8, U16]),
        Some(Core::FullLeftShift32_1) => tuple([U1, U32]),
        Some(Core::FullLeftShift32_2) => tuple([U2, U32]),
        Some(Core::FullLeftShift32_4) => tuple([U4, U32]),
        Some(Core::FullLeftShift32_8) => tuple([U8, U32]),
        Some(Core::FullLeftShift32_16) => tuple([U16, U32]),
        Some(Core::FullLeftShift64_1) => tuple([U1, U64]),
        Some(Core::FullLeftShift64_2) => tuple([U2, U64]),
        Some(Core::FullLeftShift64_4) => tuple([U4, U64]),
        Some(Core::FullLeftShift64_8) => tuple([U8, U64]),
        Some(Core::FullLeftShift64_16) => tuple([U16, U64]),
        Some(Core::FullLeftShift64_32) => tuple([U32, U64]),
        Some(Core::FullRightShift8_1) => tuple([U8, U1]),
        Some(Core::FullRightShift8_2) => tuple([U8, U2]),
        Some(Core::FullRightShift8_4) => tuple([U8, U4]),
        Some(Core::FullRightShift16_1) => tuple([U16, U1]),
        Some(Core::FullRightShift16_2) => tuple([U16, U2]),
        Some(Core::FullRightShift16_4) => tuple([U16, U4]),
        Some(Core::FullRightShift16_8) => tuple([U16, U8]),
        Some(Core::FullRightShift32_1) => tuple([U32, U1]),
        Some(Core::FullRightShift32_2) => tuple([U32, U2]),
        Some(Core::FullRightShift32_4) => tuple([U32, U4]),
        Some(Core::FullRightShift32_8) => tuple([U32, U8]),
        Some(Core::FullRightShift32_16) => tuple([U32, U16]),
        Some(Core::FullRightShift64_1) => tuple([U64, U1]),
        Some(Core::FullRightShift64_2) => tuple([U64, U2]),
        Some(Core::FullRightShift64_4) => tuple([U64, U4]),
        Some(Core::FullRightShift64_8) => tuple([U64, U8]),
        Some(Core::FullRightShift64_16) => tuple([U64, U16]),
        Some(Core::FullRightShift64_32) => tuple([U64, U32]),
        /*
         * Arithmetic
         */
        Some(Core::Le8)
        | Some(Core::Lt8)
        | Some(Core::Le16)
        | Some(Core::Lt16)
        | Some(Core::Le32)
        | Some(Core::Lt32)
        | Some(Core::Le64)
        | Some(Core::Lt64)
        | Some(Core::IsZero8)
        | Some(Core::IsOne8)
        | Some(Core::IsZero16)
        | Some(Core::IsOne16)
        | Some(Core::IsZero32)
        | Some(Core::IsOne32)
        | Some(Core::IsZero64)
        | Some(Core::IsOne64)
        | Some(Core::Divides8)
        | Some(Core::Divides16)
        | Some(Core::Divides32)
        | Some(Core::Divides64) => bool(),
        Some(Core::One8) | Some(Core::Min8) | Some(Core::Max8) | Some(Core::Divide8)
        | Some(Core::Modulo8) | Some(Core::Median8) => U8.into(),
        Some(Core::One16)
        | Some(Core::Min16)
        | Some(Core::Max16)
        | Some(Core::Divide16)
        | Some(Core::Modulo16)
        | Some(Core::Multiply8)
        | Some(Core::FullMultiply8)
        | Some(Core::Median16) => U16.into(),
        Some(Core::One32)
        | Some(Core::Min32)
        | Some(Core::Max32)
        | Some(Core::Divide32)
        | Some(Core::Modulo32)
        | Some(Core::Multiply16)
        | Some(Core::FullMultiply16)
        | Some(Core::Median32) => U32.into(),
        Some(Core::One64)
        | Some(Core::Min64)
        | Some(Core::Max64)
        | Some(Core::Divide64)
        | Some(Core::Modulo64)
        | Some(Core::Multiply32)
        | Some(Core::FullMultiply32)
        | Some(Core::Median64) => U64.into(),
        Some(Core::Multiply64) | Some(Core::FullMultiply64) => U128.into(),
        Some(Core::Increment8)
        | Some(Core::Negate8)
        | Some(Core::Decrement8)
        | Some(Core::Add8)
        | Some(Core::Subtract8)
        | Some(Core::FullAdd8)
        | Some(Core::FullSubtract8)
        | Some(Core::FullIncrement8)
        | Some(Core::FullDecrement8) => tuple([bool(), U8.into()]),
        Some(Core::Increment16)
        | Some(Core::Negate16)
        | Some(Core::Decrement16)
        | Some(Core::Add16)
        | Some(Core::Subtract16)
        | Some(Core::FullAdd16)
        | Some(Core::FullSubtract16)
        | Some(Core::FullIncrement16)
        | Some(Core::FullDecrement16) => tuple([bool(), U16.into()]),
        Some(Core::Increment32)
        | Some(Core::Negate32)
        | Some(Core::Decrement32)
        | Some(Core::Add32)
        | Some(Core::Subtract32)
        | Some(Core::FullAdd32)
        | Some(Core::FullSubtract32)
        | Some(Core::FullIncrement32)
        | Some(Core::FullDecrement32) => tuple([bool(), U32.into()]),
        Some(Core::Increment64)
        | Some(Core::Negate64)
        | Some(Core::Decrement64)
        | Some(Core::Add64)
        | Some(Core::Subtract64)
        | Some(Core::FullAdd64)
        | Some(Core::FullSubtract64)
        | Some(Core::FullIncrement64)
        | Some(Core::FullDecrement64) => tuple([bool(), U64.into()]),
        Some(Core::DivMod8) => tuple([U8, U8]),
        Some(Core::DivMod16) => tuple([U16, U16]),
        Some(Core::DivMod32) => tuple([U32, U32]),
        Some(Core::DivMod64) => tuple([U64, U64]),
        Some(Core::DivMod128_64) => tuple([U64, U64]),
        /*
         * Hash functions
         */
        Some(Core::Sha256Iv) | Some(Core::Sha256Block) | Some(Core::Sha256Ctx8Finalize) => {
            U256.into()
        }
        Some(Core::Sha256Ctx8Init)
        | Some(Core::Sha256Ctx8Add1)
        | Some(Core::Sha256Ctx8Add2)
        | Some(Core::Sha256Ctx8Add4)
        | Some(Core::Sha256Ctx8Add8)
        | Some(Core::Sha256Ctx8Add16)
        | Some(Core::Sha256Ctx8Add32)
        | Some(Core::Sha256Ctx8Add64)
        | Some(Core::Sha256Ctx8Add128)
        | Some(Core::Sha256Ctx8Add256)
        | Some(Core::Sha256Ctx8Add512)
        | Some(Core::Sha256Ctx8AddBuffer511) => Ctx8.into(),
        /*
         * Elliptic curve functions
         */
        Some(Core::PointVerify1) | Some(Core::LinearVerify1) => AliasedType::unit(),
        Some(Core::GejIsInfinity)
        | Some(Core::GejEquiv)
        | Some(Core::GejGeEquiv)
        | Some(Core::GejXEquiv)
        | Some(Core::GejYIsOdd)
        | Some(Core::GejIsOnCurve)
        | Some(Core::GeIsOnCurve)
        | Some(Core::ScalarIsZero)
        | Some(Core::FeIsZero)
        | Some(Core::FeIsOdd) => bool(),
        Some(Core::GeNegate) | Some(Core::HashToCurve) | Some(Core::Swu) => Ge.into(),
        Some(Core::Decompress) | Some(Core::GejNormalize) => option(Ge),
        Some(Core::LinearCombination1)
        | Some(Core::Scale)
        | Some(Core::Generate)
        | Some(Core::GejInfinity)
        | Some(Core::GejNegate)
        | Some(Core::GejDouble)
        | Some(Core::GejAdd)
        | Some(Core::GejGeAdd)
        | Some(Core::GejRescale) => Gej.into(),
        Some(Core::GejGeAddEx) => tuple([Fe, Gej]),
        Some(Core::ScalarNormalize)
        | Some(Core::ScalarNegate)
        | Some(Core::ScalarAdd)
        | Some(Core::ScalarSquare)
        | Some(Core::ScalarMultiply)
        | Some(Core::ScalarMultiplyLambda)
        | Some(Core::ScalarInvert) => Scalar.into(),
        Some(Core::FeNormalize)
        | Some(Core::FeNegate)
        | Some(Core::FeAdd)
        | Some(Core::FeSquare)
        | Some(Core::FeMultiply)
        | Some(Core::FeMultiplyBeta)
        | Some(Core::FeInvert) => Fe.into(),
        Some(Core::FeSquareRoot) => option(Fe),
        /*
         * Digital signatures
         */
        Some(Core::CheckSigVerify) | Some(Core::Bip0340Verify) => AliasedType::unit(),
        /*
         * Bitcoin (without primitives)
         */
        Some(Core::ParseLock) => either(Height, Time),
        Some(Core::ParseSequence) => option(either(Distance, Duration)),
        Some(Core::TapdataInit) => Ctx8.into(),
        /*
         * Script operations
         */
        None => {
            let (get_opcode, get_pubkey) = (
                CustomJet::from_str("get_opcode_from_script").unwrap(),
                CustomJet::from_str("get_pubkey_from_script").unwrap(),
            );

            if jet == get_opcode {
                U8.into()
            } else if jet == get_pubkey {
                Pubkey.into()
            } else {
                unreachable!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simplicity::jet::Jet;

    #[test]
    fn compatible_source_type() {
        for jet in CustomJet::all() {
            let resolved_ty = ResolvedType::tuple(
                source_type_elements(*jet)
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
        for jet in CustomJet::all() {
            let resolved_ty = target_type_elements(*jet).resolve_builtin().unwrap();
            let structural_ty = StructuralType::from(&resolved_ty);
            let simplicity_ty = jet.target_ty().to_final();

            println!("{jet}");
            assert_eq!(structural_ty.as_ref(), simplicity_ty.as_ref());
        }
    }
}
