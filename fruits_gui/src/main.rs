use bevy::{app::{App, Startup}, prelude::*, winit::WinitSettings, DefaultPlugins};
use fruits_serialization::json_reflection::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));

    let mut object = JsonObject::new();
    object.push_field("name", String::from("Peter")).ok().unwrap();
    object.push_field("age", 25).ok().unwrap();
    object.push_field("friend", {
        let mut friend = JsonObject::new();
        friend.push_field("name", String::from("Alex")).ok().unwrap();
        friend.push_field("age", 22).ok().unwrap();
        friend.push_field("account", {
            let mut account = JsonObject::new();
            account.push_field("name", String::from("superhero1337")).ok().unwrap();
            account.push_field("password", String::from("123456789")).ok().unwrap();
            account
        }).ok().unwrap();
        friend
    }).ok().unwrap();
    object.push_field("account", {
        let mut account = JsonObject::new();
        account.push_field("name", String::from("underground_sniper")).ok().unwrap();
        account.push_field("password", String::from("und777sn1pr")).ok().unwrap();
        account
    }).ok().unwrap();
    object.push_field("nicknames", {
        let mut nicknames = JsonArray::new();
        nicknames.elements_mut().push(String::from("superhot").into());
        nicknames.elements_mut().push(String::from("noobs_killer").into());
        nicknames.elements_mut().push(String::from("password-rememberer").into());
        nicknames.elements_mut().push(String::from("bestplay3r").into());
        nicknames
    }).ok().unwrap();


    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        ..Default::default()
    }).with_children(|parent| {
        for (name, value) in object.fields() {
            spawn_field(parent, &asset_server, name, value, 1);
        }
    });
}

fn spawn_field(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    name: &str,
    value: &JsonValue,
    indent: usize,
) {
    // todo: don't use recursion
    if let JsonValue::Object(object) = value {
        spawn_node_with_text(parent, asset_server, name, "", indent);
        for (name, value) in object.fields() {
            spawn_field(parent, asset_server, name, value, indent + 1);
        }
    } else if let JsonValue::Array(array) = value {
        spawn_node_with_text(parent, asset_server, name, "", indent);
        for element in array.elements().iter() {
            spawn_field(parent, asset_server, "", element, indent + 1);
        }
    } else if let JsonValue::String(string) = value {
        spawn_node_with_text(parent, asset_server, name, string, indent);
    } else {
        spawn_node_with_text(parent, asset_server, name, &value.to_json(&mut JsonIndentation::Default), indent);
    }
}

fn spawn_node_with_text(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    name: &str,
    value: &str,
    indent: usize,
) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            display: Display::Flex,
            justify_content: JustifyContent::End,
            ..Default::default()
        },
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(indent as f32 * 20.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        });

        parent.spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                flex_grow: 1.0,
                ..Default::default()
            },
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                name,
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 30.0,
                    ..default()
                },
            ));
        });

        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                value,
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 30.0,
                    ..default()
                },
            ));
        });
    });
}
