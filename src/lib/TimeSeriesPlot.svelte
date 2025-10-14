<script>
    import * as d3 from 'd3';

	/** @type {{ data: [number, number][]; width?: number; height?: number }} */
    let { data, width = 320, height = 150 } = $props();

	/** @type {HTMLElement} */
    let container;

    $effect(() => {
        if (container && data && data.length > 0) {
            const margin = { top: 20, right: 20, bottom: 30, left: 40 };
            const innerWidth = width - margin.left - margin.right;
            const innerHeight = height - margin.top - margin.bottom;

            // Clean up previous SVG
            d3.select(container).select("svg").remove();

            const svg = d3.select(container)
                .append("svg")
                .attr("width", width)
                .attr("height", height)
                .append("g")
                .attr("transform", `translate(${margin.left}, ${margin.top})`);

            // X scale
            const x = d3.scaleLinear()
                .domain(d3.extent(data, (/** @type {[number, number]} */ d) => d[0]))
                .range([0, innerWidth]);
            svg.append("g")
                .attr("transform", `translate(0, ${innerHeight})`)
                .call(d3.axisBottom(x).ticks(5));

            // Y scale
            const y = d3.scaleLinear()
                .domain(d3.extent(data, (/** @type {[number, number]} */ d) => d[1]))
                .range([innerHeight, 0]);
            svg.append("g").call(d3.axisLeft(y).ticks(5));

            // Line generator
            const line = d3.line()
                .x((/** @type {[number, number]} */ d) => x(d[0]))
                .y((/** @type {[number, number]} */ d) => y(d[1]));

            // Draw the line
            svg.append("path")
                .datum(data)
                .attr("fill", "none")
                .attr("stroke", "steelblue")
                .attr("stroke-width", 1.5)
                .attr("d", line);

            // Title
            svg.append("text")
                .attr("x", innerWidth / 2)
                .attr("y", -5)
                .attr("text-anchor", "middle")
                .style("font-size", "12px")
                .text("Cumulative Sum");
        }
    });
</script>

<div bind:this={container}></div>