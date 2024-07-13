use bevy::ecs::query::{QueryData, QueryFilter, WorldQuery};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Filter CollisionEvent::Started events
pub fn start_event_filter(event: &CollisionEvent) -> Option<(&Entity, &Entity)> {
    match event {
        CollisionEvent::Started(e1, e2, _) => Some((e1, e2)),
        _ => None,
    }
}

/// QueryEither
///
/// Example:
/// ```
/// query.iter()
///     .filter_map(query.get_either(e1, e2))
///     .map(|(data, found_entity, other_entity)| {
///         //...
///     })
/// ```
pub trait QueryEither<'w, D>
where
    D: QueryData<ReadOnly = D>,
{
    /// get either `e1` or `e2`, returning a `([QueryData], [Entity from query], [other Entity])`
    fn get_either(
        &'w self,
        e1: Entity,
        e2: Entity,
    ) -> Option<(<D as WorldQuery>::Item<'w>, Entity, Entity)>;
}

impl<'w, D, F> QueryEither<'w, D> for Query<'w, '_, D, F>
where
    D: QueryData<ReadOnly = D>,
    F: QueryFilter,
{
    fn get_either(
        &'w self,
        e1: Entity,
        e2: Entity,
    ) -> Option<(<D as WorldQuery>::Item<'w>, Entity, Entity)> {
        self.get(e1)
            .map(|data| (data, e1, e2))
            .or(self.get(e2).map(|data| (data, e2, e1)))
            .ok()
    }
}

/// The [EqEither] trait allow to check if self is equal to either
/// one value or another
#[allow(dead_code)]
pub trait EqEither {
    fn eq_either(&self, v1: Self, v2: Self) -> bool;
}

impl<T> EqEither for T
where
    T: Copy + PartialEq,
{
    fn eq_either(&self, v1: Self, v2: Self) -> bool {
        self == &v1 || self == &v2
    }
}

//
// TODO: finish it
// Trait to filter a collision with 2 queries
//
// pub trait CollisionEventFilter {
//     fn start<'w, D1, F1, D2, F2>(
//         &self,
//         q1: Query<'w, '_, D1, F1>,
//         q2: Query<'w, '_, D2, F2>,
//     ) -> Option<(ROQueryItem<'w, D1>, ROQueryItem<'w, D2>)>
//     where
//         D1: QueryData<ReadOnly = D1>,
//         F1: QueryFilter,
//         D2: QueryData,
//         F2: QueryFilter;
// }

// impl CollisionEventFilter for CollisionEvent {
//     fn start<'w, D1, F1, D2, F2>(
//         &self,
//         q1: Query<'w, '_, D1, F1>,
//         q2: Query<'w, '_, D2, F2>,
//     ) -> Option<(ROQueryItem<'w, D1>, ROQueryItem<'w, D2>)>
//     where
//         D1: QueryData<ReadOnly = D1>,
//         F1: QueryFilter,
//         D2: QueryData,
//         F2: QueryFilter,
//     {
//         if let CollisionEvent::Started(e1, e2, _) = self {
//             let res = q1
//                 .get(*e1)
//                 .map(move |r1| (r1, q2.get(*e2)))
//                 .or(q1.get(*e2).map(|r1| (r1, q2.get(*e1))));

//             match res {
//                 Ok((r1, Ok(r2))) => Some((r1, r2)),
//                 _ => None,
//             }
//         } else {
//             None
//         }
//     }
// }
