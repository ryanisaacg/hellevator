use super::*;


pub struct LoadingScreen {
    images: JoinAll<Vec<ImageLoader>>,
    sounds: JoinAll<Vec<SoundLoader>>,
}

impl InitialScreen for LoadingScreen {
    fn configure() -> (Window, Canvas) {
        WindowBuilder::new()
            .with_show_cursor(false)
            .build("Hellevator", 960, 540)
    }

    fn new() -> Self {
        LoadingScreen {
            images: join_all(vec![
                Image::load("img/ah_stand.png"),
                Image::load("img/crosshair.png"),
                Image::load("img/gun.png"),
                Image::load("img/wood.png"),
                Image::load("img/shadow.png"),
                Image::load("img/wall.png"),
                Image::load("img/bat.png"),
                Image::load("img/md_stand.png"),
                Image::load("img/spider.png"),
                Image::load("img/angry_spider.png")]),
            sounds: join_all(vec![
                Sound::load("snd/gun.wav"),
                Sound::load("snd/bat-death.wav")])
        }
    }
}

impl Screen for LoadingScreen {
    fn update(&mut self, _window: &mut Window, _canvas: &mut Canvas) -> Option<Box<Screen>> {
        //TODO: error screen
        if let (Ok(Async::Ready(images)), Ok(Async::Ready(sounds))) = (self.images.poll(), self.sounds.poll()) {
            let player_image = images[0].clone();
            let crosshair = images[1].clone();
            let gun = images[2].clone();
            let wood = images[3].clone();
            let shadow = images[4].clone();
            let wall = images[5].clone();
            let bat = images[6].clone();
            let medic = images[7].clone();
            let spider = images[8].clone();
            let angry_spider = images[9].clone();
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
                death,
                medic,
                spider,
                angry_spider
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
