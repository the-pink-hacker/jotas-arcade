mod frontend {
    use yew::prelude::*;

    #[function_component]
    fn GameCanvas() -> Html {
        html! {
            <canvas id="game-canvas"/>
        }
    }

    #[function_component]
    fn App() -> Html {
        let load_game_callback = Callback::from(|_| crate::game::start());
        html! {
            <div>
                <button onclick={load_game_callback}>{"load game"}</button>
                <p>{"Hello, World!"}</p>
                <GameCanvas/>
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
                    canvas: Some(String::from("#game-canvas")),
                    ..default()
                }),
                ..default()
            }))
            .run();
    }
}

fn main() {
    frontend::start();
}
