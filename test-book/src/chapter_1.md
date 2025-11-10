# Chapter 1

```rust,fp=main.rs,icon=@https://icons.veryicon.com/png/o/business/vscode-program-item-icon/assembly-7.png


pub const async unsafe extern "C" fn function(a: b, mut c: D) -> Vec<TestType> {
    asm!(
        // Clear Interrupt Flag.
        // This is done because we can't let random hardware interrupts
        // to interfere with the lgdt instruction.
        // This will be useful in the future until we set up interrupts
        "cli",
        // Then, load the table using our now created register.
        "lgdt [{}]",
        inout("eax") &global_descriptor_table_register,
        options(readonly, nostack, preserves_flags)
    );
}
```
