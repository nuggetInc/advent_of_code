view the original on <a href=https://adventofcode.com/2024/day/11>adventofcode.com</a>
<h2>--- Day 11: Plutonian Pebbles ---</h2><p>The ancient civilization on <a href="/2019/day/20">Pluto</a> was known for its ability to manipulate spacetime, and while The Historians explore their infinite corridors, you've noticed a strange set of physics-defying stones.</p>
<p>At first glance, they seem like normal stones: they're arranged in a perfectly <b>straight line</b>, and each stone has a <b>number</b> engraved on it.</p>
<p>The strange part is that every time you <span title="No, they're not statues. Why do you ask?">blink</span>, the stones <b>change</b>.</p>
<p>Sometimes, the number engraved on a stone changes. Other times, a stone might <b>split in two</b>, causing all the other stones to shift over a bit to make room in their perfectly straight line.</p>
<p>As you observe them for a while, you find that the stones have a consistent behavior. Every time you blink, the stones each <b>simultaneously</b> change according to the <b>first applicable rule</b> in this list:</p>
<ul>
<li>If the stone is engraved with the number <code>0</code>, it is replaced by a stone engraved with the number <code>1</code>.</li>
<li>If the stone is engraved with a number that has an <b>even</b> number of digits, it is replaced by <b>two stones</b>. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: <code>1000</code> would become stones <code>10</code> and <code>0</code>.)</li>
<li>If none of the other rules apply, the stone is replaced by a new stone; the old stone's number <b>multiplied by 2024</b> is engraved on the new stone.</li>
</ul>
<p>No matter how the stones change, their <b>order is preserved</b>, and they stay on their perfectly straight line.</p>
<p>How will the stones evolve if you keep blinking at them? You take a note of the number engraved on each stone in the line (your puzzle input).</p>
<p>If you have an arrangement of five stones engraved with the numbers <code>0 1 10 99 999</code> and you blink once, the stones transform as follows:</p>
<ul>
<li>The first stone, <code>0</code>, becomes a stone marked <code>1</code>.</li>
<li>The second stone, <code>1</code>, is multiplied by 2024 to become <code>2024</code>.</li>
<li>The third stone, <code>10</code>, is split into a stone marked <code>1</code> followed by a stone marked <code>0</code>.</li>
<li>The fourth stone, <code>99</code>, is split into two stones marked <code>9</code>.</li>
<li>The fifth stone, <code>999</code>, is replaced by a stone marked <code>2021976</code>.</li>
</ul>
<p>So, after blinking once, your five stones would become an arrangement of seven stones engraved with the numbers <code>1 2024 1 0 9 9 2021976</code>.</p>
<p>Here is a longer example:</p>
<pre><code>Initial arrangement:
125 17

After 1 blink:
253000 1 7

After 2 blinks:
253 0 2024 14168

After 3 blinks:
512072 1 20 24 28676032

After 4 blinks:
512 72 2024 2 0 2 4 2867 6032

After 5 blinks:
1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32

After 6 blinks:
2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2
</code></pre>
<p>In this example, after blinking six times, you would have <code>22</code> stones. After blinking 25 times, you would have <code><b>55312</b></code> stones!</p>
<p>Consider the arrangement of stones in front of you. <b>How many stones will you have after blinking 25 times?</b></p>
<h2 id="part2">--- Part Two ---</h2><p>The Historians sure are taking a long time. To be fair, the infinite corridors <b>are</b> very large.</p>
<p><b>How many stones would you have after blinking a total of 75 times?</b></p>

