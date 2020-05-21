use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// `ParentPath` represents data to the Parent. It is composed
/// of a name and a path, and it directly describes only its immediate
/// parent.
///
/// For example, a project like so:
///```
/// root
/// |---sprites
///     |---spr_player
///         |---frame0
///         |---frame1
///```
/// `spr_player` will have a `ParentPath`:
/// ```rs
/// ParentPath {
///    name: "Sprites",
///    path: "folders/Sprites.yy"
/// }
/// ```
///
/// and frame0 might have a `ParentPath`:
/// ```
/// ParentPath {
///     name: "spr_player",
///     path: "sprites/spr_player/spr_player.yy",
/// }
/// ```
#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq)]
pub struct ParentPath {
    /// The human readable name of the parent. for a `spr_player`, this
    /// might correspond to `Sprites`.
    name: String,
    /// The direct path from the `.yyp` directory to the resource needed. This
    /// is not directly related to parentage at all, as GMS2 does not use the FileSystem
    /// for parentage.
    path: PathBuf,
}