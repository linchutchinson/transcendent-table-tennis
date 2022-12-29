use legion::{
    systems::{Builder, CommandBuffer},
    world::SubWorld,
    *,
};
use macroquad::{
    prelude::{Rect, RED},
    shapes::draw_rectangle_lines,
    window::{screen_height, screen_width},
};

pub struct UIRoot;

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

pub fn add_ui_systems_to_schedule(builder: &mut Builder) {
    builder
        .add_system(size_ui_root_system())
        .flush()
        .add_system(layout_ui_system())
        .flush()
        .add_thread_local(debug_rect_draw_system())
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
    let sizes: Vec<UISize> = container
        .children
        .iter()
        .map(|c| {
            if let Ok(entry) = world.entry_ref(*c) {
                if let Ok(size) = entry.get_component::<UISize>() {
                    *size
                } else {
                    panic!("There is a child ui entity with no defined size!");
                }
            } else {
                panic!("There's a reference to a child ui component that doesn't exist!");
            }
        })
        .collect();

    let (constant_used_space, flex_units): (f32, usize) =
        sizes.iter().fold((0.0, 0), |acc, s| match s {
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
        inner_rect.h - constant_used_space - (container.gap * (sizes.len() - 1) as f32);
    let flex_unit_size = flex_space / flex_units as f32;

    let mut draw_pos = inner_rect.y;

    sizes.iter().enumerate().for_each(|(idx, size)| {
        let x = inner_rect.x;
        let w = inner_rect.w;
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
