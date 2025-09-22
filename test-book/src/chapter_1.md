# Chapter 1
```hlrs,fp=testing/testing.rs

fn test() {

}

fn test2() {
    let x = 4;
    x.max_size();
    self.test();
}

```

```hlrs
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

```hlrs,fp=main.rs
#[unsafe(no_mangle)]
fn main() {
    let msg = b"Hello, World!";
    for &ch in msg {
        unsafe {
            asm!(
                "mov ah, 0x0E",   // INT 10h function to print a char
                "mov al, {0}",    // The input ASCII char
                "int 0x10",       // Call the BIOS Interrupt Function
                // --- settings ---
                in(reg_byte) ch,  // {0} Will become the register with the char
                out("ax") _,      // Lock the 'ax' as output reg, so it won't be used elsewhere
            );
        }
    }

    unsafe {
        asm!("hlt"); // Halt the system
    }
}
```



```hlrs,fp=main.rs,icon=@https://icons.veryicon.com/png/o/business/vscode-program-item-icon/assembly-7.png


pub const async unsafe extern "C" fn function(a: b, mut c: D) -> TestType {}
```


```json
mov eax, ebx
```