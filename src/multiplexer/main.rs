struct Pane {
    id: usize,
    pty_master: File, // or async reader
    buffer: String,   // output from shell
    // position/size info for rendering
}

struct Multiplexer {
    panes: Vec<Pane>,
    focused_pane: usize,
    layout: Layout, // your custom struct
}

impl Multiplexer {
    fn vsplit(&mut self) { /* split focused pane vertically, spawn new shell */ }
    fn hsplit(&mut self) { /* split horizontally */ }
    fn exit(&mut self) { /* close focused pane */ }
    fn move_focus(&mut self, pane_id: usize) { /* change focused pane */ }
    fn run(&mut self) { /* main event loop */ }
}
