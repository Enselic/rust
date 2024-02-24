# ignore-cross-compile because we run the compiled code
# only-unix

# See main.rs for a description of this test.

include ../tools.mk

all:
	$(RUSTC) assert-sigpipe-disposition.rs -o $(TMPDIR)/assert-sigpipe-disposition; \
	for revision in default sig_dfl sig_ign; do \
		$(RUSTC) main.rs --cfg $$revision -o $(TMPDIR)/main.$$revision; \
		$(TMPDIR)/main.$$revision $(TMPDIR)/assert-sigpipe-disposition; \
	done
