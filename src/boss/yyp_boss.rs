use super::{
    texture_group_ext::TextureGroupExt,
    yy_typings::{
        resources::{sprite::*, texture_group::*, ResourceType},
        yyp::*,
    },
    YyResource,
};
use anyhow::Result;
use std::collections::HashMap;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct YypBoss {
    yyp: Yyp,
    dirty: bool,
    absolute_path: PathBuf,
    sprites: YyResourceHandler<Sprite>,
    texture_groups: Vec<TextureGroup>,
}

impl YypBoss {
    /// Creates a new YyBoss Manager. Note that it performs the
    /// fs itself here. See `with_components` if handling this in
    /// some other way.
    pub fn new(path_to_yyp: PathBuf) -> Result<YypBoss> {
        let yy_file = fs::read_to_string(&path_to_yyp)?;
        let yyp: Yyp = serde_json::from_str(&yy_file)?;

        let mut yyp_boss = Self {
            yyp,
            absolute_path: path_to_yyp,
            dirty: false,
            sprites: YyResourceHandler::new(),
            texture_groups: Vec::new(),
        };

        // Load in TextureGroups...*shudder*
        if let Some(main_options) = yyp_boss
            .yyp
            .resources
            .iter()
            .find_map(|value: &YypResource| {
                if value.value.resource_type == ResourceType::GmMainOptions {
                    Some(value.value.resource_path.clone())
                } else {
                    None
                }
            })
        {
            let options_path = yyp_boss.absolute_path.join(main_options);
            let options_text = fs::read_to_string(options_path)?;

            yyp_boss.texture_groups = TextureGroup::parse_options_file(&options_text)?;
        }

        // Load in Sprites
        for sprite_resources in yyp_boss
            .yyp
            .resources
            .iter()
            .filter(|value| value.value.resource_type == ResourceType::GmSprite)
        {
            let sprite_resource: &YypResource = sprite_resources;
            let sprite_path = yyp_boss
                .absolute_path
                .parent()
                .unwrap()
                .join(&sprite_resource.value.resource_path);

            let sprite_yy: Sprite = deserialize(&sprite_path)?;

            let frame_buffers: Vec<_> = sprite_yy
                .frames
                .iter()
                .filter_map(|frame: &Frame| {
                    let path_to_image = sprite_path
                        .parent()
                        .unwrap()
                        .join(Path::new(&frame.id.inner().to_string()).with_extension(".png"));

                    match image::open(&path_to_image) {
                        Ok(image) => Some((frame.id, image.to_rgba())),
                        Err(e) => {
                            log::error!("We couldn't read {:?} -- {}", path_to_image, e);
                            None
                        }
                    }
                })
                .collect();

            yyp_boss.sprites.add_new(sprite_yy, frame_buffers);
        }

        Ok(yyp_boss)
    }

    pub fn add_sprite(
        &mut self,
        sprite: Sprite,
        associated_data: <Sprite as YyResource>::AssociatedData,
    ) {
        self.add_new_resource(&sprite, None);
        self.sprites.add_new(sprite, associated_data);
    }

    /// Adds a new Resource to be tracked by the YYP. The Resource also will
    /// need to serialize themselves and any additional files which they manage.
    ///
    /// This might include serializing sprites or sprite frames for Sprites, or `.gml`
    /// files for scripts or objects.
    fn add_new_resource(
        &mut self,
        new_resource: &impl YyResource,
        config_deltas: Option<Vec<String>>,
    ) {
        // New Resource:
        let new_yy_resource = YypResource {
            key: new_resource.id().into(),
            value: YypResourceValue {
                config_deltas,
                id: YypResourceId::new(),
                resource_path: new_resource.relative_filepath().to_owned(),
                resource_type: new_resource.yy_resource_type(),
            },
        };

        // Update the Resource
        self.yyp.resources.push(new_yy_resource);
        self.dirty = true;
    }

    pub fn serialize(&mut self) -> Result<()> {
        if self.dirty {
            self.yyp
                .resources
                .sort_by(|lr, rr| lr.value.id.inner().cmp(&rr.value.id.inner()));
            // Serialize Ourselves:
            serialize(&self.absolute_path, &self.yyp)?;

            // Check if Sprite is Dirty and Serialize that:
            self.sprites.serialize()?;

            self.dirty = false;
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct YyResourceData<T: YyResource> {
    pub yy_resouce: T,
    pub associated_data: T::AssociatedData,
}

#[derive(Debug, Default)]
pub struct YyResourceHandler<T: YyResource> {
    dirty: bool,
    resources: HashMap<T::Key, YyResourceData<T>>,
    dirty_resources: Vec<T::Key>,
}

impl<T: YyResource> YyResourceHandler<T> {
    pub fn new() -> Self {
        Self {
            dirty: false,
            resources: HashMap::new(),
            dirty_resources: Vec::new(),
        }
    }

    pub fn add_new(&mut self, value: T, associated_data: T::AssociatedData) {
        self.dirty_resources.push(value.id());
        self.dirty = true;

        self.resources.insert(
            value.id(),
            YyResourceData {
                yy_resouce: value,
                associated_data,
            },
        );
    }

    pub fn serialize(&mut self) -> Result<()> {
        if self.dirty {
            while let Some(dirty_resource) = self.dirty_resources.pop() {
                let resource = self
                    .resources
                    .get(&dirty_resource)
                    .expect("This should always be valid.");

                let yy_path = resource.yy_resouce.relative_filepath();
                serialize(&yy_path, &resource.yy_resouce)?;

                if let Some(parent_dir) = yy_path.parent() {
                    T::serialize_associated_data(
                        &resource.yy_resouce,
                        parent_dir,
                        &resource.associated_data,
                    )?;
                }
            }
        }

        Ok(())
    }
}

fn serialize(absolute_path: &Path, data: &impl serde::Serialize) -> Result<()> {
    let data = serde_json::to_string_pretty(data)?;
    fs::write(absolute_path, data)?;
    Ok(())
}

fn deserialize<T>(path: &Path) -> Result<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    let file_string = fs::read_to_string(path)?;
    let data = serde_json::from_str(&file_string)?;
    Ok(data)
}