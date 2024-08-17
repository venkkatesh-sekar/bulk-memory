(module
        (func $test (export "canister_update test")
            (local $i i32)
            i32.const 4096
            local.set $i ;; set start of memory index
    
            (loop $loop_a
                
                local.get $i
                i32.const 65536
                i32.rem_u
                i32.const 0
                i32.eq
                (if
                    (then (drop (memory.grow (i32.const 1))) )
                )

                local.get $i ;; memory.copy dst
                i32.const 0  ;; memory.copy start (from data segment)
                i32.const 4096 ;; length of data section
                memory.copy
                
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
        (memory (export "memory") 2)
        (data (i32.const 0) "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")
    )
