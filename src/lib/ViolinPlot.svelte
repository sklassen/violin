<script>
    import * as d3 from 'd3';

    let { data, width = 200, height = 400, title = "Violin Plot" } = $props();
    let container;

    $effect(() => {
        if (container && data) {
            const margin = { top: 30, right: 30, bottom: 30, left: 40 };
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

            // Y scale
            const y = d3.scaleLinear()
                .domain([data.min, data.max])
                .range([innerHeight, 0]);
            svg.append("g").call(d3.axisLeft(y));

            // X scale for the violin plot width
            const xMax = d3.max(data.kde_points, d => d[1]);
            const x = d3.scaleLinear()
                .domain([-xMax, xMax])
                .range([0, innerWidth]);

            // Area generator for the violin shape
            const area = d3.area()
                .x0(d => x(-d[1]))
                .x1(d => x(d[1]))
                .y(d => y(d[0]))
                .curve(d3.curveCatmullRom);

            // Draw the violin area
            svg.append("path")
                .datum(data.kde_points)
                .attr("d", area)
                .style("fill", "#69b3a2")
                .style("stroke", "black");

            // Center line for the box plot
            const boxWidth = 20;
            const center = innerWidth / 2;

            // Main vertical line for whiskers
            svg.append("line")
                .attr("x1", center)
                .attr("x2", center)
                .attr("y1", y(data.lower_whisker))
                .attr("y2", y(data.upper_whisker))
                .attr("stroke", "black");

            // Box for Q1 to Q3
            svg.append("rect")
                .attr("x", center - boxWidth / 2)
                .attr("y", y(data.q3))
                .attr("height", y(data.q1) - y(data.q3))
                .attr("width", boxWidth)
                .attr("stroke", "black")
                .style("fill", "#E0E0E0");

            // Median line
            svg.append("line")
                .attr("class", "median-line")
                .attr("x1", center - boxWidth / 2)
                .attr("x2", center + boxWidth / 2)
                .attr("y1", y(data.median))
                .attr("y2", y(data.median))
                .attr("stroke", "black")
                .style("stroke-width", "2px");

            // Title
            svg.append("text")
                .attr("x", innerWidth / 2)
                .attr("y", -10)
                .attr("text-anchor", "middle")
                .style("font-size", "16px")
                .text(title);
        }
    });
</script>

<div bind:this={container}></div>