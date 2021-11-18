use {
    crate::styles::colors,
    iced::{slider, Color},
};

pub struct Slider;

impl slider::StyleSheet for Slider {
    fn active(&self) -> slider::Style {
        slider::Style {
            rail_colors: (
                colors::blue::_500,
                Color {
                    a: 0.1,
                    ..colors::blue::_500
                },
            ),
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 9. },
                color: colors::blue::_500,
                border_width: 0.,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self) -> slider::Style {
        let active = self.active();

        slider::Style {
            handle: slider::Handle {
                color: colors::blue::_700,
                ..active.handle
            },
            ..active
        }
    }

    fn dragging(&self) -> slider::Style {
        let active = self.active();

        slider::Style {
            handle: slider::Handle {
                color: colors::blue::_300,
                ..active.handle
            },
            ..active
        }
    }
}
