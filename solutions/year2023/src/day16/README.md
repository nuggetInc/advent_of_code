view the original on <a href=https://adventofcode.com/2023/day/16>adventofcode.com</a>
<h2>--- Day 16: The Floor Will Be Lava ---</h2><p>With the beam of light completely focused <b>somewhere</b>, the reindeer leads you deeper still into the Lava Production Facility. At some point, you realize that the steel facility walls have been replaced with cave, and the doorways are just cave, and the floor is cave, and you're pretty sure this is actually just a giant cave.</p>
<p>Finally, as you approach what must be the heart of the mountain, you see a bright light in a cavern up ahead. There, you discover that the <span title="Not anymore, there's a blanket!">beam</span> of light you so carefully focused is emerging from the cavern wall closest to the facility and pouring all of its energy into a contraption on the opposite side.</p>
<p>Upon closer inspection, the contraption appears to be a flat, two-dimensional square grid containing <b>empty space</b> (<code>.</code>), <b>mirrors</b> (<code>/</code> and <code>\</code>), and <b>splitters</b> (<code>|</code> and <code>-</code>).</p>
<p>The contraption is aligned so that most of the beam bounces around the grid, but each tile on the grid converts some of the beam's light into <b>heat</b> to melt the rock in the cavern.</p>
<p>You note the layout of the contraption (your puzzle input). For example:</p>
<pre><code>.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
</code></pre>
<p>The beam enters in the top-left corner from the left and heading to the <b>right</b>. Then, its behavior depends on what it encounters as it moves:</p>
<ul>
<li>If the beam encounters <b>empty space</b> (<code>.</code>), it continues in the same direction.</li>
<li>If the beam encounters a <b>mirror</b> (<code>/</code> or <code>\</code>), the beam is <b>reflected</b> 90 degrees depending on the angle of the mirror. For instance, a rightward-moving beam that encounters a <code>/</code> mirror would continue <b>upward</b> in the mirror's column, while a rightward-moving beam that encounters a <code>\</code> mirror would continue <b>downward</b> from the mirror's column.</li>
<li>If the beam encounters the <b>pointy end of a splitter</b> (<code>|</code> or <code>-</code>), the beam passes through the splitter as if the splitter were <b>empty space</b>. For instance, a rightward-moving beam that encounters a <code>-</code> splitter would continue in the same direction.</li>
<li>If the beam encounters the <b>flat side of a splitter</b> (<code>|</code> or <code>-</code>), the beam is <b>split into two beams</b> going in each of the two directions the splitter's pointy ends are pointing. For instance, a rightward-moving beam that encounters a <code>|</code> splitter would split into two beams: one that continues <b>upward</b> from the splitter's column and one that continues <b>downward</b> from the splitter's column.</li>
</ul>
<p>Beams do not interact with other beams; a tile can have many beams passing through it at the same time. A tile is <b>energized</b> if that tile has at least one beam pass through it, reflect in it, or split in it.</p>
<p>In the above example, here is how the beam of light bounces around the contraption:</p>
<pre><code>&gt;|&lt;&lt;&lt;\....
|v-.\^....
.v...|-&gt;&gt;&gt;
.v...v^.|.
.v...v^...
.v...v^..\
.v../2\\..
&lt;-&gt;-/vv|..
.|&lt;&lt;&lt;2-|.\
.v//.|.v..
</code></pre>
<p>Beams are only shown on empty tiles; arrows indicate the direction of the beams. If a tile contains beams moving in multiple directions, the number of distinct directions is shown instead. Here is the same diagram but instead only showing whether a tile is <b>energized</b> (<code>#</code>) or not (<code>.</code>):</p>
<pre><code>######....
.#...#....
.#...#####
.#...##...
.#...##...
.#...##...
.#..####..
########..
.#######..
.#...#.#..
</code></pre>
<p>Ultimately, in this example, <code><b>46</b></code> tiles become <b>energized</b>.</p>
<p>The light isn't energizing enough tiles to produce lava; to debug the contraption, you need to start by analyzing the current situation. With the beam starting in the top-left heading right, <b>how many tiles end up being energized?</b></p>
<h2 id="part2">--- Part Two ---</h2><p>As you try to work out what might be wrong, the reindeer tugs on your shirt and leads you to a nearby control panel. There, a collection of buttons lets you align the contraption so that the beam enters from <b>any edge tile</b> and heading away from that edge. (You can choose either of two directions for the beam if it starts on a corner; for instance, if the beam starts in the bottom-right corner, it can start heading either left or upward.)</p>
<p>So, the beam could start on any tile in the top row (heading downward), any tile in the bottom row (heading upward), any tile in the leftmost column (heading right), or any tile in the rightmost column (heading left). To produce lava, you need to find the configuration that <b>energizes as many tiles as possible</b>.</p>
<p>In the above example, this can be achieved by starting the beam in the fourth tile from the left in the top row:</p>
<pre><code>.|&lt;2&lt;\....
|v-v\^....
.v.v.|-&gt;&gt;&gt;
.v.v.v^.|.
.v.v.v^...
.v.v.v^..\
.v.v/2\\..
&lt;-2-/vv|..
.|&lt;&lt;&lt;2-|.\
.v//.|.v..
</code></pre>
<p>Using this configuration, <code><b>51</b></code> tiles are energized:</p>
<pre><code>.#####....
.#.#.#....
.#.#.#####
.#.#.##...
.#.#.##...
.#.#.##...
.#.#####..
########..
.#######..
.#...#.#..
</code></pre>
<p>Find the initial beam configuration that energizes the largest number of tiles; <b>how many tiles are energized in that configuration?</b></p>

