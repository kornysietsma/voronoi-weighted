# Weighted Voronoi Map - work in progress

This is *NOT* working code!  It's a tiny start on a project, and probably abandoned for a while - I'm sharing it to discuss with others.

This was going to be a rust port of https://github.com/Kcnarf/d3-voronoi-treemap and https://github.com/Kcnarf/d3-voronoi-map which is itself a port of the Java original https://github.com/ArlindNocaj/power-voronoi-diagram

The algorithm is in the paper [Computing Voronoi Treemaps: Faster, Simpler, and Resolution‐independent by Arlind Nocaj and Ulrik Brandes](https://onlinelibrary.wiley.com/doi/abs/10.1111/j.1467-8659.2012.03078.x)

I have only moved a few core data structures across, before realising that the whole Convex Hull / Conflict List stuff is going to be quite hard to port to a language with strong checking of mutability and ownership.

There may be rust approaches I could take from [spade](https://github.com/Stoeoef/spade) or [delaunator](https://docs.rs/delaunator/0.2.0/delaunator/) but I just don't have time - for now I'm going to calculate voronoi diagrams using Kcnarf's code, offline, and just cope with the slowness.

License - there isn't enough here to warrant a license.  It'd need to be something compatible with the licenses of the original code, unless the port is such a radical rewrite that it isn't necessary.

- Korny