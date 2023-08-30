use libcalcore::express::error::CalError;
use slint::SharedString;

slint::include_modules!();

enum SymbolMeans {
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

    let mut cursor_pos: i32 = 0;
    let mut express_str = String::from("|");
    app.set_express(SharedString::from(express_str.clone()));

    let app_weak = app.as_weak();
    app.on_clicked_button(move |ch| {
        let app = app_weak.unwrap();
        // let cursor_pos = app.get_cursor_pos() as usize;
        let symbol_type = SymbolMeans::new(ch.as_str());
        match symbol_type {
            SymbolMeans::SymbolInPexress => {
                express_str.insert_str(cursor_pos as usize, ch.as_str());
                cursor_pos += 1;
                if cursor_pos as usize >= express_str.len() {
                    cursor_pos -= 1;
                }
                app.set_express(SharedString::from(express_str.clone()));
            }
            SymbolMeans::Delete => {
                if express_str.len() > 0 {
                    express_str.remove(ternary_operations(cursor_pos - 1 >= 0, cursor_pos - 1, 0) as usize);
                    cursor_pos -= 1;
                    if cursor_pos < 0 {
                        cursor_pos = 0;
                    }
                }
                app.set_express(SharedString::from(express_str.clone()));
            }
            SymbolMeans::Clear => {
                express_str.clear();
                express_str.push('|');
                cursor_pos = 0;
                app.set_express(SharedString::from(express_str.clone()))
            }
            SymbolMeans::Left => {
                express_str = move_cursor_pos(&express_str, cursor_pos, -1);
                cursor_pos -= 1;
                if cursor_pos < 0 {
                    cursor_pos = 0;
                }
                app.set_express(SharedString::from(express_str.clone()));
            }
            SymbolMeans::Right => {
                express_str = move_cursor_pos(&express_str, cursor_pos, 1);
                cursor_pos += 1;
                if cursor_pos as usize >= express_str.len() {
                    cursor_pos -= 1;
                }
                app.set_express(SharedString::from(express_str.clone()));
            }
            SymbolMeans::ShowResult => {
                let mut exp_str = express_str.clone();
                exp_str.remove(cursor_pos as usize);
                match calculate_express(&exp_str) {
                    Ok(res) => app.set_result(SharedString::from(res.to_string())),
                    Err(err) => {
                        app.set_message_title(SharedString::from("Error"));
                        app.set_message_content(SharedString::from(err.to_string()));
                    }
                }
            },
            SymbolMeans::Unknow => {
                app.set_message_title(SharedString::from("Error"));
                app.set_message_content(SharedString::from("Unknow operate"));
            }
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

fn move_cursor_pos(str: &String, pos: i32, offset: i32) -> String {
    if pos + offset < 0 || (pos + offset) as usize >= str.len() {
        return str.clone();
    }
    let mut str_clone = str.clone();
    str_clone.remove(pos as usize);
    str_clone.insert((pos + offset) as usize, '|');
    str_clone
}

fn ternary_operations<T: Sized>(cond: bool, y: T, n: T) -> T {
    if cond {
        y
    } else {
        n
    }
}
