use crate::opcode::*;
use aoc_runner_derive::{aoc, aoc_generator};
extern crate permute;
use permute::permute;
use itertools::{all, any};

pub struct Amp {
    phase: usize,
    input: Option<usize>,
    pub runner: CodeRunner,
}

impl Amp {

    pub fn new(mem: &[isize], phase: usize) -> Amp {
        let mut runner = CodeRunner::new(mem);

        Amp { phase, input: None, runner }
    }

    pub fn run_until_output(&mut self) -> isize {

        loop {
            let cur_opcode = self.runner.parse_cur_opcode();
            self.runner.run_opcode(cur_opcode);

            if let Opcode::Output(_) = cur_opcode { break; }
        }

        self.runner.output().unwrap()
    }

    pub fn halted(&self) -> bool { self.runner.has_halted() }
}

pub struct AmpChain {
    amps: Vec<Amp>,
    currently_running_amp: usize,
}

impl AmpChain {
    fn get_input_from_prev_amp(&mut self) -> usize {
        self.amps[self.next_amp_index()].input.expect("no input on prev amp")
    }

    fn new(amps: Vec<Amp>, initial_input: usize) -> AmpChain {
        assert_eq!(amps.len(), 5);

        let mut chain = AmpChain { amps: amps, currently_running_amp: 0 };
        chain.amps[0].input = Some(initial_input);

        chain
    }

    fn next_amp_index(&self) -> usize {
        (self.currently_running_amp + 1) % 5
    }

    pub fn halted(&self) -> bool {
        all(&self.amps, |amp| amp.halted())
    }

    pub fn run(&mut self) -> usize {

        while !self.halted() {

        }

        final_output
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input.split(',').map(|val| val.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn day1(mem: &[isize]) -> Option<isize> {
    let amp_phase_settings_perms = permute(vec![0, 1, 2, 3, 4]);

    let initial_input = 0;

    let mut outputs = Vec::new();

    for amp_phase_settings in amp_phase_settings_perms {
        let mut runners: Vec<CodeRunner> = Vec::new();
        for amp_phase_setting in amp_phase_settings {
            let mut cr = CodeRunner::new(mem);
            cr.push_input(amp_phase_setting);
            runners.push(cr);
        }

        let mut next_output = initial_input;

        for cr in runners.iter_mut() {
            cr.push_input_front(next_output);

            while cr.output().is_none() {
                let cur_opcode = cr.parse_cur_opcode();
                cr.run_opcode(cur_opcode);
            }

            next_output = cr.output().expect("No output");
        }

        outputs.push(next_output);
    }

    outputs.iter().cloned().max()

    //[3,4,2,1,0] produces the max output (929800)
}

#[aoc(day7, part2)]
pub fn day2(mem: &[isize]) -> Option<isize> {
    let amp_phase_settings_perms = permute(vec![5, 6, 7, 8, 9]);

    let initial_input = 0;

    let mut outputs = Vec::new();

    for amp_phase_settings in amp_phase_settings_perms {
        let mut amps: Vec<Amp> = Vec::new();

        for amp_phase_setting in amp_phase_settings {
            let mut cr = CodeRunner::new(mem);
            let mut amp = Amp { phase: amp_phase_setting, input: None, runner: cr };
            amps.push(amp);
        }

        let mut ampchain = AmpChain { amps, initial_input, currently_running_amp: 0, };

        let mut next_output = initial_input;

        let chain_result = ampchain.run();

        outputs.push(chain_result);

    }

    outputs.iter().cloned().max()

}
