use html_to_string_macro::html;

#[test]
fn test_injection() {
    let injected = html! {
        <p>"Some text <em>with embedded emphasis tag</em>."</p>
    };
    assert_eq!(
        injected,
        r#"<p>Some text &lt;em&gt;with embedded emphasis tag&lt;/em&gt;.</p>"#
    );
}

#[test]
fn test_injection2() {
    let injected = html! {
        <p>"Some text "{ html!{ <em>"with embedded html! invocation"</em> } }"."</p>
    };
    assert_eq!(
        injected,
        r#"<p>Some text <em>with embedded html! invocation</em>.</p>"#
    );
}

#[test]
fn test() {
    let world = "planet";
    assert_eq!(
        html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <title>"Example"</title>
                </head>
                <body>
                    <!-- "comment" -->
                    <div hello={world} />
                    <>
                        <div>"1"</div>
                        <div>"2"</div>
                        <div>"3"</div>
                        <div {"some-attribute-from-rust-block"}/>
                    </>
                </body>
            </html>
        },
        r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>Example</title>
                </head>
                <body>
                    <!-- comment -->
                    <div hello="planet"></div>
                    <div>1</div>
                    <div>2</div>
                    <div>3</div>
                    <div some-attribute-from-rust-block></div>
                </body>
            </html>
        "#
        .split('\n')
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
        .join("")
    );
}
