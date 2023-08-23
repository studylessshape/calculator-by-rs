slint::include_modules!();

fn main() {
    let app = CalculatorWindow::new().unwrap();
    // app.global::<Schemes>().set_dark_scheme(true);
    app.run().unwrap();
}
