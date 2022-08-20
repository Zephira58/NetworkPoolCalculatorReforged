window.SIDEBAR_ITEMS = {"enum":[["Corner","Where to place the plot legend."],["LineStyle",""],["MarkerShape",""],["Orientation","Determines whether a plot element is vertically or horizontally oriented."]],"fn":[["log_grid_spacer","Recursively splits the grid into `base` subdivisions (e.g. 100, 10, 1)."],["uniform_grid_spacer","Splits the grid into uniform-sized spacings (e.g. 100, 25, 1)."]],"struct":[["Arrows","A set of arrows."],["Bar","One bar in a [`BarChart`]. Potentially floating, allowing stacked bar charts. Width can be changed to allow variable-width histograms."],["BarChart","A bar chart."],["BoxElem","A box in a [`BoxPlot`] diagram. This is a low level graphical element; it will not compute quartiles and whiskers, letting one use their preferred formula. Use [`Points`][`super::Points`] to draw the outliers."],["BoxPlot","A diagram containing a series of [`BoxElem`] elements."],["BoxSpread","Contains the values of a single box in a box plot."],["CoordinatesFormatter","Specifies the coordinates formatting when passed to [`Plot::coordinates_formatter`]."],["GridInput","Input for “grid spacer” functions."],["GridMark","One mark (horizontal or vertical line) in the background grid of a plot."],["HLine","A horizontal line in a plot, filling the full width"],["Legend","The configuration for a plot legend."],["Line","A series of values forming a path."],["LinkedAxisGroup","Defines how multiple plots share the same range for one or both of their axes. Can be added while building a plot with [`Plot::link_axis`]. Contains an internal state, meaning that this object should be stored by the user between frames."],["Plot","A 2D plot, e.g. a graph of a function."],["PlotBounds","2D bounding box of f64 precision. The range of data values we show."],["PlotImage","An image in the plot."],["PlotUi","Provides methods to interact with a plot while building it. It is the single argument of the closure provided to [`Plot::show`]. See [`Plot`] for an example of how to use it."],["Points","A set of points."],["Polygon","A convex polygon."],["Text","Text inside the plot."],["VLine","A vertical line in a plot, filling the full width"],["Value","A value in the value-space of the plot."],["Values",""]]};