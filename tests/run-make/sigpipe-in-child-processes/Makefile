# ignore-cross-compile because we run the compiled code
# only-unix

# See main.rs for a description of this test.

include ../tools.mk

all:
	$(RUSTC) main.rs -o $(TMPDIR)/main
	$(RUSTC) assert-sigpipe-disposition.rs -o $(TMPDIR)/assert-sigpipe-disposition
	$(TMPDIR)/main $(TMPDIR)/assert-sigpipe-disposition
