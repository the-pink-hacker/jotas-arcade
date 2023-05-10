mod frontend {
    use yew::prelude::*;

    #[function_component]
    fn App() -> Html {
        html! {
            <div>
                <p>{"Hello, World!"}</p>
            </div>
        }
    }

    pub fn start() {
        yew::Renderer::<App>::new().render();
    }
}

mod game {
    use bevy::prelude::*;

    pub fn start() {
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Bevy Game Title"),
                    ..default()
                }),
                ..default()
            }))
            .run();
    }
}

fn main() {
    frontend::start();
    game::start();
}
