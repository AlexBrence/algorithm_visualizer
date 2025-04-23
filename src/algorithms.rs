use core::fmt;
use macroquad::prelude::*;


pub const BAR_WIDTH: f32 = 30.0;

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
    pub async fn do_magic(&self, values: &mut [i32]) {
        if values.is_empty() {
            return;
        }

        match &self.current_type {
            Type::BubbleSort => {
                bubble_sort(values, self.slowdown).await;
            },
            Type::QuickSort => {
                quick_sort(values, 0, values.len() - 1, self.slowdown).await;
            }
            Type::SelectionSort => {
                selection_sort(values, self.slowdown).await;
            },
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
            slowdown: 0.0,
        }
    }
}

pub fn draw_bars(values: &[i32], highlighted1: usize, highlighted2: usize) {
    for (i, &value) in values.iter().enumerate() {
        let color = if i == highlighted1 || i == highlighted2 { RED } else { BLUE };

        let bar_width = if values.len() > 150 {
            BAR_WIDTH / 2.5
        } else if values.len() > 80 {
            BAR_WIDTH / 2.0
        } else {
            BAR_WIDTH
        };
        
        draw_rectangle(i as f32 * bar_width, screen_height() - value as f32, bar_width - 5.0, value as f32, color);
    }
}

async fn redraw_and_slowdown(values: &[i32], highlighted1: usize, highlighted2: usize, slowdown: f32) {
    clear_background(BLACK);
    draw_bars(values, highlighted1, highlighted2);
    next_frame().await; // This keeps the loop running
    std::thread::sleep(std::time::Duration::from_secs_f32(slowdown)); // Slows down sorting for visibility
}

// ========== Bubble sort ==========
async fn bubble_sort(values: &mut [i32], slowdown: f32) {
    let len = values.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if values[j] > values[j + 1] {
                values.swap(j, j + 1);

                // Redraw bars
                redraw_and_slowdown(values, j, j + 1, slowdown).await;
            }
        }
    }
}


/*
 * Quick sort with Lomuto partition
 * https://www.geeksforgeeks.org/quick-sort-algorithm/
 */

async fn partition(values: &mut [i32], low: usize, high: usize, slowdown: f32) -> usize {
    let pivot = values[high];
    let mut i = low;

    for j in low..high {
        if values[j] < pivot {
            values.swap(i, j);
            redraw_and_slowdown(values, j, i, slowdown).await;
            i += 1;
        }
    }
    values.swap(i, high);
    redraw_and_slowdown(values, i, high, slowdown).await;

    i
}

async fn quick_sort(values: &mut [i32], low: usize, high: usize, slowdown: f32) {
    if low < high {
        let partition_idx = partition(values, low, high, slowdown).await;

        if partition_idx > 0 {
            Box::pin(quick_sort(values, low, partition_idx - 1, slowdown)).await;
        }
        Box::pin(quick_sort(values, partition_idx + 1, high, slowdown)).await;
    }
}

/*
 * Selection Sort
 */

async fn selection_sort(values: &mut [i32], slowdown: f32) {
    for i in 0..values.len() - 1 {
        let mut min_idx: usize = i;

        for j in i+1..values.len() {
            if values[j] < values[min_idx] {
                min_idx = j;
            }
        }
        redraw_and_slowdown(values, i, min_idx, slowdown).await;
        values.swap(i, min_idx);
    }
}
