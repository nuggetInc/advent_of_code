view the original on <a href=https://adventofcode.com/2024/day/1>adventofcode.com</a>
<h2>--- Day 1: Historian Hysteria ---</h2><p>The <b>Chief Historian</b> is always present for the big Christmas sleigh launch, but nobody has seen him in months! Last anyone heard, he was visiting locations that are historically significant to the North Pole; a group of Senior Historians has asked you to accompany them as they check the places they think he was most likely to visit.</p>
<p>As each location is checked, they will mark it on their list with a <em class="star">star</b>. They figure the Chief Historian <b>must</b> be in one of the first fifty places they'll look, so in order to save Christmas, you need to help them get <em class="star">fifty stars</b> on their list before Santa takes off on December 25th.</p>
<p>Collect stars by solving puzzles.  Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first.  Each puzzle grants <em class="star">one star</b>. Good luck!</p>
<p>You haven't even left yet and the group of Elvish Senior Historians has already hit a problem: their list of locations to check is currently <b>empty</b>. Eventually, someone decides that the best place to check first would be the Chief Historian's office.</p>
<p>Upon pouring into the office, everyone confirms that the Chief Historian is indeed nowhere to be found. Instead, the Elves discover an assortment of notes and lists of historically significant locations! This seems to be the planning the Chief Historian was doing before he left. Perhaps these notes can be used to determine which locations to search?</p>
<p>Throughout the Chief's office, the historically significant locations are listed not by name but by a unique number called the <b>location ID</b>. To make sure they don't miss anything, The Historians split into two groups, each searching the office and trying to create their own complete list of location IDs.</p>
<p>There's just one problem: by holding the two lists up <b>side by side</b> (your puzzle input), it quickly becomes clear that the lists aren't very similar. Maybe you can help The Historians reconcile their lists?</p>
<p>For example:</p>
<pre><code>3   4
4   3
2   5
1   3
3   9
3   3
</code></pre>
<p>Maybe the lists are only off by a small amount! To find out, pair up the numbers and measure how far apart they are. Pair up the <b>smallest number in the left list</b> with the <b>smallest number in the right list</b>, then the <b>second-smallest left number</b> with the <b>second-smallest right number</b>, and so on.</p>
<p>Within each pair, figure out <b>how far apart</b> the two numbers are; you'll need to <b>add up all of those distances</b>. For example, if you pair up a <code>3</code> from the left list with a <code>7</code> from the right list, the distance apart is <code>4</code>; if you pair up a <code>9</code> with a <code>3</code>, the distance apart is <code>6</code>.</p>
<p>In the example list above, the pairs and distances would be as follows:</p>
<ul>
<li>The smallest number in the left list is <code>1</code>, and the smallest number in the right list is <code>3</code>. The distance between them is <code><b>2</b></code>.</li>
<li>The second-smallest number in the left list is <code>2</code>, and the second-smallest number in the right list is another <code>3</code>. The distance between them is <code><b>1</b></code>.</li>
<li>The third-smallest number in both lists is <code>3</code>, so the distance between them is <code><b>0</b></code>.</li>
<li>The next numbers to pair up are <code>3</code> and <code>4</code>, a distance of <code><b>1</b></code>.</li>
<li>The fifth-smallest numbers in each list are <code>3</code> and <code>5</code>, a distance of <code><b>2</b></code>.</li>
<li>Finally, the largest number in the left list is <code>4</code>, while the largest number in the right list is <code>9</code>; these are a distance <code><b>5</b></code> apart.</li>
</ul>
<p>To find the <b>total distance</b> between the left list and the right list, add up the distances between all of the pairs you found. In the example above, this is <code>2 + 1 + 0 + 1 + 2 + 5</code>, a total distance of <code><b>11</b></code>!</p>
<p>Your actual left and right lists contain many location IDs. <b>What is the total distance between your lists?</b></p>
<h2 id="part2">--- Part Two ---</h2><p>Your analysis only confirmed what everyone feared: the two lists of location IDs are indeed very different.</p>
<p>Or are they?</p>
<p>The Historians can't agree on which group made the mistakes <b>or</b> how to read most of the Chief's handwriting, but in the commotion you notice an interesting detail: <span title="We were THIS close to summoning the Alot of Location IDs!">a lot</span> of location IDs appear in both lists! Maybe the other numbers aren't location IDs at all but rather misinterpreted handwriting.</p>
<p>This time, you'll need to figure out exactly how often each number from the left list appears in the right list. Calculate a total <b>similarity score</b> by adding up each number in the left list after multiplying it by the number of times that number appears in the right list.</p>
<p>Here are the same example lists again:</p>
<pre><code>3   4
4   3
2   5
1   3
3   9
3   3
</code></pre>
<p>For these example lists, here is the process of finding the similarity score:</p>
<ul>
<li>The first number in the left list is <code>3</code>. It appears in the right list three times, so the similarity score increases by <code>3 * 3 = <b>9</b></code>.</li>
<li>The second number in the left list is <code>4</code>. It appears in the right list once, so the similarity score increases by <code>4 * 1 = <b>4</b></code>.</li>
<li>The third number in the left list is <code>2</code>. It does not appear in the right list, so the similarity score does not increase (<code>2 * 0 = 0</code>).</li>
<li>The fourth number, <code>1</code>, also does not appear in the right list.</li>
<li>The fifth number, <code>3</code>, appears in the right list three times; the similarity score increases by <code><b>9</b></code>.</li>
<li>The last number, <code>3</code>, appears in the right list three times; the similarity score again increases by <code><b>9</b></code>.</li>
</ul>
<p>So, for these example lists, the similarity score at the end of this process is <code><b>31</b></code> (<code>9 + 4 + 0 + 0 + 9 + 9</code>).</p>
<p>Once again consider your left and right lists. <b>What is their similarity score?</b></p>

