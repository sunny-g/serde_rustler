# Benchmark

Benchmark run from 2019-06-09 02:10:02.047383Z UTC

## System

Benchmark suite executing on the following system:

<table style="width: 1%">
  <tr>
    <th style="width: 1%; white-space: nowrap">Operating System</th>
    <td>macOS</td>
  </tr><tr>
    <th style="white-space: nowrap">CPU Information</th>
    <td style="white-space: nowrap">Intel(R) Core(TM) i7-6820HQ CPU @ 2.70GHz</td>
  </tr><tr>
    <th style="white-space: nowrap">Number of Available Cores</th>
    <td style="white-space: nowrap">8</td>
  </tr><tr>
    <th style="white-space: nowrap">Available Memory</th>
    <td style="white-space: nowrap">16 GB</td>
  </tr><tr>
    <th style="white-space: nowrap">Elixir Version</th>
    <td style="white-space: nowrap">1.8.2</td>
  </tr><tr>
    <th style="white-space: nowrap">Erlang Version</th>
    <td style="white-space: nowrap">21.3.8.2</td>
  </tr>
</table>

## Configuration

Benchmark suite executing with the following configuration:

<table style="width: 1%">
  <tr>
    <th style="width: 1%">:time</th>
    <td style="white-space: nowrap">5 s</td>
  </tr><tr>
    <th>:parallel</th>
    <td style="white-space: nowrap">4</td>
  </tr><tr>
    <th>:warmup</th>
    <td style="white-space: nowrap">2 s</td>
  </tr>
</table>

## Statistics



__Input: Blockchain__

Run Time
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">2.53 K</td>
    <td style="white-space: nowrap; text-align: right">395.85 μs</td>
    <td style="white-space: nowrap; text-align: right">±32.64%</td>
    <td style="white-space: nowrap; text-align: right">335 μs</td>
    <td style="white-space: nowrap; text-align: right">823 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">2.50 K</td>
    <td style="white-space: nowrap; text-align: right">400.64 μs</td>
    <td style="white-space: nowrap; text-align: right">±22.58%</td>
    <td style="white-space: nowrap; text-align: right">376 μs</td>
    <td style="white-space: nowrap; text-align: right">702 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">1.78 K</td>
    <td style="white-space: nowrap; text-align: right">563.12 μs</td>
    <td style="white-space: nowrap; text-align: right">±10.34%</td>
    <td style="white-space: nowrap; text-align: right">555 μs</td>
    <td style="white-space: nowrap; text-align: right">792 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">1.13 K</td>
    <td style="white-space: nowrap; text-align: right">882.23 μs</td>
    <td style="white-space: nowrap; text-align: right">±16.54%</td>
    <td style="white-space: nowrap; text-align: right">857 μs</td>
    <td style="white-space: nowrap; text-align: right">1407 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">1.03 K</td>
    <td style="white-space: nowrap; text-align: right">971.35 μs</td>
    <td style="white-space: nowrap; text-align: right">±15.38%</td>
    <td style="white-space: nowrap; text-align: right">960 μs</td>
    <td style="white-space: nowrap; text-align: right">1581.59 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">0.81 K</td>
    <td style="white-space: nowrap; text-align: right">1228.93 μs</td>
    <td style="white-space: nowrap; text-align: right">±16.43%</td>
    <td style="white-space: nowrap; text-align: right">1201 μs</td>
    <td style="white-space: nowrap; text-align: right">1972 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">0.77 K</td>
    <td style="white-space: nowrap; text-align: right">1304.18 μs</td>
    <td style="white-space: nowrap; text-align: right">±15.10%</td>
    <td style="white-space: nowrap; text-align: right">1270 μs</td>
    <td style="white-space: nowrap; text-align: right">1947 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">0.40 K</td>
    <td style="white-space: nowrap; text-align: right">2490.59 μs</td>
    <td style="white-space: nowrap; text-align: right">±19.10%</td>
    <td style="white-space: nowrap; text-align: right">2373 μs</td>
    <td style="white-space: nowrap; text-align: right">3870.04 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">0.26 K</td>
    <td style="white-space: nowrap; text-align: right">3808.63 μs</td>
    <td style="white-space: nowrap; text-align: right">±11.53%</td>
    <td style="white-space: nowrap; text-align: right">3812 μs</td>
    <td style="white-space: nowrap; text-align: right">4829.50 μs</td>
  </tr>
</table>

Comparsion
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap;text-align: right">2.53 K</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">2.50 K</td>
    <td style="white-space: nowrap; text-align: right">1.01x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">1.78 K</td>
    <td style="white-space: nowrap; text-align: right">1.42x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">1.13 K</td>
    <td style="white-space: nowrap; text-align: right">2.23x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">1.03 K</td>
    <td style="white-space: nowrap; text-align: right">2.45x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">0.81 K</td>
    <td style="white-space: nowrap; text-align: right">3.1x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">0.77 K</td>
    <td style="white-space: nowrap; text-align: right">3.29x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">0.40 K</td>
    <td style="white-space: nowrap; text-align: right">6.29x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">0.26 K</td>
    <td style="white-space: nowrap; text-align: right">9.62x</td>
  </tr>
</table>

Memory Usage
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
      <th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">1.59 KB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap">1.52 KB</td>
    <td>0.96x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">77.62 KB</td>
    <td>48.7x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">262.86 KB</td>
    <td>164.93x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">326.70 KB</td>
    <td>204.99x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">540.09 KB</td>
    <td>338.88x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">550.15 KB</td>
    <td>345.19x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">1490.06 KB</td>
    <td>934.94x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">1.59 KB</td>
    <td>1.0x</td>
  </tr>
</table>

<hr/>

__Input: Giphy__

Run Time
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">184.87</td>
    <td style="white-space: nowrap; text-align: right">5.41 ms</td>
    <td style="white-space: nowrap; text-align: right">±4.95%</td>
    <td style="white-space: nowrap; text-align: right">5.37 ms</td>
    <td style="white-space: nowrap; text-align: right">6.30 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">171.29</td>
    <td style="white-space: nowrap; text-align: right">5.84 ms</td>
    <td style="white-space: nowrap; text-align: right">±9.30%</td>
    <td style="white-space: nowrap; text-align: right">5.79 ms</td>
    <td style="white-space: nowrap; text-align: right">7.65 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">120.58</td>
    <td style="white-space: nowrap; text-align: right">8.29 ms</td>
    <td style="white-space: nowrap; text-align: right">±52.41%</td>
    <td style="white-space: nowrap; text-align: right">9.07 ms</td>
    <td style="white-space: nowrap; text-align: right">15.79 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">91.25</td>
    <td style="white-space: nowrap; text-align: right">10.96 ms</td>
    <td style="white-space: nowrap; text-align: right">±7.58%</td>
    <td style="white-space: nowrap; text-align: right">10.76 ms</td>
    <td style="white-space: nowrap; text-align: right">13.81 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">86.43</td>
    <td style="white-space: nowrap; text-align: right">11.57 ms</td>
    <td style="white-space: nowrap; text-align: right">±7.18%</td>
    <td style="white-space: nowrap; text-align: right">11.38 ms</td>
    <td style="white-space: nowrap; text-align: right">14.82 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">71.94</td>
    <td style="white-space: nowrap; text-align: right">13.90 ms</td>
    <td style="white-space: nowrap; text-align: right">±7.26%</td>
    <td style="white-space: nowrap; text-align: right">13.63 ms</td>
    <td style="white-space: nowrap; text-align: right">17.47 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">62.23</td>
    <td style="white-space: nowrap; text-align: right">16.07 ms</td>
    <td style="white-space: nowrap; text-align: right">±5.93%</td>
    <td style="white-space: nowrap; text-align: right">15.90 ms</td>
    <td style="white-space: nowrap; text-align: right">19.83 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">28.04</td>
    <td style="white-space: nowrap; text-align: right">35.67 ms</td>
    <td style="white-space: nowrap; text-align: right">±8.44%</td>
    <td style="white-space: nowrap; text-align: right">34.77 ms</td>
    <td style="white-space: nowrap; text-align: right">44.11 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">18.76</td>
    <td style="white-space: nowrap; text-align: right">53.30 ms</td>
    <td style="white-space: nowrap; text-align: right">±29.20%</td>
    <td style="white-space: nowrap; text-align: right">43.80 ms</td>
    <td style="white-space: nowrap; text-align: right">74.88 ms</td>
  </tr>
</table>

Comparsion
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap;text-align: right">184.87</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">171.29</td>
    <td style="white-space: nowrap; text-align: right">1.08x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">120.58</td>
    <td style="white-space: nowrap; text-align: right">1.53x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">91.25</td>
    <td style="white-space: nowrap; text-align: right">2.03x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">86.43</td>
    <td style="white-space: nowrap; text-align: right">2.14x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">71.94</td>
    <td style="white-space: nowrap; text-align: right">2.57x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">62.23</td>
    <td style="white-space: nowrap; text-align: right">2.97x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">28.04</td>
    <td style="white-space: nowrap; text-align: right">6.59x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">18.76</td>
    <td style="white-space: nowrap; text-align: right">9.85x</td>
  </tr>
</table>

Memory Usage
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
      <th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap">0.00150 MB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">1.12 MB</td>
    <td>752.05x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">0.00153 MB</td>
    <td>1.02x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">3.43 MB</td>
    <td>2295.4x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">4.36 MB</td>
    <td>2917.98x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">5.27 MB</td>
    <td>3521.14x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">7.00 MB</td>
    <td>4683.93x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">12.81 MB</td>
    <td>8564.05x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">0.00153 MB</td>
    <td>1.02x</td>
  </tr>
</table>

<hr/>

__Input: GitHub__

Run Time
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">956.06</td>
    <td style="white-space: nowrap; text-align: right">1.05 ms</td>
    <td style="white-space: nowrap; text-align: right">±32.63%</td>
    <td style="white-space: nowrap; text-align: right">0.93 ms</td>
    <td style="white-space: nowrap; text-align: right">1.94 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">777.65</td>
    <td style="white-space: nowrap; text-align: right">1.29 ms</td>
    <td style="white-space: nowrap; text-align: right">±52.08%</td>
    <td style="white-space: nowrap; text-align: right">1.01 ms</td>
    <td style="white-space: nowrap; text-align: right">3.24 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">618.77</td>
    <td style="white-space: nowrap; text-align: right">1.62 ms</td>
    <td style="white-space: nowrap; text-align: right">±7.76%</td>
    <td style="white-space: nowrap; text-align: right">1.57 ms</td>
    <td style="white-space: nowrap; text-align: right">2.14 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">413.89</td>
    <td style="white-space: nowrap; text-align: right">2.42 ms</td>
    <td style="white-space: nowrap; text-align: right">±11.60%</td>
    <td style="white-space: nowrap; text-align: right">2.33 ms</td>
    <td style="white-space: nowrap; text-align: right">3.75 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">378.48</td>
    <td style="white-space: nowrap; text-align: right">2.64 ms</td>
    <td style="white-space: nowrap; text-align: right">±11.26%</td>
    <td style="white-space: nowrap; text-align: right">2.57 ms</td>
    <td style="white-space: nowrap; text-align: right">4.09 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">289.16</td>
    <td style="white-space: nowrap; text-align: right">3.46 ms</td>
    <td style="white-space: nowrap; text-align: right">±9.23%</td>
    <td style="white-space: nowrap; text-align: right">3.39 ms</td>
    <td style="white-space: nowrap; text-align: right">4.85 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">208.62</td>
    <td style="white-space: nowrap; text-align: right">4.79 ms</td>
    <td style="white-space: nowrap; text-align: right">±7.92%</td>
    <td style="white-space: nowrap; text-align: right">4.69 ms</td>
    <td style="white-space: nowrap; text-align: right">6.44 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">104.16</td>
    <td style="white-space: nowrap; text-align: right">9.60 ms</td>
    <td style="white-space: nowrap; text-align: right">±7.56%</td>
    <td style="white-space: nowrap; text-align: right">9.50 ms</td>
    <td style="white-space: nowrap; text-align: right">12.24 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">87.96</td>
    <td style="white-space: nowrap; text-align: right">11.37 ms</td>
    <td style="white-space: nowrap; text-align: right">±10.01%</td>
    <td style="white-space: nowrap; text-align: right">11.36 ms</td>
    <td style="white-space: nowrap; text-align: right">14.02 ms</td>
  </tr>
</table>

Comparsion
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap;text-align: right">956.06</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">777.65</td>
    <td style="white-space: nowrap; text-align: right">1.23x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">618.77</td>
    <td style="white-space: nowrap; text-align: right">1.55x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">413.89</td>
    <td style="white-space: nowrap; text-align: right">2.31x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">378.48</td>
    <td style="white-space: nowrap; text-align: right">2.53x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">289.16</td>
    <td style="white-space: nowrap; text-align: right">3.31x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">208.62</td>
    <td style="white-space: nowrap; text-align: right">4.58x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">104.16</td>
    <td style="white-space: nowrap; text-align: right">9.18x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">87.96</td>
    <td style="white-space: nowrap; text-align: right">10.87x</td>
  </tr>
</table>

Memory Usage
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
      <th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">1.59 KB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap">1.54 KB</td>
    <td>0.97x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">191.90 KB</td>
    <td>121.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">678.27 KB</td>
    <td>427.68x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">770.84 KB</td>
    <td>486.05x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">1280 KB</td>
    <td>807.09x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">2142.23 KB</td>
    <td>1350.77x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">4747.14 KB</td>
    <td>2993.27x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">1.59 KB</td>
    <td>1.0x</td>
  </tr>
</table>

<hr/>

__Input: GovTrack__

Run Time
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">6.53</td>
    <td style="white-space: nowrap; text-align: right">153.03 ms</td>
    <td style="white-space: nowrap; text-align: right">±4.79%</td>
    <td style="white-space: nowrap; text-align: right">152.84 ms</td>
    <td style="white-space: nowrap; text-align: right">175.60 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">5.73</td>
    <td style="white-space: nowrap; text-align: right">174.41 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.43%</td>
    <td style="white-space: nowrap; text-align: right">174.45 ms</td>
    <td style="white-space: nowrap; text-align: right">180.32 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">3.35</td>
    <td style="white-space: nowrap; text-align: right">298.29 ms</td>
    <td style="white-space: nowrap; text-align: right">±56.18%</td>
    <td style="white-space: nowrap; text-align: right">383.27 ms</td>
    <td style="white-space: nowrap; text-align: right">534.15 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">2.99</td>
    <td style="white-space: nowrap; text-align: right">333.99 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.53%</td>
    <td style="white-space: nowrap; text-align: right">333.01 ms</td>
    <td style="white-space: nowrap; text-align: right">364.94 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">2.55</td>
    <td style="white-space: nowrap; text-align: right">391.42 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.47%</td>
    <td style="white-space: nowrap; text-align: right">392.12 ms</td>
    <td style="white-space: nowrap; text-align: right">422.77 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">1.86</td>
    <td style="white-space: nowrap; text-align: right">537.71 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.11%</td>
    <td style="white-space: nowrap; text-align: right">538.97 ms</td>
    <td style="white-space: nowrap; text-align: right">555.91 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">1.50</td>
    <td style="white-space: nowrap; text-align: right">665.42 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.08%</td>
    <td style="white-space: nowrap; text-align: right">668.61 ms</td>
    <td style="white-space: nowrap; text-align: right">687.54 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">0.81</td>
    <td style="white-space: nowrap; text-align: right">1230.88 ms</td>
    <td style="white-space: nowrap; text-align: right">±26.96%</td>
    <td style="white-space: nowrap; text-align: right">1250.70 ms</td>
    <td style="white-space: nowrap; text-align: right">1822.25 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">0.51</td>
    <td style="white-space: nowrap; text-align: right">1962.91 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.87%</td>
    <td style="white-space: nowrap; text-align: right">1965.70 ms</td>
    <td style="white-space: nowrap; text-align: right">1984.73 ms</td>
  </tr>
</table>

Comparsion
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap;text-align: right">6.53</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">5.73</td>
    <td style="white-space: nowrap; text-align: right">1.14x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">3.35</td>
    <td style="white-space: nowrap; text-align: right">1.95x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">2.99</td>
    <td style="white-space: nowrap; text-align: right">2.18x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">2.55</td>
    <td style="white-space: nowrap; text-align: right">2.56x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">1.86</td>
    <td style="white-space: nowrap; text-align: right">3.51x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">1.50</td>
    <td style="white-space: nowrap; text-align: right">4.35x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">0.81</td>
    <td style="white-space: nowrap; text-align: right">8.04x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">0.51</td>
    <td style="white-space: nowrap; text-align: right">12.83x</td>
  </tr>
</table>

Memory Usage
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
      <th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap">18.93 MB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">18.04 MB</td>
    <td>0.95x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">0.00155 MB</td>
    <td>0.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">60.35 MB</td>
    <td>3.19x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">71.56 MB</td>
    <td>3.78x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">117.53 MB</td>
    <td>6.21x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">148.97 MB</td>
    <td>7.87x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">0.00155 MB</td>
    <td>0.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">347.70 MB</td>
    <td>18.36x</td>
  </tr>
</table>

<hr/>

__Input: Issue 90__

Run Time
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">54.27</td>
    <td style="white-space: nowrap; text-align: right">18.43 ms</td>
    <td style="white-space: nowrap; text-align: right">±133.49%</td>
    <td style="white-space: nowrap; text-align: right">16.43 ms</td>
    <td style="white-space: nowrap; text-align: right">21.45 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">41.97</td>
    <td style="white-space: nowrap; text-align: right">23.83 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.52%</td>
    <td style="white-space: nowrap; text-align: right">24.09 ms</td>
    <td style="white-space: nowrap; text-align: right">27.27 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">30.79</td>
    <td style="white-space: nowrap; text-align: right">32.48 ms</td>
    <td style="white-space: nowrap; text-align: right">±91.51%</td>
    <td style="white-space: nowrap; text-align: right">28.85 ms</td>
    <td style="white-space: nowrap; text-align: right">299.09 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">5.73</td>
    <td style="white-space: nowrap; text-align: right">174.63 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.34%</td>
    <td style="white-space: nowrap; text-align: right">174.33 ms</td>
    <td style="white-space: nowrap; text-align: right">188.83 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">5.41</td>
    <td style="white-space: nowrap; text-align: right">184.89 ms</td>
    <td style="white-space: nowrap; text-align: right">±4.07%</td>
    <td style="white-space: nowrap; text-align: right">183.03 ms</td>
    <td style="white-space: nowrap; text-align: right">211.96 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">5.11</td>
    <td style="white-space: nowrap; text-align: right">195.77 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.95%</td>
    <td style="white-space: nowrap; text-align: right">195.60 ms</td>
    <td style="white-space: nowrap; text-align: right">221.76 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">4.99</td>
    <td style="white-space: nowrap; text-align: right">200.48 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.54%</td>
    <td style="white-space: nowrap; text-align: right">199.63 ms</td>
    <td style="white-space: nowrap; text-align: right">211.24 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">4.27</td>
    <td style="white-space: nowrap; text-align: right">234.10 ms</td>
    <td style="white-space: nowrap; text-align: right">±3.17%</td>
    <td style="white-space: nowrap; text-align: right">232.95 ms</td>
    <td style="white-space: nowrap; text-align: right">255.71 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">0.35</td>
    <td style="white-space: nowrap; text-align: right">2857.08 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.45%</td>
    <td style="white-space: nowrap; text-align: right">2859.86 ms</td>
    <td style="white-space: nowrap; text-align: right">2874.94 ms</td>
  </tr>
</table>

Comparsion
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap;text-align: right">54.27</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">41.97</td>
    <td style="white-space: nowrap; text-align: right">1.29x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">30.79</td>
    <td style="white-space: nowrap; text-align: right">1.76x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">5.73</td>
    <td style="white-space: nowrap; text-align: right">9.48x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">5.41</td>
    <td style="white-space: nowrap; text-align: right">10.03x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">5.11</td>
    <td style="white-space: nowrap; text-align: right">10.62x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">4.99</td>
    <td style="white-space: nowrap; text-align: right">10.88x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">4.27</td>
    <td style="white-space: nowrap; text-align: right">12.7x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">0.35</td>
    <td style="white-space: nowrap; text-align: right">155.05x</td>
  </tr>
</table>

Memory Usage
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
      <th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">0.00153 MB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">0.00153 MB</td>
    <td>1.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap">0.00150 MB</td>
    <td>0.98x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">2.34 MB</td>
    <td>1535.26x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">8.74 MB</td>
    <td>5726.61x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">5.76 MB</td>
    <td>3774.05x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">11.25 MB</td>
    <td>7375.44x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">4.89 MB</td>
    <td>3205.97x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">602.89 MB</td>
    <td>395106.92x</td>
  </tr>
</table>

<hr/>

__Input: JSON Generator__

Run Time
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">228.77</td>
    <td style="white-space: nowrap; text-align: right">4.37 ms</td>
    <td style="white-space: nowrap; text-align: right">±8.24%</td>
    <td style="white-space: nowrap; text-align: right">4.35 ms</td>
    <td style="white-space: nowrap; text-align: right">5.56 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">199.83</td>
    <td style="white-space: nowrap; text-align: right">5.00 ms</td>
    <td style="white-space: nowrap; text-align: right">±38.89%</td>
    <td style="white-space: nowrap; text-align: right">4.49 ms</td>
    <td style="white-space: nowrap; text-align: right">10.69 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">182.37</td>
    <td style="white-space: nowrap; text-align: right">5.48 ms</td>
    <td style="white-space: nowrap; text-align: right">±8.70%</td>
    <td style="white-space: nowrap; text-align: right">5.51 ms</td>
    <td style="white-space: nowrap; text-align: right">6.59 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">131.79</td>
    <td style="white-space: nowrap; text-align: right">7.59 ms</td>
    <td style="white-space: nowrap; text-align: right">±9.96%</td>
    <td style="white-space: nowrap; text-align: right">7.32 ms</td>
    <td style="white-space: nowrap; text-align: right">10.38 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">116.51</td>
    <td style="white-space: nowrap; text-align: right">8.58 ms</td>
    <td style="white-space: nowrap; text-align: right">±9.29%</td>
    <td style="white-space: nowrap; text-align: right">8.36 ms</td>
    <td style="white-space: nowrap; text-align: right">12.00 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">89.78</td>
    <td style="white-space: nowrap; text-align: right">11.14 ms</td>
    <td style="white-space: nowrap; text-align: right">±7.57%</td>
    <td style="white-space: nowrap; text-align: right">10.89 ms</td>
    <td style="white-space: nowrap; text-align: right">13.62 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">79.32</td>
    <td style="white-space: nowrap; text-align: right">12.61 ms</td>
    <td style="white-space: nowrap; text-align: right">±12.77%</td>
    <td style="white-space: nowrap; text-align: right">12.11 ms</td>
    <td style="white-space: nowrap; text-align: right">18.08 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">29.88</td>
    <td style="white-space: nowrap; text-align: right">33.46 ms</td>
    <td style="white-space: nowrap; text-align: right">±4.42%</td>
    <td style="white-space: nowrap; text-align: right">33.25 ms</td>
    <td style="white-space: nowrap; text-align: right">37.81 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">24.39</td>
    <td style="white-space: nowrap; text-align: right">41.01 ms</td>
    <td style="white-space: nowrap; text-align: right">±27.43%</td>
    <td style="white-space: nowrap; text-align: right">34.73 ms</td>
    <td style="white-space: nowrap; text-align: right">66.75 ms</td>
  </tr>
</table>

Comparsion
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap;text-align: right">228.77</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">199.83</td>
    <td style="white-space: nowrap; text-align: right">1.14x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">182.37</td>
    <td style="white-space: nowrap; text-align: right">1.25x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">131.79</td>
    <td style="white-space: nowrap; text-align: right">1.74x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">116.51</td>
    <td style="white-space: nowrap; text-align: right">1.96x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">89.78</td>
    <td style="white-space: nowrap; text-align: right">2.55x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">79.32</td>
    <td style="white-space: nowrap; text-align: right">2.88x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">29.88</td>
    <td style="white-space: nowrap; text-align: right">7.66x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">24.39</td>
    <td style="white-space: nowrap; text-align: right">9.38x</td>
  </tr>
</table>

Memory Usage
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
      <th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">0.73 MB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">0.00155 MB</td>
    <td>0.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap">0.00150 MB</td>
    <td>0.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">2.41 MB</td>
    <td>3.3x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">3.05 MB</td>
    <td>4.19x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">4.77 MB</td>
    <td>6.55x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">5.17 MB</td>
    <td>7.11x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">13.59 MB</td>
    <td>18.67x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">0.00155 MB</td>
    <td>0.0x</td>
  </tr>
</table>

<hr/>

__Input: JSON Generator (Pretty)__

Run Time
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">211.35</td>
    <td style="white-space: nowrap; text-align: right">4.73 ms</td>
    <td style="white-space: nowrap; text-align: right">±26.97%</td>
    <td style="white-space: nowrap; text-align: right">4.23 ms</td>
    <td style="white-space: nowrap; text-align: right">8.41 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">192.31</td>
    <td style="white-space: nowrap; text-align: right">5.20 ms</td>
    <td style="white-space: nowrap; text-align: right">±39.14%</td>
    <td style="white-space: nowrap; text-align: right">4.66 ms</td>
    <td style="white-space: nowrap; text-align: right">11.29 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">180.83</td>
    <td style="white-space: nowrap; text-align: right">5.53 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.81%</td>
    <td style="white-space: nowrap; text-align: right">5.53 ms</td>
    <td style="white-space: nowrap; text-align: right">6.71 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">118.75</td>
    <td style="white-space: nowrap; text-align: right">8.42 ms</td>
    <td style="white-space: nowrap; text-align: right">±7.71%</td>
    <td style="white-space: nowrap; text-align: right">8.21 ms</td>
    <td style="white-space: nowrap; text-align: right">11.49 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">109.08</td>
    <td style="white-space: nowrap; text-align: right">9.17 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.27%</td>
    <td style="white-space: nowrap; text-align: right">9.04 ms</td>
    <td style="white-space: nowrap; text-align: right">11.37 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">77.84</td>
    <td style="white-space: nowrap; text-align: right">12.85 ms</td>
    <td style="white-space: nowrap; text-align: right">±10.16%</td>
    <td style="white-space: nowrap; text-align: right">12.54 ms</td>
    <td style="white-space: nowrap; text-align: right">17.98 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">69.04</td>
    <td style="white-space: nowrap; text-align: right">14.49 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.30%</td>
    <td style="white-space: nowrap; text-align: right">14.22 ms</td>
    <td style="white-space: nowrap; text-align: right">17.54 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">30.51</td>
    <td style="white-space: nowrap; text-align: right">32.78 ms</td>
    <td style="white-space: nowrap; text-align: right">±3.76%</td>
    <td style="white-space: nowrap; text-align: right">32.67 ms</td>
    <td style="white-space: nowrap; text-align: right">36.72 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">25.35</td>
    <td style="white-space: nowrap; text-align: right">39.44 ms</td>
    <td style="white-space: nowrap; text-align: right">±26.52%</td>
    <td style="white-space: nowrap; text-align: right">32.91 ms</td>
    <td style="white-space: nowrap; text-align: right">60.15 ms</td>
  </tr>
</table>

Comparsion
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap;text-align: right">211.35</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">192.31</td>
    <td style="white-space: nowrap; text-align: right">1.1x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">180.83</td>
    <td style="white-space: nowrap; text-align: right">1.17x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">118.75</td>
    <td style="white-space: nowrap; text-align: right">1.78x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">109.08</td>
    <td style="white-space: nowrap; text-align: right">1.94x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">77.84</td>
    <td style="white-space: nowrap; text-align: right">2.72x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">69.04</td>
    <td style="white-space: nowrap; text-align: right">3.06x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">30.51</td>
    <td style="white-space: nowrap; text-align: right">6.93x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">25.35</td>
    <td style="white-space: nowrap; text-align: right">8.34x</td>
  </tr>
</table>

Memory Usage
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
      <th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap">0.24 MB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">0.00154 MB</td>
    <td>0.01x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">0.73 MB</td>
    <td>3.09x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">2.40 MB</td>
    <td>10.18x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">3.05 MB</td>
    <td>12.93x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">5.17 MB</td>
    <td>21.89x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">6.07 MB</td>
    <td>25.73x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">13.57 MB</td>
    <td>57.49x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">0.00154 MB</td>
    <td>0.01x</td>
  </tr>
</table>

<hr/>

__Input: Pokedex__

Run Time
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">395.81</td>
    <td style="white-space: nowrap; text-align: right">2.53 ms</td>
    <td style="white-space: nowrap; text-align: right">±9.19%</td>
    <td style="white-space: nowrap; text-align: right">2.44 ms</td>
    <td style="white-space: nowrap; text-align: right">3.43 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">313.88</td>
    <td style="white-space: nowrap; text-align: right">3.19 ms</td>
    <td style="white-space: nowrap; text-align: right">±33.71%</td>
    <td style="white-space: nowrap; text-align: right">2.97 ms</td>
    <td style="white-space: nowrap; text-align: right">5.76 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">181.38</td>
    <td style="white-space: nowrap; text-align: right">5.51 ms</td>
    <td style="white-space: nowrap; text-align: right">±69.18%</td>
    <td style="white-space: nowrap; text-align: right">3.26 ms</td>
    <td style="white-space: nowrap; text-align: right">12.84 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">179.34</td>
    <td style="white-space: nowrap; text-align: right">5.58 ms</td>
    <td style="white-space: nowrap; text-align: right">±8.69%</td>
    <td style="white-space: nowrap; text-align: right">5.39 ms</td>
    <td style="white-space: nowrap; text-align: right">7.79 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">141.52</td>
    <td style="white-space: nowrap; text-align: right">7.07 ms</td>
    <td style="white-space: nowrap; text-align: right">±8.10%</td>
    <td style="white-space: nowrap; text-align: right">6.95 ms</td>
    <td style="white-space: nowrap; text-align: right">9.20 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">123.33</td>
    <td style="white-space: nowrap; text-align: right">8.11 ms</td>
    <td style="white-space: nowrap; text-align: right">±8.18%</td>
    <td style="white-space: nowrap; text-align: right">7.85 ms</td>
    <td style="white-space: nowrap; text-align: right">10.53 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">99.72</td>
    <td style="white-space: nowrap; text-align: right">10.03 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.82%</td>
    <td style="white-space: nowrap; text-align: right">9.87 ms</td>
    <td style="white-space: nowrap; text-align: right">12.63 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">44.68</td>
    <td style="white-space: nowrap; text-align: right">22.38 ms</td>
    <td style="white-space: nowrap; text-align: right">±3.87%</td>
    <td style="white-space: nowrap; text-align: right">22.21 ms</td>
    <td style="white-space: nowrap; text-align: right">25.00 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">30.16</td>
    <td style="white-space: nowrap; text-align: right">33.16 ms</td>
    <td style="white-space: nowrap; text-align: right">±23.89%</td>
    <td style="white-space: nowrap; text-align: right">28.86 ms</td>
    <td style="white-space: nowrap; text-align: right">50.03 ms</td>
  </tr>
</table>

Comparsion
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap;text-align: right">395.81</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">313.88</td>
    <td style="white-space: nowrap; text-align: right">1.26x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">181.38</td>
    <td style="white-space: nowrap; text-align: right">2.18x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">179.34</td>
    <td style="white-space: nowrap; text-align: right">2.21x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">141.52</td>
    <td style="white-space: nowrap; text-align: right">2.8x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">123.33</td>
    <td style="white-space: nowrap; text-align: right">3.21x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">99.72</td>
    <td style="white-space: nowrap; text-align: right">3.97x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">44.68</td>
    <td style="white-space: nowrap; text-align: right">8.86x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">30.16</td>
    <td style="white-space: nowrap; text-align: right">13.13x</td>
  </tr>
</table>

Memory Usage
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
      <th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">0.59 MB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap">0.00148 MB</td>
    <td>0.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">0.00153 MB</td>
    <td>0.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">2.00 MB</td>
    <td>3.39x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">2.48 MB</td>
    <td>4.19x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">3.76 MB</td>
    <td>6.35x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">4.34 MB</td>
    <td>7.34x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">8.27 MB</td>
    <td>13.98x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">0.00153 MB</td>
    <td>0.0x</td>
  </tr>
</table>

<hr/>

__Input: UTF-8 escaped__

Run Time
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">7548.40</td>
    <td style="white-space: nowrap; text-align: right">0.132 ms</td>
    <td style="white-space: nowrap; text-align: right">±15.32%</td>
    <td style="white-space: nowrap; text-align: right">0.126 ms</td>
    <td style="white-space: nowrap; text-align: right">0.21 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">5204.15</td>
    <td style="white-space: nowrap; text-align: right">0.192 ms</td>
    <td style="white-space: nowrap; text-align: right">±15.12%</td>
    <td style="white-space: nowrap; text-align: right">0.184 ms</td>
    <td style="white-space: nowrap; text-align: right">0.32 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">4522.33</td>
    <td style="white-space: nowrap; text-align: right">0.22 ms</td>
    <td style="white-space: nowrap; text-align: right">±24.18%</td>
    <td style="white-space: nowrap; text-align: right">0.190 ms</td>
    <td style="white-space: nowrap; text-align: right">0.33 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">641.07</td>
    <td style="white-space: nowrap; text-align: right">1.56 ms</td>
    <td style="white-space: nowrap; text-align: right">±14.17%</td>
    <td style="white-space: nowrap; text-align: right">1.50 ms</td>
    <td style="white-space: nowrap; text-align: right">2.30 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">540.68</td>
    <td style="white-space: nowrap; text-align: right">1.85 ms</td>
    <td style="white-space: nowrap; text-align: right">±17.37%</td>
    <td style="white-space: nowrap; text-align: right">1.69 ms</td>
    <td style="white-space: nowrap; text-align: right">2.94 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">483.12</td>
    <td style="white-space: nowrap; text-align: right">2.07 ms</td>
    <td style="white-space: nowrap; text-align: right">±12.80%</td>
    <td style="white-space: nowrap; text-align: right">1.99 ms</td>
    <td style="white-space: nowrap; text-align: right">3.23 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">444.18</td>
    <td style="white-space: nowrap; text-align: right">2.25 ms</td>
    <td style="white-space: nowrap; text-align: right">±15.06%</td>
    <td style="white-space: nowrap; text-align: right">2.10 ms</td>
    <td style="white-space: nowrap; text-align: right">3.37 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">270.56</td>
    <td style="white-space: nowrap; text-align: right">3.70 ms</td>
    <td style="white-space: nowrap; text-align: right">±16.98%</td>
    <td style="white-space: nowrap; text-align: right">3.48 ms</td>
    <td style="white-space: nowrap; text-align: right">5.96 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">177.02</td>
    <td style="white-space: nowrap; text-align: right">5.65 ms</td>
    <td style="white-space: nowrap; text-align: right">±10.23%</td>
    <td style="white-space: nowrap; text-align: right">5.46 ms</td>
    <td style="white-space: nowrap; text-align: right">8.07 ms</td>
  </tr>
</table>

Comparsion
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap;text-align: right">7548.40</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">5204.15</td>
    <td style="white-space: nowrap; text-align: right">1.45x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">4522.33</td>
    <td style="white-space: nowrap; text-align: right">1.67x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">641.07</td>
    <td style="white-space: nowrap; text-align: right">11.77x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">540.68</td>
    <td style="white-space: nowrap; text-align: right">13.96x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">483.12</td>
    <td style="white-space: nowrap; text-align: right">15.62x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">444.18</td>
    <td style="white-space: nowrap; text-align: right">16.99x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">270.56</td>
    <td style="white-space: nowrap; text-align: right">27.9x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">177.02</td>
    <td style="white-space: nowrap; text-align: right">42.64x</td>
  </tr>
</table>

Memory Usage
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
      <th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap">0.00007 MB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">0.00005 MB</td>
    <td>0.67x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">0.00005 MB</td>
    <td>0.67x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">0.43 MB</td>
    <td>6249.11x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">1.01 MB</td>
    <td>14731.78x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">1.13 MB</td>
    <td>16409.11x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">1.14 MB</td>
    <td>16555.56x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">1.73 MB</td>
    <td>25156.89x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">3.19 MB</td>
    <td>46424.33x</td>
  </tr>
</table>

<hr/>

__Input: UTF-8 unescaped__

Run Time
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Average</th>
    <th style="text-align: right">Devitation</th>
    <th style="text-align: right">Median</th>
    <th style="text-align: right">99th&nbsp;%</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">21.50 K</td>
    <td style="white-space: nowrap; text-align: right">46.51 μs</td>
    <td style="white-space: nowrap; text-align: right">±23.34%</td>
    <td style="white-space: nowrap; text-align: right">44 μs</td>
    <td style="white-space: nowrap; text-align: right">77 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">16.46 K</td>
    <td style="white-space: nowrap; text-align: right">60.75 μs</td>
    <td style="white-space: nowrap; text-align: right">±36.82%</td>
    <td style="white-space: nowrap; text-align: right">57 μs</td>
    <td style="white-space: nowrap; text-align: right">101 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">10.93 K</td>
    <td style="white-space: nowrap; text-align: right">91.45 μs</td>
    <td style="white-space: nowrap; text-align: right">±22.84%</td>
    <td style="white-space: nowrap; text-align: right">81 μs</td>
    <td style="white-space: nowrap; text-align: right">153 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">3.84 K</td>
    <td style="white-space: nowrap; text-align: right">260.57 μs</td>
    <td style="white-space: nowrap; text-align: right">±22.51%</td>
    <td style="white-space: nowrap; text-align: right">235 μs</td>
    <td style="white-space: nowrap; text-align: right">467 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">2.77 K</td>
    <td style="white-space: nowrap; text-align: right">360.94 μs</td>
    <td style="white-space: nowrap; text-align: right">±16.39%</td>
    <td style="white-space: nowrap; text-align: right">347 μs</td>
    <td style="white-space: nowrap; text-align: right">610 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">2.44 K</td>
    <td style="white-space: nowrap; text-align: right">410.29 μs</td>
    <td style="white-space: nowrap; text-align: right">±15.96%</td>
    <td style="white-space: nowrap; text-align: right">394 μs</td>
    <td style="white-space: nowrap; text-align: right">670 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">2.43 K</td>
    <td style="white-space: nowrap; text-align: right">412.18 μs</td>
    <td style="white-space: nowrap; text-align: right">±17.75%</td>
    <td style="white-space: nowrap; text-align: right">398 μs</td>
    <td style="white-space: nowrap; text-align: right">745 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">1.57 K</td>
    <td style="white-space: nowrap; text-align: right">638.21 μs</td>
    <td style="white-space: nowrap; text-align: right">±21.27%</td>
    <td style="white-space: nowrap; text-align: right">611 μs</td>
    <td style="white-space: nowrap; text-align: right">1187 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">1.14 K</td>
    <td style="white-space: nowrap; text-align: right">876.18 μs</td>
    <td style="white-space: nowrap; text-align: right">±22.44%</td>
    <td style="white-space: nowrap; text-align: right">850 μs</td>
    <td style="white-space: nowrap; text-align: right">1509.30 μs</td>
  </tr>
</table>

Comparsion
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap;text-align: right">21.50 K</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">16.46 K</td>
    <td style="white-space: nowrap; text-align: right">1.31x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">10.93 K</td>
    <td style="white-space: nowrap; text-align: right">1.97x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">3.84 K</td>
    <td style="white-space: nowrap; text-align: right">5.6x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">2.77 K</td>
    <td style="white-space: nowrap; text-align: right">7.76x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">2.44 K</td>
    <td style="white-space: nowrap; text-align: right">8.82x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">2.43 K</td>
    <td style="white-space: nowrap; text-align: right">8.86x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">1.57 K</td>
    <td style="white-space: nowrap; text-align: right">13.72x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">1.14 K</td>
    <td style="white-space: nowrap; text-align: right">18.84x</td>
  </tr>
</table>

Memory Usage
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">Memory</th>
      <th style="text-align: right">Factor</th>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">0.0469 KB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">0.0469 KB</td>
    <td>1.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap">0.0703 KB</td>
    <td>1.5x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">15.39 KB</td>
    <td>328.33x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">37.04 KB</td>
    <td>790.17x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">92.83 KB</td>
    <td>1980.33x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">43.42 KB</td>
    <td>926.33x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">244.49 KB</td>
    <td>5215.83x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">488.69 KB</td>
    <td>10425.33x</td>
  </tr>
</table>

<hr/>

