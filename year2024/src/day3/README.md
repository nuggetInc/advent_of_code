view the original on <a href=https://adventofcode.com/2024/day/3>adventofcode.com</a>
<h2>--- Day 3: Mull It Over ---</h2><p>"Our computers are having issues, so I have no idea if we have any Chief Historians <span title="There's a spot reserved for Chief Historians between the green toboggans and the red toboggans. They've never actually had any Chief Historians in stock, but it's best to be prepared.">in stock</span>! You're welcome to check the warehouse, though," says the mildly flustered shopkeeper at the <a href="/2020/day/2">North Pole Toboggan Rental Shop</a>. The Historians head out to take a look.</p>
<p>The shopkeeper turns to you. "Any chance you can see why our computers are having issues again?"</p>
<p>The computer appears to be trying to run a program, but its memory (your puzzle input) is <b>corrupted</b>. All of the instructions have been jumbled up!</p>
<p>It seems like the goal of the program is just to <b>multiply some numbers</b>. It does that with instructions like <code>mul(X,Y)</code>, where <code>X</code> and <code>Y</code> are each 1-3 digit numbers. For instance, <code>mul(44,46)</code> multiplies <code>44</code> by <code>46</code> to get a result of <code>2024</code>. Similarly, <code>mul(123,4)</code> would multiply <code>123</code> by <code>4</code>.</p>
<p>However, because the program's memory has been corrupted, there are also many invalid characters that should be <b>ignored</b>, even if they look like part of a <code>mul</code> instruction. Sequences like <code>mul(4*</code>, <code>mul(6,9!</code>, <code>?(12,34)</code>, or <code>mul ( 2 , 4 )</code> do <b>nothing</b>.</p>
<p>For example, consider the following section of corrupted memory:</p>
<pre><code>x<b>mul(2,4)</b>%&amp;mul[3,7]!@^do_not_<b>mul(5,5)</b>+mul(32,64]then(<b>mul(11,8)mul(8,5)</b>)</code></pre>
<p>Only the four highlighted sections are real <code>mul</code> instructions. Adding up the result of each instruction produces <code><b>161</b></code> (<code>2*4 + 5*5 + 11*8 + 8*5</code>).</p>
<p>Scan the corrupted memory for uncorrupted <code>mul</code> instructions. <b>What do you get if you add up all of the results of the multiplications?</b></p>
<h2 id="part2">--- Part Two ---</h2><p>As you scan through the corrupted memory, you notice that some of the conditional statements are also still intact. If you handle some of the uncorrupted conditional statements in the program, you might be able to get an even more accurate result.</p>
<p>There are two new instructions you'll need to handle:</p>
<ul>
<li>The <code>do()</code> instruction <b>enables</b> future <code>mul</code> instructions.</li>
<li>The <code>don't()</code> instruction <b>disables</b> future <code>mul</code> instructions.</li>
</ul>
<p>Only the <b>most recent</b> <code>do()</code> or <code>don't()</code> instruction applies. At the beginning of the program, <code>mul</code> instructions are <b>enabled</b>.</p>
<p>For example:</p>
<pre><code>x<b>mul(2,4)</b>&amp;mul[3,7]!^<b>don't()</b>_mul(5,5)+mul(32,64](mul(11,8)un<b>do()</b>?<b>mul(8,5)</b>)</code></pre>
<p>This corrupted memory is similar to the example from before, but this time the <code>mul(5,5)</code> and <code>mul(11,8)</code> instructions are <b>disabled</b> because there is a <code>don't()</code> instruction before them. The other <code>mul</code> instructions function normally, including the one at the end that gets re-<b>enabled</b> by a <code>do()</code> instruction.</p>
<p>This time, the sum of the results is <code><b>48</b></code> (<code>2*4 + 8*5</code>).</p>
<p>Handle the new instructions; <b>what do you get if you add up all of the results of just the enabled multiplications?</b></p>

