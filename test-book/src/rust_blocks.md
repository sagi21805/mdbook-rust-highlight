```hlrs,fp=shared/cpu_utils/src/structures/global_descriptor_table.rs
struct AccessByte(u8);

struct LimitFlags(u8);

// The 32 flags that it for a 32bit table
// A 64bit table have a different structure
#[repr(C)]
struct GlobalDescriptorTableEntry32 {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access_byte: AccessByte,
    // Low 4 bits limit_high
    // high 4 bits flags
    limit_flags: LimitFlags,
    base_high: u8,
}
```
```hlrs, fp=shared\common\src\macros.rs
#[macro_export]
/// This macro will obtain `flag_name` and the corresponding `bit_number`
///
/// With this information it will automatically generate three methods
///
/// 1. `set_<flag_name>`: set the bit without returning self
/// 2. `<flag_name>`: set the bit and will return self
/// 3. `unset_<flag_name>:` unset the bit without returning self
/// 4. `is_<flag_name>`: return true if the flag is set or false if not
macro_rules! flag {
    ($flag_name:ident, $bit_number:literal) => {
        #[inline]
        #[allow(dead_code)]
        #[allow(unused_attributes)]
        /// Sets the corresponding flag
        ///
        /// `This method is auto-generated`
        pub const fn ${concat(set_, $flag_name)}(&mut self) {
            self.0 |= 1 << $bit_number;
        }

        #[inline]
        #[allow(dead_code)]
        #[allow(unused_attributes)]
        /// Sets the corresponding flag while returning self
        ///
        /// `This method is auto-generated`
        pub const fn $flag_name(self) -> Self {
            Self(self.0 | (1 << $bit_number))
        }

        #[inline]
        #[allow(dead_code)]
        #[allow(unused_attributes)]
        /// Unset the corresponding flag
        ///
        /// `This method is auto-generated`
        pub const fn ${concat(unset_, $flag_name)}(&mut self) {
            self.0 &= !(1 << $bit_number)
        }

        /// Checks if the corresponding flag in set to 1
        ///
        /// `This method is auto-generated`
        #[inline]
        #[allow(dead_code)]
        #[allow(unused_attributes)]
        pub const fn ${concat(is_, $flag_name)}(&self) -> bool {
            self.0 & (1 << $bit_number) != 0
        }
    };
}
```
```hlrs
struct Example(u8);

impl Example {
    flag!(first, 1);
    flag!(second, 2);
    flag!(third, 3);
}
```
```hlrs
struct Example(u8);
impl Example {
    #[inline]
    #[allow(dead_code)]
    #[allow(unused_attributes)]
    /// Sets the corresponding flag
    ///
    /// `This method is auto-generated`
    pub const fn set_first(&mut self) {
        self.0 |= 1 << 1;
    }
    #[inline]
    #[allow(dead_code)]
    #[allow(unused_attributes)]
    /// Sets the corresponding flag while returning self
    ///
    /// `This method is auto-generated`
    pub const fn first(self) -> Self {
        Self(self.0 | (1 << 1))
    }
    #[inline]
    #[allow(dead_code)]
    #[allow(unused_attributes)]
    /// Unset the corresponding flag
    ///
    /// `This method is auto-generated`
    pub const fn unset_first(&mut self) {
        self.0 &= !(1 << 1);
    }
    /// Checks if the corresponding flag in set to 1
    ///
    /// `This method is auto-generated`
    #[inline]
    #[allow(dead_code)]
    #[allow(unused_attributes)]
    pub const fn is_first(&self) -> bool {
        self.0 & (1 << 1) != 0
    }
    #[inline]
    #[allow(dead_code)]
    #[allow(unused_attributes)]
    /// Sets the corresponding flag
    ///
    /// `This method is auto-generated`
    pub const fn set_second(&mut self) {
        self.0 |= 1 << 2;
    }
    #[inline]
    #[allow(dead_code)]
    #[allow(unused_attributes)]
    /// Sets the corresponding flag while returning self
    ///
    /// `This method is auto-generated`
    pub const fn second(self) -> Self {
        Self(self.0 | (1 << 2))
    }
    #[inline]
    #[allow(dead_code)]
    #[allow(unused_attributes)]
    /// Unset the corresponding flag
    ///
    /// `This method is auto-generated`
    pub const fn unset_second(&mut self) {
        self.0 &= !(1 << 2);
    }
    /// Checks if the corresponding flag in set to 1
    ///
    /// `This method is auto-generated`
    #[inline]
    #[allow(dead_code)]
    #[allow(unused_attributes)]
    pub const fn is_second(&self) -> bool {
        self.0 & (1 << 2) != 0
    }
    #[inline]
    #[allow(dead_code)]
    #[allow(unused_attributes)]
    /// Sets the corresponding flag
    ///
    /// `This method is auto-generated`
    pub const fn set_third(&mut self) {
        self.0 |= 1 << 3;
    }
    #[inline]
    #[allow(dead_code)]
    #[allow(unused_attributes)]
    /// Sets the corresponding flag while returning self
    ///
    /// `This method is auto-generated`
    pub const fn third(self) -> Self {
        Self(self.0 | (1 << 3))
    }
    #[inline]
    #[allow(dead_code)]
    #[allow(unused_attributes)]
    /// Unset the corresponding flag
    ///
    /// `This method is auto-generated`
    pub const fn unset_third(&mut self) {
        self.0 &= !(1 << 3);
    }
    /// Checks if the corresponding flag in set to 1
    ///
    /// `This method is auto-generated`
    #[inline]
    #[allow(dead_code)]
    #[allow(unused_attributes)]
    pub const fn is_third(&self) -> bool {
        self.0 & (1 << 3) != 0
    }
}
```
```hlrs,fp=shared/cpu_utils/src/structures/global_descriptor_table.rs
impl AccessByte {
    /// Creates an access byte with all flags turned off.
    pub const fn new() -> Self {
        Self(0)
    }

    // Is this a valid segment?
    // for all active segments this should be turned on.
    flag!(present, 7);

    /// Sets the privilege level while returning self.
    /// This is corresponding to the cpu ring of this segment
    /// 0 is commonly called kernel mode, 4 is commonly called user mode
    pub const fn dpl(mut self, level: u8) -> Self {
        self.0 |= (level & 0x3) << 5;
        self
    }
    // Is this a code / data segment or a system segment.
    flag!(code_or_data, 4);
    // Will this segment contains executable code?
    flag!(executable, 3);
    // Will the segment grow downwards?
    // relevant for non executable segments
    flag!(direction, 2);
    // Can this code be executed from lower privilege segments.
    // relevant to executable segments
    flag!(conforming, 2);
    // Can this segment be read or it is only executable?
    // relevant for code segment
    flag!(readable, 1);
    // Is this segment writable?
    // relevant for data segments
    flag!(writable, 1);
}

impl LimitFlags {
    /// Creates a default limit flags with all flags turned off.
    pub const fn new() -> Self {
        Self(0)
    }
    // Toggle on paging for this segment (limit *= 0x1000)
    flag!(granularity, 7);
    // Is this segment going to use 32bit mode?
    flag!(protected, 6);
    // Set long mode flag, this will also clear protected mode
    flag!(long, 5);
}
```
```hlrs,fp=shared/cpu_utils/src/structures/global_descriptor_table.rs
impl GlobalDescriptorTableEntry32 {
    pub const fn new(
        base: u32,
        limit: u32,
        access_byte: AccessByte,
        flags: LimitFlags
    ) -> Self {

        // Split base into the appropriate parts
        let base_low = (base & 0xffff) as u16;
        let base_mid = ((base >> 0x10) & 0xff) as u8;
        let base_high = ((base >> 0x18) & 0xff) as u8;
        // Split limit into the appropriate parts
        let limit_low = (limit & 0xffff) as u16;
        let limit_high = ((limit >> 0x10) & 0xf) as u8;
        // Combine the part of the limit size with the flags
        let limit_flags = flags.0 | limit_high;
        Self {
            limit_low,
            base_low,
            base_mid,
            access_byte,
            limit_flags: LimitFlags(limit_flags),
            base_high,
        }
    }
}
```
```hlrs,fp=shared/cpu_utils/src/structures/global_descriptor_table.rs
// This structure will seem as `dead code`
// this is because we only initialize it
// and don't use the fields directly
// to remove the warning, we add the following attribute.
#[allow(dead_code)]
pub struct GlobalDescriptorTable {
    null: GlobalDescriptorTableEntry32,
    code: GlobalDescriptorTableEntry32,
    data: GlobalDescriptorTableEntry32,
}

impl GlobalDescriptorTable {
    /// Creates default global descriptor table for protected mode
    pub const fn protected_mode() -> Self {
        GlobalDescriptorTable {
            // Null entry, fields with zeros.
            null: GlobalDescriptorTableEntry32::new(
                0,
                0,
                AccessByte::new(),
                LimitFlags::new()
            ),
            code: GlobalDescriptorTableEntry32::new(
                // The base is zero, because our code is aligned to 0x0 address
                0,
                // The size is max, so we won't have any limit
                0xfffff,
                // We mark this as code segment, with the highest privileges
                AccessByte::new()
                    .present()
                    .dpl(0)
                    .code_or_data()
                    .executable()
                    .readable(),
                // Set the units of the limit to 4kib and set 32bit mode.
                LimitFlags::new()
                    .granularity()
                    .protected(),
            ),
            data: GlobalDescriptorTableEntry32::new(
                // The base is zero, because our data is aligned to 0x0 address
                0,
                // The size is max, so we won't have any limit
                0xfffff,
                // We mark this as code segment, with the highest privileges
                AccessByte::new()
                    .present()
                    .dpl(0)
                    .code_or_data()
                    .writable(),
                // Set the units of the limit to 4kib and set 32bit mode.
                LimitFlags::new()
                    .granularity()
                    .protected(),
            ),
        }
    }
}
```
```hlrs,fp=shared/cpu_utils/src/structures/global_descriptor_table.rs
// The packed and repr(C) attributes are very important.
// The repr(C) ensures the order of the data is as specified.
// The packed attribute will ignore `Data Structure Alignment`
#[repr(C, packed(2))]
pub struct GlobalDescriptorTableRegister32 {
    // This is the size of our table in bytes - 1.
    pub limit: u16,
    // This is the address of where we store the table.
    pub base: *const GlobalDescriptorTable,
}

impl GlobalDescriptorTable {

    pub unsafe fn load(&'static self) {
        let global_descriptor_table_register = {
            GlobalDescriptorTableRegister32 {
                // Set the limit to the size - 1
                limit: (size_of::<GlobalDescriptorTable>() - 1) as u16,
                // Set the base to the address of the table
                // (This is the global address of the var because it is static)
                base: self as *const GlobalDescriptorTable,
            }
        };
        unsafe {
            asm!(
                // Clear Interrupt Flag.
                // This is done because we can't let random hardware interrupts
                // to interfere with the lgdt instruction.
                // This will be useful in the future until we set up interrupts
                "cli",
                // Then, load the table using our now created register.
                "lgdt [{}]",
                in(reg) &global_descriptor_table_register,
                options(readonly, nostack, preserves_flags)
            );
        }
    }
}
```
```hlrs,fp=main.rs
use core::arch::asm;

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
```hlrs,fp=main.rs
fn main() {
    println!("Hello World!");
}
```

```hlrs,fp=<rust-doc>core/panic/panic_info.rs

pub struct PanicInfo<'a> {
    message: &'a fmt::Arguments<'a>,
    location: &'a Location<'a>,
    can_unwind: bool,
    force_no_backtrace: bool,
}
```
```hlrs,fp=main.rs
#![no_std]
fn main() {

}

#[panic_handler]
pub fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
```

```hlrs,fp=build.rs
use std::path::Path;

fn main() {
    // Environment variable that stores the current working directory
    let local_path = Path::new(env!("CARGO_MANIFEST_DIR"));

    // This tells cargo to add the `-C link-arg=--script=./linker.ld` argument.
    // Which will result in linking with our code with our linker script
    println!(
        "cargo:rustc-link-arg-bins=--script={}",
        local_path.join("linker.ld").display()
    )
}
```
```hlrs
struct A(u32);

impl A {
    pub fn new(a: u32) -> A {
        A(a)
    }
}

struct B(u32);

impl B {
    pub fn new(b: u32) -> B {
        B(b)
    }
}
```
```hlrs,fp=main.rs
#![no_std]
#![no_main]

#[unsafe(no_mangle)]
fn main() {
}

#[panic_handler]
pub fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
```
```hlrs,fp=kernel/stages/first_stage/src/disk.rs

// The `repr(C)` means that the layout in memory will be as specified
// because rust ABI doesn't state that this is promised.
//
// The `repr(Packed) states that there will no padding due to alignment
#[repr(C, packed)]
pub struct DiskAddressPacket {
    /// The size of the packet
    packet_size: u8,

    /// Zero
    zero: u8,

    /// How many sectors to read
    num_of_sectors: u16,

    /// Which address in memory to save the data
    memory_address: u16,

    /// Memory segment for the address
    segment: u16,

    /// The LBA address of the first sector
    abs_block_num: u64,
}
```
```hlrs,fp=shared\common\src\enums\bios_interrutps.rs
#[repr(u8)]
// This enum will hold all of our BIOS interrupts numbers
pub enum BiosInterrupts {
    DISK = 0x13,
}

// This enum will hold the specific functions for the disk interrupt (int 0x13)
#[repr(u8)]
pub enum Disk {
    ExtendedRead = 0x42,
}
```
```hlrs,fp=kernel\stages\first_stage\src\disk.rs
impl DiskAddressPacket {
    pub fn new(
        num_of_sectors: u16,
        memory_address: u16,
        segment: u16,
        abs_block_num: u64
    ) -> Self {
        Self {
            // The size of the packet
            packet_size: size_of::<Self>() as u8,
            // zero
            zero: 0,
            // Number of sectors to read, this can be a max of 128 sectors.
            // This is because the address increments every time we read a sector.
            // The largest number a register in this mode can hold is 2^16
            // When divided by a sector size, we get that we can read only 128 sectors.
            num_of_sectors: num_of_sectors.min(128),
            // The initial memory address
            memory_address,
            // The segment the memory address is in
            segment,
            // The starting LBA address to read from
            abs_block_num,
        }
    }
}
```
```hlrs,fp=kernel\stages\first_stage\src\disk.rs
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
                in(reg) self as *const Self as u16,
                const Disk::ExtendedRead as u8,
                in(reg_byte) disk_number,
                const BiosInterrupts::DISK as u8,
            )
        }
    }
}
```
```hlrs,fp=shared\common\src\constants\addresses.rs
#[cfg(feature = "first_stage")]
pub const DISK_NUMBER_OFFSET: u16 = 0x7BFE;
```

```

```hlrs,fp=shared/cpu_utils/src/structures/paging/page_table_entry.rs
pub struct PageTableEntry(u64);

impl PageTableEntry {
    #[inline]
    pub(crate) const fn empty() -> Self {
        Self(0)
    }

    table_entry_flags!();
}
```
```hlrs,fp=shared/common/src/macros.rs
macro_rules! impl_common_address_functions {
    ($struct_name:ident) => {
#[allow(non_snake_case)]
mod ${concat(__impl_for_, $struct_name)} {
    use super::*;
    use core::ptr::Alignment;
    impl $struct_name {
        pub const unsafe fn new_unchecked(address: usize) -> Self {
            Self(address)
        }
        pub const fn as_usize(&self) -> usize {
            self.0
        }
        pub const unsafe fn as_mut_ptr<T>(&self) -> *mut T {
            self.0 as *mut T
        }
        pub const fn as_ptr<T>(&self) -> *const T {
            self.0 as *const T
        }
        pub const fn is_aligned(&self, alignment: Alignment) -> bool {
            self.0 & (alignment.as_usize() - 1) == 0
        }
        pub const fn align_up(mut self, alignment: Alignment) -> Self {
            self.0 = (self.0 + (alignment.as_usize() - 1)) & !(alignment.as_usize() - 1);
            self
        }
        pub const fn align_down(mut self, alignment: Alignment) -> Self {
            self.0 &= !(alignment.as_usize() - 1);
            self
        }
        pub const fn alignment(&self) -> Alignment {
            unsafe { Alignment::new_unchecked(1 << self.0.trailing_zeros()) }
        }
    }
}
    };
}
```
```hlrs,fp=shared/common/src/address_types.rs
use derive_more::{
    Add, AddAssign, AsMut, AsRef, Div, DivAssign, From, Mul, MulAssign, Sub, SubAssign,
};

#[derive(
    Clone,
    Debug,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Default,
    AsMut,
    AsRef,
    From,
)]
pub struct PhysicalAddress(pub usize);

impl_common_address_functions!(PhysicalAddress);

#[derive(
    Clone,
    Debug,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Default,
    AsMut,
    AsRef,
    From,
)]
pub struct VirtualAddress(pub usize);

impl_common_address_functions!(VirtualAddress);
```
```hlrs,fp=shared/cpu_utils/src/structures/paging/entry_flags.rs
#[derive(Debug, Clone)]
pub struct PageEntryFlags(u64);

impl PageEntryFlags {

    // Same macro used on PageTableEntry for flags.
    table_entry_flags!();

    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn regular_page_flags() -> Self {
        PageEntryFlags::new().present().writable()
    }

    pub const fn as_u64(&self) -> u64 {
        self.0
    }
}
```
