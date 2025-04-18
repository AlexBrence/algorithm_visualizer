use core::fmt;
use macroquad::{prelude::*, ui::{hash, root_ui, widgets}};


const BAR_WIDTH: f32 = 30.0;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[non_exhaustive]
pub enum Type {
    #[default] 
    BubbleSort,
    SelectionSort,
    InsertionSort,
    MergeSort,
    HeapSort,
    QuickSort,
    CountingSort,
    RadixSort,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


pub struct Algorithm {
    pub current_type: Type,
    pub slowdown: f32,
}

impl Algorithm {
    pub async fn do_magic(&self, values: &mut Vec<f32>) {
        match &self.current_type {
            Type::BubbleSort => {
                bubble_sort(values, self.slowdown).await;
            },
            Type::QuickSort => todo!(),
            Type::SelectionSort => todo!(),
            Type::InsertionSort => todo!(),
            Type::MergeSort => todo!(),
            Type::HeapSort => todo!(),
            Type::CountingSort => todo!(),
            Type::RadixSort => todo!(),
        }
    }
}

impl Default for Algorithm {
    fn default() -> Self {
        Self {
            current_type: Type::default(),
            slowdown: 0.05,
        }
    }
}

fn draw_bars(values: &[f32], highlighted1: usize, highlighted2: usize) {
    for (i, &value) in values.iter().enumerate() {
        let color = if i == highlighted1 || i == highlighted2 { RED } else { BLUE };
        let bar_width = if values.len() > 150 {
            BAR_WIDTH / 2.5
        } else if values.len() > 80 {
            BAR_WIDTH / 2.0
        } else {
            BAR_WIDTH
        };
        
        draw_rectangle(i as f32 * bar_width, screen_height() - value, bar_width - 5.0, value, color);
    }
}

async fn bubble_sort(values: &mut Vec<f32>, slowdown: f32) {
    let len = values.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if values[j] > values[j + 1] {
                values.swap(j, j + 1);

                // Clear screen and redraw bars
                clear_background(BLACK);
                draw_bars(values, j, j + 1);
                next_frame().await; // This keeps the loop running
                std::thread::sleep(std::time::Duration::from_secs_f32(slowdown)); // Slows down sorting for visibility
            }
        }
    }
}
