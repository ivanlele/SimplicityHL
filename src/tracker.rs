use simplicity_unchained::jets::bitcoin::CoreExtension;
use simplicity_unchained::jets::elements::ElementsExtension;

use simplicity::bit_machine::{ExecTracker, FrameIter, NodeOutput, PruneTracker, SetTracker};
use simplicity::jet::Jet;
use simplicity::node::Inner;
use simplicity::{Ihr, RedeemNode, Value as SimValue, ValueRef};

use crate::debug::{DebugSymbols, TrackedCallName};
use crate::either::Either;
use crate::jet::{
    source_type_core_extension, source_type_elements_extension, target_type_core_extension,
    target_type_elements_extension,
};
use crate::str::AliasName;
use crate::types::AliasedType;
use crate::value::StructuralValue;
use crate::{ResolvedType, Value};

/// Callback signature for receiving debug output.
///
/// The first argument is the label (variable name or expression), and the second
/// is the formatted value.
type DebugSink<'a, J> = Box<dyn FnMut(&str, &Value<J>) + 'a>;

/// Callback signature for receiving jet execution traces.
///
/// Arguments are: the jet that was executed, its input arguments (if successfully parsed),
/// and the result (`None` if the jet failed).
type JetTraceSink<'a, J> = Box<dyn FnMut(J, Option<&[Value<J>]>, Option<Value<J>>) + 'a>;

/// Callback signature for receiving warnings during execution.
type WarningSink<'a> = Box<dyn Fn(&str) + 'a>;

/// Controls the verbosity of program execution logging.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum TrackerLogLevel {
    #[default]
    None,
    Debug,
    Warning,
    Trace,
}

/// Default debug sink that prints labeled values to stderr.
fn default_debug_sink<J: Jet>(label: &str, value: &Value<J>) {
    println!("DBG: {label} = {value}");
}

/// Default jet trace sink that prints jet calls to stderr.
fn default_jet_trace_sink<J: Jet>(jet: J, args: Option<&[Value<J>]>, result: Option<Value<J>>) {
    print!("{jet:?}(");
    if let Some(args) = args {
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{arg}");
        }
    } else {
        print!("...");
    }

    match result {
        Some(value) => println!(") = {value}"),
        None => println!(") -> [failed]"),
    }
}

/// Default warning sink that prints warnings to stderr.
fn default_warning_sink(message: &str) {
    println!("WARN: {message}");
}

/// Tracker for introspecting SimplicityHL program execution.
///
/// This tracker extends [`SetTracker`] with SimplicityHL-specific functionality:
///
/// - Decodes and forwards `dbg!()` calls to a configurable sink, using
///   the provided [`DebugSymbols`] to resolve CMRs to debug information.
/// - Optionally traces jet invocations with decoded arguments and return values.
///
/// # Example
///
/// ```rust,ignore
/// let tracker = DefaultTracker::new(&debug_symbols)
///     .with_log_level(TrackerLogLevel::Debug);
///
/// let pruned = program.prune_with_tracker(&env, &mut tracker)?;
/// ```
pub struct DefaultTracker<'a, J: Jet> {
    debug_symbols: &'a DebugSymbols<J>,
    debug_sink: Option<DebugSink<'a, J>>,
    jet_trace_sink: Option<JetTraceSink<'a, J>>,
    warning_sink: Option<WarningSink<'a>>,
    inner: SetTracker,
}

impl<'a, J: Jet> DefaultTracker<'a, J> {
    /// Creates a new tracker bound to the given debug symbol table.
    pub fn new(debug_symbols: &'a DebugSymbols<J>) -> Self {
        Self {
            debug_symbols,
            debug_sink: None,
            jet_trace_sink: None,
            warning_sink: None,
            inner: SetTracker::default(),
        }
    }

    /// Enables forwarding of `debug!()` calls to the provided sink.
    pub fn with_debug_sink<F>(mut self, sink: F) -> Self
    where
        F: FnMut(&str, &Value<J>) + 'a,
    {
        self.debug_sink = Some(Box::new(sink));
        self
    }

    /// Enables the default debug sink that prints to stderr.
    pub fn with_default_debug_sink(self) -> Self {
        self.with_debug_sink(default_debug_sink)
    }

    /// Enables forwarding of jet call traces to the provided sink.
    pub fn with_jet_trace_sink<F>(mut self, sink: F) -> Self
    where
        F: FnMut(J, Option<&[Value<J>]>, Option<Value<J>>) + 'a,
    {
        self.jet_trace_sink = Some(Box::new(sink));
        self
    }

    /// Enables the default jet trace sink that prints to stderr.
    pub fn with_default_jet_trace_sink(self) -> Self {
        self.with_jet_trace_sink(default_jet_trace_sink)
    }

    /// Enables forwarding of warnings to the provided sink.
    pub fn with_warning_sink<F>(mut self, sink: F) -> Self
    where
        F: Fn(&str) + 'a,
    {
        self.warning_sink = Some(Box::new(sink));
        self
    }

    /// Enables the default warning sink that prints to stderr.
    pub fn with_default_warning_sink(self) -> Self {
        self.with_warning_sink(default_warning_sink)
    }

    /// Gets the source type for a jet based on the generic parameter J.
    fn source_type(jet: J) -> Vec<AliasedType> {
        if std::any::TypeId::of::<J>() == std::any::TypeId::of::<ElementsExtension>() {
            unsafe {
                let elements_jet = std::mem::transmute_copy::<J, ElementsExtension>(&jet);
                source_type_elements_extension(elements_jet)
            }
        } else if std::any::TypeId::of::<J>() == std::any::TypeId::of::<CoreExtension>() {
            unsafe {
                let core_jet = std::mem::transmute_copy::<J, CoreExtension>(&jet);
                source_type_core_extension(core_jet)
            }
        } else {
            panic!("Unsupported jet type for source_type")
        }
    }

    /// Gets the target type for a jet based on the generic parameter J.
    fn target_type(jet: J) -> AliasedType {
        if std::any::TypeId::of::<J>() == std::any::TypeId::of::<ElementsExtension>() {
            unsafe {
                let elements_jet = std::mem::transmute_copy::<J, ElementsExtension>(&jet);
                target_type_elements_extension(elements_jet)
            }
        } else if std::any::TypeId::of::<J>() == std::any::TypeId::of::<CoreExtension>() {
            unsafe {
                let core_jet = std::mem::transmute_copy::<J, CoreExtension>(&jet);
                target_type_core_extension(core_jet)
            }
        } else {
            panic!("Unsupported jet type for target_type")
        }
    }

    /// Configures the tracker based on the specified log level.
    ///
    /// - [`TrackerLogLevel::None`]: No sinks enabled.
    /// - [`TrackerLogLevel::Debug`]: Default debug sink enabled.
    /// - [`TrackerLogLevel::Warning`]: Default debug and warning sinks enabled.
    /// - [`TrackerLogLevel::Trace`]: Default debug, warning, and jet trace sinks enabled.
    pub fn with_log_level(self, log_level: TrackerLogLevel) -> Self {
        let tracker = if log_level >= TrackerLogLevel::Debug {
            self.with_default_debug_sink()
        } else {
            self
        };

        let tracker = if log_level >= TrackerLogLevel::Warning {
            tracker.with_default_warning_sink()
        } else {
            tracker
        };

        if log_level >= TrackerLogLevel::Trace {
            tracker.with_default_jet_trace_sink()
        } else {
            tracker
        }
    }

    /// Handles jet node execution by decoding arguments and results.
    fn handle_jet(&mut self, node: &RedeemNode<J>, jet: J, input: &FrameIter, output: &NodeOutput) {
        if self.jet_trace_sink.is_none() {
            return;
        }

        let mut input_frame = input.clone();

        // The reason we need to advance by a bit is that the AssertL combinator is actually a Case combinator,
        // which takes a bit of input to decide which branch to take. But this bit is "meaningless" and
        // is always 0 because it's an assertion.
        let _ = input_frame.next();

        let args = match Self::parse_jet_arguments(jet, &mut input_frame) {
            Ok(args) => args,
            Err(e) => {
                self.warn(&format!("Failed to parse arguments for jet {jet:?}: {e}"));

                // Still call the sink to report the jet execution, but without arguments.
                let result = Self::parse_jet_result(node, jet, output);
                if let Some(sink) = self.jet_trace_sink.as_mut() {
                    sink(jet, None, result);
                }

                return;
            }
        };

        let result = Self::parse_jet_result(node, jet, output);

        if let Some(sink) = self.jet_trace_sink.as_mut() {
            sink(jet, Some(&args), result);
        }
    }

    /// Parses the result of a jet execution from the output frame.
    fn parse_jet_result(node: &RedeemNode<J>, jet: J, output: &NodeOutput) -> Option<Value<J>> {
        match output.clone() {
            NodeOutput::Success(mut output_frame) => {
                let target_ty = &node.arrow().target;

                // Skip the leading bit when the frame has extra padding.
                // This occurs because jets are wrapped in AssertL (a Case combinator),
                // which adds structure to the output frame for some jets.
                if output_frame.len() > target_ty.bit_width() {
                    let _ = output_frame.next();
                }

                let output_value = SimValue::from_padded_bits(&mut output_frame, target_ty)
                    .expect("output from bit machine is always well-formed");

                let target_ty = resolve_jet_type(&Self::target_type(jet));
                Value::reconstruct(&StructuralValue::from(output_value), &target_ty)
            }
            _ => None,
        }
    }

    /// Sends a warning to the warning sink if configured.
    fn warn(&self, message: &str) {
        if let Some(sink) = self.warning_sink.as_ref() {
            sink(message);
        }
    }

    /// Handles debug node execution by resolving symbols and decoding values.
    fn handle_debug(&mut self, node: &RedeemNode<J>, input: &FrameIter, cmr: &simplicity::Cmr) {
        if self.debug_sink.is_none() {
            return;
        }

        let Some(tracked_call) = self.debug_symbols.get(cmr) else {
            self.warn(&format!("Unknown debug symbol: CMR {cmr}"));
            return;
        };

        let TrackedCallName::Debug(_) = tracked_call.name() else {
            return;
        };

        let mut input_frame = input.clone();

        // Skip the Case combinator's branch selection bit (see handle_jet).
        let _ = input_frame.next();

        // The debug call has signature `dbg!(T) -> T`, so the target type
        // matches the value being debugged
        let Ok(input_val) = SimValue::from_padded_bits(&mut input_frame, &node.arrow().target)
        else {
            self.warn(&format!("Failed to decode debug value for CMR {cmr}"));
            return;
        };

        let Some(Either::Right(debug_value)) =
            tracked_call.map_value(&StructuralValue::from(input_val))
        else {
            return;
        };

        if let Some(sink) = self.debug_sink.as_mut() {
            sink(debug_value.text(), debug_value.value());
        }
    }

    /// Parses jet input arguments from the bit machine's read frame.
    fn parse_jet_arguments(jet: J, input_frame: &mut FrameIter) -> Result<Vec<Value<J>>, String> {
        let source_types = Self::source_type(jet);
        if source_types.is_empty() {
            return Ok(vec![]);
        }

        let arguments_blob = SimValue::from_padded_bits(input_frame, &jet.source_ty().to_final())
            .expect("input from bit machine is always well-formed");

        let mut args = Vec::with_capacity(source_types.len());
        collect_product_elements(&arguments_blob.as_ref(), source_types.len(), &mut args)?;

        Ok(args
            .into_iter()
            .zip(source_types.iter())
            .map(|(arg, aliased_type)| {
                Value::reconstruct(&arg.into(), &resolve_jet_type(aliased_type))
                    .expect("compiled program produces correctly structured values")
            })
            .collect())
    }
}

impl<J: Jet> PruneTracker<J> for DefaultTracker<'_, J> {
    fn contains_left(&self, ihr: Ihr) -> bool {
        if PruneTracker::<ElementsExtension>::contains_left(&self.inner, ihr) {
            return true;
        }

        if let Some(sink) = self.warning_sink.as_ref() {
            sink(&format!("Pruning unexecuted left child of IHR {ihr}"));
        }

        false
    }

    fn contains_right(&self, ihr: Ihr) -> bool {
        if PruneTracker::<J>::contains_right(&self.inner, ihr) {
            return true;
        }

        if let Some(sink) = self.warning_sink.as_ref() {
            sink(&format!("Pruning unexecuted right child of IHR {ihr}"));
        }

        false
    }
}

impl<J: Jet> ExecTracker<J> for DefaultTracker<'_, J> {
    fn visit_node(&mut self, node: &RedeemNode<J>, input: FrameIter, output: NodeOutput) {
        match node.inner() {
            Inner::Jet(jet) => self.handle_jet(node, *jet, &input, &output),
            Inner::AssertL(_, cmr) => self.handle_debug(node, &input, cmr),
            _ => {}
        }

        self.inner.visit_node(node, input, output);
    }
}

/// Recursively collects elements from a nested product type.
///
/// Given a value of type `(A, (B, (C, ...)))`, extracts `[A, B, C, ...]`.
fn collect_product_elements(
    node: &ValueRef,
    count: usize,
    elements: &mut Vec<SimValue>,
) -> Result<(), String> {
    match count {
        0 => Ok(()),
        1 => {
            elements.push(node.to_value());
            Ok(())
        }
        _ => {
            let (left, right) = node
                .as_product()
                .ok_or("expected product type while collecting arguments")?;
            elements.push(left.to_value());
            collect_product_elements(&right, count - 1, elements)
        }
    }
}

/// Resolves an aliased type to its concrete form.
fn resolve_jet_type(aliased_type: &AliasedType) -> ResolvedType {
    aliased_type
        .resolve(|_: &AliasName| None)
        .expect("jet types always resolve without aliases")
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::sync::Arc;

    use simplicity::elements::hex::FromHex;
    use simplicity::elements::taproot::ControlBlock;
    use simplicity::elements::BlockHash;
    use simplicity::elements::{self, pset::PartiallySignedTransaction};
    use simplicity::jet::elements::{ElementsEnv, ElementsUtxo};
    use simplicity::Cmr;
    use simplicity_unchained::jets::environments::ElementsUnchainedEnv;

    use crate::elements::confidential::Asset;
    use crate::elements::hashes::Hash;
    use crate::elements::pset::Input;
    use crate::elements::{AssetId, OutPoint, Script, Txid};
    use crate::{Arguments, TemplateProgram, WitnessValues};

    use super::*;

    const TEST_PROGRAM: &str = r#"
        fn get_input_explicit_asset_amount(index: u32) -> (u256, u64) {
            let pair: (Asset1, Amount1) = unwrap(jet::input_amount(index));
            let (asset, amount): (Asset1, Amount1) = dbg!(pair);
            let asset_bits: u256 = unwrap_right::<(u1, u256)>(asset);
            let amount: u64 = unwrap_right::<(u1, u256)>(amount);
            (asset_bits, amount)
        }

        fn main() {
            let a: u32 = jet::num_inputs();
            let b: bool = dbg!(jet::eq_32(20, 21));
            let c: (u256, u64) = dbg!(get_input_explicit_asset_amount(0));
        }
    "#;

    type DebugStore = Rc<RefCell<HashMap<String, String>>>;
    type JetStore = Rc<RefCell<HashMap<String, (Option<Vec<String>>, Option<String>)>>>;

    fn create_test_tracker<J: Jet>(
        debug_symbols: &DebugSymbols<J>,
    ) -> (DefaultTracker<'_, J>, DebugStore, JetStore) {
        let debug_store: DebugStore = Rc::default();
        let jet_store: JetStore = Rc::default();

        let debug_clone = debug_store.clone();
        let jet_clone = jet_store.clone();

        let tracker = DefaultTracker::new(debug_symbols)
            .with_debug_sink(move |label, value| {
                debug_clone
                    .borrow_mut()
                    .insert(label.to_string(), value.to_string());
            })
            .with_jet_trace_sink(move |jet: J, args, result| {
                jet_clone.borrow_mut().insert(
                    jet.to_string(),
                    (
                        args.map(|a| a.iter().map(|v| v.to_string()).collect()),
                        result.map(|r| r.to_string()),
                    ),
                );
            });

        (tracker, debug_store, jet_store)
    }

    fn create_test_env() -> ElementsUnchainedEnv {
        let script = Script::from_hex(
            "5221033523982d58e94be3b735731593f8225043880d53727235b566c515d24a0f7baf21025eb4655feae15a304653e27441ca8e8ced2bef89c22ab6b20424b4c07b3d14cc52ae"
        ).unwrap();

        let mut tx = PartiallySignedTransaction::new_v2();
        let outpoint = OutPoint::new(Txid::from_slice(&[2; 32]).unwrap(), 33);
        tx.add_input(Input::from_prevout(outpoint));

        let elements_env = ElementsEnv::new(
            Arc::new(tx.extract_tx().unwrap()),
            vec![ElementsUtxo {
                script_pubkey: Script::new(),
                asset: Asset::Explicit(AssetId::LIQUID_BTC),
                value: elements::confidential::Value::Explicit(1000),
            }],
            0,
            Cmr::from_byte_array([0; 32]),
            ControlBlock::from_slice(&[0xc0; 33]).unwrap(),
            None,
            BlockHash::all_zeros(),
        );

        ElementsUnchainedEnv::new(script, elements_env)
    }

    #[test]
    fn test_debug_and_jet_tracing() {
        let program = TemplateProgram::<ElementsExtension>::new(TEST_PROGRAM).unwrap();
        let program = program.instantiate(Arguments::default(), true).unwrap();
        let satisfied = program.satisfy(WitnessValues::default()).unwrap();

        let (mut tracker, debug_store, jet_store) = create_test_tracker(&satisfied.debug_symbols);
        let env = create_test_env();

        let _ = satisfied
            .redeem()
            .prune_with_tracker(&env, &mut tracker)
            .unwrap();

        let debug = debug_store.borrow();
        assert_eq!(
            debug.get("get_input_explicit_asset_amount(0)"),
            Some(
                &"(0x6d521c38ec1ea15734ae22b7c46064412829c0d0579f0a713d1c04ede979026f, 1000)"
                    .to_string()
            ),
        );
        assert_eq!(
            debug.get("pair"),
            Some(
                &"(Right(0x6d521c38ec1ea15734ae22b7c46064412829c0d0579f0a713d1c04ede979026f), Right(1000))"
                    .to_string()
            ),
        );
        assert_eq!(debug.get("jet::eq_32(20, 21)"), Some(&"false".to_string()));

        let jets = jet_store.borrow();

        assert_eq!(
            jets.get("num_inputs").unwrap().0.as_deref(),
            Some([].as_slice())
        );
        assert_eq!(jets.get("num_inputs").unwrap().1.as_deref(), Some("1"));

        assert_eq!(
            jets.get("eq_32").unwrap().0,
            Some(vec!["20".to_string(), "21".to_string()])
        );
        assert_eq!(jets.get("eq_32").unwrap().1.as_deref(), Some("false"));

        assert_eq!(
            jets.get("input_amount").unwrap().0,
            Some(vec!["0".to_string()])
        );
        assert_eq!(
            jets.get("input_amount").unwrap().1.as_deref(),
            Some("Some((Right(0x6d521c38ec1ea15734ae22b7c46064412829c0d0579f0a713d1c04ede979026f), Right(1000)))")
        );
    }
}
