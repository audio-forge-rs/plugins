use nih_plug::prelude::*;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use super::MelodyMakerParams;

const EDITOR_WIDTH: u32 = 700;
const EDITOR_HEIGHT: u32 = 500;

#[derive(Lens)]
struct Data {
    params: Arc<MelodyMakerParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (EDITOR_WIDTH, EDITOR_HEIGHT))
}

pub fn create(params: Arc<MelodyMakerParams>) -> Option<Box<dyn Editor>> {
    create_vizia_editor(
        ViziaState::new(|| (EDITOR_WIDTH, EDITOR_HEIGHT)),
        ViziaTheming::Custom,
        move |cx, _| {
            Data {
                params: params.clone(),
            }
            .build(cx);

            // Add stylesheet
            cx.add_stylesheet(include_str!("editor_style.css"))
                .expect("Failed to load stylesheet");

            VStack::new(cx, |cx| {
                // Header
                HStack::new(cx, |cx| {
                    Label::new(cx, "MELODY MAKER")
                        .class("plugin-title")
                        .font_size(36.0)
                        .font_weight(FontWeightKeyword::Bold)
                        .color(Color::rgb(255, 140, 0)); // Bright orange
                    
                    Label::new(cx, "Infinite Melodies, Always in Key")
                        .class("subtitle")
                        .font_size(14.0)
                        .color(Color::rgb(220, 220, 220));
                })
                .class("header")
                .height(Pixels(90.0))
                .col_between(Pixels(20.0));

                // Main controls
                HStack::new(cx, |cx| {
                    // Left column - SHARED parameters
                    VStack::new(cx, |cx| {
                        Label::new(cx, "🌍 SHARED (All Instances)")
                            .class("section-label")
                            .font_size(14.0)
                            .color(Color::rgb(100, 200, 255)); // Blue for shared
                        
                        Label::new(cx, "KEY")
                            .class("param-label")
                            .top(Pixels(15.0));
                        ParamButton::new(cx, Data::params, |params| &params.key)
                            .class("shared-param");

                        Label::new(cx, "MODE")
                            .class("param-label")
                            .top(Pixels(10.0));
                        ParamButton::new(cx, Data::params, |params| &params.mode)
                            .class("shared-param");

                        Label::new(cx, "PROGRESSION")
                            .class("param-label")
                            .top(Pixels(10.0));
                        ParamButton::new(cx, Data::params, |params| &params.progression_type)
                            .class("shared-param");
                    })
                    .class("column")
                    .width(Pixels(220.0));

                    // Middle column - Per-instance controls
                    VStack::new(cx, |cx| {
                        Label::new(cx, "🎸 PER-INSTANCE")
                            .class("section-label")
                            .font_size(14.0)
                            .color(Color::rgb(255, 140, 0)); // Orange for per-instance

                        Label::new(cx, "MELODY STYLE")
                            .class("param-label")
                            .top(Pixels(15.0));
                        ParamButton::new(cx, Data::params, |params| &params.melody_style)
                            .class("style-button");

                        Label::new(cx, "DENSITY")
                            .class("param-label")
                            .top(Pixels(10.0));
                        ParamSlider::new(cx, Data::params, |params| &params.density)
                            .class("param-slider");

                        Label::new(cx, "RANGE")
                            .class("param-label")
                            .top(Pixels(10.0));
                        ParamSlider::new(cx, Data::params, |params| &params.range)
                            .class("param-slider");

                        Label::new(cx, "VARIATION")
                            .class("param-label")
                            .top(Pixels(10.0));
                        ParamSlider::new(cx, Data::params, |params| &params.variation)
                            .class("param-slider");
                    })
                    .class("column")
                    .width(Pixels(220.0));

                    // Right column - Octave, Phrase, Controls
                    VStack::new(cx, |cx| {
                        Label::new(cx, "⚙️ CONTROLS")
                            .class("section-label")
                            .font_size(14.0)
                            .color(Color::rgb(200, 200, 200));

                        Label::new(cx, "OCTAVE")
                            .class("param-label")
                            .top(Pixels(15.0));
                        ParamSlider::new(cx, Data::params, |params| &params.octave)
                            .class("param-slider");

                        Label::new(cx, "PHRASE LENGTH")
                            .class("param-label")
                            .top(Pixels(10.0));
                        ParamSlider::new(cx, Data::params, |params| &params.phrase_length)
                            .class("param-slider");

                        Label::new(cx, "TEMPO")
                            .class("param-label")
                            .top(Pixels(10.0));
                        ParamSlider::new(cx, Data::params, |params| &params.tempo)
                            .class("param-slider");

                        Label::new(cx, "RANDOMIZE")
                            .class("param-label")
                            .top(Pixels(15.0));
                        ParamButton::new(cx, Data::params, |params| &params.randomize)
                            .class("randomize-button");

                        Label::new(cx, "ENABLED")
                            .class("param-label")
                            .top(Pixels(10.0));
                        ParamButton::new(cx, Data::params, |params| &params.enabled)
                            .class("enabled-button");
                    })
                    .class("column")
                    .width(Pixels(220.0));
                })
                .class("main-controls")
                .col_between(Pixels(10.0))
                .height(Stretch(1.0));

                // Footer with info
                VStack::new(cx, |cx| {
                    Label::new(cx, "🌍 Blue params sync across ALL instances | 🎸 Orange params are per-instance unique")
                        .class("footer-text")
                        .font_size(11.0)
                        .color(Color::rgba(200, 200, 200, 200));
                    
                    Label::new(cx, "Pure MIDI Output → Load instrument after this plugin (Session Guitarist, Scarbee, etc.)")
                        .class("footer-text")
                        .font_size(10.0)
                        .color(Color::rgba(180, 180, 180, 180));
                })
                .class("footer")
                .height(Pixels(60.0))
                .row_between(Pixels(5.0));
            })
            .class("container");
        },
    )
}
