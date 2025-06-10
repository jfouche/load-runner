use bevy::prelude::*;

pub fn fader(from: Color, to: Color, duration_secs: f32) -> impl Bundle {
    (
        Name::new("Fade"),
        Fader { from, to },
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        },
        BackgroundColor(from),
        ZIndex(i32::MAX - 1),
        FadeTimer(Timer::from_seconds(duration_secs, TimerMode::Once)),
    )
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
        let from = self.from.to_srgba();
        let to = self.to.to_srgba();
        let r = from.red + (to.red - from.red) * percent;
        let g = from.green + (to.green - from.green) * percent;
        let b = from.blue + (to.blue - from.blue) * percent;
        let a = from.alpha + (to.alpha - from.alpha) * percent;
        Color::srgba(r, g, b, a)
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
            // TODO: trigger
            events.write(FaderFinishEvent { entity });
        }
    }
}
