use rand::prelude::*;

/* 
def sample_data():
    n_assets = 100
    n_sectors = 10
    n_styles = 5

    np.random.seed(42)
    returns = np.random.randn(n_assets, 1)
    mkt_caps = np.abs(np.random.randn(n_assets, 1))
    sector_scores = np.random.randint(0, 2, size=(n_assets, n_sectors))
    style_scores = np.random.randn(n_assets, n_styles)

    return returns, mkt_caps, sector_scores, style_scores
*/

fn sample_data()
{
    let n_assets: i32 = 100;
    let n_sectors: i32 = 10;
    let n_styles: i32 = 5;

    // seed the rng
    let mut random_seed =  rand::rngs::StdRng::seed_from_u64(42);

}

fn main() 
{
    let i: i32 = 0;
    println!("Hello, world!");
}
