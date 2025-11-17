use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::LowRiderParams;

#[derive(Lens)]
struct Data {
    params: Arc<LowRiderParams>,
}

impl Model for Data {}

// Bass/funk inspired color scheme
const BASS_DEEP_PURPLE: Color = Color::rgb(75, 45, 95);     // Deep purple
const BASS_ORANGE: Color = Color::rgb(235, 115, 45);        // Warm orange
const BASS_DARK: Color = Color::rgb(28, 18, 35);            // Almost black with purple
const BASS_CREAM: Color = Color::rgb(248, 240, 220);        // Warm cream
const BASS_GOLD: Color = Color::rgb(218, 165, 32);          // Goldenrod
const BASS_CHARCOAL: Color = Color::rgb(45, 35, 48);        // Dark charcoal

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (600, 480))
}

pub(crate) fn create(
    params: Arc<LowRiderParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            // Header
            VStack::new(cx, |cx| {
                Label::new(cx, "LOW RIDER")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_weight(FontWeightKeyword::Bold)
                    .font_size(38.0)
                    .color(BASS_CREAM)
                    .height(Pixels(55.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Stretch(1.0));
                
                Label::new(cx, "Intelligent Bass Line Generator")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_size(14.0)
                    .color(BASS_GOLD)
                    .height(Pixels(24.0));
            })
            .height(Pixels(95.0))
            .background_color(BASS_DEEP_PURPLE)
            .border_color(BASS_DARK)
            .border_width(Pixels(2.0));

            // Main controls
            VStack::new(cx, |cx| {
                // Style and Tempo row
                HStack::new(cx, |cx| {
                    // Bass Style
                    VStack::new(cx, |cx| {
                        Label::new(cx, "BASS STYLE")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(12.0)
                            .font_weight(FontWeightKeyword::Bold)
                            .color(BASS_CREAM)
                            .height(Pixels(25.0));
                        
                        ParamSlider::new(cx, Data::params, |params| &params.style)
                            .height(Pixels(40.0))
                            .background_color(BASS_DARK);
                    })
                    .width(Stretch(2.0))
                    .child_left(Pixels(15.0))
                    .child_right(Pixels(10.0));

                    // Tempo
                    VStack::new(cx, |cx| {
                        Label::new(cx, "TEMPO")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(12.0)
                            .font_weight(FontWeightKeyword::Bold)
                            .color(BASS_CREAM)
                            .height(Pixels(25.0));
                        
                        ParamSlider::new(cx, Data::params, |params| &params.tempo)
                            .height(Pixels(40.0))
                            .background_color(BASS_DARK);
                    })
                    .width(Stretch(1.0))
                    .child_left(Pixels(10.0))
                    .child_right(Pixels(15.0));
                })
                .height(Pixels(80.0))
                .background_color(BASS_ORANGE)
                .child_top(Pixels(10.0));

                // Control parameters
                HStack::new(cx, |cx| {
                    // Activity
                    VStack::new(cx, |cx| {
                        Label::new(cx, "ACTIVITY")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(11.0)
                            .font_weight(FontWeightKeyword::Bold)
                            .color(BASS_CREAM)
                            .height(Pixels(22.0));
                        
                        ParamSlider::new(cx, Data::params, |params| &params.activity)
                            .height(Pixels(100.0))
                            .background_color(BASS_DARK);
                    })
                    .width(Stretch(1.0))
                    .child_left(Pixels(10.0))
                    .child_right(Pixels(10.0));

                    // Syncopation
                    VStack::new(cx, |cx| {
                        Label::new(cx, "SYNCOPATION")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(11.0)
                            .font_weight(FontWeightKeyword::Bold)
                            .color(BASS_CREAM)
                            .height(Pixels(22.0));
                        
                        ParamSlider::new(cx, Data::params, |params| &params.syncopation)
                            .height(Pixels(100.0))
                            .background_color(BASS_DARK);
                    })
                    .width(Stretch(1.0))
                    .child_left(Pixels(10.0))
                    .child_right(Pixels(10.0));

                    // Movement
                    VStack::new(cx, |cx| {
                        Label::new(cx, "MOVEMENT")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(11.0)
                            .font_weight(FontWeightKeyword::Bold)
                            .color(BASS_CREAM)
                            .height(Pixels(22.0));
                        
                        ParamSlider::new(cx, Data::params, |params| &params.movement)
                            .height(Pixels(100.0))
                            .background_color(BASS_DARK);
                    })
                    .width(Stretch(1.0))
                    .child_left(Pixels(10.0))
                    .child_right(Pixels(10.0));

                    // Articulation
                    VStack::new(cx, |cx| {
                        Label::new(cx, "ARTICULATION")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(11.0)
                            .font_weight(FontWeightKeyword::Bold)
                            .color(BASS_CREAM)
                            .height(Pixels(22.0));
                        
                        ParamSlider::new(cx, Data::params, |params| &params.articulation)
                            .height(Pixels(100.0))
                            .background_color(BASS_DARK);
                    })
                    .width(Stretch(1.0))
                    .child_left(Pixels(10.0))
                    .child_right(Pixels(10.0));

                    // Sustain
                    VStack::new(cx, |cx| {
                        Label::new(cx, "SUSTAIN")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(11.0)
                            .font_weight(FontWeightKeyword::Bold)
                            .color(BASS_CREAM)
                            .height(Pixels(22.0));
                        
                        ParamSlider::new(cx, Data::params, |params| &params.sustain)
                            .height(Pixels(100.0))
                            .background_color(BASS_DARK);
                    })
                    .width(Stretch(1.0))
                    .child_left(Pixels(10.0))
                    .child_right(Pixels(10.0));
                })
                .height(Pixels(140.0))
                .top(Pixels(10.0))
                .background_color(BASS_CHARCOAL)
                .child_top(Pixels(10.0));

                // Octave control
                VStack::new(cx, |cx| {
                    Label::new(cx, "OCTAVE PREFERENCE")
                        .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                        .font_size(12.0)
                        .font_weight(FontWeightKeyword::Bold)
                        .color(BASS_CREAM)
                        .height(Pixels(25.0))
                        .child_top(Pixels(8.0));
                    
                    ParamSlider::new(cx, Data::params, |params| &params.octave)
                        .height(Pixels(40.0))
                        .background_color(BASS_DARK);
                })
                .height(Pixels(80.0))
                .top(Pixels(10.0))
                .background_color(BASS_DEEP_PURPLE)
                .child_left(Pixels(15.0))
                .child_right(Pixels(15.0));
            })
            .background_color(BASS_CHARCOAL);

            // Info section
            VStack::new(cx, |cx| {
                Label::new(cx, "Optimized for Scarbee Rickenbacker Bass • Analyzes chords, generates bass lines")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_size(10.0)
                    .color(BASS_CREAM)
                    .height(Pixels(18.0));
                
                Label::new(cx, "Smart note selection • Rhythmic patterns • Automatic articulations • Musical intelligence")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_size(9.0)
                    .color(BASS_GOLD)
                    .height(Pixels(16.0));
            })
            .height(Pixels(50.0))
            .background_color(BASS_DEEP_PURPLE)
            .child_top(Pixels(6.0));

            // Footer
            Label::new(cx, "Audio Forge RS • Alt-Country Bass Vibes")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_size(11.0)
                .color(BASS_GOLD)
                .height(Pixels(28.0))
                .background_color(BASS_DARK)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0));
        })
        .background_color(BASS_CHARCOAL)
        .border_color(BASS_ORANGE)
        .border_width(Pixels(3.0));

        ResizeHandle::new(cx);
    })
}
