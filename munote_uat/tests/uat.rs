use cucumber::World;
use munote_uat::MusicWorld;

fn main() {
    futures::executor::block_on(MusicWorld::run("tests/features"));
}
