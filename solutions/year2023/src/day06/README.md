view the original on <a href=https://adventofcode.com/2023/day/6>adventofcode.com</a>
<h2>--- Day 6: Wait For It ---</h2><p>The ferry quickly brings you across Island Island. After asking around, you discover that there is indeed normally a large pile of sand somewhere near here, but you don't see anything besides lots of water and the small island where the ferry has docked.</p>
<p>As you try to figure out what to do next, you notice a poster on a wall near the ferry dock. "Boat races! Open to the public! Grand prize is an all-expenses-paid trip to <b>Desert Island</b>!" That must be where the sand comes from! Best of all, the boat races are starting in just a few minutes.</p>
<p>You manage to sign up as a competitor in the boat races just in time. The organizer explains that it's not really a traditional race - instead, you will get a fixed amount of time during which your boat has to travel as far as it can, and you win if your boat goes the farthest.</p>
<p>As part of signing up, you get a sheet of paper (your puzzle input) that lists the <b>time</b> allowed for each race and also the best <b>distance</b> ever recorded in that race. To guarantee you win the grand prize, you need to make sure you <b>go farther in each race</b> than the current record holder.</p>
<p>The organizer brings you over to the area where the boat races are held. The boats are much smaller than you expected - they're actually <b>toy boats</b>, each with a big button on top. Holding down the button <b>charges the boat</b>, and releasing the button <b>allows the boat to move</b>. Boats move faster if their button was held longer, but time spent holding the button counts against the total race time. You can only hold the button at the start of the race, and boats don't move until the button is released.</p>
<p>For example:</p>
<pre><code>Time:      7  15   30
Distance:  9  40  200
</code></pre>
<p>This document describes three races:</p>
<ul>
<li>The first race lasts 7 milliseconds. The record distance in this race is 9 millimeters.</li>
<li>The second race lasts 15 milliseconds. The record distance in this race is 40 millimeters.</li>
<li>The third race lasts 30 milliseconds. The record distance in this race is 200 millimeters.</li>
</ul>
<p>Your toy boat has a starting speed of <b>zero millimeters per millisecond</b>. For each whole millisecond you spend at the beginning of the race holding down the button, the boat's speed increases by <b>one millimeter per millisecond</b>.</p>
<p>So, because the first race lasts 7 milliseconds, you only have a few options:</p>
<ul>
<li>Don't hold the button at all (that is, hold it for <b><code>0</code> milliseconds</b>) at the start of the race. The boat won't move; it will have traveled <b><code>0</code> millimeters</b> by the end of the race.</li>
<li>Hold the button for <b><code>1</code> millisecond</b> at the start of the race. Then, the boat will travel at a speed of <code>1</code> millimeter per millisecond for 6 milliseconds, reaching a total distance traveled of <b><code>6</code> millimeters</b>.</li>
<li>Hold the button for <b><code>2</code> milliseconds</b>, giving the boat a speed of <code>2</code> millimeters per millisecond. It will then get 5 milliseconds to move, reaching a total distance of <b><code>10</code> millimeters</b>.</li>
<li>Hold the button for <b><code>3</code> milliseconds</b>. After its remaining 4 milliseconds of travel time, the boat will have gone <b><code>12</code> millimeters</b>.</li>
<li>Hold the button for <b><code>4</code> milliseconds</b>. After its remaining 3 milliseconds of travel time, the boat will have gone <b><code>12</code> millimeters</b>.</li>
<li>Hold the button for <b><code>5</code> milliseconds</b>, causing the boat to travel a total of <b><code>10</code> millimeters</b>.</li>
<li>Hold the button for <b><code>6</code> milliseconds</b>, causing the boat to travel a total of <b><code>6</code> millimeters</b>.</li>
<li>Hold the button for <b><code>7</code> milliseconds</b>. That's the entire duration of the race. You never let go of the button. The boat can't move until you let go of the button. Please make sure you let go of the button so the boat gets to move. <b><code>0</code> millimeters</b>.</li>
</ul>
<p>Since the current record for this race is <code>9</code> millimeters, there are actually <code><b>4</b></code> different ways you could win: you could hold the button for <code>2</code>, <code>3</code>, <code>4</code>, or <code>5</code> milliseconds at the start of the race.</p>
<p>In the second race, you could hold the button for at least <code>4</code> milliseconds and at most <code>11</code> milliseconds and beat the record, a total of <code><b>8</b></code> different ways to win.</p>
<p>In the third race, you could hold the button for at least <code>11</code> milliseconds and no more than <code>19</code> milliseconds and still beat the record, a total of <code><b>9</b></code> ways you could win.</p>
<p>To see how much margin of error you have, determine the <b>number of ways you can beat the record</b> in each race; in this example, if you multiply these values together, you get <code><b>288</b></code> (<code>4</code> * <code>8</code> * <code>9</code>).</p>
<p>Determine the number of ways you could beat the record in each race. <b>What do you get if you multiply these numbers together?</b></p>

<h2 id="part2">--- Part Two ---</h2><p>As the race is about to start, you realize the piece of paper with race times and record distances you got earlier actually just has <span title="Keming!">very bad</span> <a target="_blank" href="https://en.wikipedia.org/wiki/Kerning">kerning</a>. There's really <b>only one race</b> - ignore the spaces between the numbers on each line.</p>
<p>So, the example from before:</p>
<pre><code>Time:      7  15   30
Distance:  9  40  200
</code></pre>
<p>...now instead means this:</p>
<pre><code>Time:      71530
Distance:  940200
</code></pre>
<p>Now, you have to figure out how many ways there are to win this single race. In this example, the race lasts for <b><code>71530</code> milliseconds</b> and the record distance you need to beat is <b><code>940200</code> millimeters</b>. You could hold the button anywhere from <code>14</code> to <code>71516</code> milliseconds and beat the record, a total of <code><b>71503</b></code> ways!</p>
<p><b>How many ways can you beat the record in this one much longer race?</b></p>

