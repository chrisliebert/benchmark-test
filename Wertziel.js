// Copyright (C) 2017 Chris Liebert

var printConglomerates = function(x, y, z) {
    var xbackup = x;
    var ybackup = y;
    var zbackup = z;
    var collections = [];
    var a = 0, b = 0, c = 0, d = 0, e = 0, f = 0, g = 0, h = 0;
    a = 0;
    while (((x >= 0) && (y >= 0) && (z >= 0))) {
        b = 0;
        var bx = x;
        var by = y;
        var bz = z;
        while (((x >= 0) && (y >= 0) && (z >= 0))) {
            c = 0;
            var cx = x;
            var cy = y;
            var cz = z;
            while (((x >= 0) && (y >= 0) && (z >= 0))) {
                d = 0;
                var dx = x;
                var dy = y;
                var dz = z;
                while (((x >= 0) && (y >= 0) && (z >= 0))) {
                    e = 0;
                    var ex = x;
                    var ey = y;
                    var ez = z;
                    while (((x >= 0) && (y >= 0) && (z >= 0))) {
                        f = 0;
                        var fx = x;
                        var fy = y;
                        var fz = z;
                        while (((x >= 0) && (y >= 0) && (z >= 0))) {
                            g = 0;
                            var gx = x;
                            var gy = y;
                            var gz = z;
                            while (((x >= 0) && (y >= 0) && (z >= 0))) {
                                h = 0;
                                var hx = x;
                                var hy = y;
                                var hz = z;
                                while (((x >= 0) && (y >= 0) && (z >= 0))) {
                                    if ((x == 0) && (y == 0) && (z == 0)) {
                                        collections.push([a, b, c, d, e, f, g, h ]);
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
    
    var minCard = 0;
    var minCardIndex = -1;
    
    for(var i = 0; i < collections.length; i++) {
        var card = 0;
        for(var j = 0; j < 8; j++) {
            card += collections[i][j];
        }
        
        if(minCard == 0 || card < minCard) {
            minCard = card;
            minCardIndex = i;
        }
    }
    
    if(minCardIndex == -1) {
        console.log("No conglomerates found");
    }
    
    var conglomerats = collections[minCardIndex];
    
    console.log("Alphium\tBetium\tCetium\tDeltium\tEtium\tFerium\tGatium\tHerium\t|CRD|\t X\t Y\t Z\n");
    
    var s = "";
    for(var i = 0; i < 8; i++) {
        s += " " + conglomerats[i] + "\t";
    }
    s += minCard + "\t" + xbackup + "\t" + ybackup + "\t" + zbackup;
    console.log(s);
}

printConglomerates(70, 73, 77);
