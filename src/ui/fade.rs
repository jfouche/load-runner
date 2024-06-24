use super::fullscreen_style;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct FaderBundle {
    name: Name,
    fader: Fader,
    node: NodeBundle,
    timer: FadeTimer,
}

impl FaderBundle {
    pub fn new(from: Color, to: Color, duration_secs: f32) -> Self {
        FaderBundle {
            name: Name::new("Fade"),
            fader: Fader { from, to },
            node: NodeBundle {
                style: fullscreen_style(),
                background_color: from.into(),
                z_index: ZIndex::Global(-1),
                ..Default::default()
            },
            timer: FadeTimer(Timer::from_seconds(duration_secs, TimerMode::Once)),
        }
    }
}

#[derive(Event)]
pub struct FaderFinishEvent {
    pub entity: Entity,
}

#[derive(Component)]
struct Fader {
    from: Color,
    to: Color,
}

impl Fader {
    fn color(&self, percent: f32) -> Color {
        let from = self.from.as_rgba_f32();
        let to = self.to.as_rgba_f32();
        let cx = |i| from[i] + (to[i] - from[i]) * percent;
        let r = cx(0);
        let g = cx(1);
        let b = cx(2);
        let a = cx(3);
        Color::rgba(r, g, b, a)
    }
}

#[derive(Component, Deref, DerefMut)]
struct FadeTimer(Timer);

impl FadeTimer {
    fn percent(&self) -> f32 {
        self.elapsed().as_secs_f32() / self.duration().as_secs_f32()
    }
}

pub fn plugin(app: &mut App) {
    app.add_event::<FaderFinishEvent>()
        .add_systems(Update, fade);
}

fn fade(
    mut faders: Query<(Entity, &Fader, &mut FadeTimer, &mut BackgroundColor)>,
    time: Res<Time>,
    mut events: EventWriter<FaderFinishEvent>,
) {
    for (entity, fader, mut timer, mut bgcolor) in &mut faders {
        timer.tick(time.delta());
        let percent = timer.percent();
        bgcolor.0 = fader.color(percent);
        if timer.just_finished() {
            events.send(FaderFinishEvent { entity });
        }
    }
}
