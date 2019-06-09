# Benchmark

Benchmark run from 2019-06-09 03:23:55.412427Z UTC

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
    <td style="white-space: nowrap; text-align: right">7905.01</td>
    <td style="white-space: nowrap; text-align: right">0.127 ms</td>
    <td style="white-space: nowrap; text-align: right">±55.67%</td>
    <td style="white-space: nowrap; text-align: right">0.116 ms</td>
    <td style="white-space: nowrap; text-align: right">0.35 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">6229.03</td>
    <td style="white-space: nowrap; text-align: right">0.161 ms</td>
    <td style="white-space: nowrap; text-align: right">±63.33%</td>
    <td style="white-space: nowrap; text-align: right">0.122 ms</td>
    <td style="white-space: nowrap; text-align: right">0.46 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">5726.77</td>
    <td style="white-space: nowrap; text-align: right">0.175 ms</td>
    <td style="white-space: nowrap; text-align: right">±284.40%</td>
    <td style="white-space: nowrap; text-align: right">0.128 ms</td>
    <td style="white-space: nowrap; text-align: right">0.26 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">1244.29</td>
    <td style="white-space: nowrap; text-align: right">0.80 ms</td>
    <td style="white-space: nowrap; text-align: right">±195.88%</td>
    <td style="white-space: nowrap; text-align: right">0.40 ms</td>
    <td style="white-space: nowrap; text-align: right">9.87 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">670.49</td>
    <td style="white-space: nowrap; text-align: right">1.49 ms</td>
    <td style="white-space: nowrap; text-align: right">±144.80%</td>
    <td style="white-space: nowrap; text-align: right">0.70 ms</td>
    <td style="white-space: nowrap; text-align: right">10.83 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">617.99</td>
    <td style="white-space: nowrap; text-align: right">1.62 ms</td>
    <td style="white-space: nowrap; text-align: right">±158.52%</td>
    <td style="white-space: nowrap; text-align: right">0.62 ms</td>
    <td style="white-space: nowrap; text-align: right">11.57 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">523.30</td>
    <td style="white-space: nowrap; text-align: right">1.91 ms</td>
    <td style="white-space: nowrap; text-align: right">±123.18%</td>
    <td style="white-space: nowrap; text-align: right">1.00 ms</td>
    <td style="white-space: nowrap; text-align: right">10.83 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">407.02</td>
    <td style="white-space: nowrap; text-align: right">2.46 ms</td>
    <td style="white-space: nowrap; text-align: right">±114.44%</td>
    <td style="white-space: nowrap; text-align: right">1.25 ms</td>
    <td style="white-space: nowrap; text-align: right">12.14 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">364.01</td>
    <td style="white-space: nowrap; text-align: right">2.75 ms</td>
    <td style="white-space: nowrap; text-align: right">±102.48%</td>
    <td style="white-space: nowrap; text-align: right">1.44 ms</td>
    <td style="white-space: nowrap; text-align: right">12.38 ms</td>
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
    <td style="white-space: nowrap;text-align: right">7905.01</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">6229.03</td>
    <td style="white-space: nowrap; text-align: right">1.27x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">5726.77</td>
    <td style="white-space: nowrap; text-align: right">1.38x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">1244.29</td>
    <td style="white-space: nowrap; text-align: right">6.35x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">670.49</td>
    <td style="white-space: nowrap; text-align: right">11.79x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">617.99</td>
    <td style="white-space: nowrap; text-align: right">12.79x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">523.30</td>
    <td style="white-space: nowrap; text-align: right">15.11x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">407.02</td>
    <td style="white-space: nowrap; text-align: right">19.42x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">364.01</td>
    <td style="white-space: nowrap; text-align: right">21.72x</td>
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
    <td style="white-space: nowrap">8.07 KB</td>
    <td>172.17x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">102.84 KB</td>
    <td>2193.83x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">192.36 KB</td>
    <td>4103.67x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">232.30 KB</td>
    <td>4955.83x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">456.70 KB</td>
    <td>9742.83x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">507.60 KB</td>
    <td>10828.83x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">572.09 KB</td>
    <td>12204.5x</td>
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
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">855.94</td>
    <td style="white-space: nowrap; text-align: right">1.17 ms</td>
    <td style="white-space: nowrap; text-align: right">±137.09%</td>
    <td style="white-space: nowrap; text-align: right">1.06 ms</td>
    <td style="white-space: nowrap; text-align: right">1.62 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">697.18</td>
    <td style="white-space: nowrap; text-align: right">1.43 ms</td>
    <td style="white-space: nowrap; text-align: right">±29.09%</td>
    <td style="white-space: nowrap; text-align: right">1.39 ms</td>
    <td style="white-space: nowrap; text-align: right">2.54 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">592.43</td>
    <td style="white-space: nowrap; text-align: right">1.69 ms</td>
    <td style="white-space: nowrap; text-align: right">±86.66%</td>
    <td style="white-space: nowrap; text-align: right">1.29 ms</td>
    <td style="white-space: nowrap; text-align: right">7.57 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">126.86</td>
    <td style="white-space: nowrap; text-align: right">7.88 ms</td>
    <td style="white-space: nowrap; text-align: right">±50.31%</td>
    <td style="white-space: nowrap; text-align: right">5.63 ms</td>
    <td style="white-space: nowrap; text-align: right">17.80 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">70.34</td>
    <td style="white-space: nowrap; text-align: right">14.22 ms</td>
    <td style="white-space: nowrap; text-align: right">±21.43%</td>
    <td style="white-space: nowrap; text-align: right">13.44 ms</td>
    <td style="white-space: nowrap; text-align: right">20.85 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">65.88</td>
    <td style="white-space: nowrap; text-align: right">15.18 ms</td>
    <td style="white-space: nowrap; text-align: right">±16.34%</td>
    <td style="white-space: nowrap; text-align: right">14.32 ms</td>
    <td style="white-space: nowrap; text-align: right">21.68 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">47.60</td>
    <td style="white-space: nowrap; text-align: right">21.01 ms</td>
    <td style="white-space: nowrap; text-align: right">±33.43%</td>
    <td style="white-space: nowrap; text-align: right">22.48 ms</td>
    <td style="white-space: nowrap; text-align: right">34.00 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">42.48</td>
    <td style="white-space: nowrap; text-align: right">23.54 ms</td>
    <td style="white-space: nowrap; text-align: right">±19.54%</td>
    <td style="white-space: nowrap; text-align: right">23.45 ms</td>
    <td style="white-space: nowrap; text-align: right">34.60 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">35.57</td>
    <td style="white-space: nowrap; text-align: right">28.11 ms</td>
    <td style="white-space: nowrap; text-align: right">±23.63%</td>
    <td style="white-space: nowrap; text-align: right">28.62 ms</td>
    <td style="white-space: nowrap; text-align: right">42.07 ms</td>
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
    <td style="white-space: nowrap;text-align: right">855.94</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">697.18</td>
    <td style="white-space: nowrap; text-align: right">1.23x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">592.43</td>
    <td style="white-space: nowrap; text-align: right">1.44x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">126.86</td>
    <td style="white-space: nowrap; text-align: right">6.75x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">70.34</td>
    <td style="white-space: nowrap; text-align: right">12.17x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">65.88</td>
    <td style="white-space: nowrap; text-align: right">12.99x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">47.60</td>
    <td style="white-space: nowrap; text-align: right">17.98x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">42.48</td>
    <td style="white-space: nowrap; text-align: right">20.15x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">35.57</td>
    <td style="white-space: nowrap; text-align: right">24.06x</td>
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
    <td style="white-space: nowrap">0.00005 MB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">0.00005 MB</td>
    <td>1.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap">0.114 MB</td>
    <td>2491.5x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">1.16 MB</td>
    <td>25237.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">2.71 MB</td>
    <td>59295.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">2.28 MB</td>
    <td>49879.33x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">5.31 MB</td>
    <td>116011.33x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">4.76 MB</td>
    <td>103916.67x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">6.04 MB</td>
    <td>132028.83x</td>
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
    <td style="white-space: nowrap; text-align: right">2869.14</td>
    <td style="white-space: nowrap; text-align: right">0.35 ms</td>
    <td style="white-space: nowrap; text-align: right">±37.19%</td>
    <td style="white-space: nowrap; text-align: right">0.30 ms</td>
    <td style="white-space: nowrap; text-align: right">0.64 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">2200.62</td>
    <td style="white-space: nowrap; text-align: right">0.45 ms</td>
    <td style="white-space: nowrap; text-align: right">±44.15%</td>
    <td style="white-space: nowrap; text-align: right">0.39 ms</td>
    <td style="white-space: nowrap; text-align: right">0.97 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">1760.45</td>
    <td style="white-space: nowrap; text-align: right">0.57 ms</td>
    <td style="white-space: nowrap; text-align: right">±148.22%</td>
    <td style="white-space: nowrap; text-align: right">0.45 ms</td>
    <td style="white-space: nowrap; text-align: right">5.77 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">449.38</td>
    <td style="white-space: nowrap; text-align: right">2.23 ms</td>
    <td style="white-space: nowrap; text-align: right">±97.65%</td>
    <td style="white-space: nowrap; text-align: right">1.51 ms</td>
    <td style="white-space: nowrap; text-align: right">12.15 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">212.84</td>
    <td style="white-space: nowrap; text-align: right">4.70 ms</td>
    <td style="white-space: nowrap; text-align: right">±78.96%</td>
    <td style="white-space: nowrap; text-align: right">2.32 ms</td>
    <td style="white-space: nowrap; text-align: right">14.11 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">200.05</td>
    <td style="white-space: nowrap; text-align: right">5.00 ms</td>
    <td style="white-space: nowrap; text-align: right">±64.73%</td>
    <td style="white-space: nowrap; text-align: right">2.92 ms</td>
    <td style="white-space: nowrap; text-align: right">13.29 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">128.02</td>
    <td style="white-space: nowrap; text-align: right">7.81 ms</td>
    <td style="white-space: nowrap; text-align: right">±44.48%</td>
    <td style="white-space: nowrap; text-align: right">8.54 ms</td>
    <td style="white-space: nowrap; text-align: right">14.94 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">125.21</td>
    <td style="white-space: nowrap; text-align: right">7.99 ms</td>
    <td style="white-space: nowrap; text-align: right">±48.17%</td>
    <td style="white-space: nowrap; text-align: right">9.01 ms</td>
    <td style="white-space: nowrap; text-align: right">15.83 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">91.97</td>
    <td style="white-space: nowrap; text-align: right">10.87 ms</td>
    <td style="white-space: nowrap; text-align: right">±39.12%</td>
    <td style="white-space: nowrap; text-align: right">10.34 ms</td>
    <td style="white-space: nowrap; text-align: right">22.30 ms</td>
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
    <td style="white-space: nowrap;text-align: right">2869.14</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">2200.62</td>
    <td style="white-space: nowrap; text-align: right">1.3x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">1760.45</td>
    <td style="white-space: nowrap; text-align: right">1.63x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">449.38</td>
    <td style="white-space: nowrap; text-align: right">6.38x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">212.84</td>
    <td style="white-space: nowrap; text-align: right">13.48x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">200.05</td>
    <td style="white-space: nowrap; text-align: right">14.34x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">128.02</td>
    <td style="white-space: nowrap; text-align: right">22.41x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">125.21</td>
    <td style="white-space: nowrap; text-align: right">22.91x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">91.97</td>
    <td style="white-space: nowrap; text-align: right">31.2x</td>
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
    <td style="white-space: nowrap">42.85 KB</td>
    <td>914.17x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">301.06 KB</td>
    <td>6422.67x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">696.02 KB</td>
    <td>14848.33x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">581.67 KB</td>
    <td>12409.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">1817.63 KB</td>
    <td>38776.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">1756.70 KB</td>
    <td>37476.17x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">2049.65 KB</td>
    <td>43725.83x</td>
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
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">28.66</td>
    <td style="white-space: nowrap; text-align: right">34.89 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.63%</td>
    <td style="white-space: nowrap; text-align: right">34.63 ms</td>
    <td style="white-space: nowrap; text-align: right">48.12 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">26.28</td>
    <td style="white-space: nowrap; text-align: right">38.05 ms</td>
    <td style="white-space: nowrap; text-align: right">±4.80%</td>
    <td style="white-space: nowrap; text-align: right">37.70 ms</td>
    <td style="white-space: nowrap; text-align: right">43.10 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">10.18</td>
    <td style="white-space: nowrap; text-align: right">98.26 ms</td>
    <td style="white-space: nowrap; text-align: right">±490.87%</td>
    <td style="white-space: nowrap; text-align: right">25.02 ms</td>
    <td style="white-space: nowrap; text-align: right">3870.69 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">5.40</td>
    <td style="white-space: nowrap; text-align: right">185.15 ms</td>
    <td style="white-space: nowrap; text-align: right">±28.54%</td>
    <td style="white-space: nowrap; text-align: right">175.90 ms</td>
    <td style="white-space: nowrap; text-align: right">302.84 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">3.12</td>
    <td style="white-space: nowrap; text-align: right">320.95 ms</td>
    <td style="white-space: nowrap; text-align: right">±21.39%</td>
    <td style="white-space: nowrap; text-align: right">298.85 ms</td>
    <td style="white-space: nowrap; text-align: right">494.79 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">2.99</td>
    <td style="white-space: nowrap; text-align: right">334.98 ms</td>
    <td style="white-space: nowrap; text-align: right">±27.26%</td>
    <td style="white-space: nowrap; text-align: right">297.97 ms</td>
    <td style="white-space: nowrap; text-align: right">468.69 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">2.34</td>
    <td style="white-space: nowrap; text-align: right">427.59 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.09%</td>
    <td style="white-space: nowrap; text-align: right">426.73 ms</td>
    <td style="white-space: nowrap; text-align: right">454.43 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">1.81</td>
    <td style="white-space: nowrap; text-align: right">551.16 ms</td>
    <td style="white-space: nowrap; text-align: right">±26.94%</td>
    <td style="white-space: nowrap; text-align: right">543.93 ms</td>
    <td style="white-space: nowrap; text-align: right">760.25 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">1.65</td>
    <td style="white-space: nowrap; text-align: right">607.27 ms</td>
    <td style="white-space: nowrap; text-align: right">±23.95%</td>
    <td style="white-space: nowrap; text-align: right">685.26 ms</td>
    <td style="white-space: nowrap; text-align: right">753.58 ms</td>
  </tr>
</table>

Comparsion
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap;text-align: right">28.66</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">26.28</td>
    <td style="white-space: nowrap; text-align: right">1.09x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">10.18</td>
    <td style="white-space: nowrap; text-align: right">2.82x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">5.40</td>
    <td style="white-space: nowrap; text-align: right">5.31x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">3.12</td>
    <td style="white-space: nowrap; text-align: right">9.2x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">2.99</td>
    <td style="white-space: nowrap; text-align: right">9.6x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">2.34</td>
    <td style="white-space: nowrap; text-align: right">12.25x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">1.81</td>
    <td style="white-space: nowrap; text-align: right">15.8x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">1.65</td>
    <td style="white-space: nowrap; text-align: right">17.4x</td>
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
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">0.00005 MB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap">3.30 MB</td>
    <td>72171.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">0.00005 MB</td>
    <td>1.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">25.87 MB</td>
    <td>565089.5x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">60.07 MB</td>
    <td>1312337.33x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">50.69 MB</td>
    <td>1107272.17x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">118.27 MB</td>
    <td>2583541.67x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">112.51 MB</td>
    <td>2457865.5x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">137.48 MB</td>
    <td>3003214.17x</td>
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
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">47.04</td>
    <td style="white-space: nowrap; text-align: right">21.26 ms</td>
    <td style="white-space: nowrap; text-align: right">±10.49%</td>
    <td style="white-space: nowrap; text-align: right">20.96 ms</td>
    <td style="white-space: nowrap; text-align: right">30.05 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">29.15</td>
    <td style="white-space: nowrap; text-align: right">34.30 ms</td>
    <td style="white-space: nowrap; text-align: right">±861.49%</td>
    <td style="white-space: nowrap; text-align: right">11.31 ms</td>
    <td style="white-space: nowrap; text-align: right">48.86 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">15.33</td>
    <td style="white-space: nowrap; text-align: right">65.25 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.06%</td>
    <td style="white-space: nowrap; text-align: right">64.94 ms</td>
    <td style="white-space: nowrap; text-align: right">70.13 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">6.25</td>
    <td style="white-space: nowrap; text-align: right">159.87 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.77%</td>
    <td style="white-space: nowrap; text-align: right">158.41 ms</td>
    <td style="white-space: nowrap; text-align: right">172.91 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">5.81</td>
    <td style="white-space: nowrap; text-align: right">172.08 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.92%</td>
    <td style="white-space: nowrap; text-align: right">171.02 ms</td>
    <td style="white-space: nowrap; text-align: right">186.52 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">4.91</td>
    <td style="white-space: nowrap; text-align: right">203.70 ms</td>
    <td style="white-space: nowrap; text-align: right">±4.93%</td>
    <td style="white-space: nowrap; text-align: right">200.86 ms</td>
    <td style="white-space: nowrap; text-align: right">234.37 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">3.75</td>
    <td style="white-space: nowrap; text-align: right">266.65 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.45%</td>
    <td style="white-space: nowrap; text-align: right">266.25 ms</td>
    <td style="white-space: nowrap; text-align: right">276.56 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">0.98</td>
    <td style="white-space: nowrap; text-align: right">1020.95 ms</td>
    <td style="white-space: nowrap; text-align: right">±32.18%</td>
    <td style="white-space: nowrap; text-align: right">851.73 ms</td>
    <td style="white-space: nowrap; text-align: right">1489.86 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">0.59</td>
    <td style="white-space: nowrap; text-align: right">1695.14 ms</td>
    <td style="white-space: nowrap; text-align: right">±9.03%</td>
    <td style="white-space: nowrap; text-align: right">1700.34 ms</td>
    <td style="white-space: nowrap; text-align: right">1901.33 ms</td>
  </tr>
</table>

Comparsion
<table style="width: 1%">
  <tr>
    <th>Name</th>
    <th style="text-align: right">IPS</th>
    <th style="text-align: right">Slower</th>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap;text-align: right">47.04</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">29.15</td>
    <td style="white-space: nowrap; text-align: right">1.61x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">15.33</td>
    <td style="white-space: nowrap; text-align: right">3.07x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">6.25</td>
    <td style="white-space: nowrap; text-align: right">7.52x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">5.81</td>
    <td style="white-space: nowrap; text-align: right">8.09x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">4.91</td>
    <td style="white-space: nowrap; text-align: right">9.58x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">3.75</td>
    <td style="white-space: nowrap; text-align: right">12.54x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">0.98</td>
    <td style="white-space: nowrap; text-align: right">48.03x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">0.59</td>
    <td style="white-space: nowrap; text-align: right">79.74x</td>
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
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">0.00005 MB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap">0.00005 MB</td>
    <td>1.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap">0.0125 MB</td>
    <td>273.33x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">3.82 MB</td>
    <td>83365.83x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">1.46 MB</td>
    <td>31849.67x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">7.51 MB</td>
    <td>164152.17x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">3.92 MB</td>
    <td>85609.5x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">308.72 MB</td>
    <td>6744006.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">234.46 MB</td>
    <td>5121917.33x</td>
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
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">845.83</td>
    <td style="white-space: nowrap; text-align: right">1.18 ms</td>
    <td style="white-space: nowrap; text-align: right">±156.88%</td>
    <td style="white-space: nowrap; text-align: right">1.02 ms</td>
    <td style="white-space: nowrap; text-align: right">2.68 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">716.68</td>
    <td style="white-space: nowrap; text-align: right">1.40 ms</td>
    <td style="white-space: nowrap; text-align: right">±28.31%</td>
    <td style="white-space: nowrap; text-align: right">1.36 ms</td>
    <td style="white-space: nowrap; text-align: right">2.40 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">619.73</td>
    <td style="white-space: nowrap; text-align: right">1.61 ms</td>
    <td style="white-space: nowrap; text-align: right">±83.63%</td>
    <td style="white-space: nowrap; text-align: right">1.26 ms</td>
    <td style="white-space: nowrap; text-align: right">7.64 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">151.61</td>
    <td style="white-space: nowrap; text-align: right">6.60 ms</td>
    <td style="white-space: nowrap; text-align: right">±55.86%</td>
    <td style="white-space: nowrap; text-align: right">4.27 ms</td>
    <td style="white-space: nowrap; text-align: right">15.91 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">76.17</td>
    <td style="white-space: nowrap; text-align: right">13.13 ms</td>
    <td style="white-space: nowrap; text-align: right">±27.68%</td>
    <td style="white-space: nowrap; text-align: right">13.30 ms</td>
    <td style="white-space: nowrap; text-align: right">20.42 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">71.10</td>
    <td style="white-space: nowrap; text-align: right">14.06 ms</td>
    <td style="white-space: nowrap; text-align: right">±18.81%</td>
    <td style="white-space: nowrap; text-align: right">13.07 ms</td>
    <td style="white-space: nowrap; text-align: right">20.40 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">48.58</td>
    <td style="white-space: nowrap; text-align: right">20.59 ms</td>
    <td style="white-space: nowrap; text-align: right">±23.07%</td>
    <td style="white-space: nowrap; text-align: right">20.77 ms</td>
    <td style="white-space: nowrap; text-align: right">31.00 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">46.81</td>
    <td style="white-space: nowrap; text-align: right">21.36 ms</td>
    <td style="white-space: nowrap; text-align: right">±24.17%</td>
    <td style="white-space: nowrap; text-align: right">21.54 ms</td>
    <td style="white-space: nowrap; text-align: right">33.01 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">44.10</td>
    <td style="white-space: nowrap; text-align: right">22.68 ms</td>
    <td style="white-space: nowrap; text-align: right">±20.53%</td>
    <td style="white-space: nowrap; text-align: right">22.91 ms</td>
    <td style="white-space: nowrap; text-align: right">33.45 ms</td>
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
    <td style="white-space: nowrap;text-align: right">845.83</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">716.68</td>
    <td style="white-space: nowrap; text-align: right">1.18x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">619.73</td>
    <td style="white-space: nowrap; text-align: right">1.36x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">151.61</td>
    <td style="white-space: nowrap; text-align: right">5.58x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">76.17</td>
    <td style="white-space: nowrap; text-align: right">11.1x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">71.10</td>
    <td style="white-space: nowrap; text-align: right">11.9x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">48.58</td>
    <td style="white-space: nowrap; text-align: right">17.41x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">46.81</td>
    <td style="white-space: nowrap; text-align: right">18.07x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">44.10</td>
    <td style="white-space: nowrap; text-align: right">19.18x</td>
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
    <td style="white-space: nowrap">0.00005 MB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">0.00005 MB</td>
    <td>1.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap">0.108 MB</td>
    <td>2362.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">1.01 MB</td>
    <td>22087.67x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">1.94 MB</td>
    <td>42486.67x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">2.30 MB</td>
    <td>50335.83x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">4.11 MB</td>
    <td>89772.17x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">4.93 MB</td>
    <td>107643.83x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">4.45 MB</td>
    <td>97224.0x</td>
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
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">1054.87</td>
    <td style="white-space: nowrap; text-align: right">0.95 ms</td>
    <td style="white-space: nowrap; text-align: right">±183.72%</td>
    <td style="white-space: nowrap; text-align: right">0.86 ms</td>
    <td style="white-space: nowrap; text-align: right">1.50 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">918.31</td>
    <td style="white-space: nowrap; text-align: right">1.09 ms</td>
    <td style="white-space: nowrap; text-align: right">±109.69%</td>
    <td style="white-space: nowrap; text-align: right">0.83 ms</td>
    <td style="white-space: nowrap; text-align: right">6.55 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">876.60</td>
    <td style="white-space: nowrap; text-align: right">1.14 ms</td>
    <td style="white-space: nowrap; text-align: right">±30.67%</td>
    <td style="white-space: nowrap; text-align: right">0.96 ms</td>
    <td style="white-space: nowrap; text-align: right">2.08 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">161.40</td>
    <td style="white-space: nowrap; text-align: right">6.20 ms</td>
    <td style="white-space: nowrap; text-align: right">±64.85%</td>
    <td style="white-space: nowrap; text-align: right">3.63 ms</td>
    <td style="white-space: nowrap; text-align: right">15.84 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">97.74</td>
    <td style="white-space: nowrap; text-align: right">10.23 ms</td>
    <td style="white-space: nowrap; text-align: right">±39.19%</td>
    <td style="white-space: nowrap; text-align: right">10.96 ms</td>
    <td style="white-space: nowrap; text-align: right">18.62 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">84.57</td>
    <td style="white-space: nowrap; text-align: right">11.83 ms</td>
    <td style="white-space: nowrap; text-align: right">±32.80%</td>
    <td style="white-space: nowrap; text-align: right">11.72 ms</td>
    <td style="white-space: nowrap; text-align: right">19.37 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">78.95</td>
    <td style="white-space: nowrap; text-align: right">12.67 ms</td>
    <td style="white-space: nowrap; text-align: right">±27.85%</td>
    <td style="white-space: nowrap; text-align: right">12.02 ms</td>
    <td style="white-space: nowrap; text-align: right">22.81 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">64.73</td>
    <td style="white-space: nowrap; text-align: right">15.45 ms</td>
    <td style="white-space: nowrap; text-align: right">±25.22%</td>
    <td style="white-space: nowrap; text-align: right">14.98 ms</td>
    <td style="white-space: nowrap; text-align: right">27.04 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">50.70</td>
    <td style="white-space: nowrap; text-align: right">19.72 ms</td>
    <td style="white-space: nowrap; text-align: right">±25.62%</td>
    <td style="white-space: nowrap; text-align: right">19.97 ms</td>
    <td style="white-space: nowrap; text-align: right">30.35 ms</td>
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
    <td style="white-space: nowrap;text-align: right">1054.87</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">918.31</td>
    <td style="white-space: nowrap; text-align: right">1.15x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">876.60</td>
    <td style="white-space: nowrap; text-align: right">1.2x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">161.40</td>
    <td style="white-space: nowrap; text-align: right">6.54x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">97.74</td>
    <td style="white-space: nowrap; text-align: right">10.79x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">84.57</td>
    <td style="white-space: nowrap; text-align: right">12.47x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">78.95</td>
    <td style="white-space: nowrap; text-align: right">13.36x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">64.73</td>
    <td style="white-space: nowrap; text-align: right">16.3x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">50.70</td>
    <td style="white-space: nowrap; text-align: right">20.81x</td>
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
    <td style="white-space: nowrap">0.00005 MB</td>
      <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap">0.0506 MB</td>
    <td>1106.33x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap">0.00005 MB</td>
    <td>1.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">0.97 MB</td>
    <td>21294.5x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">1.82 MB</td>
    <td>39695.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">2.12 MB</td>
    <td>46407.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">2.74 MB</td>
    <td>59833.33x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">2.68 MB</td>
    <td>58540.83x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">4.11 MB</td>
    <td>89770.0x</td>
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
    <td style="white-space: nowrap; text-align: right">20.89 K</td>
    <td style="white-space: nowrap; text-align: right">47.88 μs</td>
    <td style="white-space: nowrap; text-align: right">±113.12%</td>
    <td style="white-space: nowrap; text-align: right">38 μs</td>
    <td style="white-space: nowrap; text-align: right">259 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">12.47 K</td>
    <td style="white-space: nowrap; text-align: right">80.16 μs</td>
    <td style="white-space: nowrap; text-align: right">±121.47%</td>
    <td style="white-space: nowrap; text-align: right">48 μs</td>
    <td style="white-space: nowrap; text-align: right">398 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">7.97 K</td>
    <td style="white-space: nowrap; text-align: right">125.41 μs</td>
    <td style="white-space: nowrap; text-align: right">±62.37%</td>
    <td style="white-space: nowrap; text-align: right">113 μs</td>
    <td style="white-space: nowrap; text-align: right">364 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">3.48 K</td>
    <td style="white-space: nowrap; text-align: right">287.60 μs</td>
    <td style="white-space: nowrap; text-align: right">±165.02%</td>
    <td style="white-space: nowrap; text-align: right">242 μs</td>
    <td style="white-space: nowrap; text-align: right">419 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">1.67 K</td>
    <td style="white-space: nowrap; text-align: right">600.40 μs</td>
    <td style="white-space: nowrap; text-align: right">±193.61%</td>
    <td style="white-space: nowrap; text-align: right">354 μs</td>
    <td style="white-space: nowrap; text-align: right">5710 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">1.64 K</td>
    <td style="white-space: nowrap; text-align: right">608.23 μs</td>
    <td style="white-space: nowrap; text-align: right">±144.87%</td>
    <td style="white-space: nowrap; text-align: right">460 μs</td>
    <td style="white-space: nowrap; text-align: right">5445.47 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">1.05 K</td>
    <td style="white-space: nowrap; text-align: right">950.87 μs</td>
    <td style="white-space: nowrap; text-align: right">±182.29%</td>
    <td style="white-space: nowrap; text-align: right">462 μs</td>
    <td style="white-space: nowrap; text-align: right">9781.60 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">0.45 K</td>
    <td style="white-space: nowrap; text-align: right">2242.43 μs</td>
    <td style="white-space: nowrap; text-align: right">±114.19%</td>
    <td style="white-space: nowrap; text-align: right">1094 μs</td>
    <td style="white-space: nowrap; text-align: right">11480.08 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">0.194 K</td>
    <td style="white-space: nowrap; text-align: right">5145.73 μs</td>
    <td style="white-space: nowrap; text-align: right">±75.76%</td>
    <td style="white-space: nowrap; text-align: right">2968 μs</td>
    <td style="white-space: nowrap; text-align: right">13741.52 μs</td>
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
    <td style="white-space: nowrap;text-align: right">20.89 K</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">12.47 K</td>
    <td style="white-space: nowrap; text-align: right">1.67x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">7.97 K</td>
    <td style="white-space: nowrap; text-align: right">2.62x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">3.48 K</td>
    <td style="white-space: nowrap; text-align: right">6.01x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">1.67 K</td>
    <td style="white-space: nowrap; text-align: right">12.54x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">1.64 K</td>
    <td style="white-space: nowrap; text-align: right">12.7x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">1.05 K</td>
    <td style="white-space: nowrap; text-align: right">19.86x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">0.45 K</td>
    <td style="white-space: nowrap; text-align: right">46.83x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">0.194 K</td>
    <td style="white-space: nowrap; text-align: right">107.47x</td>
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
    <td style="white-space: nowrap">0.102 KB</td>
    <td>2.17x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap">12.65 KB</td>
    <td>269.83x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap">63.73 KB</td>
    <td>1359.67x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap">36.79 KB</td>
    <td>784.83x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap">150.62 KB</td>
    <td>3213.17x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap">297.89 KB</td>
    <td>6355.0x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap">666.42 KB</td>
    <td>14217.0x</td>
  </tr>
</table>

<hr/>

