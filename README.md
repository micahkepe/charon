# charon

An implementation of the [The One Billion Row
Challenge](https://github.com/gunnarmorling/1brc):

> Your mission, should you decide to accept it, is deceptively simple: write a
> ~Java~ Rust program for retrieving temperature measurement values from a text
> file and calculating the min, mean, and max temperature per weather station.
> There’s just one caveat: the file has 1,000,000,000 rows!
>
> The text file has a simple structure with one measurement value per row:
>
> ```
> Hamburg;12.0
> Bulawayo;8.9
> Palembang;38.8
> St. John's;15.2
> Cracow;12.6
> ...
> ```
>
> The program should print out the min, mean, and max values per station,
> alphabetically ordered like so:
>
> ```
> {Abha=5.0/18.0/27.4, Abidjan=15.7/26.0/34.1, Abéché=12.1/29.4/35.6, Accra=14.7/26.4/33.1, Addis Ababa=2.1/16.0/24.3, Adelaide=4.1/17.3/29.7, ...}
> ```

## Populating CSV Data

Run the modified `create_measurements.py` script to create the 1B row input file
at `data/measurements.txt`:

```bash
./create_measurements.py 1_000_000_000
```

This took ~9 minutes on my machine:

```
Actual file size:  14.8 GiB
Elapsed time: 8 minutes 58 seconds
Test data build complete.
```
