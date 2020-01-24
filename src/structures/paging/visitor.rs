//! Visitor for page tables.

use crate::structures::paging::{
    frame::PhysFrame,
    page::Size4KiB,
    page_table::{FrameError, PageTableEntry},
    Page, PageTable,
};

/// Visits all entries and levels of a page table heirarchy.
pub trait PageTableVisit: Sized {
    fn get_page(&mut self, paddr: PhysFrame) -> Page<Size4KiB>;

    fn visit_pml4(&mut self, pml4: &PageTable) {
        visit_pml4(self, pml4)
    }

    fn visit_pml4_entry(&mut self, entry: &PageTableEntry) {
        visit_pml4_entry(self, entry)
    }

    fn visit_pdpt(&mut self, pdpt: &PageTable) {
        visit_pdpt(self, pdpt)
    }

    fn visit_pdpt_entry(&mut self, entry: &PageTableEntry) {
        visit_pdpt_entry(self, entry)
    }

    fn visit_pd(&mut self, pd: &PageTable) {
        visit_pd(self, pd)
    }

    fn visit_pd_entry(&mut self, entry: &PageTableEntry) {
        visit_pd_entry(self, entry)
    }

    fn visit_pt(&mut self, pt: &PageTable) {
        visit_pt(self, pt)
    }

    fn visit_pt_entry(&mut self, entry: &PageTableEntry) {
        visit_pt_entry(self, entry)
    }
}

pub fn visit_pml4<V: PageTableVisit>(visitor: &mut V, pml4: &PageTable) {
    pml4.iter()
        .for_each(|entry| visitor.visit_pml4_entry(entry))
}

pub fn visit_pml4_entry<V: PageTableVisit>(visitor: &mut V, entry: &PageTableEntry) {
    match entry.frame() {
        Ok(frame) => {
            let pdpt_page = visitor.get_page(frame);
            let pdpt = unsafe { &*pdpt_page.start_address().as_ptr() };
            visitor.visit_pdpt(pdpt)
        }
        Err(FrameError::HugeFrame) => unreachable!(), // 512GB pages! Not yet :P
        Err(FrameError::FrameNotPresent) => {}
    }
}

pub fn visit_pdpt<V: PageTableVisit>(visitor: &mut V, pdpt: &PageTable) {
    pdpt.iter()
        .for_each(|entry| visitor.visit_pdpt_entry(entry))
}

pub fn visit_pdpt_entry<V: PageTableVisit>(visitor: &mut V, entry: &PageTableEntry) {
    match entry.frame() {
        Ok(frame) => {
            let pd_page = visitor.get_page(frame);
            let pd = unsafe { &*pd_page.start_address().as_ptr() };
            visitor.visit_pd(pd)
        }
        Err(FrameError::HugeFrame) => {} // 1GB page
        Err(FrameError::FrameNotPresent) => {}
    }
}

pub fn visit_pd<V: PageTableVisit>(visitor: &mut V, pd: &PageTable) {
    pd.iter().for_each(|entry| visitor.visit_pd_entry(entry))
}

pub fn visit_pd_entry<V: PageTableVisit>(visitor: &mut V, entry: &PageTableEntry) {
    match entry.frame() {
        Ok(frame) => {
            let pt_page = visitor.get_page(frame);
            let pt = unsafe { &*pt_page.start_address().as_ptr() };
            visitor.visit_pd(pt)
        }
        Err(FrameError::HugeFrame) => {} // 2MB page
        Err(FrameError::FrameNotPresent) => {}
    }
}

pub fn visit_pt<V: PageTableVisit>(visitor: &mut V, pt: &PageTable) {
    pt.iter().for_each(|entry| visitor.visit_pt_entry(entry))
}

pub fn visit_pt_entry<V: PageTableVisit>(visitor: &mut V, entry: &PageTableEntry) {
    match entry.frame() {
        Ok(frame) => {}                               // 4KB page
        Err(FrameError::HugeFrame) => unreachable!(), // Not huge any more...
        Err(FrameError::FrameNotPresent) => {}
    }
}

/// Mutably visits all entries and levels of a page table heirarchy.
pub trait PageTableVisitMut: Sized {
    fn get_vaddr(&mut self, paddr: PhysAddr) -> VirtAddr;

    fn visit_pml4(&mut self, pml4: &mut PageTable) {
        visit_mut_pml4(self, pml4)
    }

    fn visit_pml4_entry(&mut self, entry: &mut PageTableEntry) {
        visit_mut_pml4_entry(self, entry)
    }

    fn visit_pdpt(&mut self, pdpt: &mut PageTable) {
        visit_mut_pdpt(self, pdpt)
    }

    fn visit_pdpt_entry(&mut self, entry: &mut PageTableEntry) {
        visit_mut_pdpt_entry(self, entry)
    }

    fn visit_pd(&mut self, pd: &mut PageTable) {
        visit_mut_pd(self, pd)
    }

    fn visit_pd_entry(&mut self, entry: &mut PageTableEntry) {
        visit_mut_pd_entry(self, entry)
    }

    fn visit_pt(&mut self, pt: &mut PageTable) {
        visit_mut_pt(self, pt)
    }

    fn visit_pt_entry(&mut self, entry: &mut PageTableEntry) {
        visit_mut_pt_entry(self, entry)
    }
}

pub fn visit_mut_pml4<V: PageTableVisitMut>(visitor: &mut V, pml4: &mut PageTable) {
    pml4.iter_mut()
        .for_each(|entry| visitor.visit_pml4_entry(entry))
}

pub fn visit_mut_pml4_entry<V: PageTableVisitMut>(visitor: &mut V, entry: &mut PageTableEntry) {
    match entry.frame() {
        Ok(frame) => {
            let pdpt_vaddr = visitor.get_vaddr(frame.start_address());
            let pdpt = unsafe { &mut *pdpt_vaddr.as_mut_ptr() };
            visitor.visit_pdpt(pdpt)
        }
        Err(FrameError::HugeFrame) => unreachable!(), // 512GB pages! Not yet :P
        Err(FrameError::FrameNotPresent) => {}
    }
}

pub fn visit_mut_pdpt<V: PageTableVisitMut>(visitor: &mut V, pdpt: &mut PageTable) {
    pdpt.iter_mut()
        .for_each(|entry| visitor.visit_pdpt_entry(entry))
}

pub fn visit_mut_pdpt_entry<V: PageTableVisitMut>(visitor: &mut V, entry: &mut PageTableEntry) {
    match entry.frame() {
        Ok(frame) => {
            let pd_vaddr = visitor.get_vaddr(frame.start_address());
            let pd = unsafe { &mut *pd_vaddr.as_mut_ptr() };
            visitor.visit_pd(pd)
        }
        Err(FrameError::HugeFrame) => {} // 1GB page
        Err(FrameError::FrameNotPresent) => {}
    }
}

pub fn visit_mut_pd<V: PageTableVisitMut>(visitor: &mut V, pd: &mut PageTable) {
    pd.iter_mut()
        .for_each(|entry| visitor.visit_pd_entry(entry))
}

pub fn visit_mut_pd_entry<V: PageTableVisitMut>(visitor: &mut V, entry: &mut PageTableEntry) {
    match entry.frame() {
        Ok(frame) => {
            let pt_vaddr = visitor.get_vaddr(frame.start_address());
            let pt = unsafe { &mut *pt_vaddr.as_mut_ptr() };
            visitor.visit_pd(pt)
        }
        Err(FrameError::HugeFrame) => {} // 2MB page
        Err(FrameError::FrameNotPresent) => {}
    }
}

pub fn visit_mut_pt<V: PageTableVisitMut>(visitor: &mut V, pt: &mut PageTable) {
    pt.iter_mut()
        .for_each(|entry| visitor.visit_pt_entry(entry))
}

pub fn visit_mut_pt_entry<V: PageTableVisitMut>(visitor: &mut V, entry: &mut PageTableEntry) {
    match entry.frame() {
        Ok(frame) => {}                               // 4KB page
        Err(FrameError::HugeFrame) => unreachable!(), // Not huge any more...
        Err(FrameError::FrameNotPresent) => {}
    }
}
