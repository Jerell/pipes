# pipe bathymetry yaml generator

reads x and y coordinates, turns them into lengths, writes .yml

splits lengths into sections less than the maximum length

currently doesn't split elevation as it should. when a pipe above the maximum length is split into several small ones, all will have the elevation of the first, rather than gradually changing to move to the elevation of the destination
