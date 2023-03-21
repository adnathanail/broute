https://osmcode.org/osmium-tool/manual.html

## Convert .osm.pbf to .osm
```
osmium cat leicestershire-latest.osm.pbf -o leicestershire-latest.osm
```

## Cut out a geographic region
```
osmium extract -b -1.263095,52.769673,-1.224116,52.755050 leicestershire-latest.osm -o theoffice-latest.osm
```

## Links
https://towardsdatascience.com/connecting-datapoints-to-a-road-graph-with-python-efficiently-cb8c6795ad5f
https://github.com/gboeing/osmnx
https://wiki.openstreetmap.org/wiki/Overpass_API
https://help.openstreetmap.org/questions/42740/how-to-maintain-a-local-copy-of-a-specific-area-of-the-osm-database
