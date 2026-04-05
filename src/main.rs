// Este código se ejecutará en Widnows y como aplicación web: Web Assembly

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

// desktop version
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    
    eframe::run_native( //Crea una ventana en el sistema operativo donde se está ejecutando
        "egui App",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark()); // Pone el tema oscuro
            Ok(Box::new(MyApp::default())) // Inicializa el estado de la app
        }),
    )
}

// web version: WebAssembly
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;
    
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        // Leeo el objeto con el id "the_canvas_id"
        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| {
                    cc.egui_ctx.set_visuals(egui::Visuals::dark());
                    Ok(Box::new(MyApp::default()))
                }),
            )
            .await;

        let loading_text = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("loading_text"));
        if let Some(loading_text) = loading_text {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

// Estado de la aplicación
#[derive(Default)] //Creamos una instancia con todos los atributos vacíos (String vacío, 0 y 0)
struct MyApp {
    name: String,
    age: u32,
    counter: i32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| { //Panel principal, aquí dibujamos cada elemento
            
            ui.heading("Welcome to egui!");
            
            ui.separator();
            
            ui.horizontal(|ui| { //Organizador modo fila, se organzan los elementos que contienen en la misma fila: Etiqueta + Texto
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age")); // Crea una barra deslizante
            
            if ui.button("Increment Counter").clicked() { // Implemetna la opción de pulsar el botón
                self.counter += 1;
            }
            
            ui.label(format!("Counter: {}", self.counter));
            
            ui.separator();
            
            if !self.name.is_empty() {
                ui.label(format!("Hello, {}! You are {} years old.", self.name, self.age));
            }

            
        });
    }
}