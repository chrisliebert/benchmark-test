#!python3
# Copyright (C) 2016 Chris Liebert

def getConglomerates(x, y, z):
    a,b,c,d,e,f,g,h = 0,0,0,0,0,0,0,0

    collections = []
    num_collections = 0

    a = 0
    while x >= 0 and y >= 0 and z >= 0:
        b = 0
        bx,by,bz = x,y,z
        while x >= 0 and y >= 0 and z >= 0:
            c = 0
            cx,cy,cz = x,y,z
            while x >= 0 and y >= 0 and z >= 0:
                d = 0
                dx,dy,dz = x,y,z
                while x >= 0 and y >= 0 and z >= 0:
                    e = 0
                    ex,ey,ez = x,y,z
                    while x >= 0 and y >= 0 and z >= 0:
                        f = 0
                        fx,fy,fz = x,y,z
                        while x >= 0 and y >= 0 and z >= 0:
                            g = 0
                            gx,gy,gz = x,y,z
                            while x >= 0 and y >= 0 and z >= 0:
                                h = 0
                                hx,hy,hz = x,y,z
                                while x >= 0 and y >= 0 and z >= 0:
                                    if x == 0 and y == 0 and z == 0:
                                        collections.append([a,b,c,d,e,f,g,h])

                                    y -= 1
                                    z -= 2
                                    h += 1

                                x = hx - 2
                                y = hy - 6
                                z = hz - 1
                                g += 1

                            x = gx - 6
                            y = gy
                            z = gz - 2
                            f += 1

                        x = fx
                        y = fy - 2
                        z = fz - 5
                        e += 1

                    x = ex - 3
                    y = ey - 3
                    z = ez - 1
                    d += 1

                x = dx
                y = dy
                z = dz - 1
                c += 1

            x = cx
            y = cy - 1
            z = cz
            b += 1

        x = bx - 1
        y = by
        z = bz
        a += 1
    
    num_collections = len(collections)
    min_cardinality = 0
    min_cardinality_index = -1
    for i in range(0, num_collections):
        cardinality = 0
        for j in range(0, 8):
            cardinality += collections[i][j]
        
        if min_cardinality == 0:
            min_cardinality = cardinality
            min_cardinality_index = i

        if cardinality < min_cardinality:
            min_cardinality = cardinality
            min_cardinality_index = i

    if min_cardinality_index == -1:
        print("No conglomerates found")
        return None
        
    return (collections[min_cardinality_index], min_cardinality)


def printHeading():
    print("Alphium\tBetium\tCetium\tDeltium\tEtium\tFerium\tGatium\tHerium\t|CRD|\t X\t Y\t Z\n")

def printConglomerates(x, y, z):
    conglomerates = getConglomerates(x, y, z)
    s = ""
    for i in range(0, 8):
        s += " " + str(conglomerates[0][i])
        s += "\t"

    print(s, conglomerates[1], '\t', x, '\t', y, '\t', z, '\n')

if __name__ == '__main__':
    printHeading()
    printConglomerates(70, 73, 77)
