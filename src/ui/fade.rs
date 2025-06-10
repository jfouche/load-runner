use bevy::prelude::*;

pub fn fader(from: Color, to: Color, duration_secs: f32) -> impl Bundle {
    (
        Name::new("Fader"),
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
pub struct FaderFinishEvent;

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

pub fn plugin(app: &mut App) {
    app.add_event::<FaderFinishEvent>()
        .add_systems(Update, fade);
}

fn fade(
    mut faders: Query<(Entity, &Fader, &mut FadeTimer, &mut BackgroundColor)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, fader, mut timer, mut bgcolor) in &mut faders {
        timer.tick(time.delta());
        bgcolor.0 = fader.color(timer.fraction());
        if timer.just_finished() {
            commands.trigger_targets(FaderFinishEvent, entity);
        }
    }
}
