use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    // Datos mockeados
    let lotes = vec![
        ("Lote 1", "En crecimiento"),
        ("Lote 2", "Listo para cosecha"),
    ];
    let eventos = vec![
        ("Riego aplicado", "Hace 2 días"),
        ("Fertilización", "Hace 5 días"),
    ];

    rsx! {
        div {
            class: "dashboard-container",
            h2 { "Bienvenido a tu Dashboard" }
            h3 { "Resumen de Lotes" }
            ul {
                { lotes.iter().map(|(nombre, estado)| rsx!(
                    div {
                        key: "{nombre}",
                        class: "p-4 mb-2 bg-gray-100 rounded-md",
                        h3 { class: "text-lg font-semibold", "{nombre}" },
                        p { "Estado: {estado}" },
                    }
                )) }
            }
            h3 { "Eventos Recientes" }
            ul {
                { eventos.iter().map(|(desc, fecha)| rsx!(
                    li { "{desc} - {fecha}" }
                )) }
            }
        }
    }
} 