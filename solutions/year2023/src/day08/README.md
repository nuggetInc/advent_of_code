view the original on <a href=https://adventofcode.com/2023/day/8>adventofcode.com</a>
<h2>--- Day 8: Haunted Wasteland ---</h2><p>You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about <b>ghosts</b> a few minutes ago.</p>
<p>One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of <b>network</b> of labeled nodes.</p>
<p>It seems like you're meant to use the <b>left/right</b> instructions to <b>navigate the network</b>. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!</p>
<p>After examining the maps for a bit, two nodes stick out: <code>AAA</code> and <code>ZZZ</code>. You feel like <code>AAA</code> is where you are now, and you have to follow the left/right instructions until you reach <code>ZZZ</code>.</p>
<p>This format defines each <b>node</b> of the network individually. For example:</p>
<pre><code>RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
</code></pre>
<p>Starting with <code>AAA</code>, you need to <b>look up the next element</b> based on the next left/right instruction in your input. In this example, start with <code>AAA</code> and go <b>right</b> (<code>R</code>) by choosing the right element of <code>AAA</code>, <code><b>CCC</b></code>. Then, <code>L</code> means to choose the <b>left</b> element of <code>CCC</code>, <code><b>ZZZ</b></code>. By following the left/right instructions, you reach <code>ZZZ</code> in <code><b>2</b></code> steps.</p>
<p>Of course, you might not find <code>ZZZ</code> right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: <code>RL</code> really means <code>RLRLRLRLRLRLRLRL...</code> and so on. For example, here is a situation that takes <code><b>6</b></code> steps to reach <code>ZZZ</code>:</p>
<pre><code>LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
</code></pre>
<p>Starting at <code>AAA</code>, follow the left/right instructions. <b>How many steps are required to reach <code>ZZZ</code>?</b></p>

<h2 id="part2">--- Part Two ---</h2><p>The <span title="Duhduhduhduhduh! Dah, duhduhduhduhduh!">sandstorm</span> is upon you and you aren't any closer to escaping the wasteland. You had the camel follow the instructions, but you've barely left your starting position. It's going to take <b>significantly more steps</b> to escape!</p>
<p>What if the map isn't for people - what if the map is for <b>ghosts</b>? Are ghosts even bound by the laws of spacetime? Only one way to find out.</p>
<p>After examining the maps a bit longer, your attention is drawn to a curious fact: the number of nodes with names ending in <code>A</code> is equal to the number ending in <code>Z</code>! If you were a ghost, you'd probably just <b>start at every node that ends with <code>A</code></b> and follow all of the paths at the same time until they all simultaneously end up at nodes that end with <code>Z</code>.</p>
<p>For example:</p>
<pre><code>LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
</code></pre>
<p>Here, there are two starting nodes, <code>11A</code> and <code>22A</code> (because they both end with <code>A</code>). As you follow each left/right instruction, use that instruction to <b>simultaneously</b> navigate away from both nodes you're currently on. Repeat this process until <b>all</b> of the nodes you're currently on end with <code>Z</code>. (If only some of the nodes you're on end with <code>Z</code>, they act like any other node and you continue as normal.) In this example, you would proceed as follows:</p>
<ul>
<li>Step 0: You are at <code>11A</code> and <code>22A</code>.</li>
<li>Step 1: You choose all of the <b>left</b> paths, leading you to <code>11B</code> and <code>22B</code>.</li>
<li>Step 2: You choose all of the <b>right</b> paths, leading you to <code><b>11Z</b></code> and <code>22C</code>.</li>
<li>Step 3: You choose all of the <b>left</b> paths, leading you to <code>11B</code> and <code><b>22Z</b></code>.</li>
<li>Step 4: You choose all of the <b>right</b> paths, leading you to <code><b>11Z</b></code> and <code>22B</code>.</li>
<li>Step 5: You choose all of the <b>left</b> paths, leading you to <code>11B</code> and <code>22C</code>.</li>
<li>Step 6: You choose all of the <b>right</b> paths, leading you to <code><b>11Z</b></code> and <code><b>22Z</b></code>.</li>
</ul>
<p>So, in this example, you end up entirely on nodes that end in <code>Z</code> after <code><b>6</b></code> steps.</p>
<p>Simultaneously start on every node that ends with <code>A</code>. <b>How many steps does it take before you're only on nodes that end with <code>Z</code>?</b></p>

