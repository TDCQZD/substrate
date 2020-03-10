use std::time::{Instant, Duration};
use codec::{Encode, Decode};
use sp_runtime_interface::pass_by::{PassBy, Codec};
use crate::StateMachineStats;

/// Measured count of operations and total bytes.
#[derive(Clone, Debug, Default, Encode, Decode)]
pub struct UsageUnit {
	/// Number of operations.
	pub ops: u64,
	/// Number of bytes.
	pub bytes: u64,
}


/// A wrapper around Instant to add a Default impl in order to skip encoding / decoding.
#[derive(Clone, Debug)]
pub struct InstantWithDefault(Instant);

impl std::default::Default for InstantWithDefault {
	fn default() -> Self {
		Self(Instant::now())
	}
}

/// SCOTT
// SCOTT INVESTIGATE
impl InstantWithDefault {
	pub fn new(instant: Instant) -> Self {
		Self(instant)
	}
}

/// Usage statistics for state backend.
#[derive(Clone, Debug, Encode, Decode)]
pub struct UsageInfo {
	/// Read statistics (total).
	pub reads: UsageUnit,
	/// Write statistics (total).
	pub writes: UsageUnit,
	/// Write trie nodes statistics.
	pub nodes_writes: UsageUnit,
	/// Write into cached state machine
	/// change overlay.
	pub overlay_writes: UsageUnit,
	/// Removed trie nodes statistics.
	pub removed_nodes: UsageUnit,
	/// Cache read statistics.
	pub cache_reads: UsageUnit,
	/// Modified value read statistics.
	pub modified_reads: UsageUnit,
	/// Memory used.
	// Encoded as u32 because wasm's usize is 32.
	pub memory: u32,

	/// Moment at which current statistics has been started being collected.
	#[codec(skip)]
	pub started: InstantWithDefault,
	/// Timespan of the statistics.
	#[codec(skip)]
	pub span: Duration,
}

impl PassBy for UsageInfo {
    type PassBy = Codec<Self>;
}

impl UsageInfo {
	/// Empty statistics.
	///
	/// Means no data was collected.
	pub fn empty() -> Self {
		Self {
			reads: UsageUnit::default(),
			writes: UsageUnit::default(),
			overlay_writes: UsageUnit::default(),
			nodes_writes: UsageUnit::default(),
			removed_nodes: UsageUnit::default(),
			cache_reads: UsageUnit::default(),
			modified_reads: UsageUnit::default(),
			memory: 0,
			started: InstantWithDefault(Instant::now()),
			span: Default::default(),
		}
	}
	/// Add collected state machine to this state.
	pub fn include_state_machine_states(&mut self, count: &StateMachineStats) {
		self.modified_reads.ops += *count.reads_modified.borrow();
		self.modified_reads.bytes += *count.bytes_read_modified.borrow();
		self.overlay_writes.ops += *count.writes_overlay.borrow();
		self.overlay_writes.bytes += *count.bytes_writes_overlay.borrow();
	}
}