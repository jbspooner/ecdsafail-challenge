// AUTO-GENERATED - content-addressed form of D2_DEEP_STRIP.
//
// D2_DEEP_STRIP names its 1999 never-firing CCX gates by ABSOLUTE INDEX into the
// final emitted op stream. That makes it silently wrong under any change that
// shifts indices: the strip then drops 1999 *live* gates instead, which shows up
// as a total correctness failure that looks like the experiment's fault rather
// than the strip's. That aliasing invalidated every stream-changing experiment
// (qubit-count sweeps, MSBS width, schedule search) attempted against it.
//
// This table names the same gates by CONTENT instead:
//   (kind, q_control2, q_control1, q_target, c_condition, occurrence_rank)
// where occurrence_rank disambiguates repeats of an identical fingerprint.
// Regenerate with: TLM_DUMP_STRIP_FP=1 on an unmodified baseline build.
pub(crate) const D2_STRIP_FP: [(u8, u64, u64, u64, u64, u32); 0] = [];
