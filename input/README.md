# input

include two excel files in this folder

## pipeline bathymetry.xlsx

one table per sheet, one sheet per section

| x [m] | y [m] | insulation   |
| ----- | ----- | ------------ |
| 0     | 3     | section name |
| 20    | 5     |              |
| 450   | 7.8   |              |

x is cumulative, metres, y is elevation at the start of the pipe segment

section name should only appear once, should match other spreadsheet

## pipeline insulations.xlsx

| name           | inside diameter [m] | r1 [m]  | Uwall [W/m²°C] | Ax [m2] | ho [W/m²°C] |
| -------------- | ------------------- | ------- | -------------- | ------- | ----------- |
| section name   | 0.4572              | 0.2286  | 0.2286         | 0.16417 | 4           |
| section name 2 | 0.8763              | 0.43815 | 0.1761         | 0.60311 | 4           |

half of these are unused

should also specify ambient temperature here probably
