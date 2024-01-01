use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::ldtk::LevelChangeEvent;
use crate::{entities::enemies::SpawnMierdaEvent, entities::items::SpawnPizzaEvent, ui::*};

#[derive(Clone)]
pub enum WaveEntry {
    SpawnMierda { count: usize },
    SpawnPizza { count: usize },
}

#[derive(Clone)]
pub struct Wave {
    pub events: Vec<WaveEntry>,
    pub event_duration: Duration,
    pub wave_duration: Duration,
}

#[derive(Resource, Default)]
pub struct GameplayState {
    pub wave_number: Option<usize>,
    pub current_level_id: Option<usize>,
    pub event_queue: Vec<WaveEntry>,
    pub wave_timer: Timer,
    pub wave_event_timer: Timer,
}

#[derive(Event, Clone)]
pub struct WaveEvent {
    pub wave_number: usize,
    pub wave_entry: WaveEntry,
}

impl GameplayState {
    pub fn current_level_waves(&self) -> Option<Vec<Wave>> {
        match self.current_level_id {
            Some(1) => Some(get_level_1_waves()),
            _ => None,
        }
    }

    pub fn current_wave(&self) -> Option<Wave> {
        match self.current_level_waves() {
            Some(waves) => match self.wave_number {
                Some(wave_number) => waves.get(wave_number).cloned(),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn select_random_wave_entry(&mut self) -> Option<WaveEntry> {
        match self.current_wave() {
            Some(wave) => {
                let mut rng = rand::thread_rng();

                if self.event_queue.is_empty() {
                    return None;
                }

                let random_index = rng.gen_range(0..self.event_queue.len());
                // remove entry with index random_index from event_queue
                self.event_queue = self
                    .event_queue
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != random_index)
                    .map(|(_, v)| v.clone())
                    .collect();

                let wave_entry = wave.events.get(random_index).unwrap();
                Some(wave_entry.clone())
            }
            _ => None,
        }
    }
}

pub fn handle_timers(
    mut gameplay_state: ResMut<GameplayState>,
    mut ew_wave: EventWriter<WaveEvent>,
    time: Res<Time>,
) {
    gameplay_state.wave_timer.tick(time.delta());
    gameplay_state.wave_event_timer.tick(time.delta());

    if gameplay_state.wave_timer.just_finished() {
        if let Some(current_level_waves) = gameplay_state.current_level_waves() {
            let max_wave_number = current_level_waves.len() - 1;
            let mut current_wave_number = gameplay_state.wave_number.unwrap_or(0);
            if current_wave_number < max_wave_number {
                current_wave_number += 1;
                gameplay_state.wave_number = Some(current_wave_number);
                gameplay_state.event_queue = current_level_waves
                    [gameplay_state.wave_number.unwrap()]
                .events
                .clone();

                gameplay_state.wave_timer = Timer::new(
                    gameplay_state.current_wave().unwrap().wave_duration,
                    TimerMode::Once,
                );

                let wave_event = gameplay_state.select_random_wave_entry().unwrap();
                ew_wave.send(WaveEvent {
                    wave_number: current_wave_number,
                    wave_entry: wave_event,
                });
            }
        }
    }

    if gameplay_state.wave_event_timer.just_finished() {
        let wave_event = gameplay_state.select_random_wave_entry();
        if wave_event.is_none() {
            return;
        }
        ew_wave.send(WaveEvent {
            wave_number: gameplay_state.wave_number.unwrap(),
            wave_entry: wave_event.unwrap(),
        });
    }
}

#[allow(clippy::single_match)]
pub fn event_on_level_change(
    mut er_on_level_change: EventReader<LevelChangeEvent>,
    mut gameplay_state: ResMut<GameplayState>,

    mut ew_wave: EventWriter<WaveEvent>,
) {
    for event in er_on_level_change.iter() {
        match event.level_id {
            1 => {
                let waves = get_level_1_waves();

                *gameplay_state = GameplayState {
                    wave_number: Some(0),
                    current_level_id: Some(1),
                    event_queue: waves[0].events.clone(),
                    ..default()
                };

                let wave_entry = gameplay_state.select_random_wave_entry().unwrap();

                ew_wave.send(WaveEvent {
                    wave_number: 1,
                    wave_entry,
                });

                gameplay_state.wave_timer = Timer::new(
                    gameplay_state.current_wave().unwrap().wave_duration,
                    TimerMode::Once,
                );

                println!("wave timer set to {:?}", gameplay_state.wave_timer);
            }
            _ => {}
        }
    }
}

pub fn event_wave(
    mut er_on_wave_change: EventReader<WaveEvent>,

    mut gameplay_state: ResMut<GameplayState>,
    mut ev_mierda_spawn: EventWriter<SpawnMierdaEvent>,
    mut ev_pizza_spawn: EventWriter<SpawnPizzaEvent>,
) {
    for event in er_on_wave_change.iter() {
        match event.wave_entry {
            WaveEntry::SpawnMierda { count } => {
                ev_mierda_spawn.send(SpawnMierdaEvent {
                    count: count as u32,
                });
            }
            WaveEntry::SpawnPizza { count } => {
                ev_pizza_spawn.send(SpawnPizzaEvent {
                    count: count as u32,
                });
            }
        }

        gameplay_state.wave_event_timer = Timer::new(
            gameplay_state.current_wave().unwrap().event_duration,
            TimerMode::Once,
        );
    }
}

pub fn ui_wave_info_text(
    mut text_query: Query<(&mut Text, &UIGameplayWave)>,
    gameplay_state: Res<GameplayState>,
) {
    for (mut text, _tag) in text_query.iter_mut() {
        let wave_seconds_left =
            (gameplay_state.wave_timer.duration() - gameplay_state.wave_timer.elapsed()).as_secs();
        let current_wave = gameplay_state.wave_number.unwrap_or(0) + 1;
        let wave_events_in_queue = gameplay_state.event_queue.len();
        let next_wave_event_in = (gameplay_state.wave_event_timer.duration()
            - gameplay_state.wave_event_timer.elapsed())
        .as_secs();
        text.sections[0].value = format!(
            "Wave: {}\t Next Wave in {} seconds\t Wave Events Left:{}\t Next Wave Event In: {}",
            current_wave, wave_seconds_left, wave_events_in_queue, next_wave_event_in
        );
    }
}

// Composition of wave events
pub fn get_level_1_waves() -> Vec<Wave> {
    vec![
        Wave {
            events: vec![WaveEntry::SpawnMierda { count: 1 }],
            event_duration: Duration::from_secs(10),
            wave_duration: Duration::from_secs(10),
        },
        Wave {
            events: vec![
                WaveEntry::SpawnMierda { count: 100 },
                WaveEntry::SpawnPizza { count: 1 },
                WaveEntry::SpawnMierda { count: 100 },
                WaveEntry::SpawnMierda { count: 100 },
                WaveEntry::SpawnMierda { count: 100 },
                WaveEntry::SpawnMierda { count: 100 },
            ],
            event_duration: Duration::from_secs(20),
            wave_duration: Duration::from_secs(120),
        },
        Wave {
            events: vec![
                WaveEntry::SpawnMierda { count: 100 },
                WaveEntry::SpawnPizza { count: 1 },
                WaveEntry::SpawnPizza { count: 2 },
                WaveEntry::SpawnPizza { count: 200 },
                WaveEntry::SpawnPizza { count: 200 },
                WaveEntry::SpawnPizza { count: 200 },
                WaveEntry::SpawnPizza { count: 500 },
                WaveEntry::SpawnPizza { count: 100 },
                WaveEntry::SpawnPizza { count: 2 },
                WaveEntry::SpawnPizza { count: 300 },
            ],
            event_duration: Duration::from_secs(20),
            wave_duration: Duration::from_secs(200),
        },
    ]
}
