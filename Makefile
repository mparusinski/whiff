PROFILE=release
EXE=target/$(PROFILE)/whiff
prefix=/usr/local
bindir=$(prefix)/bin
datadir=$(prefix)/share
exe_name=whiff

$(EXE): Cargo.toml src/main.rs
	cargo build --profile $(PROFILE) --locked

.PHONY: completions
completions: autocomplete/whiff.bash autocomplete/whiff.fish autocomplete/_whiff.ps1 autocomplete/_whiff

comp_dir=@mkdir -p autocomplete

autocomplete/whiff.bash: $(EXE)
	$(comp_dir)
	$(EXE) --gen-completions bash > $@

autocomplete/whiff.fish: $(EXE)
	$(comp_dir)
	$(EXE) --gen-completions fish > $@

autocomplete/_whiff.ps1: $(EXE)
	$(comp_dir)
	$(EXE) --gen-completions powershell > $@

autocomplete/_whiff: $(EXE)
	$(comp_dir)
	$(EXE) --gen-completions zsh > $@

install: $(EXE) completions
	install -Dm755 $(EXE) $(DESTDIR)$(bindir)/whiff
	install -Dm644 autocomplete/whiff.bash $(DESTDIR)/$(datadir)/bash-completion/completions/$(exe_name)
	install -Dm644 autocomplete/whiff.fish $(DESTDIR)/$(datadir)/fish/vendor_completions.d/$(exe_name).fish
	install -Dm644 autocomplete/_whiff $(DESTDIR)/$(datadir)/zsh/site-functions/_$(exe_name)
	install -Dm644 doc/whiff.1 $(DESTDIR)/$(datadir)/man/man1/$(exe_name).1
