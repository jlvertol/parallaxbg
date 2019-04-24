use sdl2::rect::{Rect};
use sdl2::render::{WindowCanvas, Texture};
use std::cmp;


pub trait Widget {
    fn render(&mut self, canvas: &mut WindowCanvas);
}

pub struct ParallaxBg {
    pub layers: Vec<ParallaxLayer>
}

impl ParallaxBg {
}

impl Widget for ParallaxBg {
    fn render(&mut self, canvas: &mut WindowCanvas) {
        for i in 0 .. self.layers.len() {
            let mut par_layer = self.layers.get_mut(i).unwrap();
            let width:  u32 = par_layer.layer.query().width;
            let height: u32 = par_layer.layer.query().height;
            let x_pos:  u32 = par_layer.x_pos;
            let dst_size = canvas.window().drawable_size().0;
            let mut render_len: u32 = 0;    //counter to see how much has been drawn so far
            canvas.copy(&par_layer.layer,
                        Rect::new(x_pos as i32, 0, cmp::min(width - x_pos, dst_size), height),
                        Rect::new(0, 0, cmp::min(width - x_pos, dst_size), height))
                .expect("Error rendering ParallaxBg to the backbuffer in outer loop");
            render_len += cmp::min(width - x_pos, dst_size);

            while render_len < dst_size {
                let remainder = dst_size - render_len;
                canvas.copy(&par_layer.layer,
                            Rect::new(0, 0, cmp::min(width, remainder), height),
                            Rect::new(render_len as i32, 0, cmp::min(width, remainder), height))
                    .expect("Error rendering ParallaxBg to the backbuffer in inner loop");
                render_len += cmp::min(width, remainder);
            }

            par_layer.displace();
        }
    }
}
pub struct ParallaxLayer {
    pub layer: Texture,
    pub x_increment: i32,
    x_pos: u32
}

impl ParallaxLayer {
    pub fn new(layer: Texture, x_increment: i32) -> ParallaxLayer {
        let width = layer.query().width as i32;
        ParallaxLayer {layer, x_increment: x_increment % width, x_pos: 0}
    }
    pub fn displace(&mut self) {
        self.x_pos = offset(self.x_increment, self.x_pos, self.layer.query().width)
    }
}

/** increment is how much it should move per frame,
 pos is the original offset,
 side is the size of the image to be offset, in that dimension */
fn offset(increment: i32, pos: u32, side: u32) -> u32 {
    let new_pos = increment + pos as i32;
    let remainder = new_pos % side as i32;
    if increment < 0 {
        (if remainder < 0 {side as i32 + remainder} else {remainder}) as u32
    } else {
        remainder as u32
    }
}

#[cfg(test)]
mod offset {
    use super::*;

    #[test]
    fn positive() {
        assert_eq!(offset(3, 0, 10), 3);
    }
    #[test]
    fn negative() {
        assert_eq!(offset(-3, 0, 10), 7);
    }
    #[test]
    fn positive_larger_than_side() {
        assert_eq!(offset(12, 0, 10), 2);
    }
    #[test]
    fn negative_larger_than_side() {
        assert_eq!(offset(-14, 0, 10), 6);
    }
    #[test]
    fn positive_with_pos() {
        assert_eq!(offset(3, 4, 10), 7);
    }
    #[test]
    fn negative_with_pos() {
        assert_eq!(offset(-3, 4, 10), 1);
    }
    #[test]
    fn positive_larger_than_side_with_pos() {
        assert_eq!(offset(12, 4, 10), 6);
    }
    #[test]
    fn negative_larger_than_side_with_pos() {
        assert_eq!(offset(-14, 4, 10), 0);
    }
}

//------ ~~~ 🍄 🦀 🐉 ~~~ ------//

/** Maybe not every function will have a test, since it's especially difficult to test GUIs. But we
can test the algorithm's logic. This serves a dual purpose of ensuring every corner case is covered
and documenting what the algorithm does. */

/** horizontal only for now */
#[allow(dead_code)]
fn tiled_blit_sim(src: Vec<usize>, dst_size: usize, offset: usize) -> Vec<usize> {
    let w = src.len();   //src_size
    let x = offset % w;  //actual offset

    let mut dst = vec![];

    /** Line representation of the whole idea, with labels below:
        {     }|     |       |       | |   example of blitting, {} is skipped space
        -x     0    w-x    2w-x   3w-x W
        {  x  }| w-x |   w   |   w   |?|  <--- W is the size of the whole thing  */

    fn copy(src_xw: (usize, usize), dst_xw: (usize, usize), src: &Vec<usize>, dst: &mut Vec<usize>) {
        let (sx, sw) = src_xw;
        let (_, dw) = dst_xw;
        assert_eq!(sw, dw);
        let to_copy = &src[sx..(sx+sw)];
        to_copy.iter().for_each(|elem| {
            dst.push(*elem);
        });
    }

    copy((x, cmp::min(w - x, dst_size) ),
         (0, cmp::min(w - x, dst_size) ), &src, &mut dst);

    while dst.len() < dst_size {
        let remainder = dst_size - dst.len();
        copy((0,             cmp::min(w, remainder)),
             (dst.len() - 1, cmp::min(w, remainder)), &src, &mut dst);
    }
    dst
}

//------ ~~~ 🍄 🦀 🐉 ~~~ ------//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sim_blit_one_to_one () {
        assert_eq!(tiled_blit_sim(vec![1, 2, 3, 4], 4, 0),
                   vec![1, 2, 3, 4]);
    }

    #[test]
    fn sim_blit_one_to_two () {
        assert_eq!(tiled_blit_sim(vec![1, 2, 3, 4], 8, 0),
                   vec![1, 2, 3, 4, 1, 2, 3, 4]);
    }

    #[test]
    fn sim_blit_no_offset_inexact_fit () {
        assert_eq!(tiled_blit_sim(vec![1, 2, 3], 7, 0),
                   vec![1, 2, 3, 1, 2, 3, 1]);
    }

    #[test]
    fn sim_blit_with_offset_fit_at_the_end () {
        assert_eq!(tiled_blit_sim(vec![1, 2, 3], 7, 2),
                   vec![3, 1, 2, 3, 1, 2, 3]);
   }

    #[test]
    fn sim_blit_with_remainder_on_both_sides () {
        assert_eq!(tiled_blit_sim(vec![1, 2, 3, 4, 5], 8, 3),
                   vec![4, 5, 1, 2, 3, 4, 5, 1]);
    }

    #[test]
    fn sim_blit_with_remainder_on_both_sides2 () {
        assert_eq!(tiled_blit_sim(vec![1, 2, 3, 4], 9, 1),
                   vec![2, 3, 4, 1, 2, 3, 4, 1, 2]);
    }

    #[test]
    fn sim_blit_overly_large_offset () {
        assert_eq!(tiled_blit_sim(vec![1, 2, 3], 6, 7),
                   vec![2, 3, 1, 2, 3, 1]);
    }

    #[test]
    fn sim_blit_overly_large_offset2 () {
        assert_eq!(tiled_blit_sim(vec![1, 2, 3, 4], 7, 6),
                   vec![3, 4, 1, 2, 3, 4, 1]);
    }

    #[test]
    fn sim_blit_too_small_no_offset () {
        assert_eq!(tiled_blit_sim(vec![1, 2, 3, 4], 3, 0),
                   vec![1, 2, 3]);
    }

    #[test]
    fn sim_blit_too_small_with_offset () {
        assert_eq!(tiled_blit_sim(vec![1, 2, 3, 4], 3, 2),
                   vec![3, 4, 1]);
    }
    #[test]
    fn sim_blit_too_small_with_big_offset () {
        assert_eq!(tiled_blit_sim(vec![1, 2, 3, 4, 5], 3, 14),
                   vec![5, 1, 2]);
    }
}