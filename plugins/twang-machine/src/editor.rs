use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::TwangMachineParams;

#[derive(Lens)]
struct Data {
    params: Arc<TwangMachineParams>,
}

impl Model for Data {}

// Alt-country/Americana color scheme
const TWANG_RUST: Color = Color::rgb(165, 75, 42);       // Rusty orange
const TWANG_CREAM: Color = Color::rgb(242, 235, 211);    // Cream
const TWANG_DARK_BROWN: Color = Color::rgb(62, 39, 35);  // Dark brown
const TWANG_DENIM: Color = Color::rgb(72, 91, 115);      // Denim blue
const TWANG_WHEAT: Color = Color::rgb(210, 180, 140);    // Wheat/tan
const TWANG_BLACK: Color = Color::rgb(28, 20, 13);       // Almost black

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (560, 420))
}

pub(crate) fn create(
    params: Arc<TwangMachineParams>,
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
                Label::new(cx, "TWANG MACHINE")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_weight(FontWeightKeyword::Bold)
                    .font_size(34.0)
                    .color(TWANG_CREAM)
                    .height(Pixels(50.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Stretch(1.0));
                
                Label::new(cx, "Intelligent Guitar MIDI Processor")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_size(13.0)
                    .color(TWANG_WHEAT)
                    .height(Pixels(22.0));
            })
            .height(Pixels(90.0))
            .background_color(TWANG_RUST)
            .border_color(TWANG_BLACK)
            .border_width(Pixels(2.0));

            // Main controls section
            VStack::new(cx, |cx| {
                // Mode selection
                VStack::new(cx, |cx| {
                    Label::new(cx, "PLAY MODE")
                        .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                        .font_size(12.0)
                        .font_weight(FontWeightKeyword::Bold)
                        .color(TWANG_CREAM)
                        .height(Pixels(25.0))
                        .child_top(Pixels(5.0));
                    
                    ParamSlider::new(cx, Data::params, |params| &params.mode)
                        .height(Pixels(35.0))
                        .background_color(TWANG_BLACK);
                })
                .height(Pixels(75.0))
                .background_color(TWANG_DENIM)
                .child_left(Pixels(15.0))
                .child_right(Pixels(15.0));

                // Parameter controls
                HStack::new(cx, |cx| {
                    // Strum Speed
                    VStack::new(cx, |cx| {
                        Label::new(cx, "STRUM SPEED")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(11.0)
                            .font_weight(FontWeightKeyword::Bold)
                            .color(TWANG_CREAM)
                            .height(Pixels(22.0))
                            .child_top(Pixels(5.0));
                        
                        ParamSlider::new(cx, Data::params, |params| &params.strum_speed)
                            .height(Pixels(90.0))
                            .background_color(TWANG_BLACK);
                    })
                    .width(Stretch(1.0))
                    .child_left(Pixels(10.0))
                    .child_right(Pixels(10.0));

                    // Humanize
                    VStack::new(cx, |cx| {
                        Label::new(cx, "HUMANIZE")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(11.0)
                            .font_weight(FontWeightKeyword::Bold)
                            .color(TWANG_CREAM)
                            .height(Pixels(22.0))
                            .child_top(Pixels(5.0));
                        
                        ParamSlider::new(cx, Data::params, |params| &params.humanize)
                            .height(Pixels(90.0))
                            .background_color(TWANG_BLACK);
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
                            .color(TWANG_CREAM)
                            .height(Pixels(22.0))
                            .child_top(Pixels(5.0));
                        
                        ParamSlider::new(cx, Data::params, |params| &params.articulation)
                            .height(Pixels(90.0))
                            .background_color(TWANG_BLACK);
                    })
                    .width(Stretch(1.0))
                    .child_left(Pixels(10.0))
                    .child_right(Pixels(10.0));
                })
                .height(Pixels(130.0))
                .top(Pixels(10.0))
                .background_color(TWANG_DARK_BROWN)
                .child_top(Pixels(8.0))
                .col_between(Pixels(5.0));

                // Auto-transpose section
                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "AUTO TRANSPOSE")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(12.0)
                            .font_weight(FontWeightKeyword::Bold)
                            .color(TWANG_CREAM)
                            .width(Stretch(1.0));
                        
                        ParamButton::new(cx, Data::params, |params| &params.auto_transpose)
                            .width(Pixels(60.0));
                    })
                    .height(Pixels(30.0))
                    .col_between(Pixels(10.0));
                    
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Target Center Note")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(11.0)
                            .color(TWANG_WHEAT)
                            .width(Pixels(140.0));
                        
                        ParamSlider::new(cx, Data::params, |params| &params.target_center)
                            .width(Stretch(1.0))
                            .background_color(TWANG_BLACK);
                    })
                    .height(Pixels(35.0))
                    .top(Pixels(5.0))
                    .col_between(Pixels(10.0));
                })
                .height(Pixels(85.0))
                .top(Pixels(10.0))
                .background_color(TWANG_DENIM)
                .child_left(Pixels(15.0))
                .child_right(Pixels(15.0))
                .child_top(Pixels(8.0));
            })
            .background_color(TWANG_DARK_BROWN);

            // Info section
            VStack::new(cx, |cx| {
                Label::new(cx, "Optimized for NI Session Guitarist Electric Sunburst (Melody/Mono Mode)")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_size(10.0)
                    .color(TWANG_CREAM)
                    .height(Pixels(18.0));
                
                Label::new(cx, "Liberal input • Smart transposition • Intelligent strumming • Automatic articulations")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_size(9.0)
                    .color(TWANG_WHEAT)
                    .height(Pixels(16.0));
            })
            .height(Pixels(50.0))
            .background_color(TWANG_RUST)
            .child_top(Pixels(6.0));

            // Footer
            Label::new(cx, "Audio Forge RS • Alt-Country Guitar Vibes")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_size(10.0)
                .color(TWANG_WHEAT)
                .height(Pixels(25.0))
                .background_color(TWANG_BLACK)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0));
        })
        .background_color(TWANG_DARK_BROWN)
        .border_color(TWANG_RUST)
        .border_width(Pixels(3.0));

        ResizeHandle::new(cx);
    })
}
