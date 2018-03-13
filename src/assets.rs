use super::*;

pub struct Assets {
    pub player_image: Image,
    pub crosshair: Image,
    pub gun: Image,
    pub wood: Image,
    pub shadow: Image,
    pub wall: Image,
    pub bat_up: Image,
    pub bat_down: Image,
    pub medic: Image,
    pub spider: Image,
    pub angry_spider: Image,
    pub web_spider: Image,
    pub spiderweb: Image,
    pub explode_spider: Image,
    pub mama_spider: Image,
    pub spider_skitter: [Image; 2],
    pub wire: Image,
    pub plus: Image,
    pub gear: Image,
    pub enemy_death_particle: Image,
    pub egg: Image,
    pub buffer_spider: Image,
    pub fire: Sound,
    pub death: Sound,
}

impl Assets {
    pub fn load() -> LoadingValue {
        let images = vec![Image::load("img/ah_stand.png"),
            Image::load("img/crosshair.png"),
            Image::load("img/gun.png"),
            Image::load("img/wood.png"),
            Image::load("img/shadow.png"),
            Image::load("img/wall.png"),
            Image::load("img/bat.png"),
            Image::load("img/md_stand.png"),
            Image::load("img/spider.png"),
            Image::load("img/angry_spider.png"),
            Image::load("img/gear.png"),
            Image::load("img/web_spider.png"),
            Image::load("img/spiderweb.png"),
            Image::load("img/explode_spider.png"),
            Image::load("img/mama_spider.png"),
            Image::load("img/plus.png"),
            Image::load("img/spider_skitter.png"),
            Image::load("img/wire.png"),
            Image::load("img/enemy_death_particle.png"),
            Image::load("img/egg.png"),
            Image::load("img/buffer_spider.png")];
        let sounds = vec![Sound::load("snd/gun.wav"),
            Sound::load("snd/bat-death.wav")];
        join_all(images).join(join_all(sounds))
    }

    pub fn new(loaded: (Vec<Image>, Vec<Sound>)) -> Assets {
        let (images, sounds) = loaded;
        Assets {
            player_image: images[0].clone(),
            crosshair: images[1].clone(),
            gun: images[2].clone(),
            wood: images[3].clone(),
            shadow: images[4].clone(),
            wall: images[5].clone(),
            bat_up: images[6].subimage(Rectangle::new(0, 0, 16, 16)),
            bat_down: images[6].subimage(Rectangle::new(16, 0, 16, 16)),
            medic: images[7].clone(),
            spider: images[8].clone(),
            angry_spider: images[9].clone(),
            gear: images[10].clone(),
            web_spider: images[11].clone(),
            spiderweb: images[12].clone(),
            explode_spider: images[13].clone(),
            mama_spider: images[14].clone(),
            plus: images[15].clone(),
            spider_skitter: [images[16].subimage(Rectangle::new_sized(12, 12)), images[16].subimage(Rectangle::new(12, 0, 12, 12))],
            wire: images[17].clone(),
            enemy_death_particle: images[18].clone(),
            egg: images[19].clone(),
            buffer_spider: images[20].clone(),
            fire: sounds[0].clone(),
            death: sounds[1].clone(),
        }
    }
}