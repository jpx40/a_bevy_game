use glam::{Vec2, Vec3};
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::read_to_string;
use std::io::Read;
#[derive(Serialize, Deserialize)]
pub struct PlatformFile {
    pub platforms: Vec<PlatformSprite>,
}

#[derive(Serialize, Deserialize)]
pub struct PlatformSprite {
    pub pos: Vec3,
    pub size: Vec2,
}

impl PlatformFile {
    pub fn load(path: &str) -> anyhow::Result<PlatformFile> {
        let mut buf = String::new();

        File::open(path)?.read_to_string(&mut buf)?;

        let res = ron::from_str(&buf)?;
        Ok(res)
    }
}
#[derive(Serialize, Deserialize)]
pub struct FruitFile(pub Vec<FruitSprite>);

#[derive(Serialize, Deserialize)]
pub struct FruitSprite {
    pub pos: Vec3,
    // pub size: f32,
}

impl FruitFile {
    pub fn load(path: &str) -> anyhow::Result<FruitFile> {
        let mut buf = String::new();

        File::open(path)?.read_to_string(&mut buf)?;

        let res = ron::from_str(&buf)?;
        Ok(res)
    }
}
