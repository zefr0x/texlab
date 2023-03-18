use crate::{
    db::{analysis::label, Document, Workspace},
    Db,
};

use super::{Context, Token, TokenBuilder, TokenKind, TokenModifiers};

pub(super) fn find(context: Context, builder: &mut TokenBuilder) -> Option<()> {
    let db = context.db;
    let labels = context.document.parse(db).as_tex()?.analyze(db).labels(db);
    for label in labels
        .iter()
        .filter(|label| context.viewport.intersect(label.range(db)).is_some())
    {
        let name = label.name(db).text(db);
        let modifiers = match label.origin(db) {
            label::Origin::Definition(_) => {
                if !is_label_referenced(db, context.document, name) {
                    TokenModifiers::UNUSED
                } else {
                    TokenModifiers::NONE
                }
            }
            label::Origin::Reference(_) | label::Origin::ReferenceRange(_) => {
                if !is_label_defined(db, context.document, name) {
                    TokenModifiers::UNDEFINED
                } else {
                    TokenModifiers::NONE
                }
            }
        };

        let range = label.range(db);
        builder.push(Token {
            range,
            kind: TokenKind::Label,
            modifiers,
        });
    }

    Some(())
}

fn is_label_defined(db: &dyn Db, child: Document, name: &str) -> bool {
    Workspace::get(db)
        .related(db, child)
        .iter()
        .filter_map(|document| document.parse(db).as_tex())
        .flat_map(|data| data.analyze(db).labels(db))
        .filter(|label| matches!(label.origin(db), label::Origin::Definition(_)))
        .any(|label| label.name(db).text(db) == name)
}

fn is_label_referenced(db: &dyn Db, child: Document, name: &str) -> bool {
    Workspace::get(db)
        .related(db, child)
        .iter()
        .filter_map(|document| document.parse(db).as_tex())
        .flat_map(|data| data.analyze(db).labels(db))
        .filter(|label| {
            matches!(
                label.origin(db),
                label::Origin::Reference(_) | label::Origin::ReferenceRange(_)
            )
        })
        .any(|label| label.name(db).text(db) == name)
}
