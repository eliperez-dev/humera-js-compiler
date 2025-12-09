(module
  (func $fact (param $n i32) (result i32)
    (local $result_0 i32)
    i32.const 1
    local.set $result_0
    (block $break_0
      (loop $continue_1
    local.get $n
    i32.const 0
    i32.gt_s
        i32.eqz
        br_if $break_0
    local.get $result_0
    local.get $n
    i32.mul
    local.tee $result_0
    drop
    local.get $n
    i32.const 1
    i32.sub
    local.tee $n
    drop
        br $continue_1
      )
    )
    local.get $result_0
    return
    i32.const 0
  )
  (func $main (result i32)
    i32.const 5
    call $fact
  )
  (export "_start" (func $main))
)
