https://osmcode.org/osmium-tool/manual.html

## Convert .osm.pbf to .osm
```
osmium cat leicestershire-latest.osm.pbf -o leicestershire-latest.osm
```

## Cut out a geographic region
```
osmium extract -b -1.263095,52.769673,-1.224116,52.755050 leicestershire-latest.osm -o theoffice-latest.osm
```