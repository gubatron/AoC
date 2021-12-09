import aoc
from functools import reduce
l = aoc.readFileToStringList("9.txt")
G=[]
for line in l:
  G.append(list(map(int,list(line))))
def getNeighbors(G,i,j,includeCoordinates=False):
  neighbors=[]
  # top
  if i > 0:
    if includeCoordinates:
      neighbors.append((G[i-1][j],(i-1,j)))
    else:    
      neighbors.append(G[i-1][j])
  # bottom
  if i < len(G)-1:
    if includeCoordinates:
      neighbors.append((G[i+1][j],(i+1,j)))
    else:    
      neighbors.append(G[i+1][j])
  # right
  if j < len(G[0])-1:
    if includeCoordinates:
      neighbors.append((G[i][j+1],(i,j+1)))
    else:
      neighbors.append(G[i][j+1])
  # left
  if j > 0:
    if includeCoordinates:
      neighbors.append((G[i][j-1],(i,j-1)))
    else:
      neighbors.append(G[i][j-1])
  return neighbors

def isLow(G,i,j):
  me = G[i][j]
  if me == 9:
      return False
  neighbors=sorted(getNeighbors(G,i,j))
  return me < neighbors[0]

def findLows(G, includeCoordinates=False):
    lows = []
    for row in range(len(G)):
        for col in range(len(G[0])):
            if isLow(G,row,col):
                if includeCoordinates:
                    lows.append((G[row][col],(row,col)))
                else:
                    lows.append(G[row][col])
    return lows

lows = findLows(G)
ANS1=sum_risk_levels = sum(map(lambda x:x+1, lows))
lows_w_coords = findLows(G,True)
# BFS
def findBasin(G,i,j):
    basin=[]
    queue=[]
    me = G[i][j]
    basin.append((i,j))
    queue.append((i,j))
    while queue:
      s = queue.pop(0)
      neighbors_no_nines = list(filter(lambda x: x[0] < 9, getNeighbors(G,s[0],s[1],True)))
      for n in neighbors_no_nines:
          if n[1] not in basin:
              basin.append(n[1])
              queue.append(n[1])
    return basin

basins = []
for low_coord in lows_w_coords:
    basins.append(findBasin(G, low_coord[1][0], low_coord[1][1]))

ANS2 = reduce(lambda x,y: x * y, list(map(len,sorted(basins,key=len,reverse=True)[0:3])))
print("ans1={}".format(ANS1)) # 588
print("ans2={}".format(ANS2)) # 964712
