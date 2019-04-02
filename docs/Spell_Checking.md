# Spell Checking

Planned features related to spell checking.


* Spell check all documentation files of a project, in all supported languages.
* Spell check all comments in source code.
* Spell check all strings in source code.
* Build up list of identifiers from source code to help with non-dictionary words found in the comments.
    * If `cargo check` passes, it is assumed that the identifiers in the code are correct, so the identifiers in the comments and documentation should be fixed.
    * Per-project supplemental dictionaries.
    * Also scan all imported code for relevant identifiers.
* Present a nice interactive spell-check UI for human review.
    * Show sufficient context for each misspelling.
        * May need to include function name, module name, not just surrounding text.
