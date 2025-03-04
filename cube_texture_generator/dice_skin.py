

# Imports PIL module
from PIL import Image
import math

def dist(pt1, pt2):
    x = pt1[0] - pt2[0]
    y = pt1[1] - pt2[1]

    return math.sqrt(x*x + y*y)

# creating a image object (new image object) with
# RGB mode and size 200x200
width = 1024
height = 1024
img = Image.new(mode="RGB", size=(width, height))

width_len = int(width/4)
height_len = int(height/4)

dot_size = 10

#left side - one
tl = 0
th = int(height/2)
center_pt = (tl + width_len/2, th + height_len/2)
for x in range(0, width_len):
    for y in range(0, height_len):
        if dist(center_pt, (x,y+th)) < dot_size:
            img.putpixel((x,y+th), (0,0,0,255))
        else:    
            img.putpixel((x,y+th), (255,255,255,255))

# This method will show image in any image viewer
img.save('output.png')
