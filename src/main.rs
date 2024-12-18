use rerun::{self, EntityPath, Rgba32, EXTERNAL_DATA_LOADER_INCOMPATIBLE_EXIT_CODE};

/// This is an executable data-loader plugin for the Rerun Viewer.
/// Any executable on your `$PATH` with a name that starts with [`rerun-loader-`] will be
/// treated as an external data-loader.
///
/// This particular one will log collada files as [`Mesh3d`](https://docs.rs/rerun/latest/rerun/struct.Mesh3D.html),
/// and return a special exit code to indicate that it doesn't support anything else.
#[derive(argh::FromArgs, Debug)]
struct Args {
    #[argh(positional)]
    filepath: std::path::PathBuf,

    /// optional recommended ID for the application
    #[argh(option)]
    application_id: Option<String>,

    /// optional recommended ID for the recording
    #[argh(option)]
    recording_id: Option<String>,

    /// optional prefix for all entity paths
    #[argh(option)]
    entity_path_prefix: Option<String>,

    /// deprecated: alias for `--static`
    #[argh(switch)]
    _timeless: bool,

    /// optionally mark data to be logged statically
    #[argh(arg_name = "static", switch)]
    _statically: bool,

    /// optional timestamps to log at (e.g. `--time sim_time=1709203426`) (repeatable)
    #[argh(option)]
    _time: Vec<String>,

    /// optional sequences to log at (e.g. `--sequence sim_frame=42`) (repeatable)
    #[argh(option)]
    _sequence: Vec<String>,
}

fn extension(path: &std::path::Path) -> String {
    path.extension()
        .unwrap_or_default()
        .to_ascii_lowercase()
        .to_string_lossy()
        .to_string()
}

fn load_mesh(rec: &rerun::RecordingStream, args: &Args) -> anyhow::Result<()> {
    let loader = mesh_loader::Loader::default();
    let scene = loader.load_collada(&args.filepath)?;

    for (mesh, mat) in scene.meshes.iter().zip(scene.materials.iter()) {
        let mut mesh3d = rerun::Mesh3D::new(&mesh.vertices);

        if !mesh.normals.is_empty() && !mesh.normals[0].is_empty() {
            mesh3d = mesh3d.with_vertex_normals(&mesh.normals);
        }

        if let Some(diffuse) = &mat.color.diffuse {
            mesh3d = mesh3d.with_albedo_factor(Rgba32::from_unmultiplied_rgba(
                diffuse[0] as u8,
                diffuse[1] as u8,
                diffuse[2] as u8,
                diffuse[3] as u8,
            ));
        }

        // If specified, entity_path_prefix will be the entity_path of the mesh.
        // In other cases, it will be its file path.
        let entity_path = args
            .entity_path_prefix
            .as_deref()
            .map_or_else(
                || rerun::EntityPath::from_file_path(&args.filepath),
                rerun::EntityPath::from,
            )
            .join(&EntityPath::from("/"))
            .join(&EntityPath::from(mesh.name.as_str()));

        rec.log(entity_path, &mesh3d)?;
    }

    Ok::<_, anyhow::Error>(())
}

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();

    let is_file = args.filepath.is_file();
    let is_collada_file = extension(&args.filepath) == "dae";

    // Inform the Rerun Viewer that we do not support that kind of file.
    if !is_file || !is_collada_file {
        #[allow(clippy::exit)]
        std::process::exit(EXTERNAL_DATA_LOADER_INCOMPATIBLE_EXIT_CODE);
    }

    let rec: rerun::RecordingStream = {
        let mut rec = rerun::RecordingStreamBuilder::new(
            args.application_id
                .as_deref()
                .unwrap_or("external_data_loader"),
        );

        if let Some(recording_id) = args.recording_id.as_ref() {
            rec = rec.recording_id(recording_id);
        };

        // The most important part of this: log to standard output so the Rerun Viewer can ingest it!
        rec.stdout()?
    };

    load_mesh(&rec, &args)?;

    Ok::<_, anyhow::Error>(())
}
