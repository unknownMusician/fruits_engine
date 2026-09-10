use fruits_asset_storage::AssetStorageResource;
use fruits_ecs::{ResourcesHolderMut, ResourcesHolderRef};
use fruits_render_core::{RenderApiResource, StandardMaterial, StandardMaterialAssetMetadata, StandardMaterialAssets, StandardTexture};
use fruits_serialization::{SerializedValue, SerializerCtx};

pub struct MaterialHandleLoader<'a> {
    pub render_api: &'a RenderApiResource,
    pub materials: &'a mut AssetStorageResource<StandardMaterial>,
}

impl<'a> MaterialHandleLoader<'a> {
    pub fn from_world(res: ResourcesHolderMut<'a>) -> Option<Self> {
        Some(unsafe { Self {
            render_api: &*res.get_ptr::<RenderApiResource>()?,
            materials: &mut *res.get_ptr::<AssetStorageResource<StandardMaterial>>()?,
        }})
    }
}

//

pub struct MaterialLoader<'a> {
    pub render_api: &'a RenderApiResource,
}

impl<'a> MaterialLoader<'a> {
    pub fn from_world(res: ResourcesHolderRef<'a>) -> Option<Self> {
        Some(Self {
            render_api: res.get::<RenderApiResource>()?,
        })
    }

    pub fn load_from_serialized(
        &mut self,
        mut ctx: SerializerCtx,
        value: &SerializedValue,
        textures: &AssetStorageResource<StandardTexture>,
    ) -> Option<StandardMaterial> {
        let asset_metadata = ctx.deserialize::<StandardMaterialAssetMetadata>(value)?;

        self.load_from_deserialized(asset_metadata, textures)
    }

    pub fn load_from_deserialized(
        &mut self,
        asset_metadata: StandardMaterialAssetMetadata,
        textures: &AssetStorageResource<StandardTexture>,
    ) -> Option<StandardMaterial> {
        // todo: read texture's native part to create material
        // todo: fallback to white texture on fail
        let material = self.render_api.create_material(
            StandardMaterialAssets {
                color_texture: textures.get(&asset_metadata.color_tex),
                roughness_texture: textures.get(&asset_metadata.roughness_tex),
                metallic_texture: textures.get(&asset_metadata.metallic_tex),
                normal_texture: textures.get(&asset_metadata.normal_tex),
                emission_texture: textures.get(&asset_metadata.emission_tex),
            },
            asset_metadata,
        );

        Some(material)
    }
}
