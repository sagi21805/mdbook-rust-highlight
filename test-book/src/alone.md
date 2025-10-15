```rust,fp=shared/cpu_utils/src/structures/paging/entry_flags.rs
macro_rules! table_entry_flags {
    () => {
        // Is this entry present?
        flag!(present, 0);
        // Is this page writable?
        flag!(writable, 1);
        // Can this page be accessed from user mode
        flag!(usr_access, 2);
        // Writes go directly to memory
        flag!(write_through_cache, 3);
        // Disable cache for this page
        flag!(disable_cache, 4);
        // Bits 5-6 are used only by the CPU
        // Bit 5 is the accessed bit, and is set by the cpu
        // when this entry is accessed.
        // Bit 6 is the dirty bit, and is set by the cpu
        // when a write on this page occurs
        // Marks big pages blocks
        flag!(huge_page, 7);
        // Page isn't flushed from caches on address space switch//
        // (PGE bit of CR4 register must be set)
        flag!(global, 8);
        // Bit 9-11 and also 52-62
        // are available and can be used by the OS to any purpose.
        // This page is holding data and is not executable
        flag!(not_executable, 63);
    };
}
```

