use proc_macro2::TokenStream;
use quote::quote;

pub fn style_line_core(input: String) -> TokenStream {
  let segments = parse_input(&input);
  let mut spans = Vec::new();

  for segment in segments {
    match segment {
      Segment::Text(text) => {
        spans.push(quote! { ratatui::prelude::Span::raw(#text) });
      },
      Segment::StyledText { text, fg, bg, bold, italics } => {
        spans.push(quote! {
          {
            let mut style = ratatui::prelude::Style::default();
            if !#fg.is_empty() {
              style = style
                .fg(
                  Color::from_str(#fg).expect("Unable to convert #fg to Color")
                );
            }
            if !#bg.is_empty() {
              style = style
                .bg(
                  Color::from_str(#bg).expect("Unable to convert #bg to Color")
                );
            }
            if #bold {
              style = style.add_modifier(ratatui::prelude::Modifier::BOLD);
            }
            if #italics {
              style = style.add_modifier(ratatui::prelude::Modifier::ITALIC);
            }
            ratatui::prelude::Span::styled(#text, style)
          }
        });
      },
    }
  }

  let q = quote! {
      use std::str::FromStr;
      let mut line = ratatui::prelude::Line::from(
        vec![
          #(#spans),*
        ]
      );
      line
  };
  q
}

enum Segment {
  Text(String),
  StyledText { text: String, fg: String, bg: String, bold: bool, italics: bool },
}

fn parse_input(input: &str) -> Vec<Segment> {
  let mut segments = Vec::new();
  let mut current = String::new();
  let mut chars = input.chars().peekable();

  while let Some(ch) = chars.next() {
    match ch {
      '{' => {
        if !current.is_empty() {
          segments.push(Segment::Text(current.clone()));
          current.clear();
        }
        let mut var_name = String::new();
        let mut fg = String::new();
        let mut bg = String::new();
        let mut in_fg = false;
        let mut in_bg = false;
        while let Some(&next) = chars.peek() {
          match next {
            '}' => {
              chars.next();
              break;
            },
            ':' if !in_fg => {
              in_fg = true;
              chars.next();
            },
            _ => {
              if in_fg {
                fg.push(next);
              } else {
                var_name.push(next);
              }
              chars.next();
            },
          }
        }
        if in_fg {
          segments.push(Segment::StyledText {
            text: var_name.clone(),
            fg: fg.clone(),
            bg: bg.clone(),
            bold: false,
            italics: false,
          });
        } else {
          segments.push(Segment::Text(var_name.clone()));
        }
      },
      _ => current.push(ch),
    }
  }
  if !current.is_empty() {
    segments.push(Segment::Text(current.clone()));
  }

  segments
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_style_line() {
    let result = parse_input("");
  }
}
