/// A link.
#[derive(Debug)]
pub struct Link {
    /// The link's URL.
    pub url: String,

    /// The text for the link.
    pub text: Content,
}

/// A table.
#[derive(Debug)]
pub struct Table {
    /// The table header.
    pub header: TableHeader,

    /// The table body.
    pub body: TableBody,
}

impl Table {
    /// Returns a new [`Table`].
    pub fn new() -> Self {
        Self {
            header: TableHeader::new(),
            body: TableBody::new(),
        }
    }
}

/// A table header.
#[derive(Debug)]
pub struct TableHeader {
    /// The rows in the header.
    pub rows: Vec<TableRow>,
}

impl TableHeader {
    /// Returns a new [`TableHeader`].
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    /// Adds a row to the [`TableHeader`].
    pub fn add_row(&mut self, row: TableRow) {
        self.rows.push(row)
    }
}

/// A table body.
#[derive(Debug)]
pub struct TableBody {
    /// The rows in the body.
    pub rows: Vec<TableRow>,
}

impl TableBody {
    /// Returns a new [`TableBody`].
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    /// Adds a row to the [`TableBody`].
    pub fn add_row(&mut self, row: TableRow) {
        self.rows.push(row)
    }
}

/// A row in a table.
#[derive(Debug)]
pub struct TableRow {
    /// The colums in the row.
    pub columns: Vec<TableColumn>,
}

impl TableRow {
    /// Returns a new [`TableRow`].
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
        }
    }
}

impl TableRow {
    /// Adds a column to the [`TableRow`].
    pub fn add_column(&mut self, column: TableColumn) {
        self.columns.push(column)
    }
}

/// A column in a table.
#[derive(Debug)]
pub struct TableColumn {
    /// The value of the column.
    pub value: Content,
}

impl TableColumn {
    /// Returns a new [`TableColumn`] containing the specified content.
    pub fn new(content: Content) -> Self {
        Self { value: content }
    }
}

/// A piece of content.
#[derive(Debug)]
pub enum Content {
    /// A piece of text.
    Text(String),

    /// A link.
    Link(Box<Link>),

    /// A table.
    Table(Box<Table>),
}
