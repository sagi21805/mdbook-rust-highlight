
```hlrs,fp=kernel/stages/first_stage/src/main.rs

// Static variable that holds our table
static GLOBAL_DESCRIPTOR_TABLE: GlobalDescriptorTable = {
    GlobalDescriptorTable::protected_mode()
};
pub fn first_stage() -> ! {

    // Load Global Descriptor Table
    GLOBAL_DESCRIPTOR_TABLE.load();

    // Set the Protected Mode bit in control register 0
    asm!(
        "mov eax, cr0",
        "or eax, 1",
        "mov cr0, eax",
        options(readonly, nostack, preserves_flags)
    );

    // Jump to the next stage
    // We perform a long jump, which is a jump that also loads our segment
    // from the global descriptor table.
    //
    // The segment is the offset in the global descriptor table
    // which for the code segment is 0x10 (For readability, added an enum)
    //
    // The `next_stage` is the address of the next stage
    // which is a variable in the constants.
    //
    // I want to think for yourselves what it value should be.
    // As always, the answer, i.e var that I chose, will be in the Walkthrough
    asm!(
        "jmp ${section}, ${next_stage}",
        section = const Sections::KernelCode as u8,
        next_stage = const SECOND_STAGE_OFFSET,
    );
}
```

```hlrs,fp=kernel\stages\first_stage\src\main.rs
#[unsafe(no_mangle)]
pub fn first_stage() -> ! {
    // Read the disk number the os was booted from
    let disk_number = unsafe { core::ptr::read(DISK_NUMBER_OFFSET as *const u8) };

    // Create a disk packet which will load 128 sectors (512 bytes each)
    // from the disk to memory address 0x7e00
    // The address 0x7e00 was chosen because it is exactly one sector
    //  after the initial address 0x7c00.
    let dap = DiskAddressPacket::new(
        128,    // Number of sectors
        0,      // Memory address
        0x7e0,  // Memory segment
        1,      // Starting LBA address (LBA 0 was already loaded by the BIOS)
    );
    dap.load(disk_number);
}
```hlrs,fp=shared/cpu_utils/src/structures/paging/page_table.rs

impl PageTable {
    pub unsafe fn empty_from_ptr(page_table_ptr: VirtualAddress) -> Option<&'static mut PageTable> {
        // Check if the address is in the correct alignment
        if !page_table_ptr.is_aligned(REGULAR_PAGE_ALIGNMENT) {
            return None;
        }
        unsafe {
            // Zero out all the entries and return as mut ptr
            ptr::write_volatile(
                page_table_ptr.as_mut_ptr::<PageTable>(),
                PageTable::empty()
            );
            return Some(&mut *page_table_ptr.as_mut_ptr::<PageTable>());
        }
    }
}
```
```hlrs,fp=shared/cpu_utils/src/structures/paging/page_table.rs
#[repr(align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; PAGE_DIRECTORY_ENTRIES],
}

impl PageTable {
    #[inline]
    pub const fn empty() -> Self {
        Self {
            entries: [const { PageTableEntry::empty() }; PAGE_DIRECTORY_ENTRIES]
        }
    }

}
```
```hlrs,fp=shared/cpu_utils/src/structures/paging/page_table_entry.rs

impl PageTableEntry {
    /// Set all of the flags to zero.
    pub const fn reset_flags(&mut self) {
        self.0 &= ENTRY_ADDRESS_MASK;
    }

    /// Set the flags without a reset to previous flags.
    pub const unsafe fn set_flags_unchecked(&mut self, flags: PageEntryFlags) {
        self.0 |= flags.as_u64()
    }

    /// Set the flags of the entry
    pub const fn set_flags(&mut self, flags: PageEntryFlags) {
        self.reset_flags();
        unsafe { self.set_flags_unchecked(flags) };
    }

    /// Map the frame address into the entry and also set the flags.
    pub const unsafe fn map_unchecked(&mut self, frame: PhysicalAddress, flags: PageEntryFlags) {
        *self = Self::empty();
        unsafe { self.set_flags_unchecked(flags) };
        self.set_present();
        // Set the new address
        self.0 |= frame.as_usize() as u64 & ENTRY_ADDRESS_MASK;
    }

    /// Same as map unchecked, but checking that the entry is not used
    /// and also that the address is aligned
    ///
    /// This is still not a safe function,
    /// See walkthrough documentation for more details
    pub const unsafe fn map(&mut self, frame: PhysicalAddress, flags: PageEntryFlags) {
        if !self.is_present() && frame.is_aligned(REGULAR_PAGE_ALIGNMENT) {
            unsafe { self.map_unchecked(frame, flags) };
        }
    }
}
```
```hlrs,fp=shared/cpu_utils/src/structures/paging/page_table_entry.rs
impl PageTableEntry {

    /// Extract the address from the entry and return it without checking flags
    pub const unsafe fn mapped_unchecked(&self) -> PhysicalAddress {
        unsafe {
            PhysicalAddress::new_unchecked(
                (self.0 & ENTRY_ADDRESS_MASK) as usize
            )
        }
    }
    /// Return the physical address that is mapped by this entry while checking flags
    pub fn mapped(&self) -> Result<PhysicalAddress, EntryError> {
        if self.is_present() {
            unsafe { Ok(self.mapped_unchecked()) }
        } else {
            Err(EntryError::NoMapping)
        }
    }
    /// Return the physical address mapped by this table as a reference into a page table.
    pub fn mapped_table(&self) -> Result<&PageTable, EntryError> {
        // first check if the entry is mapped.
        let table = unsafe { &*self.mapped()?.translate().as_ptr::<PageTable>() };
        // then check if it is a table.
        if self.is_huge_page() && self.is_table() {
            Ok(table)
        } else {
            Err(EntryError::NotATable)
        }
    }
    // Another `mapped_table_mut` is implemented
    // This is the same functions, just with a mut reference on return
}
```

```hlrs,fp=shared/cpu_utils/src/structures/paging/entry_flags.rs
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
        //
        // Bit 5 is the accessed bit, and is set by the cpu
        // when this entry is accessed.
        //
        // Bit 6 is the dirty bit, and is set by the cpu
        // when a write on this page occurs

        // Marks big pages blocks
        flag!(huge_page, 7);

        // Page isn’t flushed from caches on address space switch
        // (PGE bit of CR4 register must be set)
        flag!(global, 8);

        // Bit 9-11 and also 52-62
        // are available and can be used by the OS to any purpose.

        // This page is holding data and is not executable
        flag!(not_executable, 63);
    };
}
```
