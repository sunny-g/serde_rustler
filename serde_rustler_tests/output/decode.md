# Benchmark

Benchmark run from 2019-06-09 03:37:51.495009Z UTC

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
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">3.63 K</td>
    <td style="white-space: nowrap; text-align: right">275.38 μs</td>
    <td style="white-space: nowrap; text-align: right">±30.13%</td>
    <td style="white-space: nowrap; text-align: right">251.90 μs</td>
    <td style="white-space: nowrap; text-align: right">516.90 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">2.50 K</td>
    <td style="white-space: nowrap; text-align: right">399.55 μs</td>
    <td style="white-space: nowrap; text-align: right">±29.58%</td>
    <td style="white-space: nowrap; text-align: right">344.90 μs</td>
    <td style="white-space: nowrap; text-align: right">774.90 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">1.80 K</td>
    <td style="white-space: nowrap; text-align: right">556.56 μs</td>
    <td style="white-space: nowrap; text-align: right">±9.13%</td>
    <td style="white-space: nowrap; text-align: right">551.90 μs</td>
    <td style="white-space: nowrap; text-align: right">771.90 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">1.24 K</td>
    <td style="white-space: nowrap; text-align: right">808.16 μs</td>
    <td style="white-space: nowrap; text-align: right">±14.34%</td>
    <td style="white-space: nowrap; text-align: right">802.90 μs</td>
    <td style="white-space: nowrap; text-align: right">1257.90 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">1.05 K</td>
    <td style="white-space: nowrap; text-align: right">950.31 μs</td>
    <td style="white-space: nowrap; text-align: right">±14.46%</td>
    <td style="white-space: nowrap; text-align: right">952.90 μs</td>
    <td style="white-space: nowrap; text-align: right">1486.90 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">0.85 K</td>
    <td style="white-space: nowrap; text-align: right">1182.96 μs</td>
    <td style="white-space: nowrap; text-align: right">±15.21%</td>
    <td style="white-space: nowrap; text-align: right">1188.90 μs</td>
    <td style="white-space: nowrap; text-align: right">1724.90 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">0.78 K</td>
    <td style="white-space: nowrap; text-align: right">1288.66 μs</td>
    <td style="white-space: nowrap; text-align: right">±13.46%</td>
    <td style="white-space: nowrap; text-align: right">1267.90 μs</td>
    <td style="white-space: nowrap; text-align: right">1805.29 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">0.39 K</td>
    <td style="white-space: nowrap; text-align: right">2585.69 μs</td>
    <td style="white-space: nowrap; text-align: right">±20.68%</td>
    <td style="white-space: nowrap; text-align: right">2481.90 μs</td>
    <td style="white-space: nowrap; text-align: right">4222.20 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">0.27 K</td>
    <td style="white-space: nowrap; text-align: right">3750.74 μs</td>
    <td style="white-space: nowrap; text-align: right">±10.98%</td>
    <td style="white-space: nowrap; text-align: right">3758.90 μs</td>
    <td style="white-space: nowrap; text-align: right">4655.60 μs</td>
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
    <td style="white-space: nowrap;text-align: right">3.63 K</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">2.50 K</td>
    <td style="white-space: nowrap; text-align: right">1.45x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">1.80 K</td>
    <td style="white-space: nowrap; text-align: right">2.02x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">1.24 K</td>
    <td style="white-space: nowrap; text-align: right">2.93x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">1.05 K</td>
    <td style="white-space: nowrap; text-align: right">3.45x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">0.85 K</td>
    <td style="white-space: nowrap; text-align: right">4.3x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">0.78 K</td>
    <td style="white-space: nowrap; text-align: right">4.68x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">0.39 K</td>
    <td style="white-space: nowrap; text-align: right">9.39x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">0.27 K</td>
    <td style="white-space: nowrap; text-align: right">13.62x</td>
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
    <td style="white-space: nowrap">1.52 KB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">1.59 KB</td>
    <td>1.05x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">77.62 KB</td>
    <td>50.95x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">262.86 KB</td>
    <td>172.54x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">326.70 KB</td>
    <td>214.45x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">540.09 KB</td>
    <td>354.52x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">550.15 KB</td>
    <td>361.12x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">1490.06 KB</td>
    <td>978.09x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">1.59 KB</td>
    <td>1.05x</td>
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
    <td style="white-space: nowrap; text-align: right">373.38</td>
    <td style="white-space: nowrap; text-align: right">2.68 ms</td>
    <td style="white-space: nowrap; text-align: right">±17.85%</td>
    <td style="white-space: nowrap; text-align: right">2.54 ms</td>
    <td style="white-space: nowrap; text-align: right">4.16 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">175.77</td>
    <td style="white-space: nowrap; text-align: right">5.69 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.49%</td>
    <td style="white-space: nowrap; text-align: right">5.67 ms</td>
    <td style="white-space: nowrap; text-align: right">6.92 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">114.08</td>
    <td style="white-space: nowrap; text-align: right">8.77 ms</td>
    <td style="white-space: nowrap; text-align: right">±52.48%</td>
    <td style="white-space: nowrap; text-align: right">9.41 ms</td>
    <td style="white-space: nowrap; text-align: right">16.50 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">94.52</td>
    <td style="white-space: nowrap; text-align: right">10.58 ms</td>
    <td style="white-space: nowrap; text-align: right">±8.03%</td>
    <td style="white-space: nowrap; text-align: right">10.35 ms</td>
    <td style="white-space: nowrap; text-align: right">14.45 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">83.92</td>
    <td style="white-space: nowrap; text-align: right">11.92 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.66%</td>
    <td style="white-space: nowrap; text-align: right">11.83 ms</td>
    <td style="white-space: nowrap; text-align: right">14.56 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">71.85</td>
    <td style="white-space: nowrap; text-align: right">13.92 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.64%</td>
    <td style="white-space: nowrap; text-align: right">13.97 ms</td>
    <td style="white-space: nowrap; text-align: right">16.39 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">59.64</td>
    <td style="white-space: nowrap; text-align: right">16.77 ms</td>
    <td style="white-space: nowrap; text-align: right">±4.84%</td>
    <td style="white-space: nowrap; text-align: right">16.65 ms</td>
    <td style="white-space: nowrap; text-align: right">18.92 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">29.75</td>
    <td style="white-space: nowrap; text-align: right">33.61 ms</td>
    <td style="white-space: nowrap; text-align: right">±4.25%</td>
    <td style="white-space: nowrap; text-align: right">33.55 ms</td>
    <td style="white-space: nowrap; text-align: right">37.60 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">18.79</td>
    <td style="white-space: nowrap; text-align: right">53.23 ms</td>
    <td style="white-space: nowrap; text-align: right">±26.82%</td>
    <td style="white-space: nowrap; text-align: right">43.94 ms</td>
    <td style="white-space: nowrap; text-align: right">77.11 ms</td>
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
    <td style="white-space: nowrap;text-align: right">373.38</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">175.77</td>
    <td style="white-space: nowrap; text-align: right">2.12x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">114.08</td>
    <td style="white-space: nowrap; text-align: right">3.27x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">94.52</td>
    <td style="white-space: nowrap; text-align: right">3.95x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">83.92</td>
    <td style="white-space: nowrap; text-align: right">4.45x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">71.85</td>
    <td style="white-space: nowrap; text-align: right">5.2x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">59.64</td>
    <td style="white-space: nowrap; text-align: right">6.26x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">29.75</td>
    <td style="white-space: nowrap; text-align: right">12.55x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">18.79</td>
    <td style="white-space: nowrap; text-align: right">19.88x</td>
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
    <td style="white-space: nowrap">0.40 MB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">1.12 MB</td>
    <td>2.8x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">0.00153 MB</td>
    <td>0.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">3.43 MB</td>
    <td>8.56x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">4.36 MB</td>
    <td>10.88x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">5.27 MB</td>
    <td>13.13x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">7.00 MB</td>
    <td>17.47x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">12.81 MB</td>
    <td>31.93x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">0.00153 MB</td>
    <td>0.0x</td>
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
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">1315.82</td>
    <td style="white-space: nowrap; text-align: right">0.76 ms</td>
    <td style="white-space: nowrap; text-align: right">±19.02%</td>
    <td style="white-space: nowrap; text-align: right">0.75 ms</td>
    <td style="white-space: nowrap; text-align: right">1.29 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">960.83</td>
    <td style="white-space: nowrap; text-align: right">1.04 ms</td>
    <td style="white-space: nowrap; text-align: right">±27.68%</td>
    <td style="white-space: nowrap; text-align: right">0.88 ms</td>
    <td style="white-space: nowrap; text-align: right">1.80 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">625.62</td>
    <td style="white-space: nowrap; text-align: right">1.60 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.67%</td>
    <td style="white-space: nowrap; text-align: right">1.55 ms</td>
    <td style="white-space: nowrap; text-align: right">2.12 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">420.11</td>
    <td style="white-space: nowrap; text-align: right">2.38 ms</td>
    <td style="white-space: nowrap; text-align: right">±10.40%</td>
    <td style="white-space: nowrap; text-align: right">2.32 ms</td>
    <td style="white-space: nowrap; text-align: right">3.69 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">379.85</td>
    <td style="white-space: nowrap; text-align: right">2.63 ms</td>
    <td style="white-space: nowrap; text-align: right">±10.79%</td>
    <td style="white-space: nowrap; text-align: right">2.58 ms</td>
    <td style="white-space: nowrap; text-align: right">4.04 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">292.15</td>
    <td style="white-space: nowrap; text-align: right">3.42 ms</td>
    <td style="white-space: nowrap; text-align: right">±9.49%</td>
    <td style="white-space: nowrap; text-align: right">3.36 ms</td>
    <td style="white-space: nowrap; text-align: right">5.17 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">211.85</td>
    <td style="white-space: nowrap; text-align: right">4.72 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.14%</td>
    <td style="white-space: nowrap; text-align: right">4.67 ms</td>
    <td style="white-space: nowrap; text-align: right">6.03 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">110.72</td>
    <td style="white-space: nowrap; text-align: right">9.03 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.13%</td>
    <td style="white-space: nowrap; text-align: right">9.01 ms</td>
    <td style="white-space: nowrap; text-align: right">11.01 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">92.73</td>
    <td style="white-space: nowrap; text-align: right">10.78 ms</td>
    <td style="white-space: nowrap; text-align: right">±11.44%</td>
    <td style="white-space: nowrap; text-align: right">10.72 ms</td>
    <td style="white-space: nowrap; text-align: right">13.95 ms</td>
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
    <td style="white-space: nowrap;text-align: right">1315.82</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">960.83</td>
    <td style="white-space: nowrap; text-align: right">1.37x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">625.62</td>
    <td style="white-space: nowrap; text-align: right">2.1x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">420.11</td>
    <td style="white-space: nowrap; text-align: right">3.13x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">379.85</td>
    <td style="white-space: nowrap; text-align: right">3.46x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">292.15</td>
    <td style="white-space: nowrap; text-align: right">4.5x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">211.85</td>
    <td style="white-space: nowrap; text-align: right">6.21x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">110.72</td>
    <td style="white-space: nowrap; text-align: right">11.88x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">92.73</td>
    <td style="white-space: nowrap; text-align: right">14.19x</td>
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
    <td style="white-space: nowrap">47.79 KB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">1.59 KB</td>
    <td>0.03x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">191.90 KB</td>
    <td>4.02x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">678.27 KB</td>
    <td>14.19x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">770.84 KB</td>
    <td>16.13x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">1280 KB</td>
    <td>26.78x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">2142.23 KB</td>
    <td>44.83x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">4747.14 KB</td>
    <td>99.34x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">1.59 KB</td>
    <td>0.03x</td>
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
    <td style="white-space: nowrap; text-align: right">8.06</td>
    <td style="white-space: nowrap; text-align: right">124.06 ms</td>
    <td style="white-space: nowrap; text-align: right">±4.38%</td>
    <td style="white-space: nowrap; text-align: right">123.77 ms</td>
    <td style="white-space: nowrap; text-align: right">140.70 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">5.86</td>
    <td style="white-space: nowrap; text-align: right">170.57 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.26%</td>
    <td style="white-space: nowrap; text-align: right">170.64 ms</td>
    <td style="white-space: nowrap; text-align: right">179.73 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">3.15</td>
    <td style="white-space: nowrap; text-align: right">317.37 ms</td>
    <td style="white-space: nowrap; text-align: right">±55.21%</td>
    <td style="white-space: nowrap; text-align: right">396.39 ms</td>
    <td style="white-space: nowrap; text-align: right">537.82 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">3.04</td>
    <td style="white-space: nowrap; text-align: right">329.31 ms</td>
    <td style="white-space: nowrap; text-align: right">±3.14%</td>
    <td style="white-space: nowrap; text-align: right">330.57 ms</td>
    <td style="white-space: nowrap; text-align: right">355.27 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">2.62</td>
    <td style="white-space: nowrap; text-align: right">382.18 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.89%</td>
    <td style="white-space: nowrap; text-align: right">382.23 ms</td>
    <td style="white-space: nowrap; text-align: right">412.25 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">1.84</td>
    <td style="white-space: nowrap; text-align: right">544.00 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.07%</td>
    <td style="white-space: nowrap; text-align: right">545.58 ms</td>
    <td style="white-space: nowrap; text-align: right">561.21 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">1.51</td>
    <td style="white-space: nowrap; text-align: right">662.64 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.16%</td>
    <td style="white-space: nowrap; text-align: right">665.79 ms</td>
    <td style="white-space: nowrap; text-align: right">685.72 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">0.85</td>
    <td style="white-space: nowrap; text-align: right">1180.78 ms</td>
    <td style="white-space: nowrap; text-align: right">±28.71%</td>
    <td style="white-space: nowrap; text-align: right">1193.39 ms</td>
    <td style="white-space: nowrap; text-align: right">1769.40 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">0.54</td>
    <td style="white-space: nowrap; text-align: right">1858.16 ms</td>
    <td style="white-space: nowrap; text-align: right">±0.65%</td>
    <td style="white-space: nowrap; text-align: right">1858.24 ms</td>
    <td style="white-space: nowrap; text-align: right">1886.21 ms</td>
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
    <td style="white-space: nowrap;text-align: right">8.06</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">5.86</td>
    <td style="white-space: nowrap; text-align: right">1.37x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">3.15</td>
    <td style="white-space: nowrap; text-align: right">2.56x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">3.04</td>
    <td style="white-space: nowrap; text-align: right">2.65x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">2.62</td>
    <td style="white-space: nowrap; text-align: right">3.08x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">1.84</td>
    <td style="white-space: nowrap; text-align: right">4.38x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">1.51</td>
    <td style="white-space: nowrap; text-align: right">5.34x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">0.85</td>
    <td style="white-space: nowrap; text-align: right">9.52x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">0.54</td>
    <td style="white-space: nowrap; text-align: right">14.98x</td>
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
    <td style="white-space: nowrap">25.41 MB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">18.04 MB</td>
    <td>0.71x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">0.00155 MB</td>
    <td>0.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">60.35 MB</td>
    <td>2.38x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">71.56 MB</td>
    <td>2.82x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">117.53 MB</td>
    <td>4.63x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">148.97 MB</td>
    <td>5.86x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">0.00155 MB</td>
    <td>0.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">347.70 MB</td>
    <td>13.68x</td>
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
    <td style="white-space: nowrap; text-align: right">54.37</td>
    <td style="white-space: nowrap; text-align: right">18.39 ms</td>
    <td style="white-space: nowrap; text-align: right">±127.15%</td>
    <td style="white-space: nowrap; text-align: right">15.95 ms</td>
    <td style="white-space: nowrap; text-align: right">24.12 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">44.39</td>
    <td style="white-space: nowrap; text-align: right">22.53 ms</td>
    <td style="white-space: nowrap; text-align: right">±5.00%</td>
    <td style="white-space: nowrap; text-align: right">22.53 ms</td>
    <td style="white-space: nowrap; text-align: right">25.25 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">37.01</td>
    <td style="white-space: nowrap; text-align: right">27.02 ms</td>
    <td style="white-space: nowrap; text-align: right">±4.76%</td>
    <td style="white-space: nowrap; text-align: right">26.82 ms</td>
    <td style="white-space: nowrap; text-align: right">31.29 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">5.86</td>
    <td style="white-space: nowrap; text-align: right">170.76 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.86%</td>
    <td style="white-space: nowrap; text-align: right">169.89 ms</td>
    <td style="white-space: nowrap; text-align: right">181.48 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">5.52</td>
    <td style="white-space: nowrap; text-align: right">181.16 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.71%</td>
    <td style="white-space: nowrap; text-align: right">180.96 ms</td>
    <td style="white-space: nowrap; text-align: right">194.10 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">5.24</td>
    <td style="white-space: nowrap; text-align: right">190.67 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.89%</td>
    <td style="white-space: nowrap; text-align: right">190.68 ms</td>
    <td style="white-space: nowrap; text-align: right">205.84 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">4.91</td>
    <td style="white-space: nowrap; text-align: right">203.76 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.86%</td>
    <td style="white-space: nowrap; text-align: right">204.18 ms</td>
    <td style="white-space: nowrap; text-align: right">217.49 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">4.37</td>
    <td style="white-space: nowrap; text-align: right">228.83 ms</td>
    <td style="white-space: nowrap; text-align: right">±3.21%</td>
    <td style="white-space: nowrap; text-align: right">229.38 ms</td>
    <td style="white-space: nowrap; text-align: right">246.86 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">0.34</td>
    <td style="white-space: nowrap; text-align: right">2912.91 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.58%</td>
    <td style="white-space: nowrap; text-align: right">2905.44 ms</td>
    <td style="white-space: nowrap; text-align: right">2969.06 ms</td>
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
    <td style="white-space: nowrap;text-align: right">54.37</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">44.39</td>
    <td style="white-space: nowrap; text-align: right">1.22x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">37.01</td>
    <td style="white-space: nowrap; text-align: right">1.47x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">5.86</td>
    <td style="white-space: nowrap; text-align: right">9.28x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">5.52</td>
    <td style="white-space: nowrap; text-align: right">9.85x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">5.24</td>
    <td style="white-space: nowrap; text-align: right">10.37x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">4.91</td>
    <td style="white-space: nowrap; text-align: right">11.08x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">4.37</td>
    <td style="white-space: nowrap; text-align: right">12.44x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">0.34</td>
    <td style="white-space: nowrap; text-align: right">158.38x</td>
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
    <td style="white-space: nowrap">0.0116 MB</td>
    <td>7.61x</td>
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
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">332.38</td>
    <td style="white-space: nowrap; text-align: right">3.01 ms</td>
    <td style="white-space: nowrap; text-align: right">±27.81%</td>
    <td style="white-space: nowrap; text-align: right">2.96 ms</td>
    <td style="white-space: nowrap; text-align: right">4.87 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">230.16</td>
    <td style="white-space: nowrap; text-align: right">4.34 ms</td>
    <td style="white-space: nowrap; text-align: right">±7.62%</td>
    <td style="white-space: nowrap; text-align: right">4.37 ms</td>
    <td style="white-space: nowrap; text-align: right">5.21 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">199.06</td>
    <td style="white-space: nowrap; text-align: right">5.02 ms</td>
    <td style="white-space: nowrap; text-align: right">±38.07%</td>
    <td style="white-space: nowrap; text-align: right">4.52 ms</td>
    <td style="white-space: nowrap; text-align: right">10.61 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">138.25</td>
    <td style="white-space: nowrap; text-align: right">7.23 ms</td>
    <td style="white-space: nowrap; text-align: right">±8.29%</td>
    <td style="white-space: nowrap; text-align: right">7.01 ms</td>
    <td style="white-space: nowrap; text-align: right">9.65 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">119.06</td>
    <td style="white-space: nowrap; text-align: right">8.40 ms</td>
    <td style="white-space: nowrap; text-align: right">±7.20%</td>
    <td style="white-space: nowrap; text-align: right">8.23 ms</td>
    <td style="white-space: nowrap; text-align: right">10.85 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">90.19</td>
    <td style="white-space: nowrap; text-align: right">11.09 ms</td>
    <td style="white-space: nowrap; text-align: right">±9.19%</td>
    <td style="white-space: nowrap; text-align: right">10.77 ms</td>
    <td style="white-space: nowrap; text-align: right">14.78 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">81.83</td>
    <td style="white-space: nowrap; text-align: right">12.22 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.44%</td>
    <td style="white-space: nowrap; text-align: right">12.07 ms</td>
    <td style="white-space: nowrap; text-align: right">14.49 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">30.43</td>
    <td style="white-space: nowrap; text-align: right">32.86 ms</td>
    <td style="white-space: nowrap; text-align: right">±3.82%</td>
    <td style="white-space: nowrap; text-align: right">32.80 ms</td>
    <td style="white-space: nowrap; text-align: right">36.08 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">24.71</td>
    <td style="white-space: nowrap; text-align: right">40.47 ms</td>
    <td style="white-space: nowrap; text-align: right">±26.86%</td>
    <td style="white-space: nowrap; text-align: right">33.97 ms</td>
    <td style="white-space: nowrap; text-align: right">63.25 ms</td>
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
    <td style="white-space: nowrap;text-align: right">332.38</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">230.16</td>
    <td style="white-space: nowrap; text-align: right">1.44x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">199.06</td>
    <td style="white-space: nowrap; text-align: right">1.67x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">138.25</td>
    <td style="white-space: nowrap; text-align: right">2.4x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">119.06</td>
    <td style="white-space: nowrap; text-align: right">2.79x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">90.19</td>
    <td style="white-space: nowrap; text-align: right">3.69x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">81.83</td>
    <td style="white-space: nowrap; text-align: right">4.06x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">30.43</td>
    <td style="white-space: nowrap; text-align: right">10.92x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">24.71</td>
    <td style="white-space: nowrap; text-align: right">13.45x</td>
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
    <td style="white-space: nowrap">0.31 MB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">0.73 MB</td>
    <td>2.38x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">0.00155 MB</td>
    <td>0.01x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">2.41 MB</td>
    <td>7.87x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">3.05 MB</td>
    <td>10.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">4.77 MB</td>
    <td>15.61x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">5.17 MB</td>
    <td>16.93x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">13.59 MB</td>
    <td>44.49x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">0.00155 MB</td>
    <td>0.01x</td>
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
    <td style="white-space: nowrap; text-align: right">280.09</td>
    <td style="white-space: nowrap; text-align: right">3.57 ms</td>
    <td style="white-space: nowrap; text-align: right">±24.47%</td>
    <td style="white-space: nowrap; text-align: right">3.30 ms</td>
    <td style="white-space: nowrap; text-align: right">5.88 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">188.34</td>
    <td style="white-space: nowrap; text-align: right">5.31 ms</td>
    <td style="white-space: nowrap; text-align: right">±38.54%</td>
    <td style="white-space: nowrap; text-align: right">4.79 ms</td>
    <td style="white-space: nowrap; text-align: right">11.15 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">187.93</td>
    <td style="white-space: nowrap; text-align: right">5.32 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.26%</td>
    <td style="white-space: nowrap; text-align: right">5.36 ms</td>
    <td style="white-space: nowrap; text-align: right">6.36 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">115.66</td>
    <td style="white-space: nowrap; text-align: right">8.65 ms</td>
    <td style="white-space: nowrap; text-align: right">±8.74%</td>
    <td style="white-space: nowrap; text-align: right">8.31 ms</td>
    <td style="white-space: nowrap; text-align: right">10.90 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">108.72</td>
    <td style="white-space: nowrap; text-align: right">9.20 ms</td>
    <td style="white-space: nowrap; text-align: right">±5.42%</td>
    <td style="white-space: nowrap; text-align: right">9.16 ms</td>
    <td style="white-space: nowrap; text-align: right">10.93 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">80.56</td>
    <td style="white-space: nowrap; text-align: right">12.41 ms</td>
    <td style="white-space: nowrap; text-align: right">±7.33%</td>
    <td style="white-space: nowrap; text-align: right">12.17 ms</td>
    <td style="white-space: nowrap; text-align: right">15.60 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">64.14</td>
    <td style="white-space: nowrap; text-align: right">15.59 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.88%</td>
    <td style="white-space: nowrap; text-align: right">15.60 ms</td>
    <td style="white-space: nowrap; text-align: right">19.22 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">29.98</td>
    <td style="white-space: nowrap; text-align: right">33.36 ms</td>
    <td style="white-space: nowrap; text-align: right">±4.43%</td>
    <td style="white-space: nowrap; text-align: right">33.20 ms</td>
    <td style="white-space: nowrap; text-align: right">36.99 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">25.48</td>
    <td style="white-space: nowrap; text-align: right">39.24 ms</td>
    <td style="white-space: nowrap; text-align: right">±24.92%</td>
    <td style="white-space: nowrap; text-align: right">33.65 ms</td>
    <td style="white-space: nowrap; text-align: right">58.80 ms</td>
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
    <td style="white-space: nowrap;text-align: right">280.09</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">188.34</td>
    <td style="white-space: nowrap; text-align: right">1.49x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">187.93</td>
    <td style="white-space: nowrap; text-align: right">1.49x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">115.66</td>
    <td style="white-space: nowrap; text-align: right">2.42x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">108.72</td>
    <td style="white-space: nowrap; text-align: right">2.58x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">80.56</td>
    <td style="white-space: nowrap; text-align: right">3.48x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">64.14</td>
    <td style="white-space: nowrap; text-align: right">4.37x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">29.98</td>
    <td style="white-space: nowrap; text-align: right">9.34x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">25.48</td>
    <td style="white-space: nowrap; text-align: right">10.99x</td>
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
    <td style="white-space: nowrap">0.55 MB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">0.00154 MB</td>
    <td>0.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">0.73 MB</td>
    <td>1.32x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">2.40 MB</td>
    <td>4.36x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">3.05 MB</td>
    <td>5.53x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">5.17 MB</td>
    <td>9.37x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">6.07 MB</td>
    <td>11.01x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">13.57 MB</td>
    <td>24.61x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">0.00154 MB</td>
    <td>0.0x</td>
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
    <td style="white-space: nowrap; text-align: right">409.52</td>
    <td style="white-space: nowrap; text-align: right">2.44 ms</td>
    <td style="white-space: nowrap; text-align: right">±8.42%</td>
    <td style="white-space: nowrap; text-align: right">2.38 ms</td>
    <td style="white-space: nowrap; text-align: right">3.13 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">381.32</td>
    <td style="white-space: nowrap; text-align: right">2.62 ms</td>
    <td style="white-space: nowrap; text-align: right">±24.18%</td>
    <td style="white-space: nowrap; text-align: right">2.37 ms</td>
    <td style="white-space: nowrap; text-align: right">4.19 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">182.16</td>
    <td style="white-space: nowrap; text-align: right">5.49 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.71%</td>
    <td style="white-space: nowrap; text-align: right">5.37 ms</td>
    <td style="white-space: nowrap; text-align: right">6.94 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">179.35</td>
    <td style="white-space: nowrap; text-align: right">5.58 ms</td>
    <td style="white-space: nowrap; text-align: right">±69.02%</td>
    <td style="white-space: nowrap; text-align: right">3.28 ms</td>
    <td style="white-space: nowrap; text-align: right">12.61 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">145.71</td>
    <td style="white-space: nowrap; text-align: right">6.86 ms</td>
    <td style="white-space: nowrap; text-align: right">±8.51%</td>
    <td style="white-space: nowrap; text-align: right">6.70 ms</td>
    <td style="white-space: nowrap; text-align: right">8.79 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">124.74</td>
    <td style="white-space: nowrap; text-align: right">8.02 ms</td>
    <td style="white-space: nowrap; text-align: right">±8.58%</td>
    <td style="white-space: nowrap; text-align: right">7.86 ms</td>
    <td style="white-space: nowrap; text-align: right">10.14 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">97.87</td>
    <td style="white-space: nowrap; text-align: right">10.22 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.10%</td>
    <td style="white-space: nowrap; text-align: right">10.16 ms</td>
    <td style="white-space: nowrap; text-align: right">12.21 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">44.76</td>
    <td style="white-space: nowrap; text-align: right">22.34 ms</td>
    <td style="white-space: nowrap; text-align: right">±4.48%</td>
    <td style="white-space: nowrap; text-align: right">22.19 ms</td>
    <td style="white-space: nowrap; text-align: right">25.66 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">29.91</td>
    <td style="white-space: nowrap; text-align: right">33.44 ms</td>
    <td style="white-space: nowrap; text-align: right">±24.06%</td>
    <td style="white-space: nowrap; text-align: right">28.76 ms</td>
    <td style="white-space: nowrap; text-align: right">49.09 ms</td>
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
    <td style="white-space: nowrap;text-align: right">409.52</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">381.32</td>
    <td style="white-space: nowrap; text-align: right">1.07x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">182.16</td>
    <td style="white-space: nowrap; text-align: right">2.25x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">179.35</td>
    <td style="white-space: nowrap; text-align: right">2.28x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">145.71</td>
    <td style="white-space: nowrap; text-align: right">2.81x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">124.74</td>
    <td style="white-space: nowrap; text-align: right">3.28x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">97.87</td>
    <td style="white-space: nowrap; text-align: right">4.18x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">44.76</td>
    <td style="white-space: nowrap; text-align: right">9.15x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">29.91</td>
    <td style="white-space: nowrap; text-align: right">13.69x</td>
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
    <td style="white-space: nowrap">0.108 MB</td>
    <td>0.18x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">2.00 MB</td>
    <td>3.39x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">0.00153 MB</td>
    <td>0.0x</td>
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
    <td style="white-space: nowrap; text-align: right">10590.01</td>
    <td style="white-space: nowrap; text-align: right">0.0944 ms</td>
    <td style="white-space: nowrap; text-align: right">±13.50%</td>
    <td style="white-space: nowrap; text-align: right">0.0919 ms</td>
    <td style="white-space: nowrap; text-align: right">0.155 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">5238.49</td>
    <td style="white-space: nowrap; text-align: right">0.191 ms</td>
    <td style="white-space: nowrap; text-align: right">±50.17%</td>
    <td style="white-space: nowrap; text-align: right">0.186 ms</td>
    <td style="white-space: nowrap; text-align: right">0.31 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">4523.71</td>
    <td style="white-space: nowrap; text-align: right">0.22 ms</td>
    <td style="white-space: nowrap; text-align: right">±24.23%</td>
    <td style="white-space: nowrap; text-align: right">0.188 ms</td>
    <td style="white-space: nowrap; text-align: right">0.32 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">601.23</td>
    <td style="white-space: nowrap; text-align: right">1.66 ms</td>
    <td style="white-space: nowrap; text-align: right">±17.18%</td>
    <td style="white-space: nowrap; text-align: right">1.63 ms</td>
    <td style="white-space: nowrap; text-align: right">2.39 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">566.54</td>
    <td style="white-space: nowrap; text-align: right">1.77 ms</td>
    <td style="white-space: nowrap; text-align: right">±16.81%</td>
    <td style="white-space: nowrap; text-align: right">1.63 ms</td>
    <td style="white-space: nowrap; text-align: right">2.78 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">492.66</td>
    <td style="white-space: nowrap; text-align: right">2.03 ms</td>
    <td style="white-space: nowrap; text-align: right">±16.03%</td>
    <td style="white-space: nowrap; text-align: right">1.96 ms</td>
    <td style="white-space: nowrap; text-align: right">3.70 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">454.94</td>
    <td style="white-space: nowrap; text-align: right">2.20 ms</td>
    <td style="white-space: nowrap; text-align: right">±13.90%</td>
    <td style="white-space: nowrap; text-align: right">2.08 ms</td>
    <td style="white-space: nowrap; text-align: right">3.20 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">266.58</td>
    <td style="white-space: nowrap; text-align: right">3.75 ms</td>
    <td style="white-space: nowrap; text-align: right">±9.88%</td>
    <td style="white-space: nowrap; text-align: right">3.60 ms</td>
    <td style="white-space: nowrap; text-align: right">5.10 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">175.02</td>
    <td style="white-space: nowrap; text-align: right">5.71 ms</td>
    <td style="white-space: nowrap; text-align: right">±9.07%</td>
    <td style="white-space: nowrap; text-align: right">5.59 ms</td>
    <td style="white-space: nowrap; text-align: right">7.36 ms</td>
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
    <td style="white-space: nowrap;text-align: right">10590.01</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">5238.49</td>
    <td style="white-space: nowrap; text-align: right">2.02x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">4523.71</td>
    <td style="white-space: nowrap; text-align: right">2.34x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">601.23</td>
    <td style="white-space: nowrap; text-align: right">17.61x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">566.54</td>
    <td style="white-space: nowrap; text-align: right">18.69x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">492.66</td>
    <td style="white-space: nowrap; text-align: right">21.5x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">454.94</td>
    <td style="white-space: nowrap; text-align: right">23.28x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">266.58</td>
    <td style="white-space: nowrap; text-align: right">39.73x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">175.02</td>
    <td style="white-space: nowrap; text-align: right">60.51x</td>
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
    <td style="white-space: nowrap; text-align: right">21.88 K</td>
    <td style="white-space: nowrap; text-align: right">45.70 μs</td>
    <td style="white-space: nowrap; text-align: right">±20.43%</td>
    <td style="white-space: nowrap; text-align: right">43.90 μs</td>
    <td style="white-space: nowrap; text-align: right">74.90 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">16.17 K</td>
    <td style="white-space: nowrap; text-align: right">61.84 μs</td>
    <td style="white-space: nowrap; text-align: right">±40.44%</td>
    <td style="white-space: nowrap; text-align: right">56.90 μs</td>
    <td style="white-space: nowrap; text-align: right">102.90 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">12.50 K</td>
    <td style="white-space: nowrap; text-align: right">80.01 μs</td>
    <td style="white-space: nowrap; text-align: right">±53.59%</td>
    <td style="white-space: nowrap; text-align: right">76.90 μs</td>
    <td style="white-space: nowrap; text-align: right">132.90 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">3.98 K</td>
    <td style="white-space: nowrap; text-align: right">251.32 μs</td>
    <td style="white-space: nowrap; text-align: right">±15.87%</td>
    <td style="white-space: nowrap; text-align: right">234.90 μs</td>
    <td style="white-space: nowrap; text-align: right">391.90 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">2.81 K</td>
    <td style="white-space: nowrap; text-align: right">356.46 μs</td>
    <td style="white-space: nowrap; text-align: right">±14.57%</td>
    <td style="white-space: nowrap; text-align: right">345.90 μs</td>
    <td style="white-space: nowrap; text-align: right">593.90 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">2.53 K</td>
    <td style="white-space: nowrap; text-align: right">395.48 μs</td>
    <td style="white-space: nowrap; text-align: right">±14.44%</td>
    <td style="white-space: nowrap; text-align: right">393.90 μs</td>
    <td style="white-space: nowrap; text-align: right">692.90 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">2.49 K</td>
    <td style="white-space: nowrap; text-align: right">401.72 μs</td>
    <td style="white-space: nowrap; text-align: right">±13.79%</td>
    <td style="white-space: nowrap; text-align: right">390.90 μs</td>
    <td style="white-space: nowrap; text-align: right">642.90 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">1.58 K</td>
    <td style="white-space: nowrap; text-align: right">631.39 μs</td>
    <td style="white-space: nowrap; text-align: right">±14.86%</td>
    <td style="white-space: nowrap; text-align: right">614.90 μs</td>
    <td style="white-space: nowrap; text-align: right">1090.90 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">1.08 K</td>
    <td style="white-space: nowrap; text-align: right">925.65 μs</td>
    <td style="white-space: nowrap; text-align: right">±25.58%</td>
    <td style="white-space: nowrap; text-align: right">877.90 μs</td>
    <td style="white-space: nowrap; text-align: right">1577.90 μs</td>
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
    <td style="white-space: nowrap;text-align: right">21.88 K</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">16.17 K</td>
    <td style="white-space: nowrap; text-align: right">1.35x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">12.50 K</td>
    <td style="white-space: nowrap; text-align: right">1.75x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">3.98 K</td>
    <td style="white-space: nowrap; text-align: right">5.5x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">2.81 K</td>
    <td style="white-space: nowrap; text-align: right">7.8x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">2.53 K</td>
    <td style="white-space: nowrap; text-align: right">8.65x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">2.49 K</td>
    <td style="white-space: nowrap; text-align: right">8.79x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">1.58 K</td>
    <td style="white-space: nowrap; text-align: right">13.81x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">1.08 K</td>
    <td style="white-space: nowrap; text-align: right">20.25x</td>
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
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">43.42 KB</td>
    <td>926.33x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">92.83 KB</td>
    <td>1980.33x</td>
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

