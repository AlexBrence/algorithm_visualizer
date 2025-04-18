mod algorithms;
use macroquad::{prelude::*, ui::{hash, root_ui, widgets}};

const BAR_WIDTH: f32 = 30.0;

// async fn bubble_sort(values: &mut Vec<f32>, slowdown: f32) {
//     let len = values.len();
//     for i in 0..len {
//         for j in 0..len - i - 1 {
//             if values[j] > values[j + 1] {
//                 values.swap(j, j + 1);
//
//                 // Clear screen and redraw bars
//                 clear_background(BLACK);
//                 draw_bars(values, j, j + 1);
//                 next_frame().await; // This keeps the loop running
//                 std::thread::sleep(std::time::Duration::from_secs_f32(slowdown)); // Slows down sorting for visibility
//             }
//         }
//     }
// }

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

// #[derive(Clone, Copy, Debug, PartialEq, Default)]
// #[non_exhaustive]
// enum Algorithm {
//     #[default] 
//     BubbleSort,
//     SelectionSort,
//     InsertionSort,
//     MergeSort,
//     HeapSort,
//     QuickSort,
//     CountingSort,
//     RadixSort,
// }
//
// impl fmt::Display for Algorithm {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }


#[macroquad::main("Algorithm Visualizer")]
async fn main() {
    let mut number_of_bars: f32 = 40.0;
    let mut values: Vec<f32> = (1..=number_of_bars as u32).map(|_| rand::gen_range(50.0, 400.0)).collect();
    let mut algo_duration: Option<f64> = None; 
    let mut algorithm: algorithms::Algorithm = Default::default();

    loop {
        clear_background(DARKGRAY);

        // Settings window
        widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(500.0, 400.0))
            .label("Settings")
            .ui(&mut root_ui(), |ui| {
                // Algorithm selection
                ui.tree_node(hash!(), "Algorithms", |ui| {
                    let previous_algo = algorithm.current_type;
                    ui.label(None, format!("Current algorithm: {}", algorithm.current_type).as_str());

                    if ui.button(None, algorithms::Type::BubbleSort.to_string()) {
                        algorithm.current_type = algorithms::Type::BubbleSort; 
                    }
                    ui.same_line(0.0);
                    if ui.button(None, algorithms::Type::QuickSort.to_string()) {
                        algorithm.current_type = algorithms::Type::QuickSort; 
                    }
                    ui.same_line(0.0);
                    if ui.button(None, algorithms::Type::CountingSort.to_string()) {
                        algorithm.current_type = algorithms::Type::CountingSort; 
                    }
                    ui.same_line(0.0);
                    if ui.button(None, algorithms::Type::RadixSort.to_string()) {
                        algorithm.current_type = algorithms::Type::RadixSort; 
                    }

                    if algorithm.current_type != previous_algo {
                        println!("Algorithm change: {} -> {}", previous_algo, algorithm.current_type);
                    }
                });

                ui.tree_node(hash!(), "Data", |ui| {
                    ui.slider(hash!(), "Num of bars", 2.0..200.0, &mut number_of_bars);
                    ui.slider(hash!(), "Slowdown [s]", 0.0..1.0, &mut algorithm.slowdown);
                });
            });

        draw_bars(&values, usize::MAX, usize::MAX);

        // Run button action
        if widgets::Button::new("Run")
            .size(vec2(120.0, 70.0))
            .position(vec2(screen_width() - 300.0, 30.0))
            .ui(&mut root_ui())
        {
            let start_time = std::time::Instant::now();
            algorithm.do_magic(&mut values).await;
            algo_duration = Some(start_time.elapsed().as_secs_f64());
        }

        // Reset button action
        if widgets::Button::new("Reset")
            .size(vec2(120.0, 70.0))
            .position(vec2(screen_width() - 160.0, 30.0))
            .ui(&mut root_ui())
        {
            values = (1..=number_of_bars as u32).map(|_| rand::gen_range(50.0, 400.0)).collect();
        }

        // Show time elapsed
        if let Some(time) = algo_duration {
            draw_text(
                format!("Time elapsed: {:.2} seconds", time).as_str(), 
                screen_width() / 2.5, 
                50.0, 
                20.0, 
                WHITE);
        }

        next_frame().await; // Keeps the loop running
    }
}

