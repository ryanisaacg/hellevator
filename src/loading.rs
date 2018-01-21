use super::*;

pub struct LoadingScreen {
    player: LoadingAsset<Image>,
    crosshair: LoadingAsset<Image>
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
            crosshair: Image::load("img/crosshair.png")
        }
    }
}

impl Screen for LoadingScreen {
    fn update(&mut self, _window: &mut Window, _canvas: &mut Canvas) -> Option<Box<Screen>> {
        if let LoadingAsset::Loaded(ref player_image) = self.player {
            if let LoadingAsset::Loaded(ref crosshair) = self.crosshair {
                let player_image = player_image.clone();
                let crosshair = crosshair.clone();
                let player_pos = Circle::newi(100, 100, PLAYER_RADIUS);
                let enemies = vec![Enemy::new(Circle::newi(400, 400, PLAYER_RADIUS/2)),
                                   Enemy::new(Circle::newi(300, 400, PLAYER_RADIUS/2)),
                                   Enemy::new(Circle::newi(200, 250, PLAYER_RADIUS/2))];
                let projectiles = vec![];
                let shoot_cooldown = 0;
                Some(Box::new(GameScreen { player_pos, enemies, projectiles, player_image, crosshair, shoot_cooldown }))
            } else {
                self.crosshair.update();
                None
            }
        } else {
            self.player.update();
            None
        }
    }

    fn draw(&mut self, window: &mut Window, canvas: &mut Canvas) {
        canvas.clear(Color::white());
        canvas.present(window);
    }
}
