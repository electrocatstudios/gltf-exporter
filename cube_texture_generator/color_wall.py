

# Imports PIL module
from PIL import Image
 
# creating a image object (new image object) with
# RGB mode and size 200x200
width = 1024
height = 1024
im = Image.new(mode="RGB", size=(width, height))

for x in range(0,width):
    for y in range(0,height):
            red = int((x/width) * 255)
            blue = int((y/height) * 255)
            im.putpixel((x,y), (red, 155, blue, 255))
            # image.putpixel( (x, x), (0, 0, 0, 255) ) 

# This method will show image in any image viewer
im.save('output.png')
