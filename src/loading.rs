use super::*;

pub struct LoadingScreen {
    player: LoadingAsset<Image>,
    crosshair: LoadingAsset<Image>,
    gun: LoadingAsset<Image>,
    wood: LoadingAsset<Image>,
    shadow: LoadingAsset<Image>,
    wall: LoadingAsset<Image>,
    fire: LoadingAsset<Sound>
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
            fire: Sound::load("snd/gun.wav")
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
            &mut self.wall
        ];
        let sounds = &mut [
            &mut self.fire
        ];
        if let (Some(images), Some(sounds)) = (update_all(images), update_all(sounds)) {
            let player_image = images[0].clone();
            let crosshair = images[1].clone();
            let gun = images[2].clone();
            let wood = images[3].clone();
            let shadow = images[4].clone();
            let wall = images[5].clone();
            let fire = sounds[0].clone();
            let player_pos = Circle::newi(100, 100, PLAYER_RADIUS);
            let enemies = vec![Enemy::new(Circle::newi(400, 400, PLAYER_RADIUS/2)),
                               Enemy::new(Circle::newi(300, 400, PLAYER_RADIUS/2)),
                               Enemy::new(Circle::newi(200, 250, PLAYER_RADIUS/2))];
            let projectiles = vec![];
            let shoot_cooldown = 0;
            Some(Box::new(GameScreen { 
                player_pos, 
                enemies, 
                projectiles, 
                player_image, 
                crosshair, 
                gun,
                wood,
                shadow,
                wall,
                fire,
                wall_scroll: 0.0,
                shoot_cooldown }))
        } else {
            None
        }
    }

    fn draw(&mut self, window: &mut Window, canvas: &mut Canvas) {
        canvas.clear(Color::white());
        canvas.present(window);
    }
}
