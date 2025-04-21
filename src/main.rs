mod algorithms;
use algorithms::draw_bars;
use macroquad::{prelude::*, ui::{hash, root_ui, widgets}};


#[macroquad::main("Algorithm Visualizer")]
async fn main() {
    let mut number_of_bars: f32 = 40.0;
    let mut values: Vec<i32> = (1..=number_of_bars as u32).map(|_| rand::gen_range(50, 400)).collect();
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
                });

                ui.tree_node(hash!(), "Data", |ui| {
                    ui.slider(hash!(), "Num of bars", 2.0..200.0, &mut number_of_bars);
                    ui.slider(hash!(), "Slowdown [s]", 0.0..1.0, &mut algorithm.slowdown);
                });
            });

        algorithms::draw_bars(&values, usize::MAX, usize::MAX);

        // Run button action
        if widgets::Button::new("Run")
            .size(vec2(120.0, 70.0))
            .position(vec2(screen_width() - 300.0, 30.0))
            .ui(&mut root_ui())
        {
            let start_time = std::time::Instant::now();
            println!("Going in with: {:?}", values);
            algorithm.do_magic(&mut values).await;
            algo_duration = Some(start_time.elapsed().as_secs_f64());
            // assert!(values.is_sorted(), "Not sorted: {:?}", values);
        }

        // Reset button action
        if widgets::Button::new("Reset")
            .size(vec2(120.0, 70.0))
            .position(vec2(screen_width() - 160.0, 30.0))
            .ui(&mut root_ui())
        {
            values = (1..=number_of_bars as u32).map(|_| rand::gen_range(50, 400)).collect();
            println!("New values: {:?}", values);
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

