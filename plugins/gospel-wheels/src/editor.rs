use nih_plug::prelude::*;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use super::GospelWheelsParams;

const EDITOR_WIDTH: u32 = 600;
const EDITOR_HEIGHT: u32 = 400;

#[derive(Lens)]
struct Data {
    params: Arc<GospelWheelsParams>,
}

impl Model for Data {}

pub(crate) fn create(params: Arc<GospelWheelsParams>) -> Option<Box<dyn Editor>> {
    create_vizia_editor(
        ViziaState::new(|| (EDITOR_WIDTH, EDITOR_HEIGHT)),
        ViziaTheming::Custom,
        move |cx, _| {
            Data {
                params: params.clone(),
            }
            .build(cx);

            // Vintage organ color scheme - burgundy, gold, cream, dark wood
            cx.add_stylesheet(include_str!("editor_style.css"))
                .expect("Failed to load stylesheet");

            VStack::new(cx, |cx| {
                // Header
                HStack::new(cx, |cx| {
                    Label::new(cx, "GOSPEL WHEELS")
                        .class("plugin-title")
                        .font_size(32.0)
                        .font_weight(FontWeightKeyword::Bold)
                        .color(Color::rgb(212, 175, 55)); // Gold
                    
                    Label::new(cx, "Open Road Hymns")
                        .class("subtitle")
                        .font_size(14.0)
                        .color(Color::rgb(245, 245, 220)); // Cream
                })
                .class("header")
                .height(Pixels(80.0))
                .col_between(Pixels(20.0));

                // Main controls section
                HStack::new(cx, |cx| {
                    // Left column - Style and core parameters
                    VStack::new(cx, |cx| {
                        Label::new(cx, "STYLE")
                            .class("section-label")
                            .font_size(12.0)
                            .color(Color::rgb(212, 175, 55));

                        ParamButton::new(cx, Data::params, |params| &params.style)
                            .class("style-selector");

                        Label::new(cx, "HARMONICS")
                            .class("param-label")
                            .top(Pixels(20.0));
                        ParamSlider::new(cx, Data::params, |params| &params.harmonics)
                            .class("param-slider");

                        Label::new(cx, "SWELL")
                            .class("param-label")
                            .top(Pixels(10.0));
                        ParamSlider::new(cx, Data::params, |params| &params.swell)
                            .class("param-slider");
                    })
                    .class("column")
                    .width(Pixels(180.0));

                    // Middle column - Rhythm and voicing
                    VStack::new(cx, |cx| {
                        Label::new(cx, "RHYTHM")
                            .class("param-label");
                        ParamSlider::new(cx, Data::params, |params| &params.rhythm)
                            .class("param-slider");

                        Label::new(cx, "VOICING")
                            .class("param-label")
                            .top(Pixels(20.0));
                        ParamSlider::new(cx, Data::params, |params| &params.voicing)
                            .class("param-slider");

                        Label::new(cx, "REGISTER")
                            .class("param-label")
                            .top(Pixels(20.0));
                        ParamSlider::new(cx, Data::params, |params| &params.register)
                            .class("param-slider");
                    })
                    .class("column")
                    .width(Pixels(180.0));

                    // Right column - Options
                    VStack::new(cx, |cx| {
                        Label::new(cx, "AUTO-THICKEN")
                            .class("param-label");
                        ParamButton::new(cx, Data::params, |params| &params.thicken)
                            .class("toggle-button");

                        // Info section
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Intelligent Hammond")
                                .class("info-text")
                                .font_size(11.0)
                                .color(Color::rgb(245, 245, 220));
                            
                            Label::new(cx, "Chord Analysis • Voice Leading")
                                .class("info-text")
                                .font_size(10.0)
                                .color(Color::rgba(245, 245, 220, 160));
                            
                            Label::new(cx, "Drawbar Simulation")
                                .class("info-text")
                                .font_size(10.0)
                                .color(Color::rgba(245, 245, 220, 160));
                        })
                        .class("info-box")
                        .top(Pixels(30.0))
                        .row_between(Pixels(4.0));
                    })
                    .class("column")
                    .width(Pixels(180.0));
                })
                .class("main-controls")
                .col_between(Pixels(20.0))
                .height(Stretch(1.0));

                // Footer with style descriptions
                VStack::new(cx, |cx| {
                    Label::new(cx, "Sustained: Long held chords • Comping: Rhythmic stabs • Swell: Dynamic swells")
                        .class("footer-text")
                        .font_size(10.0)
                        .color(Color::rgba(245, 245, 220, 180));
                    
                    Label::new(cx, "Arpeggiated: Broken chords • Bass: Left hand bass • Atmospheric: Sparse & spacious")
                        .class("footer-text")
                        .font_size(10.0)
                        .color(Color::rgba(245, 245, 220, 180));
                })
                .class("footer")
                .height(Pixels(50.0))
                .row_between(Pixels(4.0));
            })
            .class("container");
        },
    )
}
