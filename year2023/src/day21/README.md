view the original on <a href=https://adventofcode.com/2023/day/21>adventofcode.com</a>
<h2>--- Day 21: Step Counter ---</h2><p>You manage to catch the <a href="7">airship</a> right as it's dropping someone else off on their all-expenses-paid trip to Desert Island! It even helpfully drops you off near the <a href="5">gardener</a> and his massive farm.</p>
<p>"You got the sand flowing again! Great work! Now we just need to wait until we have enough sand to filter the water for Snow Island and we'll have snow again in no time."</p>
<p>While you wait, one of the Elves that works with the gardener heard how good you are at solving problems and would like your help. He needs to get his <a href="https://en.wikipedia.org/wiki/Pedometer">steps</a> in for the day, and so he'd like to know <b>which garden plots he can reach with exactly his remaining <code>64</code> steps</b>.</p>
<p>He gives you an up-to-date map (your puzzle input) of his starting position (<code>S</code>), garden plots (<code>.</code>), and rocks (<code>#</code>). For example:</p>
<pre><code>...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
</code></pre>
<p>The Elf starts at the starting position (<code>S</code>) which also counts as a garden plot. Then, he can take one step north, south, east, or west, but only onto tiles that are garden plots. This would allow him to reach any of the tiles marked <code>O</code>:</p>
<pre><code>...........
.....###.#.
.###.##..#.
..#.#...#..
....#O#....
.##.OS####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
</code></pre>
<p>Then, he takes a second step. Since at this point he could be at <b>either</b> tile marked <code>O</code>, his second step would allow him to reach any garden plot that is one step north, south, east, or west of <b>any</b> tile that he could have reached after the first step:</p>
<pre><code>...........
.....###.#.
.###.##..#.
..#.#O..#..
....#.#....
.##O.O####.
.##.O#...#.
.......##..
.##.#.####.
.##..##.##.
...........
</code></pre>
<p>After two steps, he could be at any of the tiles marked <code>O</code> above, including the starting position (either by going north-then-south or by going west-then-east).</p>
<p>A single third step leads to even more possibilities:</p>
<pre><code>...........
.....###.#.
.###.##..#.
..#.#.O.#..
...O#O#....
.##.OS####.
.##O.#...#.
....O..##..
.##.#.####.
.##..##.##.
...........
</code></pre>
<p>He will continue like this until his steps for the day have been exhausted. After a total of <code>6</code> steps, he could reach any of the garden plots marked <code>O</code>:</p>
<pre><code>...........
.....###.#.
.###.##.O#.
.O#O#O.O#..
O.O.#.#.O..
.##O.O####.
.##.O#O..#.
.O.O.O.##..
.##.#.####.
.##O.##.##.
...........
</code></pre>
<p>In this example, if the Elf's goal was to get exactly <code>6</code> more steps today, he could use them to reach any of <code><b>16</b></code> garden plots.</p>
<p>However, the Elf <b>actually needs to get <code>64</code> steps today</b>, and the map he's handed you is much larger than the example map.</p>
<p>Starting from the garden plot marked <code>S</code> on your map, <b>how many garden plots could the Elf reach in exactly <code>64</code> steps?</b></p>
<h2 id="part2">--- Part Two ---</h2><p>The Elf seems confused by your answer until he realizes his mistake: he was reading from a <span title="Next up: 729.">list</span> of his favorite numbers that are both perfect squares and perfect cubes, not his step counter.</p>
<p>The <b>actual</b> number of steps he needs to get today is exactly <code><b>26501365</b></code>.</p>
<p>He also points out that the garden plots and rocks are set up so that the map <b>repeats infinitely</b> in every direction.</p>
<p>So, if you were to look one additional map-width or map-height out from the edge of the example map above, you would find that it keeps repeating:</p>
<pre><code>.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##..S####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
</code></pre>
<p>This is just a tiny three-map-by-three-map slice of the inexplicably-infinite farm layout; garden plots and rocks repeat as far as you can see. The Elf still starts on the one middle tile marked <code>S</code>, though - every other repeated <code>S</code> is replaced with a normal garden plot (<code>.</code>).</p>
<p>Here are the number of reachable garden plots in this new infinite version of the example map for different numbers of steps:</p>
<ul>
<li>In exactly <code>6</code> steps, he can still reach <code><b>16</b></code> garden plots.</li>
<li>In exactly <code>10</code> steps, he can reach any of <code><b>50</b></code> garden plots.</li>
<li>In exactly <code>50</code> steps, he can reach <code><b>1594</b></code> garden plots.</li>
<li>In exactly <code>100</code> steps, he can reach <code><b>6536</b></code> garden plots.</li>
<li>In exactly <code>500</code> steps, he can reach <code><b>167004</b></code> garden plots.</li>
<li>In exactly <code>1000</code> steps, he can reach <code><b>668697</b></code> garden plots.</li>
<li>In exactly <code>5000</code> steps, he can reach <code><b>16733044</b></code> garden plots.</li>
</ul>
<p>However, the step count the Elf needs is much larger! Starting from the garden plot marked <code>S</code> on your infinite map, <b>how many garden plots could the Elf reach in exactly <code>26501365</code> steps?</b></p>
