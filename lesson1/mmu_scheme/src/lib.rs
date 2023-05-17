#![no_std]
#![feature(asm_const)]

#[cfg(all(feature = "sv39", feature = "sv48"))]
compile_error!();
#[cfg(not(any(feature = "sv39", feature = "sv48")))]
compile_error!();

use riscv::register::satp;

// 1111111111111111 111111111 111111111 000000000 000000000 000000000000
const KERNEL_BASE_PAGE: usize = 0x_ffff_ffff_c000_0___;
pub const KERNEL_BASE: usize = KERNEL_BASE_PAGE << BITS_PAGE_OFFSET;
// pub const KERNEL_BASE: usize = 0xffff_ffff_c000_0000;

// 1111111111111111 111111111 100000000 000000000 000000000 000000000000
const VIRTUAL_BASE_OFFSET_PAGE: usize = 0xffff_ffc0_0000_0___;
const PHYS_VIRT_OFFSET: usize = VIRTUAL_BASE_OFFSET_PAGE << BITS_PAGE_OFFSET;

/// Length in bits of bytes offset in the page
// 4096 bytes per page
const BITS_PAGE_OFFSET: usize = 12;

/// Length in bits of each level of page index.
// 3 levels in SV39, 4 levels in SV48
const BITS_PAGE_INDEX: usize = 9;

/// Length in bits of page table entries flags.
const BITS_PTE_FLAGS: usize = 10;

/// _ _ Dirty Accessed Global User eXecute Write Read Valid
type Flag = u64;

/// Length in bits of each page table entry.
const BITS_PTE: usize = 64;

// Page table entry
type PTE = u64;

type PageNumber = u64;

// Count of page table entries in each page of page table
const COUNT_PTE: usize = ((1 << BITS_PAGE_OFFSET) * 8) / BITS_PTE;

unsafe fn get_page_number(pt: &PageTable) -> PageNumber {
    let page_table_root: PageNumber = pt.as_ptr() as PageNumber;
    let page_table_root_page: PageNumber = page_table_root >> BITS_PAGE_OFFSET;
    page_table_root_page
}

const fn to_page_table_entry(page: PageNumber, flag: Flag) -> PTE {
    page << BITS_PTE_FLAGS | flag
}

type PageTable = [PTE; COUNT_PTE];

#[cfg(feature = "sv39")]
#[link_section = ".data.boot_page_table"]
// 64 bits per entry
static mut BOOT_PT_SV39: PageTable = [0; COUNT_PTE];

#[cfg(feature = "sv39")]
const fn full_page_index_sv39(i1: u64, i2: u64, i3: u64) -> PageNumber {
    (i1 << BITS_PAGE_INDEX | i2) << BITS_PAGE_INDEX | i3
}

#[cfg(feature = "sv39")]
pub unsafe fn pre_mmu() {
    // Set Page Table Entries
    const FLAG: Flag = 0b11101111;
    const PTE: PTE = full_page_index_sv39(2, 0, 0) << BITS_PTE_FLAGS | FLAG;
    const _PTE: PTE = (0x80000 << 10) | 0xef;
    // 0x8000_0000..0xc000_0000
    // VRWX_GAD
    // 1G block
    BOOT_PT_SV39[2] = PTE;
    // KERNEL_BASE
    // 0xffff_ffc0_8000_0000..0xffff_ffc0_c000_0000, VRWX_GAD, 1G block
    BOOT_PT_SV39[0x102] = PTE;
    // PHYS_VIRT_OFFSET
    // 0xffff_ffff_c000_0000..highest, VRWX_GAD, 1G block
    BOOT_PT_SV39[0x1ff] = PTE;
}

#[cfg(feature = "sv39")]
pub unsafe fn enable_mmu() {
    satp::set(satp::Mode::Sv39, 0, get_page_number(&BOOT_PT_SV39) as usize);
    riscv::asm::sfence_vma_all();
}

#[cfg(feature = "sv48")]
struct PageTableSV48 {
    root: PageTable,
    _0: PageTable,
    _1: PageTable,
}

#[cfg(feature = "sv48")]
#[link_section = ".data.boot_page_table"]
// 64 bits per entry
static mut BOOT_PT_SV48: PageTableSV48 = PageTableSV48 {
    root: [0; COUNT_PTE],
    _0: [0; COUNT_PTE],
    _1: [0; COUNT_PTE],
};

#[cfg(feature = "sv48")]
const fn full_page_index_sv48(i1: u64, i2: u64, i3: u64, i4: u64) -> PageNumber {
    ((i1 << BITS_PAGE_INDEX | i2) << BITS_PAGE_INDEX | i3) << BITS_PAGE_INDEX | i4
}

#[cfg(feature = "sv48")]
pub unsafe fn pre_mmu() {
    // Set Page Table Entries
    const FLAG: Flag = 0b11101111;
    const PTE: PTE = to_page_table_entry(full_page_index_sv48(0, 2, 0, 0), FLAG);
    const _PTE: PTE = (0x80000 << 10) | 0xef;
    const P_FLAG: Flag = 0x00000001;

    // 000000000
    BOOT_PT_SV48.root[0x000] = to_page_table_entry(get_page_number(&BOOT_PT_SV48._0), P_FLAG);
    // 111111111
    BOOT_PT_SV48.root[0x1ff] = to_page_table_entry(get_page_number(&BOOT_PT_SV48._1), P_FLAG);
    // 000000010
    BOOT_PT_SV48._0[0x002] = PTE;
    // KERNEL_BASE
    // 100000010
    BOOT_PT_SV48._1[0x102] = PTE;
    // PHYS_VIRT_OFFSET
    // 111111111
    BOOT_PT_SV48._1[0x1ff] = PTE;
}

#[cfg(feature = "sv48")]
pub unsafe fn enable_mmu() {
    satp::set(
        satp::Mode::Sv48,
        0,
        get_page_number(&BOOT_PT_SV48.root) as usize,
    );
    riscv::asm::sfence_vma_all();
}

pub unsafe fn post_mmu() {
    core::arch::asm!("
        li      t0, {phys_virt_offset}  // fix up virtual high address
        add     sp, sp, t0              // convert stack pointer to virtual address
        add     ra, ra, t0              // convert return address to virtual address
        ret     ",
        phys_virt_offset = const PHYS_VIRT_OFFSET,
    )
}
