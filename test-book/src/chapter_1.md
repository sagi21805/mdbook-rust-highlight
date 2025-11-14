# Chapter 1

```rust,fp=main.rs,icon=@https://icons.veryicon.com/png/o/business/vscode-program-item-icon/assembly-7.png

asm!(
    "mov eax, cr0",
    "or eax, 1",
    "mov cr0, eax",
    options(readonly, nostack, preserves_flags)
);
```
