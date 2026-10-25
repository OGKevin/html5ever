// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::tendril::StrTendril;
use crate::tokenizer::{Doctype, Pi, Tag};

/// Tree-builder insertion mode.
///
/// After the document element closes, the builder enters [`End`], or
/// [`Start`] when `XmlTreeBuilderOpts::extra_roots` is set. [`End`] is also
/// used for EOF.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum XmlPhase {
    /// No current document element. The next start or empty tag is appended as a document child.
    Start,
    /// A document element is open.
    Main,
    /// Parsing is finished. Further elements are dropped.
    End,
}

/// A subset/refinement of `tokenizer::Token`.  Everything else is handled
/// specially at the beginning of `process_token`.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Token {
    Tag(Tag),
    Doctype(Doctype),
    Comment(StrTendril),
    Characters(StrTendril),
    Pi(Pi),
    NullCharacter,
    Eof,
}

pub enum XmlProcessResult<Handle> {
    Done,
    Reprocess(XmlPhase, Token),
    Script(Handle),
}
