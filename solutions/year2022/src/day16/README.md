view the original on <a href=https://adventofcode.com/2022/day/16>adventofcode.com</a>
<h2>--- Day 16: Proboscidea Volcanium ---</h2><p>The sensors have led you to the origin of the distress signal: yet another handheld device, just like the one the Elves gave you. However, you don't see any Elves around; instead, the device is surrounded by elephants! They must have gotten lost in these tunnels, and one of the elephants apparently figured out how to turn on the distress signal.</p>
<p>The ground rumbles again, much stronger this time. What kind of cave is this, exactly? You scan the cave with your handheld device; it reports mostly igneous rock, some ash, pockets of pressurized gas, magma... this isn't just a cave, it's a volcano!</p>
<p>You need to get the elephants out of here, quickly. Your device estimates that you have <b>30 minutes</b> before the volcano erupts, so you don't have time to go back out the way you came in.</p>
<p>You scan the cave for other options and discover a network of pipes and pressure-release <b>valves</b>. You aren't sure how such a system got into a volcano, but you don't have time to complain; your device produces a report (your puzzle input) of each valve's <b>flow rate</b> if it were opened (in pressure per minute) and the tunnels you could use to move between the valves.</p>
<p>There's even a valve in the room you and the elephants are currently standing in labeled <code>AA</code>. You estimate it will take you one minute to open a single valve and one minute to follow any tunnel from one valve to another. What is the most pressure you could release?</p>
<p>For example, suppose you had the following scan output:</p>
<pre><code>Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
</code></pre>
<p>All of the valves begin <b>closed</b>. You start at valve <code>AA</code>, but it must be damaged or <span title="Wait, sir! The valve, sir! it appears to be... jammed!">jammed</span> or something: its flow rate is <code>0</code>, so there's no point in opening it. However, you could spend one minute moving to valve <code>BB</code> and another minute opening it; doing so would release pressure during the remaining <b>28 minutes</b> at a flow rate of <code>13</code>, a total eventual pressure release of <code>28 * 13 = <b>364</b></code>. Then, you could spend your third minute moving to valve <code>CC</code> and your fourth minute opening it, providing an additional <b>26 minutes</b> of eventual pressure release at a flow rate of <code>2</code>, or <code><b>52</b></code> total pressure released by valve <code>CC</code>.</p>
<p>Making your way through the tunnels like this, you could probably open many or all of the valves by the time 30 minutes have elapsed. However, you need to release as much pressure as possible, so you'll need to be methodical. Instead, consider this approach:</p>
<pre><code>== Minute 1 ==
No valves are open.
You move to valve DD.

== Minute 2 ==
No valves are open.
You open valve DD.

== Minute 3 ==
Valve DD is open, releasing <b>20</b> pressure.
You move to valve CC.

== Minute 4 ==
Valve DD is open, releasing <b>20</b> pressure.
You move to valve BB.

== Minute 5 ==
Valve DD is open, releasing <b>20</b> pressure.
You open valve BB.

== Minute 6 ==
Valves BB and DD are open, releasing <b>33</b> pressure.
You move to valve AA.

== Minute 7 ==
Valves BB and DD are open, releasing <b>33</b> pressure.
You move to valve II.

== Minute 8 ==
Valves BB and DD are open, releasing <b>33</b> pressure.
You move to valve JJ.

== Minute 9 ==
Valves BB and DD are open, releasing <b>33</b> pressure.
You open valve JJ.

== Minute 10 ==
Valves BB, DD, and JJ are open, releasing <b>54</b> pressure.
You move to valve II.

== Minute 11 ==
Valves BB, DD, and JJ are open, releasing <b>54</b> pressure.
You move to valve AA.

== Minute 12 ==
Valves BB, DD, and JJ are open, releasing <b>54</b> pressure.
You move to valve DD.

== Minute 13 ==
Valves BB, DD, and JJ are open, releasing <b>54</b> pressure.
You move to valve EE.

== Minute 14 ==
Valves BB, DD, and JJ are open, releasing <b>54</b> pressure.
You move to valve FF.

== Minute 15 ==
Valves BB, DD, and JJ are open, releasing <b>54</b> pressure.
You move to valve GG.

== Minute 16 ==
Valves BB, DD, and JJ are open, releasing <b>54</b> pressure.
You move to valve HH.

== Minute 17 ==
Valves BB, DD, and JJ are open, releasing <b>54</b> pressure.
You open valve HH.

== Minute 18 ==
Valves BB, DD, HH, and JJ are open, releasing <b>76</b> pressure.
You move to valve GG.

== Minute 19 ==
Valves BB, DD, HH, and JJ are open, releasing <b>76</b> pressure.
You move to valve FF.

== Minute 20 ==
Valves BB, DD, HH, and JJ are open, releasing <b>76</b> pressure.
You move to valve EE.

== Minute 21 ==
Valves BB, DD, HH, and JJ are open, releasing <b>76</b> pressure.
You open valve EE.

== Minute 22 ==
Valves BB, DD, EE, HH, and JJ are open, releasing <b>79</b> pressure.
You move to valve DD.

== Minute 23 ==
Valves BB, DD, EE, HH, and JJ are open, releasing <b>79</b> pressure.
You move to valve CC.

== Minute 24 ==
Valves BB, DD, EE, HH, and JJ are open, releasing <b>79</b> pressure.
You open valve CC.

== Minute 25 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing <b>81</b> pressure.

== Minute 26 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing <b>81</b> pressure.

== Minute 27 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing <b>81</b> pressure.

== Minute 28 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing <b>81</b> pressure.

== Minute 29 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing <b>81</b> pressure.

== Minute 30 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing <b>81</b> pressure.
</code></pre>
<p>This approach lets you release the most pressure possible in 30 minutes with this valve layout, <code><b>1651</b></code>.</p>
<p>Work out the steps to release the most pressure in 30 minutes. <b>What is the most pressure you can release?</b></p>
<h2 id="part2">--- Part Two ---</h2><p>You're worried that even with an optimal approach, the pressure released won't be enough. What if you got one of the elephants to help you?</p>
<p>It would take you 4 minutes to teach an elephant how to open the right valves in the right order, leaving you with only <b>26 minutes</b> to actually execute your plan. Would having two of you working together be better, even if it means having less time? (Assume that you teach the elephant before opening any valves yourself, giving you both the same full 26 minutes.)</p>
<p>In the example above, you could teach the elephant to help you as follows:</p>
<pre><code>== Minute 1 ==
No valves are open.
You move to valve II.
The elephant moves to valve DD.

== Minute 2 ==
No valves are open.
You move to valve JJ.
The elephant opens valve DD.

== Minute 3 ==
Valve DD is open, releasing <b>20</b> pressure.
You open valve JJ.
The elephant moves to valve EE.

== Minute 4 ==
Valves DD and JJ are open, releasing <b>41</b> pressure.
You move to valve II.
The elephant moves to valve FF.

== Minute 5 ==
Valves DD and JJ are open, releasing <b>41</b> pressure.
You move to valve AA.
The elephant moves to valve GG.

== Minute 6 ==
Valves DD and JJ are open, releasing <b>41</b> pressure.
You move to valve BB.
The elephant moves to valve HH.

== Minute 7 ==
Valves DD and JJ are open, releasing <b>41</b> pressure.
You open valve BB.
The elephant opens valve HH.

== Minute 8 ==
Valves BB, DD, HH, and JJ are open, releasing <b>76</b> pressure.
You move to valve CC.
The elephant moves to valve GG.

== Minute 9 ==
Valves BB, DD, HH, and JJ are open, releasing <b>76</b> pressure.
You open valve CC.
The elephant moves to valve FF.

== Minute 10 ==
Valves BB, CC, DD, HH, and JJ are open, releasing <b>78</b> pressure.
The elephant moves to valve EE.

== Minute 11 ==
Valves BB, CC, DD, HH, and JJ are open, releasing <b>78</b> pressure.
The elephant opens valve EE.

(At this point, all valves are open.)

== Minute 12 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing <b>81</b> pressure.

...

== Minute 20 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing <b>81</b> pressure.

...

== Minute 26 ==
Valves BB, CC, DD, EE, HH, and JJ are open, releasing <b>81</b> pressure.
</code></pre>
<p>With the elephant helping, after 26 minutes, the best you could do would release a total of <code><b>1707</b></code> pressure.</p>
<p><b>With you and an elephant working together for 26 minutes, what is the most pressure you could release?</b></p>

