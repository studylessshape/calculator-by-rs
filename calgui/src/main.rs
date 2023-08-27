use slint::{Model, SharedString};
use libcalcore::express::error::CalError;

slint::include_modules!();

enum SymbolMeans{
    SymbolInPexress,
    Delete,
    Clear,
    Left,
    Right,
    ShowResult,
    Unknow,
}

static EXPRESS_SYMBOLS: &str = "1234567890.()+-*/%^";

impl SymbolMeans {
    pub fn new(ch: &str) -> Self {
        if EXPRESS_SYMBOLS.contains(ch) {
            return Self::SymbolInPexress;
        }
        match ch {
            "C" => Self::Clear,
            "D" => Self::Delete,
            "←" => Self::Left,
            "→" => Self::Right,
            "=" => Self::ShowResult,
            _ => Self::Unknow,
        }
    }
}

fn main() {
    let app = CalculatorWindow::new().unwrap();
    
    let app_weak = app.as_weak();
    app.on_clicked_button(move |ch| {
        let app = app_weak.unwrap();
        let mut express_str = app.get_express().as_str().to_string();
        let cursor_pos = app.get_cursor_pos() as usize;
        let symbol_type = SymbolMeans::new(ch.as_str());
        match symbol_type {
            SymbolMeans::SymbolInPexress => {
                express_str.insert_str(cursor_pos, ch.as_str());
                app.set_express(SharedString::from(express_str));
            },
            SymbolMeans::Delete => {
                if express_str.len() > 0 {
                    express_str.drain(cursor_pos..=cursor_pos);
                }
                app.set_express(SharedString::from(express_str));
            },
            SymbolMeans::Clear => app.set_express(SharedString::new()),
            SymbolMeans::Left => app.set_cursor_pos((if cursor_pos > 0 { cursor_pos - 1 } else { cursor_pos }) as f32),
            SymbolMeans::Right => app.set_cursor_pos((if cursor_pos < express_str.len() - 1 { cursor_pos + 1 } else { cursor_pos }) as f32),
            SymbolMeans::ShowResult => {
                match calculate_express(&express_str) {
                    Ok(res) => app.set_result(SharedString::from(res.to_string())),
                    Err(err) => {
                        app.set_message_title(SharedString::from("Error"));
                        app.set_message_content(SharedString::from(err.to_string()));
                    },
                }
            },
            SymbolMeans::Unknow => {
                app.set_message_title(SharedString::from("Error"));
                app.set_message_content(SharedString::from("Unknow operate"));
            },
        }
    });

    app.run().unwrap();
}

fn calculate_express(express: &str) -> Result<f64, CalError> {
    use libcalcore::express::parser::lookahead::LookAhead;
    let mut parser = LookAhead::try_from(express)?;
    let ast = parser.parse_expr()?;
    Ok(ast.eval().unwrap())
}