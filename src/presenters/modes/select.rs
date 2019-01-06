use crate::errors::*;
use crate::models::application::modes::SelectMode;
use scribe::Workspace;
use scribe::buffer::Range;
use crate::presenters::current_buffer_status_line_data;
use crate::view::{Colors, StatusLineData, Style, View};

pub fn display(workspace: &mut Workspace, mode: &SelectMode, view: &mut View) -> Result<()> {
    let mut presenter = view.build_presenter()?;

    // Wipe the slate clean.
    presenter.clear();

    let buffer_status = current_buffer_status_line_data(workspace);

    if let Some(buf) = workspace.current_buffer() {
        let selected_range = Range::new(mode.anchor, *buf.cursor.clone());

        // Draw the visible set of tokens to the terminal.
        presenter.draw_buffer(buf, Some(&[selected_range]), None)?;

        // Draw the status line.
        presenter.draw_status_line(&[
            StatusLineData {
                content: " SELECT ".to_string(),
                style: Style::Default,
                colors: Colors::SelectMode,
            },
            buffer_status
        ]);
    } else {
        // There's no buffer; clear the cursor.
        presenter.set_cursor(None);
    }

    // Render the changes to the screen.
    presenter.present();

    Ok(())
}
