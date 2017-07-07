-- Copyright (C) 2016 Chris Liebert

function getConglomerates(x, y, z)
  
  collections = {}
  num_collections = 0

--       Alphium (A): X
--       Betium (B):  Y
--       Cetium (C):  Z
--       Deltium (D): XXXYYYZ (3X3YZ)
--       Etium (E):   YYZZZZZ (2Y5Z)
--       Ferium (F):  XXXXXXZZ (6X2Z)
--       Gatium (G):  XXYYYYYYZ (2X6YZ)
--       Herium (H):  YZZ (Y2Z)

  -- Get all conglamorations
  a = 0
  while x >= 0 and y >= 0 and z >= 0 do
    b = 0
    bx, by, bz = x, y, z
    while x >= 0 and y >= 0 and z >= 0 do
      c = 0
      cx, cy, cz = x, y, z
      while x >= 0 and y >= 0 and z >= 0 do
        d = 0
        dx, dy, dz = x, y, z
        while x >= 0 and y >= 0 and z >= 0 do
          e = 0
          ex, ey, ez = x, y, z
          while x >= 0 and y >= 0 and z >= 0 do
            f = 0
            fx, fy, fz = x, y, z
            while x >= 0 and y >= 0 and z >= 0 do
              g = 0
              gx, gy, gz = x, y, z
              while x >= 0 and y >= 0 and z >= 0 do
                h = 0
                hx, hy, hz = x, y, z
                while x >= 0 and y >= 0 and z >= 0 do
                  
                  if x == 0  and y == 0 and z == 0 then
                    num_collections = num_collections + 1
                    collections[num_collections] = { a, b, c, d, e, f, g, h }
                  end
                  
                  --Y2Z
                  y = y - 1
                  z = z - 2

                  h = h + 1
                end
            

                --2X6YZ
                x = hx - 2
                y = hy - 6
                z = hz - 1

                g = g + 1
              end
              
              --6X2Z
              x = gx - 6
              y = gy
              z = gz - 2

              f = f + 1
            end

            --2Y5Z
            x = fx
            y = fy - 2
            z = fz - 5

            e = e + 1
          end

          --3X3YZ
          x = ex - 3
          y = ey - 3
          z = ez - 1

          d = d + 1
        end

        --Z
        x = dx
        y = dy
        z = dz - 1

        c = c + 1
      end

      --Y
      x = cx
      y = cy - 1
      z = cz

      b = b + 1
    end
    x = bx - 1
    y = by
    z = bz
    
    a = a + 1
  end

  min_cardinality = 0
  min_cardinality_index = 0
  for i = 1, num_collections do
    -- Calculate cardinality of each
    cardinality = 0
    
    for j = 1, 8 do
      cardinality = cardinality + collections[i][j]
    end

    if min_cardinality == 0 then
      min_cardinality = cardinality
      min_cardinality_index = i
    end

    if cardinality < min_cardinality then
      min_cardinality = cardinality
      min_cardinality_index = i
    end
  end

  if min_cardinality_index == 0 then
    print("No conglomarates found")
    return nil
  end

  return collections[min_cardinality_index], min_cardinality

end


function printColumns()
  print('Alphium\tBetium\tCetium\tDeltium\tEtium\tFerium\tGatium\tHerium\t|CRD|\t X\t Y\t Z\n')
end


function printConglomerates(x, y, z)
  conglomerates, cardinality = getConglomerates(x, y, z)
  if conglomerates == nil then return end
  str = ''
  for i = 1, 8 do  -- Table iteration.
    str = str .. conglomerates[i] .. '\t'
  end
  print(str .. cardinality .. '\t' .. x .. '\t' .. y .. '\t' .. z )
end


printColumns()

printConglomerates(70, 73, 77)

