# Vested DXD calculator

`vested-dxd-calculator` is an utility program that DXdao contributors can use to calculate how much DXD they're owed for their services.

## Installation

### macOS and Linux

For macOS and Linux, installing should be as easy as opening a terminal and running:

```
curl -L https://raw.githubusercontent.com/luzzif/vested-dxd-calculator/main/install.sh | sh
```

The installer will ask you for your account's password in order to be able to move the binary to `/usr/local/bin`. If you don't trust the installation script with sudo access, you can simply download the correct binary for your platform (remember to pick the `arm64` version for Mac if you have the M1 chip) from the latest release and manually put it into `/usr/local/bin`, renaming it to `vested-dxd-calculator` and making it executable (`sudo chmod +x /usr/local/bin/vested-dxd-calculator`). After doing so, you should see the command being picked up by whatever shell you like to use.

### Windows

Windows installation instructions will be added in the future.

## Usage

`vested-dxd-calculator` takes 4 parameters as input:

- `--from`: the date from which the selected period starts from (dd-mm-yyyy format).
- `--to`: the date in which the selected period ends (dd-mm-yyyy format)
- `--level`: the level the worker was/is at in the specified period.
- `--full-time-percentage`: the percentage of time the worker was active in the specified period (1 to 100, supporting decimal values).
- `--trial`: pass this flag if the specified period represented a trial (50% of the full payment).

Additionally, you can run `vested-dxd-calculator --help` to check out all the possible options while using the binary.

## How it works

The binary treats pre and post 2022 periods differently due to changes in the salary structure.

Generally speaking, a period cannot start in 2021 and finish in 2022 due to these changes, and in order to perform calculations on the salary, the period cannot be longer than a month.

The compensation tables used for the calculations are the following:

Pre-2022:

| Level | USD equivalent value of DXD |
| ----- | --------------------------- |
| 1     | 2000                        |
| 2     | 3000                        |
| 3     | 4000                        |
| 4     | 5000                        |
| 5     | 6000                        |

Post-2022:

| Level | USD equivalent value of DXD |
| ----- | --------------------------- |
| 1     | 1500                        |
| 2     | 2000                        |
| 3     | 3000                        |
| 4     | 4000                        |
| 5     | 5000                        |
| 6     | 6000                        |
| 7     | 7500                        |
| 8     | 9500                        |

When specifying a period, the program calculates the amount of working days in the month starting from the specified `from` date and confronts it with the amount of working days in the specified period, in order to calculate a multiplier that will be used to determine the actual USD salary value in the specified period.

### Example:

_All date are expressed in dd-mm-yyyy format_

- `from`: `01-10-2020`
- `to`: `10-10-2020`
- `level`: `3`
- `full-time-percentage`: `100`
- `trial`: `false`

Working days in the full month from `01-10-2020` to `01-11-2020` as calculated by the binary: `22`.
Working days in the period as calculated by the binary: `7`.
The USD value of the salary given out in the period, at level `3` will then be calculated as to be `4000 * 7 / 22 = ~1272.7273`.

Now, depending on the other parameters, the salary can be further affected. If the `--trial` flag is active, the salary will be half of what is would originally have been, while depending on the `--full-time-percentage` value the salary can be further reduced.

After determining the USD value of the salary, the amount of DXD is determined following 2 separate logics depending on the given period of time:

- Pre-2022, the DXD ATH vs USD in the given period is simply determined and DXD given out depending on that value.
- Post-2022, the DXD value in USD at the `--from` date is used and DXD given out depending on that value.

In both cases, the Covalent API is used to fetch pricing data (Coingecko being the ultimate price source).

**_Being that Coingecko is used for price data, no price prior to 20-05-2020 is available and the binary will fail in determining the DXD amount to be given_**
