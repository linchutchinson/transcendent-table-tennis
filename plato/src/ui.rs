use legion::{
    systems::{Builder, CommandBuffer},
    world::SubWorld,
    *,
};
use macroquad::{
    prelude::{
        is_mouse_button_down, is_mouse_button_pressed, Color, Rect, Vec2, BLUE, DARKBLUE, WHITE,
    },
    shapes::{draw_circle, draw_line, draw_rectangle},
    text::{draw_text, measure_text},
    window::{screen_height, screen_width},
};

#[cfg(feature = "debug_ui")]
use macroquad::{prelude::RED, shapes::draw_rectangle_lines};

use crate::{app_state::AppState, input::{MousePosition, Input, MouseButton}};

/// The root rect that contains all further UI Containers
/// for a layout. Used in conjunction with UIContainer and
/// Rect.
pub struct UIRoot;

/// Container for UI elements used in flexible layouts.
/// Currently only supports column-type layouts.
pub struct UIContainer {
    children: Vec<Entity>,
    margin: f32,
    gap: f32,
}

impl UIContainer {
    pub fn empty() -> Self {
        Self {
            children: Vec::new(),
            margin: 4.0,
            gap: 4.0,
        }
    }

    pub fn add_child(&mut self, c: Entity) {
        self.children.push(c);
    }
}

#[derive(Copy, Clone)]
pub enum UISize {
    Constant(f32),
    Grow(usize),
}

#[derive(Copy, Clone)]
pub struct UIConstraint {
    width: Option<f32>,
}

impl UIConstraint {
    pub fn width_constraint(width: f32) -> Self {
        Self { width: Some(width) }
    }
}

pub struct Label {
    text: String,
    font_size: f32,
}

struct Button {
    state: UIState,
    transition: f32,
    click_pos: Vec2,
    click_effect_progress: f32,
}

impl Button {
    fn new() -> Self {
        Self {
            state: UIState::Normal,
            transition: 0.0,
            click_pos: Vec2::ZERO,
            click_effect_progress: 1.0,
        }
    }
}

pub struct QuitButton;

#[derive(PartialEq)]
pub enum UIState {
    Normal,
    Hover,
    Click,
}

pub struct Text(pub String);

impl Label {
    pub fn new(text: String, font_size: f32) -> Self {
        Self { text, font_size }
    }
}

pub fn add_ui_systems_to_schedule(builder: &mut Builder) {
    builder
        .add_system(size_ui_root_system())
        .flush()
        .add_system(layout_ui_system())
        .flush()
        .add_system(update_button_state_system())
        .add_system(handle_button_clicked_system())
        .flush();

    #[cfg(feature = "debug_ui")]
    builder.add_thread_local(debug_rect_draw_system());

    builder
        .add_thread_local(draw_labels_system())
        .add_thread_local(draw_buttons_system())
        .flush();
}

#[system(for_each)]
fn size_ui_root(rect: &mut Rect, _: &UIRoot) {
    let h = screen_height();
    let w = screen_width();
    rect.x = 0.0;
    rect.y = 0.0;
    rect.h = h;
    rect.w = w;
}

#[system]
#[read_component(Rect)]
#[read_component(UIContainer)]
#[read_component(UIRoot)]
#[read_component(UISize)]
#[read_component(UIConstraint)]
fn layout_ui(world: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut root_query = <(&Rect, &UIContainer, &UIRoot)>::query();

    root_query.iter(world).for_each(|(rect, container, _)| {
        calculate_and_apply_child_ui_sizes(*rect, container, &world, commands);
    });
}

fn calculate_and_apply_child_ui_sizes(
    container_rect: Rect,
    container: &UIContainer,
    world: &SubWorld,
    commands: &mut CommandBuffer,
) {
    let size_info: Vec<(UISize, Option<UIConstraint>)> = container
        .children
        .iter()
        .map(|c| {
            if let Ok(entry) = world.entry_ref(*c) {
                let constraint = if let Ok(constraint) = entry.get_component::<UIConstraint>() {
                    Some(*constraint)
                } else {
                    None
                };
                if let Ok(size) = entry.get_component::<UISize>() {
                    (*size, constraint)
                } else {
                    panic!("There is a child ui entity with no defined size!");
                }
            } else {
                panic!("There's a reference to a child ui component that doesn't exist!");
            }
        })
        .collect();

    let (constant_used_space, flex_units): (f32, usize) =
        size_info.iter().fold((0.0, 0), |acc, (s, _)| match s {
            //TODO: When vertical constraints are implemented they have to be taken into account here.
            UISize::Constant(s) => (acc.0 + *s, acc.1),
            UISize::Grow(units) => (acc.0, acc.1 + *units),
        });

    let inner_rect = Rect::new(
        container_rect.x + container.margin,
        container_rect.y + container.margin,
        container_rect.w - container.margin * 2.0,
        container_rect.h - container.margin * 2.0,
    );
    let flex_space =
        inner_rect.h - constant_used_space - (container.gap * (size_info.len() - 1) as f32);
    let flex_unit_size = flex_space / flex_units as f32;

    let mut draw_pos = inner_rect.y;

    size_info
        .iter()
        .enumerate()
        .for_each(|(idx, (size, constraint))| {
            let (x, w) = {
                let initial_width = inner_rect.w;

                let max_width = if let Some(c) = constraint {
                    c.width
                } else {
                    None
                };

                if let Some(width) = max_width {
                    let constrained_width = initial_width.min(width);
                    let centered_x =
                        inner_rect.x + (inner_rect.w * 0.5) - (constrained_width * 0.5);
                    (centered_x, constrained_width)
                } else {
                    (inner_rect.x, initial_width)
                }
            };
            let h = match size {
                UISize::Constant(s) => *s,
                UISize::Grow(units) => flex_unit_size * *units as f32,
            };

            let child_rect = Rect::new(x, draw_pos, w, h);

            let child_ref = world.entry_ref(container.children[idx]).unwrap();
            if let Ok(child_container) = child_ref.get_component::<UIContainer>() {
                calculate_and_apply_child_ui_sizes(child_rect, child_container, world, commands);
            }

            commands.add_component(container.children[idx], child_rect);

            draw_pos += child_rect.h + container.gap;
        });
}

#[cfg(feature = "debug_ui")]
#[system(for_each)]
fn debug_rect_draw(rect: &Rect) {
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, RED);
}

#[system(for_each)]
fn draw_labels(rect: &Rect, label: &Label) {
    draw_centered_text(rect, &label.text, label.font_size);
}

fn draw_centered_text(rect: &Rect, text: &str, font_size: f32) {
    let (x, y) = {
        let text_dims = measure_text(text, None, font_size as u16, 1.0);
        let center = rect.center();
        let x = center.x - text_dims.width * 0.5;
        let y = center.y;
        (x, y)
    };

    draw_text(text, x, y, font_size, WHITE);
}

#[system(for_each)]
fn update_button_state(rect: &Rect, button: &mut Button, #[resource] mouse_pos: &MousePosition, #[resource] mouse_btns: &Input<MouseButton>) {
    if mouse_btns.is_pressed(MouseButton::Left) && button.state == UIState::Click
    {
        // Don't change state.
    } else {
        if rect.contains(Vec2::new(mouse_pos.0.x, mouse_pos.0.y)) {
            if mouse_btns.is_just_pressed(MouseButton::Left) {
                button.state = UIState::Click;
                button.click_pos = mouse_pos.0;
                button.click_effect_progress = 0.0;
            } else {
                button.state = UIState::Hover;
            }
        } else {
            button.state = UIState::Normal;
        }
    }

    const TRANSITION_SPEED: f32 = 0.05;
    match button.state {
        UIState::Normal => {
            button.transition = (button.transition - TRANSITION_SPEED).max(0.0);
        }
        UIState::Hover | UIState::Click => {
            button.transition = (button.transition + TRANSITION_SPEED).min(1.0);
        }
    }

    const CLICK_EFFECT_SPEED: f32 = 0.05;
    button.click_effect_progress = (button.click_effect_progress + CLICK_EFFECT_SPEED).min(1.0);
}

#[system(for_each)]
fn draw_buttons(
    rect: &Rect,
    button: &Button,
    text: Option<&Text>,
    #[resource] mouse_position: &MousePosition,
) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, BLUE);

    let overlay_width = rect.w * ease_in_out_sine(button.transition);
    draw_rectangle(rect.x, rect.y, overlay_width, rect.h, DARKBLUE);

    let line_spacing = rect.h / 10.0;
    let top_line_y = rect.y + line_spacing;
    let bottom_line_y = rect.y + rect.h - line_spacing;

    draw_line(
        rect.x,
        top_line_y,
        rect.x + overlay_width,
        top_line_y,
        2.0,
        BLUE,
    );
    draw_line(
        rect.x,
        bottom_line_y,
        rect.x + overlay_width,
        bottom_line_y,
        2.0,
        BLUE,
    );

    if let Some(t) = text {
        draw_centered_text(rect, &t.0, 32.0);
    }

    if button.state == UIState::Hover || button.state == UIState::Click {
        let mouse_pos = mouse_position.0;

        //TODO: Figure out how to do this with shaders so it cuts out past the button's rect.
        let mouse_hover_circle_radius = 64.0;
        draw_circle(
            mouse_pos.x,
            mouse_pos.y,
            mouse_hover_circle_radius,
            Color::new(0.0, 0.0, 0.0, 0.05),
        );
        draw_circle(
            mouse_pos.x,
            mouse_pos.y,
            mouse_hover_circle_radius * 0.5,
            Color::new(0.0, 0.0, 0.0, 0.05),
        );
        draw_circle(
            mouse_pos.x,
            mouse_pos.y,
            mouse_hover_circle_radius * 0.25,
            Color::new(0.0, 0.0, 0.0, 0.05),
        );
    }

    let click_effect_alpha = 1.0 - ease_in_out_sine(button.click_effect_progress);
    let click_effect_color = Color::new(0.0, 0.0, 0.7, click_effect_alpha);
    let click_effect_radius = ease_in_out_sine(button.click_effect_progress) * 64.0;

    draw_circle(
        button.click_pos.x,
        button.click_pos.y,
        click_effect_radius,
        click_effect_color,
    );
}

pub fn spawn_button(ecs: &mut World, label: &str) -> Entity {
    const BUTTON_WIDTH: f32 = 256.0;
    ecs.push((
        UISize::Grow(1),
        UIConstraint::width_constraint(BUTTON_WIDTH),
        Button::new(),
        Text(label.to_string()),
        UIState::Normal,
    ))
}

#[system(for_each)]
fn handle_button_clicked(
    rect: &Rect,
    _: &Button,
    _: &QuitButton,
    #[resource] app_state: &mut AppState,
    #[resource] mouse_pos: &MousePosition,
    #[resource] mouse_btns: &Input<MouseButton>,
) {
    if mouse_btns.is_just_pressed(MouseButton::Left) && rect.contains(mouse_pos.0)
    {
        *app_state = AppState::Quit;
    }
}

fn ease_in_out_sine(x: f32) -> f32 {
    -((std::f32::consts::PI * x).cos() - 1.0) / 2.0
}
