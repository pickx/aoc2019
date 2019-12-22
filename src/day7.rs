use crate::opcode::*;
use aoc_runner_derive::{aoc, aoc_generator};
extern crate permute;
use itertools::all;
use permute::permute;
use std::borrow::BorrowMut;

pub struct Amp {
    _phase: isize,
    pub runner: OpcodeRunner,
}

impl Amp {
    pub fn new(mem: &[isize], phase: isize) -> Amp {
        let mut runner = OpcodeRunner::new(mem);
        runner.push_input(phase);
        Amp {
            _phase: phase,
            runner,
        }
    }
}

pub struct AmpChain {
    /// a chain of Amps, each feeding the next one's input in a chain.
    amps: Vec<Amp>,
    //    last_output: Option<isize>,
    running_amp: usize,
}

#[derive(Eq, PartialEq)]
pub enum RunMode {
    SinglePass,
    FeedbackLoop,
}

impl AmpChain {
    const NUM_AMPS_IN_CHAIN: usize = 5;

    fn num_amps(&self) -> usize {
        self.amps.len()
    }

    pub fn new(amp_phases: &[isize], initial_input: isize, mem: &[isize]) -> AmpChain {
        let num_amps = amp_phases.len();
        assert_eq!(num_amps, AmpChain::NUM_AMPS_IN_CHAIN);

        let mut amps: Vec<Amp> = Vec::with_capacity(num_amps);

        for &phase in amp_phases {
            let amp = Amp::new(mem, phase);
            amps.push(amp);
        }

        amps[0].runner.push_input_front(initial_input); //pushed in front as a workaround

        AmpChain {
            amps,
            running_amp: 0,
        }
    }

    fn current_runner(&mut self) -> &mut OpcodeRunner {
        self.amps[self.running_amp].runner.borrow_mut()
    }

    fn next_amp(&self) -> usize {
        (self.running_amp + 1) % self.num_amps()
    }

    pub fn has_halted(&self) -> bool {
        all(&self.amps, |amp| amp.runner.has_halted())
    }

    pub fn run(&mut self, run_mode: RunMode) -> Option<isize> {
        let mut last_output = None;

        while !self.has_halted() {
            let runner = self.current_runner();

            if let Some(output) = last_output.take() {
                runner.push_input_front(output);
            }


            while !runner.has_halted() {
                let cur_opcode = runner.parse_cur_opcode();
                let got_output = runner.exec_opcode(cur_opcode);

                if got_output {
                    last_output = runner.output();
                    break;
                }
            }

            if run_mode == RunMode::SinglePass && self.running_amp == self.num_amps() - 1 {
                break;
            }
            self.running_amp = self.next_amp();
        }

        self.amps[self.num_amps() - 1].runner.output()
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input.split(',').map(|val| val.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn day1(mem: &[isize]) -> Option<isize> {
    let amp_phase_permutations = permute(vec![0, 1, 2, 3, 4]);

    let initial_input = 0;

    let mut outputs = Vec::new();

    for phase_permutation in amp_phase_permutations {
        let mut chain = AmpChain::new(&phase_permutation, initial_input, mem);
        let run_result = chain.run(RunMode::SinglePass);
        if let Some(output) = run_result {
            outputs.push(output);
        }
    }

    outputs.iter().cloned().max()

    //[3,4,2,1,0] produces the max output (929800)
}

#[aoc(day7, part2)]
pub fn day2(mem: &[isize]) -> Option<isize> {
    let amp_phase_permutations = permute(vec![5, 6, 7, 8, 9]);

    let initial_input = 0;

    let mut outputs = Vec::new();

    for phase_permutation in amp_phase_permutations {
        let mut chain = AmpChain::new(&phase_permutation, initial_input, mem);
        let run_result = chain.run(RunMode::FeedbackLoop);
        if let Some(output) = run_result {
            outputs.push(output);
        }
    }

    outputs.iter().cloned().max()
}
