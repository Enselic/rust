# only-unix

include ../tools.mk

all:
	$(RUSTC) foo.rs --crate-type=lib --emit=obj=$(TMPDIR)/foo.o -Cpanic=abort --edition 2021 -Z validate-mir
	objdump --dwarf=frames $(TMPDIR)/foo.o | $(CGREP) -v 'DW_CFA'
