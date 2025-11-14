# Chapter 1

```rust,fp=main.rs,icon=@https://icons.veryicon.com/png/o/business/vscode-program-item-icon/assembly-7.png
impl DiskAddressPacket {
    pub fn load(&self, disk_number: u8) {
        unsafe {
            // This is an inline assembly block
            // This block's assembly will be injected to the function.
            asm!(
                // si register is required for llvm it's content needs to be saved
                "push si",
                // Set the packet address in `si` and format it for a 16bit register
                "mov si, {0:x}",
                // Put function code in `ah`
                "mov ah, {1}",
                // Put disk number in `dl`
                "mov dl, {2}",
                // Call the `disk interrupt`
                "int {3}",
                // Restore si for llvm internal use.
                "pop si",
                inout(reg) self as *const Self as u16,
                const Disk::ExtendedRead as u8,
                out(reg_byte) disk_number,
                const BiosInterrupts::DISK as u8,
            );
        }
    }
}

macro_rules! table_entry_flags {
    () => {
        // Is this page present?
        common::flag!(present, 0);

        // Is this page writable?
        common::flag!(writable, 1);

        // Can this page be accessed from user mode
        common::flag!(usr_access, 2);

        // Writes go directly to memory
        common::flag!(write_through_cache, 3);

        // Disable cache for this page
        common::flag!(disable_cache, 4);

        // This flag can help identifying if an entry is the
        // last one, or it is pointing to another directory
        // Is this page points to a custom memory address
        // and not a page table?
        common::flag!(huge_page, 7);

        // Page isn't flushed from caches on address space
        // switch (PGE bit of CR4 register must be set)
        common::flag!(global, 8);

        // mark a table as full
        common::flag!(full, 9);

        // This entry points to a table
        common::flag!(table, 10);

        // This entry is at the top of the hierarchy.
        common::flag!(root_entry, 11);

        // This page is holding data and is not executable
        common::flag!(not_executable, 63);
    };
}

```
