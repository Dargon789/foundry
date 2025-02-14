//! Anvil specific [`revm::Inspector`] implementation

use crate::revm::Database;
use alloy_primitives::{Address, Log};
use foundry_common::sh_println;
use foundry_evm::{
    call_inspectors,
    decode::decode_console_logs,
    inspectors::{LogCollector, TracingInspector},
    revm::{
        interpreter::{
            CallInputs, CallOutcome, CreateInputs, CreateOutcome, EOFCreateInputs, Interpreter,
        },
        primitives::U256,
        EvmContext,
    },
    traces::{render_trace_arena_inner, SparsedTraceArena, TracingInspectorConfig},
};

/// The [`revm::Inspector`] used when transacting in the evm
#[derive(Clone, Debug, Default)]
pub struct Inspector {
    /// Collects all traces
    pub tracer: Option<TracingInspector>,
    /// Collects all `console.sol` logs
    pub log_collector: Option<LogCollector>,
}

impl Inspector {
    /// Called after the inspecting the evm
    ///
    /// This will log all `console.sol` logs
    pub fn print_logs(&self) {
        if let Some(collector) = &self.log_collector {
            print_logs(&collector.logs);
        }
    }

    /// Called after the inspecting the evm
    /// This will log all traces
    pub fn print_traces(&self) {
        let traces = &self
            .tracer
            .clone()
            .map(|tracer| tracer.into_traces())
            .map(|arena| SparsedTraceArena { arena, ignored: Default::default() });

        if let Some(traces) = &traces {
            let _ = sh_println!("\nTraces:");
            let _ = sh_println!("{}", render_trace_arena_inner(traces, false, true));
        }
    }

    /// Configures the `Tracer` [`revm::Inspector`]
    pub fn with_tracing(mut self) -> Self {
        self.tracer = Some(TracingInspector::new(TracingInspectorConfig::all().set_steps(false)));
        self
    }

    pub fn with_tracing_config(mut self, config: TracingInspectorConfig) -> Self {
        self.tracer = Some(TracingInspector::new(config));
        self
    }

    /// Enables steps recording for `Tracer`.
    pub fn with_steps_tracing(mut self) -> Self {
        self.tracer = Some(TracingInspector::new(TracingInspectorConfig::all().with_state_diffs()));
        self
    }

    /// Configures the `Tracer` [`revm::Inspector`] with a log collector
    pub fn with_log_collector(mut self) -> Self {
        self.log_collector = Some(Default::default());
        self
    }

    /// Configures the `Tracer` [`revm::Inspector`] with a trace printer
    pub fn with_trace_printer(mut self) -> Self {
        self.tracer = Some(TracingInspector::new(TracingInspectorConfig::all().with_state_diffs()));
        self
    }
}

impl<DB: Database> revm::Inspector<DB> for Inspector {
    fn initialize_interp(&mut self, interp: &mut Interpreter, ecx: &mut EvmContext<DB>) {
        call_inspectors!([&mut self.tracer], |inspector| {
            inspector.initialize_interp(interp, ecx);
        });
    }

    fn step(&mut self, interp: &mut Interpreter, ecx: &mut EvmContext<DB>) {
        call_inspectors!([&mut self.tracer], |inspector| {
            inspector.step(interp, ecx);
        });
    }

    fn step_end(&mut self, interp: &mut Interpreter, ecx: &mut EvmContext<DB>) {
        call_inspectors!([&mut self.tracer], |inspector| {
            inspector.step_end(interp, ecx);
        });
    }

    fn log(&mut self, interp: &mut Interpreter, ecx: &mut EvmContext<DB>, log: &Log) {
        call_inspectors!([&mut self.tracer, &mut self.log_collector], |inspector| {
            inspector.log(interp, ecx, log);
        });
    }

    fn call(&mut self, ecx: &mut EvmContext<DB>, inputs: &mut CallInputs) -> Option<CallOutcome> {
        call_inspectors!(
            #[ret]
            [&mut self.tracer, &mut self.log_collector],
            |inspector| inspector.call(ecx, inputs).map(Some),
        );
        None
    }

    fn call_end(
        &mut self,
        ecx: &mut EvmContext<DB>,
        inputs: &CallInputs,
        outcome: CallOutcome,
    ) -> CallOutcome {
        if let Some(tracer) = &mut self.tracer {
            return tracer.call_end(ecx, inputs, outcome);
        }

        outcome
    }

    fn create(
        &mut self,
        ecx: &mut EvmContext<DB>,
        inputs: &mut CreateInputs,
    ) -> Option<CreateOutcome> {
        if let Some(tracer) = &mut self.tracer {
            if let Some(out) = tracer.create(ecx, inputs) {
                return Some(out);
            }
        }
        None
    }

    fn create_end(
        &mut self,
        ecx: &mut EvmContext<DB>,
        inputs: &CreateInputs,
        outcome: CreateOutcome,
    ) -> CreateOutcome {
        if let Some(tracer) = &mut self.tracer {
            return tracer.create_end(ecx, inputs, outcome);
        }

        outcome
    }

    #[inline]
    fn eofcreate(
        &mut self,
        ecx: &mut EvmContext<DB>,
        inputs: &mut EOFCreateInputs,
    ) -> Option<CreateOutcome> {
        if let Some(tracer) = &mut self.tracer {
            if let Some(out) = tracer.eofcreate(ecx, inputs) {
                return Some(out);
            }
        }
        None
    }

    #[inline]
    fn eofcreate_end(
        &mut self,
        ecx: &mut EvmContext<DB>,
        inputs: &EOFCreateInputs,
        outcome: CreateOutcome,
    ) -> CreateOutcome {
        if let Some(tracer) = &mut self.tracer {
            return tracer.eofcreate_end(ecx, inputs, outcome);
        }

        outcome
    }

    #[inline]
    fn selfdestruct(&mut self, contract: Address, target: Address, value: U256) {
        if let Some(tracer) = &mut self.tracer {
            revm::Inspector::<DB>::selfdestruct(tracer, contract, target, value);
        }
    }
}

/// Prints all the logs
pub fn print_logs(logs: &[Log]) {
    for log in decode_console_logs(logs) {
        tracing::info!(target: crate::logging::EVM_CONSOLE_LOG_TARGET, "{}", log);
    }
}
