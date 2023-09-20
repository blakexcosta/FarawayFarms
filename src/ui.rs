use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    prelude::{Backend, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{self, Line, Span},
    widgets::*,
    Frame, Terminal,
};
use std::{
    error::Error,
    io::{self, Stdout},
    thread,
    time::Duration,
};

// fn main() -> Result<(), io::Error> {
//     // setup terminal
//     enable_raw_mode()?;
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;

//     terminal.draw(|f| {
//         let size = f.size();
//         let block = Block::default().title("Block").borders(Borders::ALL);
//         f.render_widget(block, size);
//     })?;

//     // Start a thread to discard any input events. Without handling events, the
//     // stdin buffer will fill up, and be read into the shell when the program exits.
//     thread::spawn(|| loop {
//         event::read();
//     });

//     thread::sleep(Duration::from_millis(5000));

//     // restore terminal
//     disable_raw_mode()?;
//     execute!(
//         terminal.backend_mut(),
//         LeaveAlternateScreen,
//         DisableMouseCapture
//     )?;
//     terminal.show_cursor()?;

//     Ok(())
// }

// TODO:
//[X] - Get custom display
//[NOPE] - Figure out state management
// [] - copy code from rat-test, main.rs. This has custom rendering.
// [] - Review state management from rat-test, it is inside app.rs and ui.rs. You can see this code in action in ratatui folder. Then running  `cargo run --example demo`. I just copied it into rat-test for reference for myself.

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

// USE THIS CODE
fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(5),
                Constraint::Percentage(5),
                Constraint::Percentage(10),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(f.size());
    let greeting = Paragraph::new("Hello World!");
    f.render_widget(greeting, f.size());
    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default().title("Block 2").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
    let block = Block::default().title("Block 3").borders(Borders::ALL);
    f.render_widget(block, chunks[2]);
    //     let barchart = BarChart::default()
    //         .block(Block::default().title("BarChart").borders(Borders::ALL))
    //         .bar_width(5)
    //         .bar_gap(2)
    //         .group_gap(3)
    //         .bar_style(Style::new().blue().on_red())
    //         .value_style(Style::new().gray().bold())
    //         .label_style(Style::new().white())
    //         .data(&[("B0", 0), ("B1", 2), ("B2", 4), ("B3", 3)])
    //         .data(BarGroup::default().bars(&[Bar::default().value(10), Bar::default().value(20)]))
    //         .max(4);
    //     f.render_widget(barchart, chunks[3]);
    let table = Table::new(vec![
        // Row can be created from simple strings.
        Row::new(vec!["Row11", "Row12", "Row13"]),
        // You can style the entire row.
        Row::new(vec!["Row21", "Row22", "Row23"]).style(Style::default().fg(Color::Blue)),
        // If you need more control over the styling you may need to create Cells directly
        Row::new(vec![
            Cell::from("Row31"),
            Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
            Cell::from(Line::from(vec![
                Span::raw("Row"),
                Span::styled("33", Style::default().fg(Color::Green)),
            ])),
        ]),
        // If a Row need to display some content over multiple lines, you just have to change
        // its height.
        Row::new(vec![
            Cell::from("Row\n41"),
            Cell::from("Row\n42"),
            Cell::from("Row\n43"),
        ])
        .height(2),
    ])
    // You can set the style of the entire Table.
    .style(Style::default().fg(Color::White))
    // It has an optional header, which is simply a Row always visible at the top.
    .header(
        Row::new(vec!["Col1", "Col2", "Col3"])
            .style(Style::default().fg(Color::Yellow))
            // If you want some space between the header and the rest of the rows, you can always
            // specify some margin at the bottom.
            .bottom_margin(1),
    )
    // As any other widget, a Table can be wrapped in a Block.
    .block(Block::default().title("Table"))
    // Columns widths are constrained in the same way as Layout...
    .widths(&[
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(10),
    ])
    // ...and they can be separated by a fixed spacing.
    .column_spacing(1)
    // If you wish to highlight a row in any specific way when it is selected...
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    // ...and potentially show a symbol in front of the selection.
    .highlight_symbol(">>");
    f.render_widget(table, chunks[3]);

    // custom paragrapg block
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Footer",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let text = vec![
        text::Line::from("This is a paragraph with several lines. You can change style your text the way you want"),
        text::Line::from(""),
        text::Line::from(vec![
            Span::from("For example: "),
            Span::styled("under", Style::default().fg(Color::Red)),
            Span::raw(" "),
            Span::styled("the", Style::default().fg(Color::Green)),
            Span::raw(" "),
            Span::styled("rainbow", Style::default().fg(Color::Blue)),
            Span::raw("."),
        ]),
        text::Line::from(vec![
            Span::raw("Oh and if you didn't "),
            Span::styled("notice", Style::default().add_modifier(Modifier::ITALIC)),
            Span::raw(" you can "),
            Span::styled("automatically", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled("wrap", Style::default().add_modifier(Modifier::REVERSED)),
            Span::raw(" your "),
            Span::styled("text", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::raw(".")
        ]),
        text::Line::from(
            "One more thing is that it should display unicode characters: 10€"
        ),
        text::Line::from("
BBY]fBBt]QBB]]]]]]]]]]]]]]]]]]]]]]]]]]%BQ]tBBt]XBB\n
BBMa*BB*aWBB++++++++++++++++++++++++++$BWa*BB*aMBB\n
BBOrYBBYrqBB++++++++++++++++++++++++++BBqrYBBYrOBB\n
BBpU0BB0UbBB++++++++++++++++++++++++++BBbU0BB0UpBB\n
BBBBBBBBBBBB++++++++++++++++++++++++++@BBBBBBBBBBB\n
M#&%%%%%%&##++++++++++++++++++++++++++W#&%%%%%%&#W\n
|+QoaWWaoY++++++++++++++++++++++++++++++YoaWWaoQ+t\n
W*W%%BB%%W#&&&&&&&&&&&&&&&&&&&&&&&&&&&W#W%%BB%%&*M\n
%8%BBBBBB%%BBBBBBBBBBBBBBBBBBBBBBBBBBB%%%BBBBBB%88\n
0Uh&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&kUO\n
|+m8888WMW88MM&88MM88&MM88WMW88MM&88MM88&MM8888Z+t\n
|+mBBBB*aMB%aa&B&aa%BWaoBB*aMBBaaWB8aa%BWaoBBBBm+t\n
|+wBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBm+t\n
|+wBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBm+t\n
|+m%8888888888888888888888888888888888888888888Z+t\n
wQo88&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&88oQq\n
qOo88888888888888888888888888888888888888888888o0p\n
|+m88888888888888888888888888888888888888888888Z+t\n
|+wBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBm+t\n
|+wBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBm+t\n
|+wBBBBBBW*Moo###BBBBBBBBBBBBBBBBM##ooM*WBBBBBBm+t\n
|+wBBBBBB&M&ooWWMBBBBBBBBBBBBBBBBWW&oo8MWBBBBBBm+t\n
|+wBBBBBB&M8**&WMBBBBBBBBBBBBBBBBWW&**8WWBBBBBBm+t\n
|+wBBBBBB&M8**&WMBBBBBBBBBBBBBBBBWW&**8WWBBBBBBm+t\n
j1p%%%%%%&W8MM&&W%%%%%%%%%%%%%%%%&&8MM8W&%%%%%%q1n\n
        "),
    ];
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(
        paragraph,
        Rect {
            x: 50,
            y: 5,
            width: 100,
            height: 10,
        },
        //chunks[4],
    ); //chunks[4]);
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    Ok(loop {
        terminal.draw(|frame| {
            ui(frame); // render ui
                       //frame
        })?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
        }
    })
}
