use super::{Span, Spanned};
use ariadne::{Color, Label};
use color_eyre::owo_colors::OwoColorize;
use std::{collections::HashMap, ops::Range};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum CodegenError<'src> {
    #[error("redeclaration of variables not allowed")]
    RedeclaredVariable(&'src str, Span),

    #[error("use of undeclared variable")]
    UndeclaredVariable(&'src str),

    #[error("invalid assignment target")]
    InvalidAssignmentTarget,
}

pub trait IntoLabels {
    fn into_label(self, src_id: String) -> Vec<Label<(String, Range<usize>)>>;
}

impl<'src> IntoLabels for Spanned<CodegenError<'src>> {
    fn into_label(self, src_id: String) -> Vec<Label<(String, Range<usize>)>> {
        use CodegenError as E;

        match self {
            (E::RedeclaredVariable(name, initial_span), err_span) => {
                vec![
                    Label::new((src_id.clone(), initial_span.into_range()))
                        .with_message(format!(
                            "variable '{}' initially declared here",
                            name.bright_black()
                        ))
                        .with_color(Color::Red),
                    Label::new((src_id, err_span.into_range()))
                        .with_message(format!("declared '{}' again here", name.bright_black()))
                        .with_color(Color::Red),
                ]
            }

            (E::UndeclaredVariable(name), err_span) => {
                vec![Label::new((src_id.clone(), err_span.into_range()))
                    .with_message(format!(
                        "variable '{}' not found in current scope",
                        name.bright_black()
                    ))
                    .with_color(Color::Red)]
            }

            (E::InvalidAssignmentTarget, span) => {
                vec![Label::new((src_id.clone(), span.into_range()))
                    .with_message("unable to assign to this")
                    .with_color(Color::Red)]
            }
        }
    }
}
pub trait Codegen<'src> {
    fn code_gen(
        self,
        label_idx: &mut usize,
        stack_ptr: &mut isize,
        env: &mut HashMap<String, (isize, Span)>,
    ) -> Result<String, Spanned<CodegenError<'src>>>;
}
