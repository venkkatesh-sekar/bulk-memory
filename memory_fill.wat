(module
        (func $test (export "canister_update test")
            (local $i i32)
            i32.const 0
            local.set $i ;; set start of memory index
    
            (loop $loop_a
                (drop (memory.grow (i32.const 1)))

                local.get $i
                i32.const 118
                i32.const 4096
                memory.fill
                
                local.get $i
                i32.const 4096
                i32.add
                local.set $i

                local.get $i
                i32.const 4_294_963_199
                i32.lt_u
                br_if $loop_a
            )
        )
        (memory (export "memory") 1)
    )