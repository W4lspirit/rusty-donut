use core::time;
use std::f32::consts::PI;
use std::thread;

const THETA_SPACING: f32 = 0.07;
const PHI_SPACING: f32 = 0.02;
const TWO_PI: f32 = 2.0 * PI;
const R_1: i32 = 1;
const R_2: i32 = 2;
const K_2: i32 = 5;
const RF_1: f32 = R_1 as f32;
const RF_2: f32 = R_2 as f32;
const KF_2: f32 = K_2 as f32;
const SCREEN_WIDTH: i32 = 20;
const SCREEN_HEIGHT: i32 = 10;
const HALF_SCREEN_WIDTH: i32 = SCREEN_WIDTH / 2;
const HALF_SCREEN_HEIGHT: i32 = SCREEN_HEIGHT / 2;
const K_1: i32 = SCREEN_WIDTH * K_2 * 3 / (8 * (R_1 + R_2));
const KF_1: f32 = K_1 as f32;

const LUMINANCE_CHAR: [char; 12] = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

fn main() {
    /*Calculate
    K1 based on screen size:
     * the maximum x-distance occurs  roughly at the edge of the torus, which is at x=R1+R2, z=0.
    We want that to be displaced 3/8ths of the width of the screen, which is 3/4th of  the way from the center to the side of the screen.

     SCREEN_WIDTH*3/8 = K1*(R1+R2)/(K2+0)
     SCREEN_WIDTH*K2*3/(8*(R1+R2)) = K1
     */
    let mut a = 0.;
    let mut b = 0.;
    loop {
        render_frame(a, b);
        a += 0.07;
        b += 0.03;
    }
}

fn render_frame(a: f32, b: f32) {
    // precompute sines and cosines of A and B
    let cos_a = a.cos();
    let sin_a = a.sin();
    let cos_b = b.cos();
    let sin_b = b.sin();

    let mut output = vec![vec![' '; (SCREEN_WIDTH + 1) as usize]; (SCREEN_HEIGHT + 1) as usize];
    let mut z_buffer = vec![vec![0.; (SCREEN_WIDTH + 1) as usize]; (SCREEN_HEIGHT + 1) as usize];

    let mut theta = 0.;
    // theta goes around the cross-sectional circle of a torus
    while theta < TWO_PI {
        // precompute sines and cosines of theta
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        let mut phi: f32 = 0.;
        // phi goes around the center of revolution of a torus
        while phi < TWO_PI {
            // println!("(phi&theta |{}_{}|)", phi, theta);
            // precompute sines and cosines of phi
            let sin_phi: f32 = phi.sin();
            let cos_phi: f32 = phi.cos();
            // the x,y coordinate of the circle, before revolving (factored out of the above equations)
            let circle_x: f32 = RF_2 + RF_1 * cos_theta;
            let circle_y: f32 = RF_1 * sin_theta;
            // println!("(circle |{}_{}|)", circle_x, circle_y);
            // final 3D (x,y,z) coordinate after rotations, directly from our math above
            let x: f32 =
                circle_x * (cos_b * cos_phi + sin_a * sin_b * sin_phi) - (circle_y * cos_a * cos_b);

            let y: f32 =
                circle_x * (sin_b * cos_phi - sin_a * cos_b * sin_phi) + (circle_y * cos_a * cos_b);

            let z: f32 = KF_2 + cos_a * circle_x * sin_phi + circle_y * sin_a;
            let ooz: f32 = 1. / z; // "one over z"
                                   // println!("(xyzooz |{}_{}_{}_{}|)", x, y, z, ooz);

            // x and y projection.  note that y is negated here, because y goes up in 3D space but down on 2D displays.
            let x_p = HALF_SCREEN_WIDTH as f32 + KF_1 * ooz * x;
            let y_p = HALF_SCREEN_HEIGHT as f32 + KF_1 * ooz * y;
            // println!("x_p y_p|{}_{}|", x_p, y_p);

            // calculate luminance.  ugly, but correct.
            // luminance, scaled back to 0 to 1
            let lum: f32 =
                (cos_phi * cos_theta * sin_b) - (cos_a * cos_theta * sin_phi) - (sin_a * sin_theta)
                    + cos_b * (cos_a * sin_theta - cos_theta * sin_a * sin_phi);

            // L ranges from -sqrt(2) to +sqrt(2).  If it's < 0, the surface is pointing away from us, so we won't bother trying to plot it.
            if lum > 0. {
                if x_p >= SCREEN_WIDTH as f32 {
                    // todo investigate  projection of x can be outside canvas
                    //println!("x_KF_1 ooz x|{}*{}*{}|", KF_1, ooz, x);

                    break;
                }
                if y_p >= SCREEN_HEIGHT as f32 {
                    //todo  investigate projection of y can be outside canvas
                    //  println!("x_KF_1 ooz y|{}*{}*{}|", KF_1, ooz, y);
                    break;
                }
                // test against the z-buffer.  larger 1/z means the pixel is closer to the viewer than what's already plotted.
                if ooz > z_buffer[y_p as usize][x_p as usize] {
                    z_buffer[y_p as usize][x_p as usize] = ooz;
                    let lum_index = (lum * 8.) as usize;
                    // luminance_index is now in the range 0..11 (8*sqrt(2) = 11.3) now we lookup the character corresponding to the luminance and plot it in our output:
                    output[y_p as usize][x_p as usize] = LUMINANCE_CHAR[lum_index];
                }
            }
            phi += PHI_SPACING;
        }
        theta += THETA_SPACING;
    }
    print!("\x1B[2J\x1B[1;1H");

    for i in 0..(SCREEN_HEIGHT as usize) {
        for j in 0..(SCREEN_WIDTH as usize) {
            print!("{}", output[i][j]);
        }
        print!("\n");
    }
    // ~60fps
    thread::sleep(time::Duration::from_millis(17));
}
