## Technology Stack

RMAP is built around a combination of systems programming, modern frontend development, and desktop application engineering. Each technology serves a specific architectural purpose, allowing the project to explore performance, memory management, data modeling, and user interface design within a single application.

| Technology | Role |
|---|---|
| **Rust** | Core application logic, data modeling, file processing, and performance-sensitive operations |
| **Tauri** | Desktop application runtime and communication between the frontend and backend |
| **TypeScript** | Type-safe frontend development and communication contracts |
| **Svelte** | Reactive user interface and editor tooling |
| **RMAP binary format** | Custom structured serialization format for map documents |

### Rust — Systems Programming and Data Integrity

Rust is the foundation of RMAP's core logic.

A map editor must manage potentially large collections of objects, transformations, layers, and metadata. As these collections grow, memory usage, data consistency, and the cost of repeated operations become increasingly important.

Rust provides explicit control over data structures and ownership, along with compile-time guarantees that help prevent entire classes of memory-safety bugs. Its type system also makes it possible to represent application concepts and valid states directly in the codebase.

In RMAP, Rust is used to explore and implement:

- Structured domain modeling for maps, objects, layers, and settings.
- Efficient file I/O and binary serialization.
- Validation and error handling when loading user-generated documents.
- Memory-conscious data management for increasingly complex maps.
- A foundation for future performance-sensitive operations.

The goal is not to assume that Rust automatically makes the application faster, but to establish a core that can be profiled, measured, and optimized as the editor's requirements grow.

### Tauri — Native Desktop Integration

Tauri provides the desktop runtime that connects RMAP's Rust backend to its web-based frontend.

Rather than implementing the entire interface using a traditional native GUI framework, RMAP uses web technologies for the presentation layer while keeping core document operations in Rust.

This architecture offers several advantages:

- Access to a mature ecosystem for building complex user interfaces.
- A native application shell without bundling a separate browser runtime in the same way Electron typically does.
- A clear boundary between interface state and core application logic.
- The ability to invoke backend operations through Tauri's command system.
- A cross-platform foundation for future desktop releases.

Tauri's use of the system WebView can reduce distribution size and duplicated runtime overhead compared with bundling Chromium. However, actual memory consumption still depends on the WebView, application architecture, and rendering workload.

For a map editor, this distinction matters: the interface must remain responsive while the application manages large scenes, and expensive operations should not unnecessarily block interaction.

### TypeScript — Reliable Frontend Contracts

TypeScript provides a typed foundation for the editor's frontend.

As the number of tools, object properties, configuration options, and editor modes increases, loosely structured state becomes difficult to maintain. TypeScript helps make these structures explicit and catches many inconsistencies during development.

Its role extends beyond component development. It also provides a basis for defining the data exchanged between the frontend and Rust backend, reducing ambiguity around command arguments and returned values.

RMAP uses TypeScript to support:

- Explicit models for frontend state and editor entities.
- Better-defined interfaces between UI components.
- Typed command arguments and responses.
- Safer refactoring as the application grows.
- More maintainable interactions between editing tools and document data.

TypeScript's compile-time guarantees do not replace runtime validation. Files loaded from disk must still be treated as untrusted input and validated by the backend.

### Svelte — Reactive Editor Architecture

Svelte is used to build the application's user interface.

A desktop editor involves more than a collection of static screens. Its interface must coordinate selection, object properties, layer visibility, tool states, dialogs, and document changes without becoming difficult to maintain.

Svelte's compiler-based approach and reactive state model provide a practical foundation for this complexity.

The frontend is intended to separate interface concerns from document manipulation, allowing components to focus on presentation and user interaction while Rust handles the core data operations.

This separation also creates room to investigate rendering strategies appropriate for large maps, where updating the entire interface for every object transformation would be undesirable.

Svelte does not, by itself, guarantee efficient rendering of thousands of map objects. That remains an architectural concern to be addressed through careful component design, state management, and appropriate rendering techniques.

### RMAP — Designing a Custom Binary Format

Instead of relying exclusively on a general-purpose serialization format, RMAP is developing its own binary document format.

This is both a practical engineering decision and an opportunity to explore how structured application data is represented, persisted, and recovered.

The format is being designed around identifiable sections and explicitly defined field types. Its planned structure includes metadata, map dimensions, settings, objects, layers, and future extensions for areas and additional document features.

The implementation explores:

- Binary representation of primitive values.
- Endianness and fixed-width numeric fields.
- Structured section boundaries and length-prefixed data.
- Sequential parsing and controlled file seeking.
- Validation of magic bytes, markers, and malformed input.
- Format evolution and backward-compatibility considerations.

A custom format introduces additional responsibilities: the application must define its specification, reject invalid documents safely, and handle future format changes deliberately.

Its performance and storage benefits will need to be established through benchmarks against suitable alternatives. The primary motivation is to maintain a format whose structure is designed around RMAP's actual data model and long-term requirements.

## Engineering Challenges

RMAP is also an opportunity to investigate several problems that emerge when building a stateful desktop editor.

### Large-scene performance

The intended use case includes maps containing many independently editable objects. This creates challenges around scene representation, selection, rendering, and frequent state updates.

Future optimization work may include profiling, spatial indexing, incremental updates, and separating document data from rendering data where appropriate.

### Document lifecycle

Creating, loading, modifying, and saving a map requires consistent document state and reliable error handling.

The custom format makes it especially important to test malformed files, truncated sections, invalid values, and compatibility between format versions.

### Separation of editing and presentation

The editor and the player-facing view have different requirements. The editor needs access to the complete document, while the public view must expose only the information intended for the players.

This is both a user experience problem and a data-access boundary. The architecture must prevent private map information from leaking into the player-facing presentation.

### Cross-platform desktop development

Tauri provides a shared application architecture, but platform-specific WebView behavior, filesystem permissions, window management, and display configuration still require testing.

Native projector support will make these concerns particularly relevant because the application must coordinate the master's workspace with a separate player-facing display.

## Project Goals

RMAP aims to become a practical map editor while serving as a deeper exploration of desktop application engineering.

Its development combines:

- Systems programming with Rust.
- Cross-platform desktop development with Tauri.
- Typed, reactive frontend architecture with TypeScript and Svelte.
- Custom binary serialization and file format design.
- Performance analysis for data-heavy interactive applications.
- Secure separation of private editing data and public presentation.
- Iterative architecture design supported by testing and profiling.

The project prioritizes a functional application first, with each new capability contributing to a coherent, extensible codebase.