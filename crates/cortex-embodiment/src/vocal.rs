//! Vocal output as motor output (whitepaper §5.2.4, §8.16; ADR-0027). A [`VocalFrame`] per
//! 1 ms epoch carries the articulatory parameters the engine decides (fundamental, three
//! formants with their bandwidths, amplitude, jitter, shimmer, aspiration, voicing), the way a
//! `TorqueFrame` carries joint torques; [`VocalSynth`] renders a frame as Q16.16 samples with a
//! source–filter model in integer arithmetic: an impulse train at the fundamental, perturbed
//! deterministically by jitter and shimmer from a seeded generator, mixed with aspiration
//! noise, through three second-order resonators in cascade (Klatt, 1980). The audio device
//! that plays the samples is an actuator behind the embodiment ring, like a joint (Specified).
//!
//! Every coefficient is derived in Q16.16 from the frame: $e^{-x}$ by a cubic and $\cos\theta$
//! by an eighth-order series, both within $3 \times 10^{-5}$ of the real functions over the
//! ranges a formant below a quarter of the sample rate and a bandwidth below a thirty-second of
//! it need; the values a test pins were computed by the same integer arithmetic and checked
//! against the real formulas.

/// The sample rate a neutral frame carries.
pub const VOCAL_SAMPLE_RATE_HZ: u16 = 16_000;
/// Formants per frame.
pub const FORMANTS: usize = 3;
/// $\pi$ in Q16.16 (205 887.4 rounded).
pub const PI_Q16: u32 = 205_887;
/// The lowest fundamental `shape` will set, Hz.
pub const F0_MIN_HZ: u32 = 50;
/// The highest fundamental `shape` will set, Hz.
pub const F0_MAX_HZ: u32 = 500;

/// Tones a frame can be shaped by: the values of `cortex-linguistic`'s `PROSODY_*` markers,
/// restated here because state crates do not depend on one another.
pub const TONE_NEUTRAL: u8 = 0;
/// A softened utterance: lower, breathier, quieter.
pub const TONE_SOFTEN: u8 = 1;
/// A suggestion: a slightly raised fundamental.
pub const TONE_SUGGEST: u8 = 2;
/// A reflection: lower and quieter.
pub const TONE_REFLECT: u8 = 3;
/// A topic shift: louder and raised.
pub const TONE_TOPIC_SHIFT: u8 = 4;
/// Play: raised, with more jitter and shimmer.
pub const TONE_PLAYFUL: u8 = 5;

const Q16_ONE: i64 = 1 << 16;

/// One epoch's vocal command, engine → audio actuator. 64 B, align 64.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct VocalFrame {
    pub epoch: u64,                    // [0..8] Simulation epoch this command belongs to
    pub f0_hz_q16: u32,                // [8..12] Fundamental frequency, Hz (Q16.16)
    pub formant_hz: [u16; FORMANTS],   // [12..18] F1, F2, F3 in Hz
    pub bandwidth_hz: [u16; FORMANTS], // [18..24] Their bandwidths in Hz; never zero when rendered
    pub amplitude_q1_15: i16,          // [24..26] Glottal pulse amplitude in [0, 1) (Q1.15)
    pub jitter_q0_8: u8, // [26] Period perturbation as a fraction of the period (Q0.8)
    pub shimmer_q0_8: u8, // [27] Pulse amplitude perturbation (Q0.8)
    pub aspiration_q0_8: u8, // [28] Noise mixed in, as a fraction of the amplitude (Q0.8)
    pub voicing: u8,     // [29] 1 voiced (pulses), 0 unvoiced (noise only)
    pub sample_rate_hz: u16, // [30..32] Samples per second the frame is rendered at
    pub seed: u32,       // [32..36] Seed of the perturbation generator
    pub _reserved: [u8; 28], // [36..64] Reserved; MUST be zero
}

impl VocalFrame {
    /// A neutral voice for `epoch`: 120 Hz, formants 500 / 1 500 / 2 500 Hz with bandwidths
    /// 60 / 90 / 120 Hz, amplitude 0.5, no jitter, shimmer or aspiration, voiced, 16 kHz, the
    /// generator seeded from the epoch.
    pub const fn new(epoch: u64) -> Self {
        Self {
            epoch,
            f0_hz_q16: 120 << 16,
            formant_hz: [500, 1_500, 2_500],
            bandwidth_hz: [60, 90, 120],
            amplitude_q1_15: 0x4000,
            jitter_q0_8: 0,
            shimmer_q0_8: 0,
            aspiration_q0_8: 0,
            voicing: 1,
            sample_rate_hz: VOCAL_SAMPLE_RATE_HZ,
            seed: (epoch as u32) ^ ((epoch >> 32) as u32) ^ 0x9E37_79B9,
            _reserved: [0; 28],
        }
    }

    /// Shapes the voice by a tone (a `PROSODY_*` marker), the register (formal voices are
    /// steadier) and the valence of what is said, all in fixed fractions: the fundamental
    /// moves by a quarter of the valence and by the tone's step, clamped to [50, 500] Hz; a
    /// negative valence adds breath and takes amplitude; a soft or reflective tone lowers and
    /// quietens, a suggestion or a topic shift raises, play adds jitter and shimmer; a formal
    /// register halves jitter and shimmer. Refused, with nothing changed, for an unknown tone.
    pub fn shape(&mut self, tone: u8, formal: bool, valence_q16: i32) -> bool {
        let (f0_shift_q16, amp_shift_q16, breath, jitter, shimmer): (i64, i64, u8, u8, u8) =
            match tone {
                TONE_NEUTRAL => (0, 0, 0, 0, 0),
                TONE_SOFTEN => (-(Q16_ONE >> 4), -(Q16_ONE >> 3), 32, 0, 0),
                TONE_SUGGEST => (Q16_ONE >> 4, 0, 0, 0, 0),
                TONE_REFLECT => (-(Q16_ONE >> 3), -(Q16_ONE >> 3), 0, 0, 0),
                TONE_TOPIC_SHIFT => (Q16_ONE >> 3, Q16_ONE >> 3, 0, 0, 0),
                TONE_PLAYFUL => (Q16_ONE >> 3, 0, 0, 16, 16),
                _ => return false,
            };
        let valence = (valence_q16 as i64).clamp(-Q16_ONE, Q16_ONE);
        let f0_factor = Q16_ONE + f0_shift_q16 + (valence >> 2);
        let f0 = ((self.f0_hz_q16 as i64 * f0_factor) >> 16)
            .clamp((F0_MIN_HZ as i64) << 16, (F0_MAX_HZ as i64) << 16);
        self.f0_hz_q16 = f0 as u32;
        let amp_factor = Q16_ONE + amp_shift_q16 - (valence.min(0).abs() >> 2);
        self.amplitude_q1_15 =
            ((self.amplitude_q1_15 as i64 * amp_factor) >> 16).clamp(0, i16::MAX as i64) as i16;
        let valence_breath = ((valence.min(0).abs() * 64) >> 16) as u8;
        self.aspiration_q0_8 = self
            .aspiration_q0_8
            .saturating_add(breath)
            .saturating_add(valence_breath);
        self.jitter_q0_8 = self.jitter_q0_8.saturating_add(jitter);
        self.shimmer_q0_8 = self.shimmer_q0_8.saturating_add(shimmer);
        if formal {
            self.jitter_q0_8 >>= 1;
            self.shimmer_q0_8 >>= 1;
        }
        true
    }
}

/// $(a \times b) \gg 16$ for Q16.16 operands in `i64`.
#[inline(always)]
const fn mul_q16(a: i64, b: i64) -> i64 {
    (a * b) >> 16
}

/// $e^{-x}$ for $0 \le x \lesssim 0.4$ in Q16.16 by $1 - x + x^2/2 - x^3/6$.
const fn exp_neg_q16(x: i64) -> i64 {
    let x2 = mul_q16(x, x);
    let x3 = mul_q16(x2, x);
    Q16_ONE - x + x2 / 2 - x3 / 6
}

/// $\cos\theta$ for $0 \le \theta \le \pi/2$ in Q16.16 by the series to $\theta^8$.
const fn cos_q16(theta: i64) -> i64 {
    let t2 = mul_q16(theta, theta);
    let t4 = mul_q16(t2, t2);
    let t6 = mul_q16(t4, t2);
    let t8 = mul_q16(t4, t4);
    Q16_ONE - t2 / 2 + t4 / 24 - t6 / 720 + t8 / 40_320
}

/// A second-order resonator, $y_n = x_n + B\,y_{n-1} + C\,y_{n-2}$ with
/// $B = 2 e^{-\pi\,\text{bw}/f_s} \cos(2\pi f / f_s)$ and $C = -e^{-2\pi\,\text{bw}/f_s}$, in
/// Q16.16 with `i64` products and a clamp to the `i32` range.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Resonator {
    b_q16: i32,
    c_q16: i32,
    y1: i32,
    y2: i32,
}

impl Resonator {
    /// A resonator at `f_hz` with bandwidth `bw_hz` at `sample_rate_hz`. `None` for a zero
    /// rate, a zero bandwidth (an undamped pole), a frequency above a quarter of the rate
    /// (outside the cosine series' range), or a bandwidth above a thirty-second of the rate
    /// (outside the exponential's: the damping is then at most $\pi/32$, where the cubic is
    /// within $4 \times 10^{-6}$; wider, the pole would leave the unit circle).
    pub const fn new(f_hz: u16, bw_hz: u16, sample_rate_hz: u16) -> Option<Self> {
        if sample_rate_hz == 0
            || bw_hz == 0
            || f_hz as u32 * 4 > sample_rate_hz as u32
            || bw_hz as u32 * 32 > sample_rate_hz as u32
        {
            return None;
        }
        let fs = sample_rate_hz as i64;
        let theta = (2 * PI_Q16 as i64 * f_hz as i64) / fs;
        let damping = (PI_Q16 as i64 * bw_hz as i64) / fs;
        let r = exp_neg_q16(damping);
        let b = 2 * mul_q16(r, cos_q16(theta));
        let c = -mul_q16(r, r);
        Some(Self {
            b_q16: b as i32,
            c_q16: c as i32,
            y1: 0,
            y2: 0,
        })
    }

    /// The coefficients $(B, C)$ in Q16.16.
    pub const fn coefficients(&self) -> (i32, i32) {
        (self.b_q16, self.c_q16)
    }

    /// One sample in, one out.
    #[inline]
    pub fn step(&mut self, x_q16: i32) -> i32 {
        let y = x_q16 as i64
            + mul_q16(self.b_q16 as i64, self.y1 as i64)
            + mul_q16(self.c_q16 as i64, self.y2 as i64);
        let y = y.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
        self.y2 = self.y1;
        self.y1 = y;
        y
    }

    /// Silence: the state is cleared.
    pub fn reset(&mut self) {
        self.y1 = 0;
        self.y2 = 0;
    }
}

/// The renderer of one frame: the excitation (pulses and noise) and the three resonators.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VocalSynth {
    voiced: bool,
    base_period: u32,
    period: u32,
    phase: u32,
    amplitude_q16: i64,
    jitter_q0_8: u8,
    shimmer_q0_8: u8,
    aspiration_q0_8: u8,
    generator: u32,
    resonators: [Resonator; FORMANTS],
}

impl VocalSynth {
    /// The renderer of `frame`. `None` when a resonator is refused (see [`Resonator::new`]), the
    /// amplitude is negative, or the frame is voiced with a fundamental that gives no period
    /// or one longer than 65 535 samples.
    pub fn from_frame(frame: &VocalFrame) -> Option<Self> {
        if frame.amplitude_q1_15 < 0 || frame.sample_rate_hz == 0 {
            return None;
        }
        let voiced = frame.voicing != 0;
        let base_period = if voiced {
            if frame.f0_hz_q16 == 0 {
                return None;
            }
            let period = ((frame.sample_rate_hz as u64) << 16) / frame.f0_hz_q16 as u64;
            if period == 0 || period > u16::MAX as u64 {
                return None;
            }
            period as u32
        } else {
            0
        };
        let mut resonators = [Resonator::new(1, 1, VOCAL_SAMPLE_RATE_HZ)?; FORMANTS];
        for (k, resonator) in resonators.iter_mut().enumerate() {
            *resonator = Resonator::new(
                frame.formant_hz[k],
                frame.bandwidth_hz[k],
                frame.sample_rate_hz,
            )?;
        }
        Some(Self {
            voiced,
            base_period,
            period: base_period,
            phase: 0,
            // Q1.15 to Q16.16.
            amplitude_q16: (frame.amplitude_q1_15 as i64) << 1,
            jitter_q0_8: frame.jitter_q0_8,
            shimmer_q0_8: frame.shimmer_q0_8,
            aspiration_q0_8: frame.aspiration_q0_8,
            generator: frame.seed,
            resonators,
        })
    }

    /// A signed fraction in $[-1, 1)$ (Q16.16) from the generator.
    fn draw(&mut self) -> i64 {
        self.generator = self
            .generator
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        ((self.generator >> 16) as i64 - 0x8000) << 1
    }

    /// The next excitation sample before the filter: a pulse at the start of each period
    /// (its amplitude perturbed by shimmer, the next period's length by jitter), plus
    /// aspiration noise. Q16.16.
    pub fn next_source(&mut self) -> i32 {
        let mut x: i64 = 0;
        if self.voiced {
            if self.phase == 0 {
                let shimmer = mul_q16(
                    self.amplitude_q16 * self.shimmer_q0_8 as i64 / 256,
                    self.draw(),
                );
                x += self.amplitude_q16 + shimmer;
                let jitter =
                    (self.base_period as i64 * self.jitter_q0_8 as i64 * self.draw()) >> 24;
                self.period = (self.base_period as i64 + jitter).clamp(1, u16::MAX as i64) as u32;
            }
            self.phase += 1;
            if self.phase >= self.period {
                self.phase = 0;
            }
        }
        if self.aspiration_q0_8 != 0 {
            let noise = mul_q16(
                self.amplitude_q16 * self.aspiration_q0_8 as i64 / 256,
                self.draw(),
            );
            x += noise;
        }
        x.clamp(i32::MIN as i64, i32::MAX as i64) as i32
    }

    /// The next output sample: the excitation through the three resonators. Q16.16.
    #[inline]
    pub fn next_sample(&mut self) -> i32 {
        let mut y = self.next_source();
        for r in self.resonators.iter_mut() {
            y = r.step(y);
        }
        y
    }

    /// Fills `out` with consecutive samples.
    pub fn render(&mut self, out: &mut [i32]) {
        for sample in out.iter_mut() {
            *sample = self.next_sample();
        }
    }

    /// The current period in samples (0 when unvoiced).
    pub const fn period(&self) -> u32 {
        self.period
    }
}

const _: () = {
    assert!(core::mem::size_of::<VocalFrame>() == 64);
    assert!(core::mem::align_of::<VocalFrame>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    const FS: u16 = VOCAL_SAMPLE_RATE_HZ;

    #[test]
    fn the_frame_is_one_cache_line_and_a_new_frame_is_the_neutral_voice() {
        assert_eq!(core::mem::size_of::<VocalFrame>(), 64);
        assert_eq!(core::mem::align_of::<VocalFrame>(), 64);
        let f = VocalFrame::new(7);
        assert_eq!(
            (f.epoch, f.f0_hz_q16 >> 16, f.voicing, f.sample_rate_hz),
            (7, 120, 1, FS)
        );
        assert_eq!(
            (f.formant_hz, f.bandwidth_hz),
            ([500, 1500, 2500], [60, 90, 120])
        );
        assert_eq!(f.amplitude_q1_15, 0x4000);
        assert_ne!(
            f.seed,
            VocalFrame::new(8).seed,
            "the seed follows the epoch"
        );
        assert_eq!(VocalFrame::default().voicing, 0, "a zeroed frame is silent");
    }

    #[test]
    fn resonator_coefficients_match_the_real_formulas_and_the_refusals_hold() {
        assert_eq!(
            Resonator::new(500, 50, FS).unwrap().coefficients(),
            (127_298, -64_262),
            "B = 2 e^-pi 50/16000 cos(2 pi 500/16000) = 1.94241; C = -e^-2 pi 50/16000 = -0.98056"
        );
        assert_eq!(
            Resonator::new(1500, 80, FS).unwrap().coefficients(),
            (107_286, -63_509)
        );
        assert_eq!(
            Resonator::new(2500, 120, FS).unwrap().coefficients(),
            (71_126, -62_519)
        );
        assert!(
            Resonator::new(4000, 100, FS).is_some(),
            "a quarter of the rate is allowed"
        );
        assert!(
            Resonator::new(4001, 100, FS).is_none(),
            "above it the series is out of range"
        );
        assert!(Resonator::new(500, 0, FS).is_none(), "an undamped pole");
        assert!(Resonator::new(500, 50, 0).is_none());
        assert!(
            Resonator::new(500, 500, FS).is_some(),
            "a thirty-second of the rate is allowed"
        );
        assert!(
            Resonator::new(500, 501, FS).is_none(),
            "wider is outside the exponential's range"
        );
        assert!(
            Resonator::new(0, 60, 1).is_none(),
            "a rate of one refuses every bandwidth"
        );
    }

    #[test]
    fn an_impulse_makes_a_resonator_ring_at_its_frequency_and_decay() {
        let mut r = Resonator::new(500, 50, FS).unwrap();
        let mut crossings = 0;
        let mut previous = 0i32;
        let mut peak = 0i32;
        let mut last = 0i32;
        for n in 0..1600 {
            let y = r.step(if n == 0 { Q16_ONE as i32 } else { 0 });
            if n > 0 && (y > 0) != (previous > 0) {
                crossings += 1;
            }
            previous = y;
            peak = peak.max(y);
            last = y;
        }
        // 500 Hz over 0.1 s is 50 cycles: about a hundred zero crossings.
        assert!((90..=104).contains(&crossings), "{crossings} crossings");
        assert!(peak > Q16_ONE as i32, "the resonance gains");
        assert!(last.abs() < peak / 100, "and decays: {last} of {peak}");
        r.reset();
        assert_eq!(r.step(0), 0);
    }

    fn frame(f0_hz: u32) -> VocalFrame {
        let mut f = VocalFrame::new(1);
        f.f0_hz_q16 = f0_hz << 16;
        f
    }

    #[test]
    fn a_voiced_source_pulses_every_period_and_jitter_and_shimmer_perturb_it_within_bounds() {
        let mut steady = VocalSynth::from_frame(&frame(100)).unwrap();
        assert_eq!(steady.period(), 160, "16 000 / 100");
        let mut pulses = [0usize; 16];
        let mut n = 0;
        for i in 0..1600 {
            if steady.next_source() != 0 {
                pulses[n] = i;
                n += 1;
            }
        }
        assert_eq!(n, 10);
        assert_eq!(
            &pulses[..10],
            &[0, 160, 320, 480, 640, 800, 960, 1120, 1280, 1440]
        );

        let mut shaky = frame(100);
        shaky.jitter_q0_8 = 32; // periods within 12.5 % of 160
        shaky.shimmer_q0_8 = 64; // amplitudes within 25 % of 0.5
        let mut synth = VocalSynth::from_frame(&shaky).unwrap();
        let mut last_pulse = None;
        let mut periods_seen = 0;
        let mut varied = false;
        for i in 0..4000u32 {
            let x = synth.next_source();
            if x != 0 {
                let amp = x as i64;
                let nominal = (0x4000i64) << 1;
                assert!(
                    (amp - nominal).abs() <= nominal / 4 + 1,
                    "shimmer bound: {amp}"
                );
                if let Some(p) = last_pulse {
                    let period = i - p;
                    assert!((140..=180).contains(&period), "jitter bound: {period}");
                    if period != 160 {
                        varied = true;
                    }
                    periods_seen += 1;
                }
                last_pulse = Some(i);
            }
        }
        assert!(periods_seen > 20 && varied, "jitter varies the period");
    }

    #[test]
    fn an_unvoiced_frame_is_noise_or_silence_and_aspiration_is_bounded() {
        let mut silent = VocalFrame::new(1);
        silent.voicing = 0;
        let mut synth = VocalSynth::from_frame(&silent).unwrap();
        assert_eq!(synth.period(), 0);
        let mut out = [1i32; 64];
        synth.render(&mut out);
        assert!(
            out.iter().all(|&s| s == 0),
            "no voicing, no aspiration: silence"
        );
        let mut breathy = silent;
        breathy.aspiration_q0_8 = 128;
        let mut synth = VocalSynth::from_frame(&breathy).unwrap();
        let bound = ((0x4000i64) << 1) / 2;
        let mut any = false;
        for _ in 0..1000 {
            let x = synth.next_source() as i64;
            assert!(
                x.abs() <= bound + 1,
                "aspiration is half the amplitude at most: {x}"
            );
            any |= x != 0;
        }
        assert!(any, "aspiration is not silence");
    }

    #[test]
    fn rendering_is_deterministic_per_seed_and_refuses_what_it_cannot_render() {
        let mut f = frame(150);
        f.jitter_q0_8 = 40;
        let (mut a, mut b) = (
            VocalSynth::from_frame(&f).unwrap(),
            VocalSynth::from_frame(&f).unwrap(),
        );
        let (mut x, mut y) = ([0i32; 256], [0i32; 256]);
        a.render(&mut x);
        b.render(&mut y);
        assert_eq!(x, y);
        assert!(x.iter().any(|&s| s != 0));
        for bit in [0, 20, 31] {
            let mut other = f;
            other.seed ^= 1 << bit;
            let mut c = VocalSynth::from_frame(&other).unwrap();
            let mut z = [0i32; 256];
            c.render(&mut z);
            assert_ne!(x, z, "seed bit {bit} matters");
        }
        let mut zero = f;
        zero.seed = 0;
        let mut c = VocalSynth::from_frame(&zero).unwrap();
        let mut z = [0i32; 256];
        c.render(&mut z);
        assert!(
            z.iter().any(|&s| s != 0),
            "seed zero is a seed like any other"
        );

        let mut loud = frame(120);
        loud.amplitude_q1_15 = i16::MAX;
        loud.bandwidth_hz = [1, 1, 1];
        let mut hot = VocalSynth::from_frame(&loud).unwrap();
        let mut buf = [0i32; 2000];
        hot.render(&mut buf); // saturates, never wraps or panics

        let mut negative = frame(120);
        negative.amplitude_q1_15 = -1;
        assert!(VocalSynth::from_frame(&negative).is_none());
        assert!(
            VocalSynth::from_frame(&frame(0)).is_none(),
            "voiced needs a fundamental"
        );
        let mut slow = frame(0);
        slow.f0_hz_q16 = 1; // a period of a million samples
        assert!(VocalSynth::from_frame(&slow).is_none());
        let mut high = frame(120);
        high.formant_hz[2] = 5000;
        assert!(VocalSynth::from_frame(&high).is_none());
    }

    #[test]
    fn shape_moves_the_voice_by_tone_register_and_valence() {
        let neutral = VocalFrame::new(1);
        let mut soft = neutral;
        assert!(soft.shape(TONE_SOFTEN, false, 0));
        assert!(soft.f0_hz_q16 < neutral.f0_hz_q16, "softer is lower");
        assert!(soft.aspiration_q0_8 > 0, "and breathier");
        assert!(
            soft.amplitude_q1_15 < neutral.amplitude_q1_15,
            "and quieter"
        );
        let mut suggest = neutral;
        assert!(suggest.shape(TONE_SUGGEST, false, 0));
        assert!(suggest.f0_hz_q16 > neutral.f0_hz_q16);
        let mut playful = neutral;
        assert!(playful.shape(TONE_PLAYFUL, false, 0));
        assert_eq!((playful.jitter_q0_8, playful.shimmer_q0_8), (16, 16));
        let mut formal = neutral;
        assert!(formal.shape(TONE_PLAYFUL, true, 0));
        assert_eq!(
            (formal.jitter_q0_8, formal.shimmer_q0_8),
            (8, 8),
            "a formal register is steadier"
        );
        let mut glad = neutral;
        assert!(glad.shape(TONE_NEUTRAL, false, 1 << 16));
        assert_eq!(
            glad.f0_hz_q16 >> 16,
            150,
            "a valence of +1 raises the fundamental by a quarter"
        );
        let mut sad = neutral;
        assert!(sad.shape(TONE_NEUTRAL, false, -(1 << 16)));
        assert_eq!(sad.f0_hz_q16 >> 16, 90);
        assert_eq!(sad.aspiration_q0_8, 64);
        assert!(sad.amplitude_q1_15 < neutral.amplitude_q1_15);
        let mut floor = neutral;
        for _ in 0..20 {
            assert!(floor.shape(TONE_REFLECT, false, -(1 << 16)));
        }
        assert_eq!(floor.f0_hz_q16 >> 16, F0_MIN_HZ, "clamped at the floor");
        let mut ceiling = neutral;
        for _ in 0..40 {
            assert!(ceiling.shape(TONE_TOPIC_SHIFT, false, 1 << 16));
        }
        assert_eq!(ceiling.f0_hz_q16 >> 16, F0_MAX_HZ);
        let mut unknown = neutral;
        assert!(!unknown.shape(9, false, 0));
        assert_eq!(unknown, neutral);
        assert!(
            VocalSynth::from_frame(&ceiling).is_some(),
            "every shaped voice renders"
        );
    }
}
