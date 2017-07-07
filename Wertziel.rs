// Copyright (C) 2017 Chris Liebert

#[allow(unused_assignments)]

pub fn print_conglomerates(x0: i32, y0: i32, z0: i32) {
    let mut x = x0;
    let mut y = y0;
    let mut z = z0;
    let xbackup = x0;
    let ybackup = y0;
    let zbackup = z0;
    let mut collections: Vec<[i32; 8]> = vec!();
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    let mut d: i32 = 0;
    let mut e: i32 = 0;
    let mut f: i32 = 0;
    let mut g: i32 = 0;
    let mut h: i32 = 0;
    a = 0;
    while (x >= 0) && (y >= 0) && (z >= 0) {
        b = 0;
        let bx = x;
        let by = y;
        let bz = z;
        while (x >= 0) && (y >= 0) && (z >= 0) {
            c = 0;
            let cx = x;
            let cy = y;
            let cz = z;
            while (x >= 0) && (y >= 0) && (z >= 0) {
                d = 0;
                let dx = x;
                let dy = y;
                let dz = z;
                while (x >= 0) && (y >= 0) && (z >= 0) {
                    e = 0;
                    let ex = x;
                    let ey = y;
                    let ez = z;
                    while (x >= 0) && (y >= 0) && (z >= 0) {
                        f = 0;
                        let fx = x;
                        let fy = y;
                        let fz = z;
                        while (x >= 0) && (y >= 0) && (z >= 0) {
                            g = 0;
                            let gx = x;
                            let gy = y;
                            let gz = z;
                            while (x >= 0) && (y >= 0) && (z >= 0) {
                                h = 0;
                                let hx = x;
                                let hy = y;
                                let hz = z;
                                while (x >= 0) && (y >= 0) && (z >= 0) {
                                    if (x == 0) && (y == 0) && (z == 0) {
                                        let collection: [i32; 8] = [a, b, c, d, e, f, g, h];
                                        collections.push(collection);
                                    }
                                    y -= 1;
                                    z -= 2;
                                    h += 1;
                                }
                                x = hx - 2;
                                y = hy - 6;
                                z = hz - 1;
                                g += 1;
                            }
                            x = gx - 6;
                            y = gy;
                            z = gz - 2;
                            f += 1;
                        }
                        x = fx;
                        y = fy - 2;
                        z = fz - 5;
                        e += 1;
                    }
                    x = ex - 3;
                    y = ey - 3;
                    z = ez - 1;
                    d += 1;
                }
                x = dx;
                y = dy;
                z = dz - 1;
                c += 1;
            }
            x = cx;
            y = cy - 1;
            z = cz;
            b += 1;
        }
        x = bx - 1;
        y = by;
        z = bz;
        a += 1;
    }
    
    let mut min_card = 0;
    let mut min_card_index: i32 = -1;
    
    for i in 0..collections.len() {
        let mut card = 0;
        for j in 0..8 {
            card += collections[i][j];
        }
        
        if min_card == 0 || card < min_card {
            min_card = card;
            min_card_index = i as i32;
        }
    }
    
    if min_card_index == -1 {
        println!("No conglomerates found");
    }
    
    let conglomerats: [i32; 8] = collections[min_card_index as usize];
    
    println!("Alphium\tBetium\tCetium\tDeltium\tEtium\tFerium\tGatium\tHerium\t|CRD|\t X\t Y\t Z\n");
    
    let mut s = String::from("");
    for i in 0..8 {
        s.push_str(&format!(" {}\t", conglomerats[i]));
    }
    s.push_str(&format!("{}\t{}\t{}\t{}", min_card, xbackup, ybackup, zbackup));
    println!("{}", &s);
}

pub fn main() {
    print_conglomerates(70, 73, 77);
}
