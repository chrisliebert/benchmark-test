// Copyright (C) 2016 Chris Liebert

import java.util.ArrayList;
import java.util.List;

class Wertziel {
	public void printConglomerates(int x, int y, int z) {
		int xbackup = x;
		int ybackup = y;
		int zbackup = z;
		List<int[]> collections = new ArrayList<int[]>();
		int a, b, c, d, e, f, g, h = 0;
		a = 0;
		while (loopInv(x, y, z)) {
			b = 0;
			int bx = x;
			int by = y;
			int bz = z;
			while (loopInv(x, y, z)) {
				c = 0;
				int cx = x;
				int cy = y;
				int cz = z;
				while (loopInv(x, y, z)) {
					d = 0;
					int dx = x;
					int dy = y;
					int dz = z;
					while (loopInv(x, y, z)) {
						e = 0;
						int ex = x;
						int ey = y;
						int ez = z;
						while (loopInv(x, y, z)) {
							f = 0;
							int fx = x;
							int fy = y;
							int fz = z;
							while (loopInv(x, y, z)) {
								g = 0;
								int gx = x;
								int gy = y;
								int gz = z;
								while (loopInv(x, y, z)) {
									h = 0;
									int hx = x;
									int hy = y;
									int hz = z;
									while (loopInv(x, y, z)) {
										if ((x == 0) && (y == 0) && (z == 0)) {
											int collection[] = { a, b, c, d, e, f, g, h };
											collections.add(collection);
											//System.out.println("Adding congl:");
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
		
		for(int i = 0; i < collections.size(); i++) {
			int card = 0;
			for(int j = 0; j < 8; j++) {
				card += collections.get(i)[j];
			}
			
			if(minCard == 0 || card < minCard) {
				minCard = card;
				minCardIndex = i;
			}
		}
		
		if(minCardIndex == -1) {
			System.out.println("No conglomerates found");
		}
		
		int[] conglomerats = collections.get(minCardIndex);
		
		System.out.println("Alphium\tBetium\tCetium\tDeltium\tEtium\tFerium\tGatium\tHerium\t|CRD|\t X\t Y\t Z\n");
		
		String s = "";
		for(int i = 0; i < 8; i++) {
			s += " " + conglomerats[i] + "\t";
		}
		s += minCard + "\t" + xbackup + "\t" + ybackup + "\t" + zbackup;
		System.out.println(s);
	}

	private boolean loopInv(int x, int y, int z) {
		return ((x >= 0) && (y >= 0) && (z >= 0));
	}

	public static void main(String[] args) {
		Wertziel w = new Wertziel();
		w.printConglomerates(70, 73, 77);
	}
}
