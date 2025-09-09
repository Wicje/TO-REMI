#Doist
This a personal Todo terminal application









##Buildingphase
Phase 1 → Core Expansion (cement ownership & error handling)

✅ Add/Edit/Delete/Clear tasks.

✅ Track completion.

🆕 Edit task details (description, due_date).

🆕 Clear completed tasks.

👉 Skills you’ll sharpen here:

Mutable vs. immutable borrows (&mut Task when editing).

File I/O round-tripping with updated data.

Error handling when task index doesn’t exist.

Phase 2 → Priorities & Organization (play with enums & pattern matching)

Add priority: enum { Low, Medium, High, Critical }.

Add categories (string or enum).

Filtering by status/priority/category.

Sorting by due date/priority.

👉 Skills:

Enums & match statements.

Iterating & filtering collections.

Error handling when filters don’t match anything.

Phase 3 → Dates & Time (learn chrono crate)

Due dates (chrono::NaiveDate).

Overdue task highlighting.

Filtering by date.

👉 Skills:

External crate usage (chrono).

Parsing/formatting dates from user input.

Error handling on invalid dates.

Phase 4 → Advanced Features (big brain Rust)

Recurring tasks (generate next occurrence).

Subtasks/Dependencies (nested structs).

Statistics (counting tasks).

Calendar view (group by date).

👉 Skills:

Struct composition.

Lifetimes (if you reference subtasks).

More complex borrow-checker fights.

Phase 5 → Persistence & Sync

Swap JSON → SQLite (diesel or sqlx).

Import/Export.

Backups.

Maybe sync with a cloud service (once async networking is introduced).

👉 Skills:

Database integration.

Traits + modular storage backends.

Async/await (tokio) when syncing.

Phase 6 → UI/UX Enhancements

Colored output (colored crate).

TUI (cursive/tui-rs).

Interactive mode with arrow keys.

Progress indicators.

👉 Skills:

Terminal UI design.

Handling async input/output cleanly.
