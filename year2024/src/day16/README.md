view the original on <a href=https://adventofcode.com/2024/day/16>adventofcode.com</a>
<h2>--- Day 16: Reindeer Maze ---</h2><p>It's time again for the <a href="/2015/day/14">Reindeer Olympics</a>! This year, the big event is the <b>Reindeer Maze</b>, where the Reindeer compete for the <b><span title="I would say it's like Reindeer Golf, but knowing Reindeer, it's almost certainly nothing like Reindeer Golf.">lowest score</span></b>.</p>
<p>You and The Historians arrive to search for the Chief right as the event is about to start. It wouldn't hurt to watch a little, right?</p>
<p>The Reindeer start on the Start Tile (marked <code>S</code>) facing <b>East</b> and need to reach the End Tile (marked <code>E</code>). They can move forward one tile at a time (increasing their score by <code>1</code> point), but never into a wall (<code>#</code>). They can also rotate clockwise or counterclockwise 90 degrees at a time (increasing their score by <code>1000</code> points).</p>
<p>To figure out the best place to sit, you start by grabbing a map (your puzzle input) from a nearby kiosk. For example:</p>
<pre><code>###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
</code></pre>
<p>There are many paths through this maze, but taking any of the best paths would incur a score of only <code><b>7036</b></code>. This can be achieved by taking a total of <code>36</code> steps forward and turning 90 degrees a total of <code>7</code> times:</p>
<pre><code>
###############
#.......#....<b>E</b>#
#.#.###.#.###<b>^</b>#
#.....#.#...#<b>^</b>#
#.###.#####.#<b>^</b>#
#.#.#.......#<b>^</b>#
#.#.#####.###<b>^</b>#
#..<b>&gt;</b><b>&gt;</b><b>&gt;</b><b>&gt;</b><b>&gt;</b><b>&gt;</b><b>&gt;</b><b>&gt;</b><b>v</b>#<b>^</b>#
###<b>^</b>#.#####<b>v</b>#<b>^</b>#
#<b>&gt;</b><b>&gt;</b><b>^</b>#.....#<b>v</b>#<b>^</b>#
#<b>^</b>#.#.###.#<b>v</b>#<b>^</b>#
#<b>^</b>....#...#<b>v</b>#<b>^</b>#
#<b>^</b>###.#.#.#<b>v</b>#<b>^</b>#
#S..#.....#<b>&gt;</b><b>&gt;</b><b>^</b>#
###############
</code></pre>
<p>Here's a second example:</p>
<pre><code>#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
</code></pre>
<p>In this maze, the best paths cost <code><b>11048</b></code> points; following one such path would look like this:</p>
<pre><code>#################
#...#...#...#..<b>E</b>#
#.#.#.#.#.#.#.#<b>^</b>#
#.#.#.#...#...#<b>^</b>#
#.#.#.#.###.#.#<b>^</b>#
#<b>&gt;</b><b>&gt;</b><b>v</b>#.#.#.....#<b>^</b>#
#<b>^</b>#<b>v</b>#.#.#.#####<b>^</b>#
#<b>^</b>#<b>v</b>..#.#.#<b>&gt;</b><b>&gt;</b><b>&gt;</b><b>&gt;</b><b>^</b>#
#<b>^</b>#<b>v</b>#####.#<b>^</b>###.#
#<b>^</b>#<b>v</b>#..<b>&gt;</b><b>&gt;</b><b>&gt;</b><b>&gt;</b><b>^</b>#...#
#<b>^</b>#<b>v</b>###<b>^</b>#####.###
#<b>^</b>#<b>v</b>#<b>&gt;</b><b>&gt;</b><b>^</b>#.....#.#
#<b>^</b>#<b>v</b>#<b>^</b>#####.###.#
#<b>^</b>#<b>v</b>#<b>^</b>........#.#
#<b>^</b>#<b>v</b>#<b>^</b>#########.#
#S#<b>&gt;</b><b>&gt;</b><b>^</b>..........#
#################
</code></pre>
<p>Note that the path shown above includes one 90 degree turn as the very first move, rotating the Reindeer from facing East to facing North.</p>
<p>Analyze your map carefully. <b>What is the lowest score a Reindeer could possibly get?</b></p>
<h2 id="part2">--- Part Two ---</h2><p>Now that you know what the best paths look like, you can figure out the best spot to sit.</p>
<p>Every non-wall tile (<code>S</code>, <code>.</code>, or <code>E</code>) is equipped with places to sit along the edges of the tile. While determining which of these tiles would be the best spot to sit depends on a whole bunch of factors (how comfortable the seats are, how far away the bathrooms are, whether there's a pillar blocking your view, etc.), the most important factor is <b>whether the tile is on one of the best paths through the maze</b>. If you sit somewhere else, you'd miss all the action!</p>
<p>So, you'll need to determine which tiles are part of <b>any</b> best path through the maze, including the <code>S</code> and <code>E</code> tiles.</p>
<p>In the first example, there are <code><b>45</b></code> tiles (marked <code>O</code>) that are part of at least one of the various best paths through the maze:</p>
<pre><code>###############
#.......#....<b>O</b>#
#.#.###.#.###<b>O</b>#
#.....#.#...#<b>O</b>#
#.###.#####.#<b>O</b>#
#.#.#.......#<b>O</b>#
#.#.#####.###<b>O</b>#
#..<b>O</b><b>O</b><b>O</b><b>O</b><b>O</b><b>O</b><b>O</b><b>O</b><b>O</b>#<b>O</b>#
###<b>O</b>#<b>O</b>#####<b>O</b>#<b>O</b>#
#<b>O</b><b>O</b><b>O</b>#<b>O</b>....#<b>O</b>#<b>O</b>#
#<b>O</b>#<b>O</b>#<b>O</b>###.#<b>O</b>#<b>O</b>#
#<b>O</b><b>O</b><b>O</b><b>O</b><b>O</b>#...#<b>O</b>#<b>O</b>#
#<b>O</b>###.#.#.#<b>O</b>#<b>O</b>#
#<b>O</b>..#.....#<b>O</b><b>O</b><b>O</b>#
###############
</code></pre>
<p>In the second example, there are <code><b>64</b></code> tiles that are part of at least one of the best paths:</p>
<pre><code>#################
#...#...#...#..<b>O</b>#
#.#.#.#.#.#.#.#<b>O</b>#
#.#.#.#...#...#<b>O</b>#
#.#.#.#.###.#.#<b>O</b>#
#<b>O</b><b>O</b><b>O</b>#.#.#.....#<b>O</b>#
#<b>O</b>#<b>O</b>#.#.#.#####<b>O</b>#
#<b>O</b>#<b>O</b>..#.#.#<b>O</b><b>O</b><b>O</b><b>O</b><b>O</b>#
#<b>O</b>#<b>O</b>#####.#<b>O</b>###<b>O</b>#
#<b>O</b>#<b>O</b>#..<b>O</b><b>O</b><b>O</b><b>O</b><b>O</b>#<b>O</b><b>O</b><b>O</b>#
#<b>O</b>#<b>O</b>###<b>O</b>#####<b>O</b>###
#<b>O</b>#<b>O</b>#<b>O</b><b>O</b><b>O</b>#..<b>O</b><b>O</b><b>O</b>#.#
#<b>O</b>#<b>O</b>#<b>O</b>#####<b>O</b>###.#
#<b>O</b>#<b>O</b>#<b>O</b><b>O</b><b>O</b><b>O</b><b>O</b><b>O</b><b>O</b>..#.#
#<b>O</b>#<b>O</b>#<b>O</b>#########.#
#<b>O</b>#<b>O</b><b>O</b><b>O</b>..........#
#################
</code></pre>
<p>Analyze your map further. <b>How many tiles are part of at least one of the best paths through the maze?</b></p>

