extern crate rand;

use std::{thread, time};
use rand::Rng;
use ggez::*;
//use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
fn main() {
//    let args: Vec<String> = env::args().collect();
//    let ruleset = &args[1];
//    let size = &args[2];
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("conway_proj", "Aarne Nixon")
		.build()
		.expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
//wrap:bool, edge:bool, fillMode:u32, width:u32, height:u32)
    let mut my_game = MyGame::new(&mut ctx, true, false, 0, 75, 75, 4.0, -1);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}
struct MyGame {
    scale: f32,
    state: Vec<Vec<bool>>,
    edges: Vec<Vec<bool>>,
    width: u32,
    height: u32,
    wrap: bool,
    frame: i32,
    frame_limit: i32
    // Your state here...
}
fn randBl() -> bool{
    let mut rng = rand::thread_rng();
    let rand1: bool = rng.gen();
    if rand1 {
        return true
    }else{
        return false
	}
}
impl MyGame {
    pub fn new(_ctx: &mut Context, wrap : bool, edge:bool, fillMode:u32, width:u32, height:u32, scale:f32, frame_limit:i32) -> MyGame {
        // Load/create resources such as images here.
//        let mut boolVecInst: <Vec<bool>>;
        let mut temp_state: Vec<Vec<bool>> = std::vec::Vec::new();
        let mut temp_edges: Vec<Vec<bool>> = std::vec::Vec::new();
        let mut boolVecInst = Vec::new();
        for x in 0..width {
            temp_state.push(boolVecInst.clone());
            temp_edges.push(boolVecInst.clone());
            for y in 0..height {
                temp_state[x as usize].push(randBl());
                temp_edges[x as usize].push(false);
/*                if wrap {
        //            edges
		        }else{
        
		        }*/
			}  
		}
        // let topRow: Vec<bool> = [edge; ];
        // let row: Vec<bool> = [edge, false ; , edge];
        // let lowRow: Vec<bool> = [edge; ];
        MyGame {
            wrap: wrap,
            width: width,
            height: height,
            scale: scale,
            state: temp_state,
            edges: temp_edges,
            frame: 0,
            frame_limit: frame_limit
        }
    }
}
fn getLimit(adjust:u32, max:u32, diff:i8) -> u128{
    let mut result : i128 = adjust as i128;
    if result + diff as i128 <= 0 && diff < 0{
        result = max as i128 as i128;//+1
    }
    if result + diff as i128 >= max as i128 && diff > 0{
        result = 0;//+1
    }
    result = (result as i128 + diff as i128);
//    println!("getlimit stage end {} {} {}",result,adjust,diff);
    return result as u128;
}
fn iterate(x:u32, y:u32, state:Vec<Vec<bool>>, width:u32, height:u32, wrapped:bool) -> bool{
    let mut sum = 0;
    if wrapped {
//        println!("test {} {} {} {}", getLimit(x,width,-1), getLimit(x,width,1), getLimit(y,height,-1),getLimit(y,height,1));
// println!("test {} {} {} => {}", x, width, -1, getLimit(x,width,-1));
// println!("test {} {} {} => {}", x, width, 1, getLimit(x,width,1));
// println!("test {} {} {} => {}", y, height, -1, getLimit(y,height,-1));
// println!("test {} {} {} => {}", y, height, 1, getLimit(y,height,1));
        for i in getLimit(x,width,-1)..getLimit(x,width,2){
            for j in getLimit(y,height,-1)..getLimit(y,height,2){
//                println!("tst for {} {} on {} {}", x, y, i, j);
                if state[i as usize][j as usize]{
                    sum = sum + 1;
//                    println!("loc {}, {} +1 from {}, {}", x, y, i, j);
                }
            }
        }
    }else{
        //#ADD unwrapped
    }
    if state[x as usize][y as usize] {//live
        sum = sum - 1;
        if sum < 2{
            return false;
        }
        if sum > 3 {
            return false;
        }
        return true;
    }else{//dead
        if sum == 3{
            return true;
        }
        return false;
    }
}
impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
//	    Ok(())
//        self.dt = timer::delta(_ctx);
        // let delay = time::Duration::from_millis(50);
        // thread::sleep(delay);
        // Update code here...
        let mut new_state: Vec<Vec<bool>> = std::vec::Vec::new();
 //       let mut new_edges: Vec<Vec<bool>> = std::vec::Vec::new();
        let mut boolVecInst = Vec::new();
        println!("ping");
        for x in 0..self.width {
            new_state.push(boolVecInst.clone());
            for y in 0..self.height {
                new_state[x as usize].push(iterate(x, y, self.state.clone(), self.width, self.height, true));
			}  
        }
        for x in 0..self.width {
            for y in 0..self.height {
                self.state[x as usize][y  as usize] = new_state[x  as usize][y  as usize];
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        for x in 0..self.width{
            for y in 0..self.height{
                let fresh_cube = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new((x as f32 * self.scale), (y as f32* self.scale), 1.0  * self.scale, 1.0  * self.scale),
                    if (self.state[x as usize][y as usize]) {
                        graphics::BLACK
					}else{
                        graphics::WHITE
					}
                )?;
                // if (self.state[x as usize][y as usize]) {
                //     print!("X");
                // }else{
                //     print!("Y");
                // }
                graphics::draw(ctx, &fresh_cube, graphics::DrawParam::default())?;
            }
//            println!("");
        }
        self.frame = self.frame + 1;
        if self.frame == self.frame_limit {
            std::process::exit(0);
        }
        // Draw code here...
        graphics::present(ctx)
    }
}