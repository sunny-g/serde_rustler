# Benchmark

Benchmark run from 2019-06-09 01:56:06.600922Z UTC

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
    <td style="white-space: nowrap; text-align: right">7754.39</td>
    <td style="white-space: nowrap; text-align: right">0.129 ms</td>
    <td style="white-space: nowrap; text-align: right">±60.60%</td>
    <td style="white-space: nowrap; text-align: right">0.115 ms</td>
    <td style="white-space: nowrap; text-align: right">0.36 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">5965.47</td>
    <td style="white-space: nowrap; text-align: right">0.168 ms</td>
    <td style="white-space: nowrap; text-align: right">±69.06%</td>
    <td style="white-space: nowrap; text-align: right">0.115 ms</td>
    <td style="white-space: nowrap; text-align: right">0.48 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">3257.72</td>
    <td style="white-space: nowrap; text-align: right">0.31 ms</td>
    <td style="white-space: nowrap; text-align: right">±218.04%</td>
    <td style="white-space: nowrap; text-align: right">0.22 ms</td>
    <td style="white-space: nowrap; text-align: right">4.90 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">1359.82</td>
    <td style="white-space: nowrap; text-align: right">0.74 ms</td>
    <td style="white-space: nowrap; text-align: right">±199.63%</td>
    <td style="white-space: nowrap; text-align: right">0.35 ms</td>
    <td style="white-space: nowrap; text-align: right">9.09 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">660.01</td>
    <td style="white-space: nowrap; text-align: right">1.52 ms</td>
    <td style="white-space: nowrap; text-align: right">±145.90%</td>
    <td style="white-space: nowrap; text-align: right">0.71 ms</td>
    <td style="white-space: nowrap; text-align: right">11.01 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">616.44</td>
    <td style="white-space: nowrap; text-align: right">1.62 ms</td>
    <td style="white-space: nowrap; text-align: right">±158.34%</td>
    <td style="white-space: nowrap; text-align: right">0.61 ms</td>
    <td style="white-space: nowrap; text-align: right">11.63 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">527.24</td>
    <td style="white-space: nowrap; text-align: right">1.90 ms</td>
    <td style="white-space: nowrap; text-align: right">±124.69%</td>
    <td style="white-space: nowrap; text-align: right">0.99 ms</td>
    <td style="white-space: nowrap; text-align: right">10.91 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">396.79</td>
    <td style="white-space: nowrap; text-align: right">2.52 ms</td>
    <td style="white-space: nowrap; text-align: right">±118.00%</td>
    <td style="white-space: nowrap; text-align: right">1.25 ms</td>
    <td style="white-space: nowrap; text-align: right">12.94 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">370.05</td>
    <td style="white-space: nowrap; text-align: right">2.70 ms</td>
    <td style="white-space: nowrap; text-align: right">±104.68%</td>
    <td style="white-space: nowrap; text-align: right">1.42 ms</td>
    <td style="white-space: nowrap; text-align: right">12.36 ms</td>
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
    <td style="white-space: nowrap;text-align: right">7754.39</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">5965.47</td>
    <td style="white-space: nowrap; text-align: right">1.3x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">3257.72</td>
    <td style="white-space: nowrap; text-align: right">2.38x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">1359.82</td>
    <td style="white-space: nowrap; text-align: right">5.7x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">660.01</td>
    <td style="white-space: nowrap; text-align: right">11.75x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">616.44</td>
    <td style="white-space: nowrap; text-align: right">12.58x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">527.24</td>
    <td style="white-space: nowrap; text-align: right">14.71x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">396.79</td>
    <td style="white-space: nowrap; text-align: right">19.54x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">370.05</td>
    <td style="white-space: nowrap; text-align: right">20.95x</td>
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
    <td style="white-space: nowrap">36.47 KB</td>
    <td>778.0x</td>
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
    <td style="white-space: nowrap; text-align: right">859.96</td>
    <td style="white-space: nowrap; text-align: right">1.16 ms</td>
    <td style="white-space: nowrap; text-align: right">±138.52%</td>
    <td style="white-space: nowrap; text-align: right">1.06 ms</td>
    <td style="white-space: nowrap; text-align: right">1.86 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">697.68</td>
    <td style="white-space: nowrap; text-align: right">1.43 ms</td>
    <td style="white-space: nowrap; text-align: right">±29.58%</td>
    <td style="white-space: nowrap; text-align: right">1.39 ms</td>
    <td style="white-space: nowrap; text-align: right">2.57 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">380.55</td>
    <td style="white-space: nowrap; text-align: right">2.63 ms</td>
    <td style="white-space: nowrap; text-align: right">±77.48%</td>
    <td style="white-space: nowrap; text-align: right">1.92 ms</td>
    <td style="white-space: nowrap; text-align: right">11.78 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">148.98</td>
    <td style="white-space: nowrap; text-align: right">6.71 ms</td>
    <td style="white-space: nowrap; text-align: right">±50.94%</td>
    <td style="white-space: nowrap; text-align: right">4.48 ms</td>
    <td style="white-space: nowrap; text-align: right">14.82 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">72.50</td>
    <td style="white-space: nowrap; text-align: right">13.79 ms</td>
    <td style="white-space: nowrap; text-align: right">±21.37%</td>
    <td style="white-space: nowrap; text-align: right">13.01 ms</td>
    <td style="white-space: nowrap; text-align: right">20.33 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">63.99</td>
    <td style="white-space: nowrap; text-align: right">15.63 ms</td>
    <td style="white-space: nowrap; text-align: right">±16.78%</td>
    <td style="white-space: nowrap; text-align: right">14.75 ms</td>
    <td style="white-space: nowrap; text-align: right">22.06 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">47.46</td>
    <td style="white-space: nowrap; text-align: right">21.07 ms</td>
    <td style="white-space: nowrap; text-align: right">±33.94%</td>
    <td style="white-space: nowrap; text-align: right">22.55 ms</td>
    <td style="white-space: nowrap; text-align: right">36.83 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">41.17</td>
    <td style="white-space: nowrap; text-align: right">24.29 ms</td>
    <td style="white-space: nowrap; text-align: right">±20.19%</td>
    <td style="white-space: nowrap; text-align: right">24.16 ms</td>
    <td style="white-space: nowrap; text-align: right">36.83 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">35.60</td>
    <td style="white-space: nowrap; text-align: right">28.09 ms</td>
    <td style="white-space: nowrap; text-align: right">±23.42%</td>
    <td style="white-space: nowrap; text-align: right">28.51 ms</td>
    <td style="white-space: nowrap; text-align: right">41.66 ms</td>
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
    <td style="white-space: nowrap;text-align: right">859.96</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">697.68</td>
    <td style="white-space: nowrap; text-align: right">1.23x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">380.55</td>
    <td style="white-space: nowrap; text-align: right">2.26x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">148.98</td>
    <td style="white-space: nowrap; text-align: right">5.77x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">72.50</td>
    <td style="white-space: nowrap; text-align: right">11.86x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">63.99</td>
    <td style="white-space: nowrap; text-align: right">13.44x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">47.46</td>
    <td style="white-space: nowrap; text-align: right">18.12x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">41.17</td>
    <td style="white-space: nowrap; text-align: right">20.89x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">35.60</td>
    <td style="white-space: nowrap; text-align: right">24.16x</td>
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
    <td style="white-space: nowrap">0.33 MB</td>
    <td>7261.5x</td>
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
    <td style="white-space: nowrap; text-align: right">2886.03</td>
    <td style="white-space: nowrap; text-align: right">0.35 ms</td>
    <td style="white-space: nowrap; text-align: right">±39.01%</td>
    <td style="white-space: nowrap; text-align: right">0.30 ms</td>
    <td style="white-space: nowrap; text-align: right">0.64 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">2216.54</td>
    <td style="white-space: nowrap; text-align: right">0.45 ms</td>
    <td style="white-space: nowrap; text-align: right">±45.84%</td>
    <td style="white-space: nowrap; text-align: right">0.39 ms</td>
    <td style="white-space: nowrap; text-align: right">0.98 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">1316.24</td>
    <td style="white-space: nowrap; text-align: right">0.76 ms</td>
    <td style="white-space: nowrap; text-align: right">±138.63%</td>
    <td style="white-space: nowrap; text-align: right">0.55 ms</td>
    <td style="white-space: nowrap; text-align: right">5.93 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">532.61</td>
    <td style="white-space: nowrap; text-align: right">1.88 ms</td>
    <td style="white-space: nowrap; text-align: right">±101.32%</td>
    <td style="white-space: nowrap; text-align: right">1.25 ms</td>
    <td style="white-space: nowrap; text-align: right">10.70 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">217.92</td>
    <td style="white-space: nowrap; text-align: right">4.59 ms</td>
    <td style="white-space: nowrap; text-align: right">±80.14%</td>
    <td style="white-space: nowrap; text-align: right">2.26 ms</td>
    <td style="white-space: nowrap; text-align: right">14.04 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">188.17</td>
    <td style="white-space: nowrap; text-align: right">5.31 ms</td>
    <td style="white-space: nowrap; text-align: right">±64.77%</td>
    <td style="white-space: nowrap; text-align: right">3.08 ms</td>
    <td style="white-space: nowrap; text-align: right">14.28 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">127.54</td>
    <td style="white-space: nowrap; text-align: right">7.84 ms</td>
    <td style="white-space: nowrap; text-align: right">±44.87%</td>
    <td style="white-space: nowrap; text-align: right">8.57 ms</td>
    <td style="white-space: nowrap; text-align: right">15.04 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">122.86</td>
    <td style="white-space: nowrap; text-align: right">8.14 ms</td>
    <td style="white-space: nowrap; text-align: right">±49.25%</td>
    <td style="white-space: nowrap; text-align: right">9.09 ms</td>
    <td style="white-space: nowrap; text-align: right">16.21 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">92.73</td>
    <td style="white-space: nowrap; text-align: right">10.78 ms</td>
    <td style="white-space: nowrap; text-align: right">±40.00%</td>
    <td style="white-space: nowrap; text-align: right">10.26 ms</td>
    <td style="white-space: nowrap; text-align: right">22.40 ms</td>
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
    <td style="white-space: nowrap;text-align: right">2886.03</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">2216.54</td>
    <td style="white-space: nowrap; text-align: right">1.3x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">1316.24</td>
    <td style="white-space: nowrap; text-align: right">2.19x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">532.61</td>
    <td style="white-space: nowrap; text-align: right">5.42x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">217.92</td>
    <td style="white-space: nowrap; text-align: right">13.24x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">188.17</td>
    <td style="white-space: nowrap; text-align: right">15.34x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">127.54</td>
    <td style="white-space: nowrap; text-align: right">22.63x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">122.86</td>
    <td style="white-space: nowrap; text-align: right">23.49x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">92.73</td>
    <td style="white-space: nowrap; text-align: right">31.12x</td>
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
    <td style="white-space: nowrap">86.98 KB</td>
    <td>1855.67x</td>
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
    <td style="white-space: nowrap; text-align: right">28.45</td>
    <td style="white-space: nowrap; text-align: right">35.15 ms</td>
    <td style="white-space: nowrap; text-align: right">±6.20%</td>
    <td style="white-space: nowrap; text-align: right">34.87 ms</td>
    <td style="white-space: nowrap; text-align: right">44.25 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">12.42</td>
    <td style="white-space: nowrap; text-align: right">80.49 ms</td>
    <td style="white-space: nowrap; text-align: right">±10.08%</td>
    <td style="white-space: nowrap; text-align: right">78.84 ms</td>
    <td style="white-space: nowrap; text-align: right">114.01 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">10.31</td>
    <td style="white-space: nowrap; text-align: right">96.96 ms</td>
    <td style="white-space: nowrap; text-align: right">±533.69%</td>
    <td style="white-space: nowrap; text-align: right">25.21 ms</td>
    <td style="white-space: nowrap; text-align: right">4386.53 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">4.98</td>
    <td style="white-space: nowrap; text-align: right">200.69 ms</td>
    <td style="white-space: nowrap; text-align: right">±33.45%</td>
    <td style="white-space: nowrap; text-align: right">179.54 ms</td>
    <td style="white-space: nowrap; text-align: right">303.07 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">3.21</td>
    <td style="white-space: nowrap; text-align: right">311.17 ms</td>
    <td style="white-space: nowrap; text-align: right">±21.03%</td>
    <td style="white-space: nowrap; text-align: right">293.95 ms</td>
    <td style="white-space: nowrap; text-align: right">510.85 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">2.98</td>
    <td style="white-space: nowrap; text-align: right">335.77 ms</td>
    <td style="white-space: nowrap; text-align: right">±26.28%</td>
    <td style="white-space: nowrap; text-align: right">303.23 ms</td>
    <td style="white-space: nowrap; text-align: right">463.96 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">2.31</td>
    <td style="white-space: nowrap; text-align: right">432.66 ms</td>
    <td style="white-space: nowrap; text-align: right">±3.94%</td>
    <td style="white-space: nowrap; text-align: right">427.76 ms</td>
    <td style="white-space: nowrap; text-align: right">474.66 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">1.70</td>
    <td style="white-space: nowrap; text-align: right">589.17 ms</td>
    <td style="white-space: nowrap; text-align: right">±23.44%</td>
    <td style="white-space: nowrap; text-align: right">567.00 ms</td>
    <td style="white-space: nowrap; text-align: right">842.47 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">1.60</td>
    <td style="white-space: nowrap; text-align: right">624.96 ms</td>
    <td style="white-space: nowrap; text-align: right">±23.56%</td>
    <td style="white-space: nowrap; text-align: right">698.34 ms</td>
    <td style="white-space: nowrap; text-align: right">762.44 ms</td>
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
    <td style="white-space: nowrap;text-align: right">28.45</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">12.42</td>
    <td style="white-space: nowrap; text-align: right">2.29x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">10.31</td>
    <td style="white-space: nowrap; text-align: right">2.76x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">4.98</td>
    <td style="white-space: nowrap; text-align: right">5.71x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">3.21</td>
    <td style="white-space: nowrap; text-align: right">8.85x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">2.98</td>
    <td style="white-space: nowrap; text-align: right">9.55x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">2.31</td>
    <td style="white-space: nowrap; text-align: right">12.31x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">1.70</td>
    <td style="white-space: nowrap; text-align: right">16.76x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">1.60</td>
    <td style="white-space: nowrap; text-align: right">17.78x</td>
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
    <td style="white-space: nowrap">8.09 MB</td>
    <td>176729.0x</td>
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
    <td style="white-space: nowrap; text-align: right">44.13</td>
    <td style="white-space: nowrap; text-align: right">22.66 ms</td>
    <td style="white-space: nowrap; text-align: right">±10.12%</td>
    <td style="white-space: nowrap; text-align: right">22.53 ms</td>
    <td style="white-space: nowrap; text-align: right">30.04 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">26.04</td>
    <td style="white-space: nowrap; text-align: right">38.40 ms</td>
    <td style="white-space: nowrap; text-align: right">±828.99%</td>
    <td style="white-space: nowrap; text-align: right">12.30 ms</td>
    <td style="white-space: nowrap; text-align: right">76.32 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">15.28</td>
    <td style="white-space: nowrap; text-align: right">65.43 ms</td>
    <td style="white-space: nowrap; text-align: right">±3.54%</td>
    <td style="white-space: nowrap; text-align: right">64.91 ms</td>
    <td style="white-space: nowrap; text-align: right">77.82 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">6.21</td>
    <td style="white-space: nowrap; text-align: right">160.94 ms</td>
    <td style="white-space: nowrap; text-align: right">±3.88%</td>
    <td style="white-space: nowrap; text-align: right">159.43 ms</td>
    <td style="white-space: nowrap; text-align: right">187.24 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">6.13</td>
    <td style="white-space: nowrap; text-align: right">163.16 ms</td>
    <td style="white-space: nowrap; text-align: right">±1.92%</td>
    <td style="white-space: nowrap; text-align: right">164.02 ms</td>
    <td style="white-space: nowrap; text-align: right">171.34 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">5.08</td>
    <td style="white-space: nowrap; text-align: right">197.02 ms</td>
    <td style="white-space: nowrap; text-align: right">±5.02%</td>
    <td style="white-space: nowrap; text-align: right">195.03 ms</td>
    <td style="white-space: nowrap; text-align: right">221.67 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">3.59</td>
    <td style="white-space: nowrap; text-align: right">278.37 ms</td>
    <td style="white-space: nowrap; text-align: right">±2.05%</td>
    <td style="white-space: nowrap; text-align: right">278.21 ms</td>
    <td style="white-space: nowrap; text-align: right">300.85 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">1.00</td>
    <td style="white-space: nowrap; text-align: right">998.16 ms</td>
    <td style="white-space: nowrap; text-align: right">±31.42%</td>
    <td style="white-space: nowrap; text-align: right">846.44 ms</td>
    <td style="white-space: nowrap; text-align: right">1457.05 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">0.56</td>
    <td style="white-space: nowrap; text-align: right">1777.41 ms</td>
    <td style="white-space: nowrap; text-align: right">±8.08%</td>
    <td style="white-space: nowrap; text-align: right">1778.32 ms</td>
    <td style="white-space: nowrap; text-align: right">1962.18 ms</td>
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
    <td style="white-space: nowrap;text-align: right">44.13</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler</td>
    <td style="white-space: nowrap; text-align: right">26.04</td>
    <td style="white-space: nowrap; text-align: right">1.69x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">15.28</td>
    <td style="white-space: nowrap; text-align: right">2.89x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">6.21</td>
    <td style="white-space: nowrap; text-align: right">7.1x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">6.13</td>
    <td style="white-space: nowrap; text-align: right">7.2x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">5.08</td>
    <td style="white-space: nowrap; text-align: right">8.69x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">3.59</td>
    <td style="white-space: nowrap; text-align: right">12.28x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">1.00</td>
    <td style="white-space: nowrap; text-align: right">44.04x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">0.56</td>
    <td style="white-space: nowrap; text-align: right">78.43x</td>
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
    <td style="white-space: nowrap">0.0343 MB</td>
    <td>748.67x</td>
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
    <td style="white-space: nowrap; text-align: right">878.84</td>
    <td style="white-space: nowrap; text-align: right">1.14 ms</td>
    <td style="white-space: nowrap; text-align: right">±155.67%</td>
    <td style="white-space: nowrap; text-align: right">1.07 ms</td>
    <td style="white-space: nowrap; text-align: right">1.65 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">719.89</td>
    <td style="white-space: nowrap; text-align: right">1.39 ms</td>
    <td style="white-space: nowrap; text-align: right">±27.93%</td>
    <td style="white-space: nowrap; text-align: right">1.36 ms</td>
    <td style="white-space: nowrap; text-align: right">2.39 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">330.76</td>
    <td style="white-space: nowrap; text-align: right">3.02 ms</td>
    <td style="white-space: nowrap; text-align: right">±65.68%</td>
    <td style="white-space: nowrap; text-align: right">2.31 ms</td>
    <td style="white-space: nowrap; text-align: right">11.87 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">153.34</td>
    <td style="white-space: nowrap; text-align: right">6.52 ms</td>
    <td style="white-space: nowrap; text-align: right">±56.32%</td>
    <td style="white-space: nowrap; text-align: right">4.20 ms</td>
    <td style="white-space: nowrap; text-align: right">15.66 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">78.56</td>
    <td style="white-space: nowrap; text-align: right">12.73 ms</td>
    <td style="white-space: nowrap; text-align: right">±28.48%</td>
    <td style="white-space: nowrap; text-align: right">12.81 ms</td>
    <td style="white-space: nowrap; text-align: right">20.31 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">72.53</td>
    <td style="white-space: nowrap; text-align: right">13.79 ms</td>
    <td style="white-space: nowrap; text-align: right">±18.52%</td>
    <td style="white-space: nowrap; text-align: right">12.86 ms</td>
    <td style="white-space: nowrap; text-align: right">19.85 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">51.23</td>
    <td style="white-space: nowrap; text-align: right">19.52 ms</td>
    <td style="white-space: nowrap; text-align: right">±29.02%</td>
    <td style="white-space: nowrap; text-align: right">20.27 ms</td>
    <td style="white-space: nowrap; text-align: right">31.22 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">47.36</td>
    <td style="white-space: nowrap; text-align: right">21.11 ms</td>
    <td style="white-space: nowrap; text-align: right">±28.88%</td>
    <td style="white-space: nowrap; text-align: right">21.63 ms</td>
    <td style="white-space: nowrap; text-align: right">35.08 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">44.81</td>
    <td style="white-space: nowrap; text-align: right">22.32 ms</td>
    <td style="white-space: nowrap; text-align: right">±20.65%</td>
    <td style="white-space: nowrap; text-align: right">22.48 ms</td>
    <td style="white-space: nowrap; text-align: right">32.63 ms</td>
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
    <td style="white-space: nowrap;text-align: right">878.84</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">719.89</td>
    <td style="white-space: nowrap; text-align: right">1.22x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">330.76</td>
    <td style="white-space: nowrap; text-align: right">2.66x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">153.34</td>
    <td style="white-space: nowrap; text-align: right">5.73x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">78.56</td>
    <td style="white-space: nowrap; text-align: right">11.19x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">72.53</td>
    <td style="white-space: nowrap; text-align: right">12.12x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">51.23</td>
    <td style="white-space: nowrap; text-align: right">17.16x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">47.36</td>
    <td style="white-space: nowrap; text-align: right">18.56x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">44.81</td>
    <td style="white-space: nowrap; text-align: right">19.61x</td>
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
    <td style="white-space: nowrap">0.35 MB</td>
    <td>7646.17x</td>
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
    <td style="white-space: nowrap; text-align: right">1047.27</td>
    <td style="white-space: nowrap; text-align: right">0.95 ms</td>
    <td style="white-space: nowrap; text-align: right">±156.73%</td>
    <td style="white-space: nowrap; text-align: right">0.89 ms</td>
    <td style="white-space: nowrap; text-align: right">1.37 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">898.75</td>
    <td style="white-space: nowrap; text-align: right">1.11 ms</td>
    <td style="white-space: nowrap; text-align: right">±29.64%</td>
    <td style="white-space: nowrap; text-align: right">0.94 ms</td>
    <td style="white-space: nowrap; text-align: right">2.03 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">442.12</td>
    <td style="white-space: nowrap; text-align: right">2.26 ms</td>
    <td style="white-space: nowrap; text-align: right">±86.19%</td>
    <td style="white-space: nowrap; text-align: right">1.63 ms</td>
    <td style="white-space: nowrap; text-align: right">11.73 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">164.15</td>
    <td style="white-space: nowrap; text-align: right">6.09 ms</td>
    <td style="white-space: nowrap; text-align: right">±65.41%</td>
    <td style="white-space: nowrap; text-align: right">3.53 ms</td>
    <td style="white-space: nowrap; text-align: right">15.90 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">97.57</td>
    <td style="white-space: nowrap; text-align: right">10.25 ms</td>
    <td style="white-space: nowrap; text-align: right">±39.13%</td>
    <td style="white-space: nowrap; text-align: right">10.97 ms</td>
    <td style="white-space: nowrap; text-align: right">18.55 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">86.82</td>
    <td style="white-space: nowrap; text-align: right">11.52 ms</td>
    <td style="white-space: nowrap; text-align: right">±32.14%</td>
    <td style="white-space: nowrap; text-align: right">11.55 ms</td>
    <td style="white-space: nowrap; text-align: right">18.60 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">80.88</td>
    <td style="white-space: nowrap; text-align: right">12.36 ms</td>
    <td style="white-space: nowrap; text-align: right">±28.89%</td>
    <td style="white-space: nowrap; text-align: right">11.69 ms</td>
    <td style="white-space: nowrap; text-align: right">23.39 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">65.97</td>
    <td style="white-space: nowrap; text-align: right">15.16 ms</td>
    <td style="white-space: nowrap; text-align: right">±25.99%</td>
    <td style="white-space: nowrap; text-align: right">14.64 ms</td>
    <td style="white-space: nowrap; text-align: right">26.43 ms</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">48.27</td>
    <td style="white-space: nowrap; text-align: right">20.72 ms</td>
    <td style="white-space: nowrap; text-align: right">±20.98%</td>
    <td style="white-space: nowrap; text-align: right">20.39 ms</td>
    <td style="white-space: nowrap; text-align: right">31.67 ms</td>
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
    <td style="white-space: nowrap;text-align: right">1047.27</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">898.75</td>
    <td style="white-space: nowrap; text-align: right">1.17x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">442.12</td>
    <td style="white-space: nowrap; text-align: right">2.37x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">164.15</td>
    <td style="white-space: nowrap; text-align: right">6.38x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">97.57</td>
    <td style="white-space: nowrap; text-align: right">10.73x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">86.82</td>
    <td style="white-space: nowrap; text-align: right">12.06x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">80.88</td>
    <td style="white-space: nowrap; text-align: right">12.95x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">65.97</td>
    <td style="white-space: nowrap; text-align: right">15.88x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">48.27</td>
    <td style="white-space: nowrap; text-align: right">21.69x</td>
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
    <td style="white-space: nowrap">0.28 MB</td>
    <td>6033.83x</td>
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
    <td style="white-space: nowrap; text-align: right">22.06 K</td>
    <td style="white-space: nowrap; text-align: right">45.32 μs</td>
    <td style="white-space: nowrap; text-align: right">±122.26%</td>
    <td style="white-space: nowrap; text-align: right">36 μs</td>
    <td style="white-space: nowrap; text-align: right">246 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">12.52 K</td>
    <td style="white-space: nowrap; text-align: right">79.90 μs</td>
    <td style="white-space: nowrap; text-align: right">±119.41%</td>
    <td style="white-space: nowrap; text-align: right">46 μs</td>
    <td style="white-space: nowrap; text-align: right">384 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">7.85 K</td>
    <td style="white-space: nowrap; text-align: right">127.33 μs</td>
    <td style="white-space: nowrap; text-align: right">±54.24%</td>
    <td style="white-space: nowrap; text-align: right">117 μs</td>
    <td style="white-space: nowrap; text-align: right">345 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">3.35 K</td>
    <td style="white-space: nowrap; text-align: right">298.43 μs</td>
    <td style="white-space: nowrap; text-align: right">±173.41%</td>
    <td style="white-space: nowrap; text-align: right">243 μs</td>
    <td style="white-space: nowrap; text-align: right">438 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">1.78 K</td>
    <td style="white-space: nowrap; text-align: right">562.23 μs</td>
    <td style="white-space: nowrap; text-align: right">±198.07%</td>
    <td style="white-space: nowrap; text-align: right">327 μs</td>
    <td style="white-space: nowrap; text-align: right">5533.50 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">1.71 K</td>
    <td style="white-space: nowrap; text-align: right">585.57 μs</td>
    <td style="white-space: nowrap; text-align: right">±145.88%</td>
    <td style="white-space: nowrap; text-align: right">442 μs</td>
    <td style="white-space: nowrap; text-align: right">5304.88 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">1.06 K</td>
    <td style="white-space: nowrap; text-align: right">942.43 μs</td>
    <td style="white-space: nowrap; text-align: right">±185.79%</td>
    <td style="white-space: nowrap; text-align: right">454 μs</td>
    <td style="white-space: nowrap; text-align: right">9772.24 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">0.44 K</td>
    <td style="white-space: nowrap; text-align: right">2273.53 μs</td>
    <td style="white-space: nowrap; text-align: right">±115.56%</td>
    <td style="white-space: nowrap; text-align: right">1094 μs</td>
    <td style="white-space: nowrap; text-align: right">11701.30 μs</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">0.199 K</td>
    <td style="white-space: nowrap; text-align: right">5021.09 μs</td>
    <td style="white-space: nowrap; text-align: right">±76.41%</td>
    <td style="white-space: nowrap; text-align: right">2752.50 μs</td>
    <td style="white-space: nowrap; text-align: right">13660.89 μs</td>
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
    <td style="white-space: nowrap;text-align: right">22.06 K</td>
    <td>&nbsp;</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">serde_rustler (dirty)</td>
    <td style="white-space: nowrap; text-align: right">12.52 K</td>
    <td style="white-space: nowrap; text-align: right">1.76x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jiffy</td>
    <td style="white-space: nowrap; text-align: right">7.85 K</td>
    <td style="white-space: nowrap; text-align: right">2.81x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Jason</td>
    <td style="white-space: nowrap; text-align: right">3.35 K</td>
    <td style="white-space: nowrap; text-align: right">6.58x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSX</td>
    <td style="white-space: nowrap; text-align: right">1.78 K</td>
    <td style="white-space: nowrap; text-align: right">12.41x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Poison</td>
    <td style="white-space: nowrap; text-align: right">1.71 K</td>
    <td style="white-space: nowrap; text-align: right">12.92x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">JSON</td>
    <td style="white-space: nowrap; text-align: right">1.06 K</td>
    <td style="white-space: nowrap; text-align: right">20.79x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">jsone</td>
    <td style="white-space: nowrap; text-align: right">0.44 K</td>
    <td style="white-space: nowrap; text-align: right">50.16x</td>
  </tr>
  <tr>
    <td style="white-space: nowrap">Tiny</td>
    <td style="white-space: nowrap; text-align: right">0.199 K</td>
    <td style="white-space: nowrap; text-align: right">110.79x</td>
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
    <td style="white-space: nowrap">0.0859 KB</td>
    <td>1.83x</td>
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

