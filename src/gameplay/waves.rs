use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::entities::characters::enemy::{Enemy, EnemyType, SpawnEnemyEvent};
use crate::entities::items::item::ItemType;
use crate::gameover::{GameOverEvent, GameWinEvent};
use crate::ldtk::LevelChangeEvent;
use crate::GameState;
use crate::{entities::items::item::SpawnItemEvent, ui::*};

#[derive(Clone)]
pub enum WaveEntry {
    Mierda { count: usize },
    Pizza { count: usize },
    Pendejo { count: usize },
    Biboran { count: usize },
    Boss { count: usize },
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
    for event in er_on_level_change.read() {
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
            }
            _ => {}
        }
    }
}

pub fn check_game_won_or_lost(
    mut next_state: ResMut<NextState<GameState>>,
    gameplay_state: Res<GameplayState>,
    query: Query<&Enemy>,
    mut ev_game_won: EventWriter<GameWinEvent>,
) {
    // check if current wave is last
    if let Some(current_wave) = gameplay_state.current_wave() {
        if gameplay_state.wave_number.unwrap()
            == (gameplay_state.current_level_waves().unwrap().len() - 1)
        {
            // check if all enemies are dead
            if query.iter().count() == 0 {
                ev_game_won.send(GameWinEvent);
            }
        }
    }
}

pub fn event_wave(
    mut er_on_wave_change: EventReader<WaveEvent>,

    mut gameplay_state: ResMut<GameplayState>,
    mut ev_enemy_spawn: EventWriter<SpawnEnemyEvent>,
    mut ev_item_spawn: EventWriter<SpawnItemEvent>,
) {
    for event in er_on_wave_change.read() {
        match event.wave_entry {
            WaveEntry::Mierda { count } => {
                ev_enemy_spawn.send(SpawnEnemyEvent {
                    count: count as u32,
                    enemy_type: EnemyType::Mierda,
                });
            }
            WaveEntry::Pendejo { count } => {
                ev_enemy_spawn.send(SpawnEnemyEvent {
                    count: count as u32,
                    enemy_type: EnemyType::Pendejo,
                });
            }
            WaveEntry::Pizza { count } => {
                ev_item_spawn.send(SpawnItemEvent {
                    count: count as u32,
                    item_type: ItemType::Pizza,
                });
            }
            WaveEntry::Biboran { count } => {
                ev_item_spawn.send(SpawnItemEvent {
                    count: count as u32,
                    item_type: ItemType::Biboran,
                });
            }
            WaveEntry::Boss { count } => {
                ev_enemy_spawn.send(SpawnEnemyEvent {
                    count: count as u32,
                    enemy_type: EnemyType::Psychiatrist1,
                });
                ev_enemy_spawn.send(SpawnEnemyEvent {
                    count: count as u32,
                    enemy_type: EnemyType::Psychiatrist2,
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
        let _wave_events_in_queue = gameplay_state.event_queue.len();
        let _next_wave_event_in = (gameplay_state.wave_event_timer.duration()
            - gameplay_state.wave_event_timer.elapsed())
        .as_secs();
        text.sections[0].value = format!(
            "Wave: {}\t | {} seconds left",
            current_wave, wave_seconds_left,
        );
    }
}

#[allow(dead_code)]
pub fn get_level_1_waves() -> Vec<Wave> {
    vec![
        Wave {
            events: vec![WaveEntry::Mierda { count: 100 }],
            event_duration: Duration::from_secs(10),
            wave_duration: Duration::from_secs(10),
        },
        Wave {
            events: vec![
                WaveEntry::Mierda { count: 100 },
                WaveEntry::Pizza { count: 5 },
                WaveEntry::Mierda { count: 100 },
                WaveEntry::Biboran { count: 5 },
                WaveEntry::Mierda { count: 100 },
            ],
            event_duration: Duration::from_secs(10),
            wave_duration: Duration::from_secs(40),
        },
        Wave {
            events: vec![
                WaveEntry::Pendejo { count: 100 },
                WaveEntry::Pizza { count: 3 },
                WaveEntry::Pendejo { count: 100 },
                WaveEntry::Pizza { count: 3 },
                WaveEntry::Pendejo { count: 100 },
                WaveEntry::Pizza { count: 3 },
            ],
            event_duration: Duration::from_secs(5),
            wave_duration: Duration::from_secs(60),
        },
        Wave {
            events: vec![WaveEntry::Boss { count: 1 }],
            event_duration: Duration::from_secs(5),
            wave_duration: Duration::from_secs(120),
        },
    ]
}
