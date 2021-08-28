# R-Meter

### A Rust-based command line tool for analyzing API metrics

---

## Usage

- Single API GET call
    - ``r-meter.exe -u "<valid_uri>"``
    - ``r-meter.exe --uri "<valid_uri"``
- Multiple API GET calls
    - ``r-meter.exe -u "<valid_uri>" -m -c <count_of_calls>``
    - ``r-meter.exe -uri "<valid_uri>" --multi --count <count_of_calls>``

---

## TODOs

- :heavy_check_mark: Basic timer for single API call
- :heavy_check_mark: Basic timer for multiple API call
- :heavy_check_mark: Mean, Max, Min times for multiple API calls
- :heavy_check_mark: Support for POST, PUT, DELETE calls
- :white_square_button: Report generation
- :white_square_button: Graphical UI
