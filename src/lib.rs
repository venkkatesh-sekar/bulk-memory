use std::cell::RefCell;

const PAGE_PAYLOAD: &str = "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";
const KB: usize = 1024;
const OS_PAGE_SIZE: usize = 4 * KB;
const WASM_PAGE_SIZE: usize = 16 * OS_PAGE_SIZE;

// for memory call
const MAX_DATA_LENGTH: usize = 2 * 1024 * 1024 * 1024 - 150 * PAGE_PAYLOAD.len();
const B: u64 = 1_000_000_000;
const SAFE_HAVEN: u64 = 10_000;

thread_local! {
    static HOLDER: RefCell<Vec<u8>> = const { RefCell::new(vec![]) };
}

#[ic_cdk::update]
pub fn linear_extend_1kb() {
    HOLDER.with(|expand| loop {
        match expand.borrow_mut().try_reserve_exact(KB) {
            Ok(_) => (),
            Err(_) => return,
        }
        expand
            .borrow_mut()
            .extend_from_slice(PAGE_PAYLOAD[..KB].as_bytes());
    });
}

#[ic_cdk::update]
pub fn linear_extend_4kb() {
    HOLDER.with(|expand| loop {
        match expand.borrow_mut().try_reserve_exact(OS_PAGE_SIZE) {
            Ok(_) => (),
            Err(_) => return,
        }
        expand
            .borrow_mut()
            .extend_from_slice(PAGE_PAYLOAD.as_bytes());
    });
}

#[ic_cdk::update]
pub fn linear_extend_64kb() {
    let copy = PAGE_PAYLOAD.repeat(16);
    let payload = copy.as_bytes();

    HOLDER.with(|expand| loop {
        match expand.borrow_mut().try_reserve_exact(WASM_PAGE_SIZE) {
            Ok(_) => (),
            Err(_) => return,
        }
        expand.borrow_mut().extend_from_slice(payload);
    });
}

#[ic_cdk::update]
pub fn linear_extend_1mb() {
    let copy = PAGE_PAYLOAD.repeat(256);
    let payload = copy.as_bytes();

    HOLDER.with(|expand| loop {
        match expand.borrow_mut().try_reserve_exact(WASM_PAGE_SIZE * 16) {
            Ok(_) => (),
            Err(_) => return,
        }
        expand.borrow_mut().extend_from_slice(payload);
    });
}

#[ic_cdk::update]
fn memory(n: u8) {
    // Build the canister with export RUSTFLAGS="-Ctarget-feature=+bulk-memory"

    // prepare payload
    let mut src_buf = PAGE_PAYLOAD
        .repeat(MAX_DATA_LENGTH / PAGE_PAYLOAD.len())
        .into_bytes();
    let mut dst_buf: Vec<u8> = Vec::with_capacity(src_buf.len());

    // Check if we have enough stable memory for padding
    let size = unsafe { ic0::stable64_size() };
    if size < 65535 {
        let new_size: i64 = 65535 - size;
        let _grow = unsafe { ic0::stable64_grow(new_size) };
    }

    let now = ic_cdk::api::performance_counter(0);
    ic_cdk::println!("Instruction used before padding {}", now);

    // large stable write to pad slice limit to the end
    unsafe {
        ic0::stable64_write(0_i64, src_buf.as_ptr() as i64, calculate_offset(now) as i64);
    }

    let now = ic_cdk::api::performance_counter(0);
    ic_cdk::println!(
        "Instruction used after padding {}, src_buf length {}",
        now,
        src_buf.len()
    );

    match n {
        0 => {
            // memory.copy
            unsafe {
                std::ptr::copy_nonoverlapping(
                    src_buf.as_ptr(),
                    dst_buf.as_mut_ptr(),
                    src_buf.len(),
                );
                dst_buf.set_len(src_buf.len());
            }
        }
        1 => {
            // memory.fill
            src_buf.as_mut_slice().fill(0xab);
        }
        2 => {
            return;
        }
        _ => unimplemented!(),
    }
}

fn calculate_offset(inst: u64) -> u64 {
    let remaining_slice = (2 * B) - (inst % (2 * B));

    // remaining = 20 + bytes + (ceil(bytes / 4096) * 1000);
    // we ignore the ceil for rough calculation

    let bytes = (4096 * (remaining_slice - 20)) / 5096;
    bytes - SAFE_HAVEN
}

#[cfg(feature = "canbench-rs")]
mod benches {
    use super::*;
    use canbench_rs::bench;

    #[bench]
    fn benchmark_linear_extend_1kb() {
        linear_extend_1kb();
    }

    #[bench]
    fn benchmark_linear_extend_4kb() {
        linear_extend_4kb();
    }

    #[bench]
    fn benchmark_linear_extend_64kb() {
        linear_extend_64kb();
    }

    #[bench]
    fn benchmark_linear_extend_1mb() {
        linear_extend_1mb();
    }
}

#[cfg(test)]
mod tests {
    use candid::{encode_one, Principal};
    use pocket_ic::PocketIc;

    #[test]
    #[should_panic]
    fn test_memory_copy() {
        let pic = PocketIc::new();
        // Create an empty canister as the anonymous principal and add cycles.
        let canister_id = pic.create_canister();
        pic.add_cycles(canister_id, 20_000_000_000_000);

        let bytes = include_bytes!("../target/wasm32-unknown-unknown/release/memory_canister.wasm");
        pic.install_canister(canister_id, bytes.to_vec(), vec![], None);

        pic.update_call(
            canister_id,
            Principal::anonymous(),
            "memory",
            encode_one(0_u8).unwrap(),
        )
        .expect("Failed to call memory_canister");
    }

    #[test]
    #[should_panic]
    fn test_memory_fill() {
        let pic = PocketIc::new();
        // Create an empty canister as the anonymous principal and add cycles.
        let canister_id = pic.create_canister();
        pic.add_cycles(canister_id, 20_000_000_000_000);

        let bytes = include_bytes!("../target/wasm32-unknown-unknown/release/memory_canister.wasm");
        pic.install_canister(canister_id, bytes.to_vec(), vec![], None);

        pic.update_call(
            canister_id,
            Principal::anonymous(),
            "memory",
            encode_one(1_u8).unwrap(),
        )
        .expect("Failed to call memory_canister");
    }

    #[test]
    fn test_cycles_consumed() {
        let pic = PocketIc::new();
        let bytes = include_bytes!("../target/wasm32-unknown-unknown/release/memory_canister.wasm");

        // memory_copy
        let canister_id = pic.create_canister();
        pic.add_cycles(canister_id, 20_000_000_000_000);
        pic.install_canister(canister_id, bytes.to_vec(), vec![], None);

        let execution_0_cycles_balance_before = pic.cycle_balance(canister_id);
        let err = pic
            .update_call(
                canister_id,
                Principal::anonymous(),
                "memory",
                encode_one(0_u8).unwrap(),
            )
            .unwrap_err();
        let execution_0_cycles_balance_delta =
            execution_0_cycles_balance_before - pic.cycle_balance(canister_id);
        let nums: Vec<i64> = err
            .description
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();
        let execution_0_instructions_extra = nums.first().unwrap();

        // memory_fill
        let canister_id = pic.create_canister();
        pic.add_cycles(canister_id, 20_000_000_000_000);
        pic.install_canister(canister_id, bytes.to_vec(), vec![], None);
        let execution_1_cycles_balance_before = pic.cycle_balance(canister_id);
        let err = pic
            .update_call(
                canister_id,
                Principal::anonymous(),
                "memory",
                encode_one(1_u8).unwrap(),
            )
            .unwrap_err();
        let execution_1_cycles_balance_delta =
            execution_1_cycles_balance_before - pic.cycle_balance(canister_id);
        let nums: Vec<i64> = err
            .description
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();
        let execution_1_instructions_extra = nums.first().unwrap();

        // no large operation
        let canister_id = pic.create_canister();
        pic.add_cycles(canister_id, 20_000_000_000_000);
        pic.install_canister(canister_id, bytes.to_vec(), vec![], None);
        let execution_2_cycles_balance_before = pic.cycle_balance(canister_id);
        let _ = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "memory",
            encode_one(2_u8).unwrap(),
        );
        let execution_2_cycles_balance_delta =
            execution_2_cycles_balance_before - pic.cycle_balance(canister_id);

        println!(
            "Memory copy: Overflow Instruction {}, Cycles {}, Expected {}",
            execution_0_instructions_extra,
            execution_0_cycles_balance_delta - execution_2_cycles_balance_delta,
            (execution_0_instructions_extra / 10) * 4
        );
        println!(
            "Memory fill: Overflow Instruction {}, Cycles {}, Expected {}",
            execution_1_instructions_extra,
            execution_1_cycles_balance_delta - execution_2_cycles_balance_delta,
            (execution_0_instructions_extra / 10) * 4
        );
    }
}
