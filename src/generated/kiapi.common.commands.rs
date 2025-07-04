// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersion {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionResponse {
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<super::types::KiCadVersion>,
}
/// A command to check if the connection to KiCad is OK
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ping {}
/// Returns the full path to the given KiCad binary
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKiCadBinaryPath {
    /// The short name of the binary, such as `kicad-cli` or `kicad-cli.exe`.  If on Windows, an `.exe`
    /// extension will be assumed if not present.
    #[prost(string, tag = "1")]
    pub binary_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PathResponse {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
}
/// returns kiapi.common.types.Box2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTextExtents {
    /// A temporary text item to calculate the bounding box for
    #[prost(message, optional, tag = "1")]
    pub text: ::core::option::Option<super::types::Text>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextOrTextBox {
    #[prost(oneof = "text_or_text_box::Inner", tags = "1, 2")]
    pub inner: ::core::option::Option<text_or_text_box::Inner>,
}
/// Nested message and enum types in `TextOrTextBox`.
pub mod text_or_text_box {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Inner {
        #[prost(message, tag = "1")]
        Text(super::super::types::Text),
        #[prost(message, tag = "2")]
        Textbox(super::super::types::TextBox),
    }
}
/// Render the given text object(s) as shapes.  Depending on whether the text is using
/// the KiCad stroke font or a custom font, the response will be a compound shape containing
/// a set of polygons or a set of segments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTextAsShapes {
    #[prost(message, repeated, tag = "1")]
    pub text: ::prost::alloc::vec::Vec<TextOrTextBox>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextWithShapes {
    #[prost(message, optional, tag = "1")]
    pub text: ::core::option::Option<TextOrTextBox>,
    #[prost(message, optional, tag = "2")]
    pub shapes: ::core::option::Option<super::types::CompoundShape>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTextAsShapesResponse {
    #[prost(message, repeated, tag = "1")]
    pub text_with_shapes: ::prost::alloc::vec::Vec<TextWithShapes>,
}
/// Return a writeable path that a plugin can use for storing persistent data such as configuration
/// files, etc.  This path may not yet exist; actual creation of the directory for a given plugin is
/// up to the plugin itself.  Files in this path will not be modified if the plugin is uninstalled or
/// upgraded.
///
/// Returns StringResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPluginSettingsPath {
    /// The identifier of the plugin
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringResponse {
    #[prost(string, tag = "1")]
    pub response: ::prost::alloc::string::String,
}
/// Refreshes the given frame, if that frame is open
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshEditor {
    #[prost(enumeration = "super::types::FrameType", tag = "1")]
    pub frame: i32,
}
/// Retrieves a list of open documents of the given type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOpenDocuments {
    /// Which type of documents to query
    #[prost(enumeration = "super::types::DocumentType", tag = "1")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOpenDocumentsResponse {
    #[prost(message, repeated, tag = "1")]
    pub documents: ::prost::alloc::vec::Vec<super::types::DocumentSpecifier>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveDocument {
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<super::types::DocumentSpecifier>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveOptions {
    /// Overwrite destination file(s) if they exist
    #[prost(bool, tag = "1")]
    pub overwrite: bool,
    /// If the file being saved normally requires a project (for example, a board or schematic),
    /// this flag will cause a new project to be saved alongside the new file
    #[prost(bool, tag = "2")]
    pub include_project: bool,
}
/// Saves the given document to a new location and does not open the new copy
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveCopyOfDocument {
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<super::types::DocumentSpecifier>,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub options: ::core::option::Option<SaveOptions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevertDocument {
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<super::types::DocumentSpecifier>,
}
///
/// Runs a TOOL_ACTION using the TOOL_MANAGER of a given frame.
/// WARNING: The TOOL_ACTIONs are specifically *not* an API.
/// Command names may change as code is refactored, and commands may disappear.
/// This API method is provided for low-level prototyping purposes only.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunAction {
    /// Action name, like "eeschema.InteractiveSelection.ClearSelection"
    #[prost(string, tag = "1")]
    pub action: ::prost::alloc::string::String,
}
///
/// NOTE: At the moment, RAS_FRAME_NOT_OPEN won't be returned as the handler is inside the frame.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunActionResponse {
    #[prost(enumeration = "RunActionStatus", tag = "1")]
    pub status: i32,
}
///
/// Begins a staged set of changes.  Any modifications made to a document through the API after this
/// call will be saved to a pending commit, and will not appear in KiCad until a matching call to
/// END_COMMIT.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginCommit {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginCommitResponse {
    /// Opaque identifier tracking a commit
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::types::Kiid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndCommit {
    /// The ID that was given by BeginCommit
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::types::Kiid>,
    /// What to do with this commit
    #[prost(enumeration = "CommitAction", tag = "2")]
    pub action: i32,
    /// Optional message describing this changeset
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndCommitResponse {}
/// Creates new items on a given document
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateItems {
    /// Specifies which document to create on, which fields are included, etc.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::types::ItemHeader>,
    /// List of items to create
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// Items may be created on a top-level document (sheet, board, etc) or inside a container
    /// (symbol, footprint).  If this field is not empty, it holds the ID of a symbol or footprint
    /// that the items should be added to.  This ID must be an existing symbol (for schematic
    /// documents) or footprint (for board documents).  If the given container does not exist or is
    /// not the correct item type, the CreateItems call will fail.
    #[prost(message, optional, tag = "3")]
    pub container: ::core::option::Option<super::types::Kiid>,
}
/// Per-item status feedback for creation and update calls
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemStatus {
    #[prost(enumeration = "ItemStatusCode", tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub error_message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemCreationResult {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<ItemStatus>,
    /// The created version of the item, including an updated KIID as applicable
    #[prost(message, optional, tag = "2")]
    pub item: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateItemsResponse {
    /// Specifies which document was modified, which fields are included in created_items, etc.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::types::ItemHeader>,
    /// Status of the overall request; may return IRS_OK even if no items were created
    #[prost(enumeration = "super::types::ItemRequestStatus", tag = "2")]
    pub status: i32,
    /// Status of each item to be created
    #[prost(message, repeated, tag = "3")]
    pub created_items: ::prost::alloc::vec::Vec<ItemCreationResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetItems {
    /// Specifies which document to query, which fields to return, etc.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::types::ItemHeader>,
    /// List of one or more types of items to retreive
    #[prost(enumeration = "super::types::KiCadObjectType", repeated, tag = "2")]
    pub types: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetItemsResponse {
    /// Specifies which document was modified, which fields are included in items, etc.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::types::ItemHeader>,
    /// Status of the overall request; may return IRS_OK even if no items were retrieved
    #[prost(enumeration = "super::types::ItemRequestStatus", tag = "2")]
    pub status: i32,
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// Updates items in a given document
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateItems {
    /// Specifies which document to modify, which fields are included, etc.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::types::ItemHeader>,
    /// List of items to modify
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemUpdateResult {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<ItemStatus>,
    /// The update version of the item
    #[prost(message, optional, tag = "2")]
    pub item: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateItemsResponse {
    /// Specifies which document was modified, which fields are included in updated_items, etc.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::types::ItemHeader>,
    /// Status of the overall request; may return IRS_OK even if no items were modified
    #[prost(enumeration = "super::types::ItemRequestStatus", tag = "2")]
    pub status: i32,
    /// Status of each item to be created
    #[prost(message, repeated, tag = "3")]
    pub updated_items: ::prost::alloc::vec::Vec<ItemUpdateResult>,
}
/// Deletes items in a given document
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteItems {
    /// Specifies which document to modify
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::types::ItemHeader>,
    /// List of item KIIDs to delete
    #[prost(message, repeated, tag = "2")]
    pub item_ids: ::prost::alloc::vec::Vec<super::types::Kiid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemDeletionResult {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::types::Kiid>,
    #[prost(enumeration = "ItemDeletionStatus", tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteItemsResponse {
    /// Specifies which document was modified, etc.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::types::ItemHeader>,
    /// Status of the overall request; may return IRS_OK even if no items were deleted
    #[prost(enumeration = "super::types::ItemRequestStatus", tag = "2")]
    pub status: i32,
    /// Status of each item requested to be deleted
    #[prost(message, repeated, tag = "3")]
    pub deleted_items: ::prost::alloc::vec::Vec<ItemDeletionResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBoundingBox {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::types::ItemHeader>,
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<super::types::Kiid>,
    /// Some item types can have independently-movable text as children (e.g. footprints)
    /// This mode controls whether or not these are included in the box
    #[prost(enumeration = "BoundingBoxMode", tag = "3")]
    pub mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBoundingBoxResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<super::types::Kiid>,
    #[prost(message, repeated, tag = "2")]
    pub boxes: ::prost::alloc::vec::Vec<super::types::Box2>,
}
/// Retrieves a list of items.  Returns SelectionResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSelection {
    /// Specifies which document to query for selected items.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::types::ItemHeader>,
    /// An optional list of types to filter on.
    /// If none are provided, all selected items will be returned.
    #[prost(enumeration = "super::types::KiCadObjectType", repeated, tag = "2")]
    pub types: ::prost::alloc::vec::Vec<i32>,
}
/// The set of currently selected items
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectionResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// Adds the given items to the selection.  Returns SelectionResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddToSelection {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::types::ItemHeader>,
    /// The items to select
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<super::types::Kiid>,
}
/// Removes the given items to the selection.  Returns SelectionResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFromSelection {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::types::ItemHeader>,
    /// The items to deselect
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<super::types::Kiid>,
}
/// Removes all items from selection
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearSelection {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::types::ItemHeader>,
}
/// Tests if a certain point falls within tolerance of an item's geometry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HitTest {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::types::ItemHeader>,
    #[prost(message, optional, tag = "2")]
    pub id: ::core::option::Option<super::types::Kiid>,
    #[prost(message, optional, tag = "3")]
    pub position: ::core::option::Option<super::types::Vector2>,
    #[prost(int32, tag = "4")]
    pub tolerance: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HitTestResponse {
    #[prost(enumeration = "HitTestResult", tag = "1")]
    pub result: i32,
}
/// returns common.types.TitleBlockInfo
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTitleBlockInfo {
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<super::types::DocumentSpecifier>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveDocumentToString {
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<super::types::DocumentSpecifier>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedDocumentResponse {
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<super::types::DocumentSpecifier>,
    #[prost(string, tag = "2")]
    pub contents: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveSelectionToString {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedSelectionResponse {
    #[prost(message, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<super::types::Kiid>,
    #[prost(string, tag = "2")]
    pub contents: ::prost::alloc::string::String,
}
/// Attempts to parse the given string as a s-expression formatted container with items,
/// similar to how the Paste action inside the KiCad editor works.  If the parse is successful,
/// the items will be created and inserted into the editor.
/// Returns CreateItemsResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParseAndCreateItemsFromString {
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<super::types::DocumentSpecifier>,
    #[prost(string, tag = "2")]
    pub contents: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RunActionStatus {
    RasUnknown = 0,
    /// The action was submitted successfully.
    RasOk = 1,
    /// The action was unknown for the targeted frame.
    RasInvalid = 2,
    /// The targeted frame was not open when the call was submitted.
    RasFrameNotOpen = 3,
}
impl RunActionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RunActionStatus::RasUnknown => "RAS_UNKNOWN",
            RunActionStatus::RasOk => "RAS_OK",
            RunActionStatus::RasInvalid => "RAS_INVALID",
            RunActionStatus::RasFrameNotOpen => "RAS_FRAME_NOT_OPEN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RAS_UNKNOWN" => Some(Self::RasUnknown),
            "RAS_OK" => Some(Self::RasOk),
            "RAS_INVALID" => Some(Self::RasInvalid),
            "RAS_FRAME_NOT_OPEN" => Some(Self::RasFrameNotOpen),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommitAction {
    CmaUnknown = 0,
    /// Commit the changes to the design
    CmaCommit = 1,
    /// Cancel this commit
    CmaDrop = 2,
}
impl CommitAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CommitAction::CmaUnknown => "CMA_UNKNOWN",
            CommitAction::CmaCommit => "CMA_COMMIT",
            CommitAction::CmaDrop => "CMA_DROP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CMA_UNKNOWN" => Some(Self::CmaUnknown),
            "CMA_COMMIT" => Some(Self::CmaCommit),
            "CMA_DROP" => Some(Self::CmaDrop),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ItemStatusCode {
    IscUnknown = 0,
    /// The item was created or updated
    IscOk = 1,
    /// The item's type is not valid for the given document
    IscInvalidType = 2,
    /// The item to be created had a specified KIID and that KIID was already in use
    IscExisting = 3,
    /// The item to be updated did not exist in the given document
    IscNonexistent = 4,
    /// The item to be updated is not allowed to be modified by the API
    IscImmutable = 5,
    /// The item to be created does not have valid data for the given document
    IscInvalidData = 7,
}
impl ItemStatusCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ItemStatusCode::IscUnknown => "ISC_UNKNOWN",
            ItemStatusCode::IscOk => "ISC_OK",
            ItemStatusCode::IscInvalidType => "ISC_INVALID_TYPE",
            ItemStatusCode::IscExisting => "ISC_EXISTING",
            ItemStatusCode::IscNonexistent => "ISC_NONEXISTENT",
            ItemStatusCode::IscImmutable => "ISC_IMMUTABLE",
            ItemStatusCode::IscInvalidData => "ISC_INVALID_DATA",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ISC_UNKNOWN" => Some(Self::IscUnknown),
            "ISC_OK" => Some(Self::IscOk),
            "ISC_INVALID_TYPE" => Some(Self::IscInvalidType),
            "ISC_EXISTING" => Some(Self::IscExisting),
            "ISC_NONEXISTENT" => Some(Self::IscNonexistent),
            "ISC_IMMUTABLE" => Some(Self::IscImmutable),
            "ISC_INVALID_DATA" => Some(Self::IscInvalidData),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ItemDeletionStatus {
    IdsUnknown = 0,
    IdsOk = 1,
    /// The item did not exist in the given document
    IdsNonexistent = 2,
    /// The item is not allowed to be modified by the API
    IdsImmutable = 3,
}
impl ItemDeletionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ItemDeletionStatus::IdsUnknown => "IDS_UNKNOWN",
            ItemDeletionStatus::IdsOk => "IDS_OK",
            ItemDeletionStatus::IdsNonexistent => "IDS_NONEXISTENT",
            ItemDeletionStatus::IdsImmutable => "IDS_IMMUTABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IDS_UNKNOWN" => Some(Self::IdsUnknown),
            "IDS_OK" => Some(Self::IdsOk),
            "IDS_NONEXISTENT" => Some(Self::IdsNonexistent),
            "IDS_IMMUTABLE" => Some(Self::IdsImmutable),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BoundingBoxMode {
    BbmUnknown = 0,
    BbmItemOnly = 1,
    BbmItemAndChildText = 2,
}
impl BoundingBoxMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BoundingBoxMode::BbmUnknown => "BBM_UNKNOWN",
            BoundingBoxMode::BbmItemOnly => "BBM_ITEM_ONLY",
            BoundingBoxMode::BbmItemAndChildText => "BBM_ITEM_AND_CHILD_TEXT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BBM_UNKNOWN" => Some(Self::BbmUnknown),
            "BBM_ITEM_ONLY" => Some(Self::BbmItemOnly),
            "BBM_ITEM_AND_CHILD_TEXT" => Some(Self::BbmItemAndChildText),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HitTestResult {
    HtrUnknown = 0,
    HtrNoHit = 1,
    HtrHit = 2,
}
impl HitTestResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HitTestResult::HtrUnknown => "HTR_UNKNOWN",
            HitTestResult::HtrNoHit => "HTR_NO_HIT",
            HitTestResult::HtrHit => "HTR_HIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HTR_UNKNOWN" => Some(Self::HtrUnknown),
            "HTR_NO_HIT" => Some(Self::HtrNoHit),
            "HTR_HIT" => Some(Self::HtrHit),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNetClasses {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetClassesResponse {
    #[prost(message, repeated, tag = "1")]
    pub net_classes: ::prost::alloc::vec::Vec<super::project::NetClass>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNetClasses {
    #[prost(message, repeated, tag = "1")]
    pub net_classes: ::prost::alloc::vec::Vec<super::project::NetClass>,
    /// Whether to merge or replace the existing netclasses with the contents of this message
    /// Note that this only happens at the level of netclass name: for example, if merge_mode is set to
    /// MMM_MERGE, the design has netclasses \["Default", "HV"\], and this message has netclasses
    /// \["Default", "LV"\], the resulting set will be \["Default", "HV", "LV"\] -- the Default netclass
    /// will have its properties replaced with those in this message, the "LV" netclass will be added,
    /// and the "HV" netclass will be left alone.  If merge_mode is set to MMM_REPLACE, the "HV" class
    /// will be erased.  Note that there must always be a "Default" netclass, so it will not be erased
    /// even if merge_mode is MMM_REPLACE and there is no "Default" class specified in this message.
    #[prost(enumeration = "super::types::MapMergeMode", tag = "3")]
    pub merge_mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandTextVariables {
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<super::types::DocumentSpecifier>,
    #[prost(string, repeated, tag = "2")]
    pub text: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandTextVariablesResponse {
    #[prost(string, repeated, tag = "1")]
    pub text: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// returns kiapi.common.project.TextVariables
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTextVariables {
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<super::types::DocumentSpecifier>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTextVariables {
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<super::types::DocumentSpecifier>,
    #[prost(message, optional, tag = "2")]
    pub variables: ::core::option::Option<super::project::TextVariables>,
    /// Whether to merge or replace the existing text variables map with the contents of this message
    #[prost(enumeration = "super::types::MapMergeMode", tag = "3")]
    pub merge_mode: i32,
}
