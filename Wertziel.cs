// Copyright (C) 2017 Chris Liebert

using System.Collections.Generic;

namespace wertziel {
    class Wertziel {
        public void PrintConglomerates(int x, int y, int z) {
            int xbackup = x;
            int ybackup = y;
            int zbackup = z;
            List<int[]> collections = new List<int[]>();
            int a, b, c, d, e, f, g, h = 0;
            a = 0;
            while (((x >= 0) && (y >= 0) && (z >= 0))) {
                b = 0;
                int bx = x;
                int by = y;
                int bz = z;
                while (((x >= 0) && (y >= 0) && (z >= 0))) {
                    c = 0;
                    int cx = x;
                    int cy = y;
                    int cz = z;
                    while (((x >= 0) && (y >= 0) && (z >= 0))) {
                        d = 0;
                        int dx = x;
                        int dy = y;
                        int dz = z;
                        while (((x >= 0) && (y >= 0) && (z >= 0))) {
                            e = 0;
                            int ex = x;
                            int ey = y;
                            int ez = z;
                            while (((x >= 0) && (y >= 0) && (z >= 0))) {
                                f = 0;
                                int fx = x;
                                int fy = y;
                                int fz = z;
                                while (((x >= 0) && (y >= 0) && (z >= 0))) {
                                    g = 0;
                                    int gx = x;
                                    int gy = y;
                                    int gz = z;
                                    while (((x >= 0) && (y >= 0) && (z >= 0))) {
                                        h = 0;
                                        int hx = x;
                                        int hy = y;
                                        int hz = z;
                                        while (((x >= 0) && (y >= 0) && (z >= 0))) {
                                            if ((x == 0) && (y == 0) && (z == 0)) {
                                                int[] collection = { a, b, c, d, e, f, g, h };
                                                collections.Add(collection);
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
            
            int minCard = 0;
            int minCardIndex = -1;
            
            for(int i = 0; i < collections.Count; i++) {
                int card = 0;
                for(int j = 0; j < 8; j++) {
                    card += collections[i][j];
                }
                
                if(minCard == 0 || card < minCard) {
                    minCard = card;
                    minCardIndex = i;
                }
            }
            
            if(minCardIndex == -1) {
                System.Console.Out.WriteLine("No conglomerates found");
            }
            
            int[] conglomerats = collections[minCardIndex];
            
            System.Console.Out.WriteLine("Alphium\tBetium\tCetium\tDeltium\tEtium\tFerium\tGatium\tHerium\t|CRD|\t X\t Y\t Z\n");
            
            string s = "";
            for(int i = 0; i < 8; i++) {
                s += " " + conglomerats[i] + "\t";
            }
            s += minCard + "\t" + xbackup + "\t" + ybackup + "\t" + zbackup;
            System.Console.Out.WriteLine(s);
        }

        public static void Main(string[] args) {
            Wertziel w = new Wertziel();
            w.PrintConglomerates(70, 73, 77);
        }
    }
}