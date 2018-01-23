use super::*;

pub struct LoadingScreen {
    player: LoadingAsset<Image>,
    crosshair: LoadingAsset<Image>,
    gun: LoadingAsset<Image>,
    wood: LoadingAsset<Image>,
    shadow: LoadingAsset<Image>,
    wall: LoadingAsset<Image>,
    fire: LoadingAsset<Sound>,
    bat: LoadingAsset<Image>,
    death: LoadingAsset<Sound>
}

impl InitialScreen for LoadingScreen {
    fn configure() -> (Window, Canvas) {
        WindowBuilder::new()
            .with_show_cursor(false)
            .build("Hellevator", 960, 540)
    }

    fn new() -> Self {
        LoadingScreen {
            player: Image::load("img/ah_stand.png"),
            crosshair: Image::load("img/crosshair.png"),
            gun: Image::load("img/gun.png"),
            wood: Image::load("img/wood.png"),
            shadow: Image::load("img/shadow.png"),
            wall: Image::load("img/wall.png"),
            bat: Image::load("img/bat.png"),
            fire: Sound::load("snd/gun.wav"),
            death: Sound::load("snd/bat-death.wav")
        }
    }
}

impl Screen for LoadingScreen {
    fn update(&mut self, _window: &mut Window, _canvas: &mut Canvas) -> Option<Box<Screen>> {
        let images = &mut [
            &mut self.player,
            &mut self.crosshair,
            &mut self.gun,
            &mut self.wood,
            &mut self.shadow,
            &mut self.wall,
            &mut self.bat
        ];
        let sounds = &mut [
            &mut self.fire,
            &mut self.death
        ];
        if let (Some(images), Some(sounds)) = (update_all(images), update_all(sounds)) {
            let player_image = images[0].clone();
            let crosshair = images[1].clone();
            let gun = images[2].clone();
            let wood = images[3].clone();
            let shadow = images[4].clone();
            let wall = images[5].clone();
            let bat = images[6].clone();
            let fire = sounds[0].clone();
            let death = sounds[1].clone();
            Some(Box::new(GameScreen::new(LoadResults {
                player_image,
                crosshair,
                gun,
                wood,
                shadow,
                wall,
                bat,
                fire,
                death
            })))
        } else {
            None
        }
    }

    fn draw(&mut self, window: &mut Window, canvas: &mut Canvas) {
        canvas.clear(Color::white());
        canvas.present(window);
    }
}
