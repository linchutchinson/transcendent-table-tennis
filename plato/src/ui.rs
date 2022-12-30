use legion::{
    systems::{Builder, CommandBuffer},
    world::SubWorld,
    *,
};
use macroquad::{
    prelude::{Color, Rect, RED, WHITE},
    shapes::draw_rectangle_lines,
    text::{draw_text, measure_text},
    window::{screen_height, screen_width},
};

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
        .add_thread_local(debug_rect_draw_system())
        .add_thread_local(draw_labels_system())
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
        size_info
            .iter()
            .fold((0.0, 0), |acc, (s, constraint)| match s {
                //TODO: When vertical constraints are implemented they have to be taken into account here.
                UISize::Constant(s) => (acc.0 + *s, acc.1),
                UISize::Grow(units) => (acc.0, acc.1 + *units),
                _ => acc,
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
                    println!("Constraining to {width} from {initial_width}!");
                    let constrained_width = initial_width.min(width);
                    let centered_x = inner_rect.x + (inner_rect.w * 0.5) - (constrained_width * 0.5);
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