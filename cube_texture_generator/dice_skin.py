

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

dot_size = 18

#left side - one
tl = 0
th = int(height/2) - int(height/4)
center_pt = (tl + width_len/2, th + height_len/2)
for x in range(0, width_len):
    for y in range(0, height_len):
        if dist(center_pt, (x,y+th)) < dot_size:
            img.putpixel((x,y+th), (0,0,0,255))
        else:    
            img.putpixel((x,y+th), (255,255,255,255))

#top side - two
tl = int(width/4)
th = int(height/2)  - int(height/4)
dot1 = (tl + width_len / 4, th + height_len / 4)
dot2 = (tl + 3*(width_len / 4), th + 3*(height_len / 4))

for x in range(tl, tl+width_len):
    for y in range(th, th+height_len):
        if dist(dot1, (x,y)) < dot_size or dist(dot2, (x,y)) < dot_size :
            img.putpixel((x,y), (0,0,0,255))
        else:    
            img.putpixel((x,y), (155,155,255,255))


#back side - three
tl = int(width/4)
th = int(height/4)  - int(height/4)
dot1 = (tl + width_len / 4, th + height_len / 4)
dot2 = (tl + 3*(width_len / 4), th + 3*(height_len / 4))
dot3 = (tl + (width_len / 2), th + (height_len / 2))

for x in range(tl, tl+width_len):
    for y in range(th, th+height_len):
        if dist(dot1, (x,y)) < dot_size \
            or dist(dot2, (x,y)) < dot_size \
            or dist(dot3, (x,y)) < dot_size:
            img.putpixel((x,y), (0,0,0,255))
        else:    
            img.putpixel((x,y), (255,155,155,255))


#front side - four
tl = int(width/4)
th = (int(height/4) *3)  - int(height/4) 
dot1 = (tl + width_len / 4, th + height_len / 4)
dot2 = (tl + width_len / 4, th + 3*(height_len / 4))
dot3 = (tl + 3*(width_len / 4), th + (height_len / 4))
dot4 = (tl + 3*(width_len / 4), th + 3*(height_len / 4))

for x in range(tl, tl+width_len):
    for y in range(th, th+height_len):
        if dist(dot1, (x,y)) < dot_size \
            or dist(dot2, (x,y)) < dot_size \
            or dist(dot3, (x,y)) < dot_size \
            or dist(dot4, (x,y)) < dot_size:
            img.putpixel((x,y), (0,0,0,255))
        else:    
            img.putpixel((x,y), (155,255,155,255))

#bot side - five
tl = int(width/4) * 3
th = int(height/2)  - int(height/4)
dot1 = (tl + width_len / 4, th + height_len / 4)
dot2 = (tl + width_len / 4, th + 3*(height_len / 4))
dot3 = (tl + 3*(width_len / 4), th + (height_len / 4))
dot4 = (tl + 3*(width_len / 4), th + 3*(height_len / 4))
dot5 = (tl + (width_len / 2), th + (height_len / 2))

for x in range(tl, tl+width_len):
    for y in range(th, th+height_len):
        if dist(dot1, (x,y)) < dot_size \
            or dist(dot2, (x,y)) < dot_size \
            or dist(dot3, (x,y)) < dot_size \
            or dist(dot4, (x,y)) < dot_size \
            or dist(dot5, (x,y)) < dot_size:
            img.putpixel((x,y), (0,0,0,255))
        else:    
            img.putpixel((x,y), (255,255,155,255))

#right side - six
tl = int(width/2)
th = int(height/2)  - int(height/4)
dot1 = (tl + width_len / 4, th + height_len / 4)
dot2 = (tl + width_len / 4, th + 3*(height_len / 4))
dot3 = (tl + 3*(width_len / 4), th + (height_len / 4))
dot4 = (tl + 3*(width_len / 4), th + 3*(height_len / 4))
dot5 = (tl + (width_len / 4), th + (height_len / 2))
dot6 = (tl + 3*(width_len / 4), th + (height_len / 2))

for x in range(tl, tl+width_len):
    for y in range(th, th+height_len):
        if dist(dot1, (x,y)) < dot_size \
            or dist(dot2, (x,y)) < dot_size \
            or dist(dot3, (x,y)) < dot_size \
            or dist(dot4, (x,y)) < dot_size \
            or dist(dot5, (x,y)) < dot_size \
            or dist(dot6, (x,y)) < dot_size:
            img.putpixel((x,y), (0,0,0,255))
        else:    
            img.putpixel((x,y), (155,255,255,255))


# This method will show image in any image viewer
img.save('dice_skin.png')
