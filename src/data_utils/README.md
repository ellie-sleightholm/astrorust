# AstroRust IO Files
This documentation provides information about the IO files within AstroRust. This document will continue to be updated with any changes.

The IO file contains all functions associated with external files. For example, retrieving files from websites (IERS, NASA, etc.) for data that is needed in our calculations. A list of the functions is described below:

* `download_tai_utc_data()` : this downloads the [tai-utc.dat](https://maia.usno.navy.mil/ser7/tai-utc.dat) file that is needed to compute leap seconds in UTC conversions.
* `exists()` : this checks whether a file exists within AstroRust. This is needed when downloading data as we do not want to be unnecessarily downloading data we already have stored locally. 
* `get_tai_utc_data(update_file)` : this checks whether we have tai-utc.dat stored locally and if `update_file` is set to true, it will override any existing file with that name. If it is not stored locally, it will download from [here](https://maia.usno.navy.mil/ser7/tai-utc.dat). This function is called in `number_of_leap_seconds_from`.

