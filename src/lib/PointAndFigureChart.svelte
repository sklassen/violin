<script>
    import * as d3 from 'd3';

	/** @type {{ data: { from: number; to: number; direction: 'Up' | 'Down', start_time: number, end_time: number }[]; boxSize: number; width?: number; height?: number; predictedNextBar?: { from: number; to: number; direction: 'Up' | 'Down' } | null; title?: string }} */
    let { data, boxSize = 1.0, width = 320, height = 200, predictedNextBar = null, title = "Point & Figure Chart" } = $props();

	/** @type {HTMLElement} */
    let container;

    $effect(() => {
        if (container && data && data.length > 0 && boxSize > 0) {
            const margin = { top: 20, right: 20, bottom: 30, left: 40 };
            const innerWidth = width - margin.left - margin.right;
            const innerHeight = height - margin.top - margin.bottom;

            d3.select(container).select("svg").remove();

            const svg = d3.select(container)
                .append("svg")
                .attr("width", width)
                .attr("height", height)
                .append("g")
                .attr("transform", `translate(${margin.left}, ${margin.top})`);

            const allValues = data.flatMap((/** @type {{ from: number; to: number; }} */ d) => [d.from, d.to]);
            const yMin = Math.floor(d3.min(allValues) / boxSize) * boxSize;
            const yMax = Math.ceil(d3.max(allValues) / boxSize) * boxSize;

            const y = d3.scaleLinear()
                .domain([yMin, yMax])
                .range([innerHeight, 0]);

            const x = d3.scaleBand()
                .domain(d3.range(data.length))
                .range([0, innerWidth])
                .padding(0.1);

            const tooltip = d3.select(container)
                .append("div")
                .style("opacity", 0)
                .attr("class", "tooltip")
                .style("background-color", "white")
                .style("border", "solid")
                .style("border-width", "1px")
                .style("border-radius", "5px")
                .style("padding", "10px")
                .style("position", "absolute");

            // Add Y-axis grid lines for better readability
            svg.append("g")
                .attr("class", "grid")
                .call(d3.axisLeft(y)
                    .ticks(10)
                    .tickSize(-innerWidth)
                    .tickFormat("")
                )
                .selectAll("line")
                .attr("stroke-opacity", 0.1);

            svg.append("g").call(d3.axisLeft(y).ticks(10));
            svg.append("g")
               .attr("transform", `translate(0, ${innerHeight})`)
               .call(d3.axisBottom(x).tickFormat((/** @type {any} */ i) => i + 1));

            const columnWidth = x.bandwidth();
            const symbolSize = Math.min(columnWidth, Math.abs(y(yMax) - y(yMax + boxSize))) * 0.7;
            const symbolRadius = symbolSize / 2;

            data.forEach((col, i) => {
                const start = Math.min(col.from, col.to);
                const end = Math.max(col.from, col.to);
                const numBoxes = Math.floor(Math.abs(end - start) / boxSize);

                const columnGroup = svg.append("g")
                    .on("mouseover", function(/** @type {any} */ event) {
                        tooltip.transition()
                            .duration(200)
                            .style("opacity", .9);
                        tooltip.html(`Start: ${col.start_time}<br/>End: ${col.end_time}`)
                            .style("left", (event.pageX) + "px")
                            .style("top", (event.pageY - 28) + "px");
                    })
                    .on("mouseout", function() {
                        tooltip.transition()
                            .duration(500)
                            .style("opacity", 0);
                    });

                for (let j = 0; j <= numBoxes; j++) {
                    const yLevel = col.direction === 'Up'
                        ? start + j * boxSize
                        : end - j * boxSize;

                    const cx = x(i) + columnWidth / 2;
                    const cy = y(yLevel);

                    if (col.direction === 'Up') {
                        // Draw 'X' with two lines
                        columnGroup.append("line")
                           .attr("x1", cx - symbolRadius)
                           .attr("y1", cy - symbolRadius)
                           .attr("x2", cx + symbolRadius)
                           .attr("y2", cy + symbolRadius)
                           .attr("stroke", "green")
                           .attr("stroke-width", 1.5);
                        columnGroup.append("line")
                           .attr("x1", cx - symbolRadius)
                           .attr("y1", cy + symbolRadius)
                           .attr("x2", cx + symbolRadius)
                           .attr("y2", cy - symbolRadius)
                           .attr("stroke", "green")
                           .attr("stroke-width", 1.5);
                    } else {
                        // Draw 'O' with a circle
                        columnGroup.append("circle")
                           .attr("cx", cx)
                           .attr("cy", cy)
                           .attr("r", symbolRadius)
                           .attr("stroke", "red")
                           .attr("stroke-width", 1.5)
                           .attr("fill", "none");
                    }
                }
            });

            // Title
            svg.append("text")
                .attr("x", innerWidth / 2)
                .attr("y", -5)
                .attr("text-anchor", "middle")
                .style("font-size", "12px")
                .text(title);

            // Draw the predicted next bar
            if (predictedNextBar) {
                const col = predictedNextBar;
                const i = data.length; // Place it in the next column
                 const start = Math.min(col.from, col.to);
                const end = Math.max(col.from, col.to);
                const numBoxes = Math.floor(Math.abs(end - start) / boxSize);

                for (let j = 0; j <= numBoxes; j++) {
                    const yLevel = col.direction === 'Up'
                        ? start + j * boxSize
                        : end - j * boxSize;

                    const cx = x(i) + columnWidth / 2;
                    const cy = y(yLevel);

                    if (col.direction === 'Up') {
                        svg.append("line")
                           .attr("x1", cx - symbolRadius)
                           .attr("y1", cy - symbolRadius)
                           .attr("x2", cx + symbolRadius)
                           .attr("y2", cy + symbolRadius)
                           .attr("stroke", "lightgreen")
                           .attr("stroke-width", 1.5);
                        svg.append("line")
                           .attr("x1", cx - symbolRadius)
                           .attr("y1", cy + symbolRadius)
                           .attr("x2", cx + symbolRadius)
                           .attr("y2", cy - symbolRadius)
                           .attr("stroke", "lightgreen")
                           .attr("stroke-width", 1.5);
                    } else {
                        svg.append("circle")
                           .attr("cx", cx)
                           .attr("cy", cy)
                           .attr("r", symbolRadius)
                           .attr("stroke", "lightcoral")
                           .attr("stroke-width", 1.5)
                           .attr("fill", "none");
                    }
                }
            }
        }
    });
</script>

<div bind:this={container}></div>