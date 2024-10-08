view the original on <a href=https://adventofcode.com/2023/day/20>adventofcode.com</a>
<h2>--- Day 20: Pulse Propagation ---</h2><p>With your help, the Elves manage to find the right parts and fix all of the machines. Now, they just need to send the command to boot up the machines and get the sand flowing again.</p>
<p>The machines are far apart and wired together with long <b>cables</b>. The cables don't connect to the machines directly, but rather to communication <b>modules</b> attached to the machines that perform various initialization tasks and also act as communication relays.</p>
<p>Modules communicate using <b>pulses</b>. Each pulse is either a <b>high pulse</b> or a <b>low pulse</b>. When a module sends a pulse, it sends that type of pulse to each module in its list of <b>destination modules</b>.</p>
<p>There are several different types of modules:</p>
<p><b>Flip-flop</b> modules (prefix <code>%</code>) are either <b>on</b> or <b>off</b>; they are initially <b>off</b>. If a flip-flop module receives a high pulse, it is ignored and nothing happens. However, if a flip-flop module receives a low pulse, it <b>flips between on and off</b>. If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.</p>
<p><b>Conjunction</b> modules (prefix <code>&amp;</code>) <b>remember</b> the type of the most recent pulse received from <b>each</b> of their connected input modules; they initially default to remembering a <b>low pulse</b> for each input. When a pulse is received, the conjunction module first updates its memory for that input. Then, if it remembers <b>high pulses</b> for all inputs, it sends a <b>low pulse</b>; otherwise, it sends a <b>high pulse</b>.</p>
<p>There is a single <b>broadcast module</b> (named <code>broadcaster</code>). When it receives a pulse, it sends the same pulse to all of its destination modules.</p>
<p>Here at Desert Machine Headquarters, there is a module with a single button on it called, aptly, the <b>button module</b>. When you push the button, a single <b>low pulse</b> is sent directly to the <code>broadcaster</code> module.</p>
<p>After pushing the button, you must wait until all pulses have been delivered and fully handled before pushing it again. Never push the button if modules are still processing pulses.</p>
<p>Pulses are always processed <b>in the order they are sent</b>. So, if a pulse is sent to modules <code>a</code>, <code>b</code>, and <code>c</code>, and then module <code>a</code> processes its pulse and sends more pulses, the pulses sent to modules <code>b</code> and <code>c</code> would have to be handled first.</p>
<p>The module configuration (your puzzle input) lists each module. The name of the module is preceded by a symbol identifying its type, if any. The name is then followed by an arrow and a list of its destination modules. For example:</p>
<pre><code>broadcaster -&gt; a, b, c
%a -&gt; b
%b -&gt; c
%c -&gt; inv
&amp;inv -&gt; a
</code></pre>
<p>In this module configuration, the broadcaster has three destination modules named <code>a</code>, <code>b</code>, and <code>c</code>. Each of these modules is a flip-flop module (as indicated by the <code>%</code> prefix). <code>a</code> outputs to <code>b</code> which outputs to <code>c</code> which outputs to another module named <code>inv</code>. <code>inv</code> is a conjunction module (as indicated by the <code>&amp;</code> prefix) which, because it has only one input, acts like an <span title="This puzzle originally had a separate inverter module type until I realized it was just a worse conjunction module.">inverter</span> (it sends the opposite of the pulse type it receives); it outputs to <code>a</code>.</p>
<p>By pushing the button once, the following pulses are sent:</p>
<pre><code>button -low-&gt; broadcaster
broadcaster -low-&gt; a
broadcaster -low-&gt; b
broadcaster -low-&gt; c
a -high-&gt; b
b -high-&gt; c
c -high-&gt; inv
inv -low-&gt; a
a -low-&gt; b
b -low-&gt; c
c -low-&gt; inv
inv -high-&gt; a
</code></pre>
<p>After this sequence, the flip-flop modules all end up <b>off</b>, so pushing the button again repeats the same sequence.</p>
<p>Here's a more interesting example:</p>
<pre><code>broadcaster -&gt; a
%a -&gt; inv, con
&amp;inv -&gt; b
%b -&gt; con
&amp;con -&gt; output
</code></pre>
<p>This module configuration includes the <code>broadcaster</code>, two flip-flops (named <code>a</code> and <code>b</code>), a single-input conjunction module (<code>inv</code>), a multi-input conjunction module (<code>con</code>), and an untyped module named <code>output</code> (for testing purposes). The multi-input conjunction module <code>con</code> watches the two flip-flop modules and, if they're both on, sends a <b>low pulse</b> to the <code>output</code> module.</p>
<p>Here's what happens if you push the button once:</p>
<pre><code>button -low-&gt; broadcaster
broadcaster -low-&gt; a
a -high-&gt; inv
a -high-&gt; con
inv -low-&gt; b
con -high-&gt; output
b -high-&gt; con
con -low-&gt; output
</code></pre>
<p>Both flip-flops turn on and a low pulse is sent to <code>output</code>! However, now that both flip-flops are on and <code>con</code> remembers a high pulse from each of its two inputs, pushing the button a second time does something different:</p>
<pre><code>button -low-&gt; broadcaster
broadcaster -low-&gt; a
a -low-&gt; inv
a -low-&gt; con
inv -high-&gt; b
con -high-&gt; output
</code></pre>
<p>Flip-flop <code>a</code> turns off! Now, <code>con</code> remembers a low pulse from module <code>a</code>, and so it sends only a high pulse to <code>output</code>.</p>
<p>Push the button a third time:</p>
<pre><code>button -low-&gt; broadcaster
broadcaster -low-&gt; a
a -high-&gt; inv
a -high-&gt; con
inv -low-&gt; b
con -low-&gt; output
b -low-&gt; con
con -high-&gt; output
</code></pre>
<p>This time, flip-flop <code>a</code> turns on, then flip-flop <code>b</code> turns off. However, before <code>b</code> can turn off, the pulse sent to <code>con</code> is handled first, so it <b>briefly remembers all high pulses</b> for its inputs and sends a low pulse to <code>output</code>. After that, flip-flop <code>b</code> turns off, which causes <code>con</code> to update its state and send a high pulse to <code>output</code>.</p>
<p>Finally, with <code>a</code> on and <code>b</code> off, push the button a fourth time:</p>
<pre><code>button -low-&gt; broadcaster
broadcaster -low-&gt; a
a -low-&gt; inv
a -low-&gt; con
inv -high-&gt; b
con -high-&gt; output
</code></pre>
<p>This completes the cycle: <code>a</code> turns off, causing <code>con</code> to remember only low pulses and restoring all modules to their original states.</p>
<p>To get the cables warmed up, the Elves have pushed the button <code>1000</code> times. How many pulses got sent as a result (including the pulses sent by the button itself)?</p>
<p>In the first example, the same thing happens every time the button is pushed: <code>8</code> low pulses and <code>4</code> high pulses are sent. So, after pushing the button <code>1000</code> times, <code>8000</code> low pulses and <code>4000</code> high pulses are sent. Multiplying these together gives <code><b>32000000</b></code>.</p>
<p>In the second example, after pushing the button <code>1000</code> times, <code>4250</code> low pulses and <code>2750</code> high pulses are sent. Multiplying these together gives <code><b>11687500</b></code>.</p>
<p>Consult your module configuration; determine the number of low pulses and high pulses that would be sent after pushing the button <code>1000</code> times, waiting for all pulses to be fully handled after each push of the button. <b>What do you get if you multiply the total number of low pulses sent by the total number of high pulses sent?</b></p>
<h2 id="part2">--- Part Two ---</h2><p>The final machine responsible for moving the sand down to Island Island has a module attached named <code>rx</code>. The machine turns on when a <b>single low pulse</b> is sent to <code>rx</code>.</p>
<p>Reset all modules to their default states. Waiting for all pulses to be fully handled after each button press, <b>what is the fewest number of button presses required to deliver a single low pulse to the module named <code>rx</code>?</b></p>

