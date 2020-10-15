# Chargen

## About

Character declaration generator from 5x7 BMP images for LCD displays

## Usage

```
chargen [PATH TO BITMAP]
```

Sample output:

```c
byte symbol[8] = {
        B11110,
        B10001,
        B10001,
        B10001,
        B11110,
        B10000,
        B10000,
};
```

Original image (double width ASCII graphics):

```
████████  
██      ██
██      ██
██      ██
████████  
██        
██        
```