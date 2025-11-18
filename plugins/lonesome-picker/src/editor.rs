use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::LonesonmePickerParams;

#[derive(Lens)]
struct Data {
    params: Arc<LonesonmePickerParams>,
}

impl Model for Data {}

// Americana/banjo inspired color scheme
const BANJO_WOOD: Color = Color::rgb(139, 90, 43);          // Wood brown
const BANJO_LIGHT_WOOD: Color = Color::rgb(205, 170, 125);  // Light wood
const BANJO_BRASS: Color = Color::rgb(181, 166, 66);        // Brass fittings
const BANJO_CREAM: Color = Color::rgb(255, 253, 240);       // Off-white head
const BANJO_DARK: Color = Color::rgb(52, 34, 18);           // Dark brown/black
const BANJO_SAGE: Color = Color::rgb(145, 163, 125);        // Sage green

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (580, 460))
}

pub(crate) fn create(
    params: Arc<LonesonmePickerParams>,
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
                Label::new(cx, "LONESOME PICKER")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_weight(FontWeightKeyword::Bold)
                    .font_size(40.0)
                    .color(BANJO_CREAM)
                    .height(Pixels(56.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Stretch(1.0));
                
                Label::new(cx, "Intelligent Banjo Processor")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_size(14.0)
                    .color(BANJO_BRASS)
                    .height(Pixels(24.0));
            })
            .height(Pixels(96.0))
            .background_color(BANJO_WOOD)
            .border_color(BANJO_DARK)
            .border_width(Pixels(2.0));

            // Main controls
            VStack::new(cx, |cx| {
                // Style and Tempo row
                HStack::new(cx, |cx| {
                    // Picking Style
                    VStack::new(cx, |cx| {
                        Label::new(cx, "PICKING STYLE")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(12.0)
                            .font_weight(FontWeightKeyword::Bold)
                            .color(BANJO_CREAM)
                            .height(Pixels(25.0));
                        
                        ParamSlider::new(cx, Data::params, |params| &params.style)
                            .height(Pixels(40.0))
                            .background_color(BANJO_DARK);
                    })
                    .width(Stretch(2.0))
                    .child_left(Pixels(15.0))
                    .child_right(Pixels(10.0));

                    // Tempo
                })
                .height(Pixels(80.0))
                .background_color(BANJO_LIGHT_WOOD)
                .child_top(Pixels(10.0));

                // Control parameters - Top row
                HStack::new(cx, |cx| {
                    // Density
                    VStack::new(cx, |cx| {
                        Label::new(cx, "DENSITY")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(11.0)
                            .font_weight(FontWeightKeyword::Bold)
                            .color(BANJO_CREAM)
                            .height(Pixels(22.0));
                        
                        ParamSlider::new(cx, Data::params, |params| &params.density)
                            .height(Pixels(90.0))
                            .background_color(BANJO_DARK);
                    })
                    .width(Stretch(1.0))
                    .child_left(Pixels(10.0))
                    .child_right(Pixels(10.0));

                    // Drone
                    VStack::new(cx, |cx| {
                        Label::new(cx, "DRONE STRING")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(11.0)
                            .font_weight(FontWeightKeyword::Bold)
                            .color(BANJO_CREAM)
                            .height(Pixels(22.0));
                        
                        ParamSlider::new(cx, Data::params, |params| &params.drone)
                            .height(Pixels(90.0))
                            .background_color(BANJO_DARK);
                    })
                    .width(Stretch(1.0))
                    .child_left(Pixels(10.0))
                    .child_right(Pixels(10.0));

                    // Melodic
                    VStack::new(cx, |cx| {
                        Label::new(cx, "MELODIC")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(11.0)
                            .font_weight(FontWeightKeyword::Bold)
                            .color(BANJO_CREAM)
                            .height(Pixels(22.0));
                        
                        ParamSlider::new(cx, Data::params, |params| &params.melodic)
                            .height(Pixels(90.0))
                            .background_color(BANJO_DARK);
                    })
                    .width(Stretch(1.0))
                    .child_left(Pixels(10.0))
                    .child_right(Pixels(10.0));
                })
                .height(Pixels(130.0))
                .top(Pixels(10.0))
                .background_color(BANJO_WOOD)
                .child_top(Pixels(10.0));

                // Control parameters - Bottom row
                HStack::new(cx, |cx| {
                    // Articulation
                    VStack::new(cx, |cx| {
                        Label::new(cx, "ARTICULATION")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(11.0)
                            .font_weight(FontWeightKeyword::Bold)
                            .color(BANJO_CREAM)
                            .height(Pixels(22.0));
                        
                        ParamSlider::new(cx, Data::params, |params| &params.articulation)
                            .height(Pixels(90.0))
                            .background_color(BANJO_DARK);
                    })
                    .width(Stretch(1.0))
                    .child_left(Pixels(10.0))
                    .child_right(Pixels(10.0));

                    // Sparseness
                    VStack::new(cx, |cx| {
                        Label::new(cx, "SPARSENESS")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(11.0)
                            .font_weight(FontWeightKeyword::Bold)
                            .color(BANJO_CREAM)
                            .height(Pixels(22.0));
                        
                        ParamSlider::new(cx, Data::params, |params| &params.sparseness)
                            .height(Pixels(90.0))
                            .background_color(BANJO_DARK);
                    })
                    .width(Stretch(1.0))
                    .child_left(Pixels(10.0))
                    .child_right(Pixels(10.0));

                    // Auto Transpose toggle
                    VStack::new(cx, |cx| {
                        Label::new(cx, "AUTO TRANSPOSE")
                            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                            .font_size(11.0)
                            .font_weight(FontWeightKeyword::Bold)
                            .color(BANJO_CREAM)
                            .height(Pixels(22.0));
                        
                        ParamButton::new(cx, Data::params, |params| &params.auto_transpose)
                            .height(Pixels(90.0));
                    })
                    .width(Stretch(1.0))
                    .child_left(Pixels(10.0))
                    .child_right(Pixels(10.0));
                })
                .height(Pixels(130.0))
                .top(Pixels(10.0))
                .background_color(BANJO_SAGE)
                .child_top(Pixels(10.0));
            })
            .background_color(BANJO_WOOD);

            // Info section
            VStack::new(cx, |cx| {
                Label::new(cx, "Optimized for Ample Ethno Banjo (Picking Mode) • G-D-G-B-D Tuning")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_size(10.0)
                    .color(BANJO_CREAM)
                    .height(Pixels(18.0));
                
                Label::new(cx, "Sparse atmospheric picking • Melodic phrases • Drone string • Alt-country Americana vibes")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_size(9.0)
                    .color(BANJO_BRASS)
                    .height(Pixels(16.0));
            })
            .height(Pixels(50.0))
            .background_color(BANJO_LIGHT_WOOD)
            .child_top(Pixels(6.0));

            // Footer
            Label::new(cx, "Audio Forge RS • Lonesome Prairie Sounds")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_size(11.0)
                .color(BANJO_BRASS)
                .height(Pixels(28.0))
                .background_color(BANJO_DARK)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0));
        })
        .background_color(BANJO_WOOD)
        .border_color(BANJO_BRASS)
        .border_width(Pixels(3.0));

        ResizeHandle::new(cx);
    })
}
