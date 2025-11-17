use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::TubeScreamerParams;

#[derive(Lens)]
struct Data {
    params: Arc<TubeScreamerParams>,
}

impl Model for Data {}

// Classic Tube Screamer green color scheme
const TS_GREEN: Color = Color::rgb(76, 153, 76);      // Main green
const TS_DARK_GREEN: Color = Color::rgb(51, 102, 51);  // Dark green
const TS_LIGHT_GREEN: Color = Color::rgb(102, 204, 102); // Light green
const TS_CREAM: Color = Color::rgb(238, 232, 213);     // Cream/beige labels
const TS_BLACK: Color = Color::rgb(20, 20, 20);        // Almost black
const TS_GOLD: Color = Color::rgb(212, 175, 55);       // Gold accents

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (520, 360))
}

pub(crate) fn create(
    params: Arc<TubeScreamerParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
        }
        .build(cx);

        // Main container with classic green background
        VStack::new(cx, |cx| {
            // Title section
            VStack::new(cx, |cx| {
                Label::new(cx, "TUBE SCREAMER")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_weight(FontWeightKeyword::Bold)
                    .font_size(32.0)
                    .color(TS_CREAM)
                    .height(Pixels(50.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Stretch(1.0));
                
                Label::new(cx, "OVERDRIVE")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_size(14.0)
                    .color(TS_GOLD)
                    .height(Pixels(20.0));
            })
            .height(Pixels(90.0))
            .background_color(TS_DARK_GREEN)
            .border_color(TS_BLACK)
            .border_width(Pixels(2.0));

            // Controls section
            HStack::new(cx, |cx| {
                // Drive control
                VStack::new(cx, |cx| {
                    Label::new(cx, "DRIVE")
                        .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                        .font_size(12.0)
                        .font_weight(FontWeightKeyword::Bold)
                        .color(TS_CREAM)
                        .height(Pixels(25.0))
                        .child_top(Pixels(5.0));
                    
                    ParamSlider::new(cx, Data::params, |params| &params.drive)
                        .height(Pixels(120.0))
                        .background_color(TS_BLACK);
                    
                    Label::new(cx, "")
                        .height(Pixels(10.0));
                })
                .width(Pixels(110.0))
                .child_left(Pixels(15.0))
                .child_right(Pixels(15.0));

                // Tone control
                VStack::new(cx, |cx| {
                    Label::new(cx, "TONE")
                        .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                        .font_size(12.0)
                        .font_weight(FontWeightKeyword::Bold)
                        .color(TS_CREAM)
                        .height(Pixels(25.0))
                        .child_top(Pixels(5.0));
                    
                    ParamSlider::new(cx, Data::params, |params| &params.tone)
                        .height(Pixels(120.0))
                        .background_color(TS_BLACK);
                    
                    Label::new(cx, "")
                        .height(Pixels(10.0));
                })
                .width(Pixels(110.0))
                .child_left(Pixels(15.0))
                .child_right(Pixels(15.0));

                // Level control
                VStack::new(cx, |cx| {
                    Label::new(cx, "LEVEL")
                        .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                        .font_size(12.0)
                        .font_weight(FontWeightKeyword::Bold)
                        .color(TS_CREAM)
                        .height(Pixels(25.0))
                        .child_top(Pixels(5.0));
                    
                    ParamSlider::new(cx, Data::params, |params| &params.level)
                        .height(Pixels(120.0))
                        .background_color(TS_BLACK);
                    
                    Label::new(cx, "")
                        .height(Pixels(10.0));
                })
                .width(Pixels(110.0))
                .child_left(Pixels(15.0))
                .child_right(Pixels(15.0));

                // Mix control
                VStack::new(cx, |cx| {
                    Label::new(cx, "MIX")
                        .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                        .font_size(12.0)
                        .font_weight(FontWeightKeyword::Bold)
                        .color(TS_CREAM)
                        .height(Pixels(25.0))
                        .child_top(Pixels(5.0));
                    
                    ParamSlider::new(cx, Data::params, |params| &params.mix)
                        .height(Pixels(120.0))
                        .background_color(TS_BLACK);
                    
                    Label::new(cx, "")
                        .height(Pixels(10.0));
                })
                .width(Pixels(110.0))
                .child_left(Pixels(15.0))
                .child_right(Pixels(15.0));
            })
            .height(Pixels(180.0))
            .top(Pixels(15.0))
            .background_color(TS_GREEN)
            .child_top(Pixels(10.0))
            .col_between(Pixels(5.0));

            // Info section
            VStack::new(cx, |cx| {
                Label::new(cx, "Authentic TS808/TS9 Circuit Emulation")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_size(11.0)
                    .color(TS_CREAM)
                    .height(Pixels(20.0));
                
                Label::new(cx, "• Asymmetric soft clipping • Mid-focused EQ • Classic overdrive tone •")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_size(9.0)
                    .color(TS_LIGHT_GREEN)
                    .height(Pixels(18.0));
            })
            .height(Pixels(60.0))
            .top(Pixels(10.0))
            .background_color(TS_DARK_GREEN)
            .child_top(Pixels(8.0));

            // Footer
            Label::new(cx, "Audio Forge RS")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_size(10.0)
                .color(TS_GOLD)
                .height(Pixels(25.0))
                .background_color(TS_BLACK)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0));
        })
        .background_color(TS_GREEN)
        .border_color(TS_GOLD)
        .border_width(Pixels(3.0));

        ResizeHandle::new(cx);
    })
}
