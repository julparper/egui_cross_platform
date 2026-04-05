// Este código se ejecutará en Widnows y como aplicación web: Web Assembly

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

// desktop version: Esta solo se usa si lo lanzamos en modo escritorio.
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result { //Devuelve un valor eframe
    //let options = eframe::NativeOptions::default(); //Configuramos opciones iniciales de la ventana: Transparencia, bordes,..
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 600.0])
            .with_decorations(false) // Esto quita la barra de título y los bordes
            .with_transparent(true),  // Opcional: para esquinas redondeadas reales
        ..Default::default()
    };
    
/*     eframe::run_native( //Crea una ventana en el sistema operativo donde se está ejecutando
        "Demo de Egui y Rust", //Título de la ventana
        options, // Opciones configuradas antes
        Box::new(|cc| { // Inicialización. Se una una closure, es como una función anónima donde los parámetros sele pasan usando |
            cc.egui_ctx.set_visuals(egui::Visuals::dark()); // Pone el tema oscuro
            Ok(Box::new(MiAppEstado::default())) // Inicializa el estado de la app. Agui le pasamos la estructura a la aplicación.
        }),
    )*/
    eframe::run_native(
        "Mi App Premium",
        options,
        Box::new(|cc| {
            let mut visuals = egui::Visuals::dark();

           // Fondo principal azul muy oscuro
            visuals.panel_fill = egui::Color32::from_rgb(15, 18, 25);
            // Fondo de widgets (botones, cajas de texto)
            visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(25, 30, 45);
            // Color de resaltado (azul brillante)
            visuals.selection.bg_fill = egui::Color32::from_rgb(0, 120, 215);

            
            // 2. Aplicamos el estilo
            cc.egui_ctx.set_visuals(visuals);
            
            Ok(Box::new(MiAppEstado::default()))
        }),
    )
}

// web version: WebAssembly; esta solo se usa si se lanza en modo navegador
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
                    Ok(Box::new(MiAppEstado::default()))
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
struct MiAppEstado {
    nombre: String,
    edad: u32,
    contador: i32,
}

impl eframe::App for MiAppEstado {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| { //Panel principal, aquí dibujamos cada elemento
            
            ui.heading("Ejemplo usando Egui y Rust");
            
            ui.separator();
            
            ui.horizontal(|ui| { //Organizador modo fila, se organzan los elementos que contienen en la misma fila: Etiqueta + Texto
                ui.label("Introduce tu nombre: ");
                ui.text_edit_singleline(&mut self.nombre);
            });
            
            ui.add(egui::Slider::new(&mut self.edad, 0..=120).text("edad")); // Crea una barra deslizante
            
            if ui.button("Incrementar el contador").clicked() { // Implemetna la opción de pulsar el botón
                self.contador += 1;
            }
            
            ui.label(format!("Contador: {}", self.contador));
            
            ui.separator();
            
            if !self.nombre.is_empty() {
                ui.label(format!("Hola, {}! Tienes {} años.", self.nombre, self.edad));
            }

            
        });
    }
}