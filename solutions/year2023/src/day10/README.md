view the original on <a href=https://adventofcode.com/2023/day/10>adventofcode.com</a>
<h2>--- Day 10: Pipe Maze ---</h2><p>You use the hang glider to ride the hot air from Desert Island all the way up to the floating metal island. This island is surprisingly cold and there definitely aren't any thermals to glide on, so you leave your hang glider behind.</p>
<p>You wander around for a while, but you don't find any people or animals. However, you do occasionally find signposts labeled "<a href="https://en.wikipedia.org/wiki/Hot_spring">Hot Springs</a>" pointing in a seemingly consistent direction; maybe you can find someone at the hot springs and ask them where the desert-machine parts are made.</p>
<p>The landscape here is alien; even the flowers and trees are made of metal. As you stop to admire some metal grass, you notice something metallic scurry away in your peripheral vision and jump into a big pipe! It didn't look like any animal you've ever seen; if you want a better look, you'll need to get ahead of it.</p>
<p>Scanning the area, you discover that the entire field you're standing on is <span title="Manufactured by Hamilton and Hilbert Pipe Company">densely packed with pipes</span>; it was hard to tell at first because they're the same metallic silver color as the "ground". You make a quick sketch of all of the surface pipes you can see (your puzzle input).</p>
<p>The pipes are arranged in a two-dimensional grid of <b>tiles</b>:</p>
<ul>
<li><code>|</code> is a <b>vertical pipe</b> connecting north and south.</li>
<li><code>-</code> is a <b>horizontal pipe</b> connecting east and west.</li>
<li><code>L</code> is a <b>90-degree bend</b> connecting north and east.</li>
<li><code>J</code> is a <b>90-degree bend</b> connecting north and west.</li>
<li><code>7</code> is a <b>90-degree bend</b> connecting south and west.</li>
<li><code>F</code> is a <b>90-degree bend</b> connecting south and east.</li>
<li><code>.</code> is <b>ground</b>; there is no pipe in this tile.</li>
<li><code>S</code> is the <b>starting position</b> of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.</li>
</ul>
<p>Based on the acoustics of the animal's scurrying, you're confident the pipe that contains the animal is <b>one large, continuous loop</b>.</p>
<p>For example, here is a square loop of pipe:</p>
<pre><code>.....
.F-7.
.|.|.
.L-J.
.....
</code></pre>
<p>If the animal had entered this loop in the northwest corner, the sketch would instead look like this:</p>
<pre><code>.....
.<b>S</b>-7.
.|.|.
.L-J.
.....
</code></pre>
<p>In the above diagram, the <code>S</code> tile is still a 90-degree <code>F</code> bend: you can tell because of how the adjacent pipes connect to it.</p>
<p>Unfortunately, there are also many pipes that <b>aren't connected to the loop</b>! This sketch shows the same loop as above:</p>
<pre><code>-L|F7
7S-7|
L|7||
-L-J|
L|-JF
</code></pre>
<p>In the above diagram, you can still figure out which pipes form the main loop: they're the ones connected to <code>S</code>, pipes those pipes connect to, pipes <b>those</b> pipes connect to, and so on. Every pipe in the main loop connects to its two neighbors (including <code>S</code>, which will have exactly two pipes connecting to it, and which is assumed to connect back to those two pipes).</p>
<p>Here is a sketch that contains a slightly more complex main loop:</p>
<pre><code>..F7.
.FJ|.
SJ.L7
|F--J
LJ...
</code></pre>
<p>Here's the same example sketch with the extra, non-main-loop pipe tiles also shown:</p>
<pre><code>7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
</code></pre>
<p>If you want to <b>get out ahead of the animal</b>, you should find the tile in the loop that is <b>farthest</b> from the starting position. Because the animal is in the pipe, it doesn't make sense to measure this by direct distance. Instead, you need to find the tile that would take the longest number of steps <b>along the loop</b> to reach from the starting point - regardless of which way around the loop the animal went.</p>
<p>In the first example with the square loop:</p>
<pre><code>.....
.S-7.
.|.|.
.L-J.
.....
</code></pre>
<p>You can count the distance each tile in the loop is from the starting point like this:</p>
<pre><code>.....
.012.
.1.3.
.23<b>4</b>.
.....
</code></pre>
<p>In this example, the farthest point from the start is <code><b>4</b></code> steps away.</p>
<p>Here's the more complex loop again:</p>
<pre><code>..F7.
.FJ|.
SJ.L7
|F--J
LJ...
</code></pre>
<p>Here are the distances for each tile on that loop:</p>
<pre><code>..45.
.236.
01.7<b>8</b>
14567
23...
</code></pre>
<p>Find the single giant loop starting at <code>S</code>. <b>How many steps along the loop does it take to get from the starting position to the point farthest from the starting position?</b></p>
<h2 id="part2">--- Part Two ---</h2><p>You quickly reach the farthest point of the loop, but the animal never emerges. Maybe its nest is <b>within the area enclosed by the loop</b>?</p>
<p>To determine whether it's even worth taking the time to search for such a nest, you should calculate how many tiles are contained within the loop. For example:</p>
<pre><code>...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
</code></pre>
<p>The above loop encloses merely <b>four tiles</b> - the two pairs of <code>.</code> in the southwest and southeast (marked <code>I</code> below). The middle <code>.</code> tiles (marked <code>O</code> below) are <b>not</b> in the loop. Here is the same loop again with those regions marked:</p>
<pre><code>...........
.S-------7.
.|F-----7|.
.||<b>OOOOO</b>||.
.||<b>OOOOO</b>||.
.|L-7<b>O</b>F-J|.
.|<b>II</b>|<b>O</b>|<b>II</b>|.
.L--J<b>O</b>L--J.
.....<b>O</b>.....
</code></pre>
<p>In fact, there doesn't even need to be a full tile path to the outside for tiles to count as outside the loop - squeezing between pipes is also allowed! Here, <code>I</code> is still within the loop and <code>O</code> is still outside the loop:</p>
<pre><code>..........
.S------7.
.|F----7|.
.||<b>OOOO</b>||.
.||<b>OOOO</b>||.
.|L-7F-J|.
.|<b>II</b>||<b>II</b>|.
.L--JL--J.
..........
</code></pre>
<p>In both of the above examples, <code><b>4</b></code> tiles are enclosed by the loop.</p>
<p>Here's a larger example:</p>
<pre><code>.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
</code></pre>
<p>The above sketch has many random bits of ground, some of which are in the loop (<code>I</code>) and some of which are outside it (<code>O</code>):</p>
<pre><code><b>O</b>F----7F7F7F7F-7<b>OOOO</b>
<b>O</b>|F--7||||||||FJ<b>OOOO</b>
<b>O</b>||<b>O</b>FJ||||||||L7<b>OOOO</b>
FJL7L7LJLJ||LJ<b>I</b>L-7<b>OO</b>
L--J<b>O</b>L7<b>III</b>LJS7F-7L7<b>O</b>
<b>OOOO</b>F-J<b>II</b>F7FJ|L7L7L7
<b>OOOO</b>L7<b>I</b>F7||L7|<b>I</b>L7L7|
<b>OOOOO</b>|FJLJ|FJ|F7|<b>O</b>LJ
<b>OOOO</b>FJL-7<b>O</b>||<b>O</b>||||<b>OOO</b>
<b>OOOO</b>L---J<b>O</b>LJ<b>O</b>LJLJ<b>OOO</b>
</code></pre>
<p>In this larger example, <code><b>8</b></code> tiles are enclosed by the loop.</p>
<p>Any tile that isn't part of the main loop can count as being enclosed by the loop. Here's another example with many bits of junk pipe lying around that aren't connected to the main loop at all:</p>
<pre><code>FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
</code></pre>
<p>Here are just the tiles that are <b>enclosed by the loop</b> marked with <code>I</code>:</p>
<pre><code>FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ<b>I</b>F7FJ-
L---JF-JLJ<b>IIII</b>FJLJJ7
|F|F-JF---7<b>III</b>L7L|7|
|FFJF7L7F-JF7<b>II</b>L---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
</code></pre>
<p>In this last example, <code><b>10</b></code> tiles are enclosed by the loop.</p>
<p>Figure out whether you have time to search for the nest by calculating the area within the loop. <b>How many tiles are enclosed by the loop?</b></p>

