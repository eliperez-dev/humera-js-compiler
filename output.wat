(module
  (func $gcd (param $a i32) (param $b i32) (result i32)
    (local $t_0 i32)
    (block $break_0
      (loop $continue_1
    local.get $b
    i32.const 0
    i32.ne
        i32.eqz
        br_if $break_0
    local.get $b
    local.set $t_0
    local.get $a
    local.get $b
    i32.rem_s
    local.tee $b
    drop
    local.get $t_0
    local.tee $a
    drop
        br $continue_1
      )
    )
    local.get $a
    return
    i32.const 0
  )
  (func $main (result i32)
    i32.const 48
    i32.const 18
    call $gcd
  )
  (export "_start" (func $main))
)
